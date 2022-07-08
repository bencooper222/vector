use std::{any::Any, fmt, sync::Arc};

use anymap::AnyMap;
use diagnostic::{DiagnosticMessage, Label, Note, Urls};

use super::Block;
use crate::{
    expression::{levenstein, ExpressionError, FunctionArgument},
    function::{
        closure::{self, VariableKind},
        ArgumentList, Example, FunctionClosure, FunctionCompileContext, Parameter,
        ResolvedArgument,
    },
    parser::{Ident, Node},
    state::{ExternalEnv, LocalEnv},
    type_def::Details,
    value::Kind,
    Context, Expression, Function, Resolved, Span, TypeDef,
};

pub(crate) struct Builder<'a> {
    abort_on_error: bool,
    arguments_with_unknown_type_validity: Vec<(Parameter, Node<FunctionArgument>)>,
    call_span: Span,
    ident_span: Span,
    function_id: usize,
    arguments: Arc<Vec<Node<FunctionArgument>>>,
    closure: Option<(Vec<Ident>, closure::Input)>,
    list: ArgumentList,
    function: &'a dyn Function,
}

impl<'a> Builder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        call_span: Span,
        ident: Node<Ident>,
        abort_on_error: bool,
        arguments: Vec<Node<FunctionArgument>>,
        funcs: &'a [Box<dyn Function>],
        local: &mut LocalEnv,
        external: &mut ExternalEnv,
        closure_variables: Option<Node<Vec<Node<Ident>>>>,
    ) -> Result<Self, Error> {
        let (ident_span, ident) = ident.take();

        // Check if function exists.
        let (function_id, function) = match funcs
            .iter()
            .enumerate()
            .find(|(_pos, f)| f.identifier() == ident.as_ref())
        {
            Some(function) => function,
            None => {
                let idents = funcs
                    .iter()
                    .map(|func| func.identifier())
                    .collect::<Vec<_>>();

                return Err(Error::Undefined {
                    ident_span,
                    ident: ident.clone(),
                    idents,
                });
            }
        };

        // Check function arity.
        if arguments.len() > function.parameters().len() {
            let arguments_span = {
                let start = arguments.first().unwrap().span().start();
                let end = arguments.last().unwrap().span().end();

                Span::new(start, end)
            };

            return Err(Error::WrongNumberOfArgs {
                arguments_span,
                max: function.parameters().len(),
            });
        }

        // Keeps track of positional argument indices.
        //
        // Used to map a positional argument to its keyword. Keyword arguments
        // can be used in any order, and don't count towards the index of
        // positional arguments.
        let mut index = 0;
        let mut list = ArgumentList::default();

        let mut arguments_with_unknown_type_validity = vec![];
        for node in &arguments {
            let (argument_span, argument) = node.clone().take();

            let parameter = match argument.keyword() {
                // positional argument
                None => {
                    index += 1;
                    function.parameters().get(index - 1)
                }

                // keyword argument
                Some(k) => function
                    .parameters()
                    .iter()
                    .enumerate()
                    .find(|(_, param)| param.keyword == k)
                    .map(|(pos, param)| {
                        if pos == index {
                            index += 1;
                        }

                        param
                    }),
            }
            .ok_or_else(|| Error::UnknownKeyword {
                keyword_span: argument.keyword_span().expect("exists"),
                ident_span,
                keywords: function.parameters().iter().map(|p| p.keyword).collect(),
            })?;

            // Check if the argument is of the expected type.
            let argument_type_def = argument.type_def((local, external));
            let expr_kind = argument_type_def.kind();
            let param_kind = parameter.kind();

            if !param_kind.intersects(expr_kind) {
                return Err(Error::InvalidArgumentKind {
                    function_ident: function.identifier(),
                    abort_on_error,
                    arguments_fmt: arguments
                        .iter()
                        .map(|arg| arg.inner().to_string())
                        .collect::<Vec<_>>(),
                    parameter: *parameter,
                    got: expr_kind.clone(),
                    argument,
                    argument_span,
                });
            } else if !param_kind.is_superset(expr_kind) {
                arguments_with_unknown_type_validity.push((*parameter, node.clone()));
            }

            // Check if the argument is infallible.
            if argument_type_def.is_fallible() {
                return Err(Error::FallibleArgument {
                    expr_span: argument.span(),
                });
            }

            list.insert(parameter.keyword, argument.into_inner());
        }

        // Check missing required arguments.
        function
            .parameters()
            .iter()
            .enumerate()
            .filter(|(_, p)| p.required)
            .filter(|(_, p)| !list.keywords().contains(&p.keyword))
            .try_for_each(|(i, p)| -> Result<_, _> {
                Err(Error::MissingArgument {
                    call_span,
                    keyword: p.keyword,
                    position: i,
                })
            })?;

        // Check function closure validity.
        let closure = match (function.closure(), closure_variables) {
            // Error if closure is provided for function that doesn't support
            // any.
            (None, Some(variables)) => {
                let closure_span = variables.span();

                return Err(Error::UnexpectedClosure {
                    call_span,
                    closure_span,
                });
            }

            // Error if closure is missing from function that expects one.
            (Some(definition), None) => {
                let example = definition.inputs.get(0).map(|input| input.example);

                return Err(Error::MissingClosure { call_span, example });
            }

            // Check for invalid closure signature.
            (Some(definition), Some(variables)) => {
                let mut matched = None;
                let mut err_found_type_def = None;

                for input in definition.inputs {
                    // Check type definition for linked parameter.
                    match list.arguments.get(input.parameter_keyword) {
                        // No argument provided for the given parameter keyword.
                        //
                        // This means the closure can't act on the input
                        // definition, so we continue on to the next. If no
                        // input definitions are valid, the closure is invalid.
                        None => continue,

                        // We've found the function argument over which the
                        // closure is going to resolve. We need to ensure the
                        // type of this argument is as expected by the closure.
                        Some(expr) => {
                            let type_def = expr.type_def((local, external));

                            // The type definition of the value does not match
                            // the expected closure type, continue to check if
                            // the closure eventually accepts this definition.
                            //
                            // Keep track of the type information, so that we
                            // can report these in a diagnostic error if no
                            // other input definition matches.
                            if !input.kind.is_superset(type_def.kind()) {
                                err_found_type_def = Some(type_def.kind().clone());
                                continue;
                            }

                            matched = Some((input.clone(), expr));
                            break;
                        }
                    };
                }

                // None of the inputs matched the value type, this is a user error.
                match matched {
                    None => {
                        return Err(Error::ClosureParameterTypeMismatch {
                            call_span,
                            found_kind: err_found_type_def.unwrap_or_else(Kind::any),
                        })
                    }

                    Some((input, target)) => {
                        // Now that we know we have a matching parameter argument with a valid type
                        // definition, we can move on to checking/defining the closure arguments.
                        //
                        // In doing so we:
                        //
                        // - check the arity of the closure arguments
                        // - set the expected type definition of each argument
                        if input.variables.len() != variables.len() {
                            let closure_arguments_span =
                                variables.first().map_or(call_span, |node| {
                                    (node.span().start(), variables.last().unwrap().span().end())
                                        .into()
                                });

                            return Err(Error::ClosureArityMismatch {
                                ident_span,
                                closure_arguments_span,
                                expected: input.variables.len(),
                                supplied: variables.len(),
                            });
                        }

                        // Get the provided argument identifier in the same position as defined in the
                        // input definition.
                        //
                        // That is, if the function closure definition expects:
                        //
                        //   [bytes, integer]
                        //
                        // Then, given for an actual implementation of:
                        //
                        //   foo() -> { |bar, baz| }
                        //
                        // We set "bar" (index 0) to return bytes, and "baz" (index 1) to return an
                        // integer.
                        for (index, input_var) in input.variables.clone().into_iter().enumerate() {
                            let call_ident = &variables[index];
                            let type_def = target.type_def((local, external));

                            let (type_def, value) = match input_var.kind {
                                // The variable kind is expected to be exactly
                                // the kind provided by the closure definition.
                                VariableKind::Exact(kind) => (kind.into(), None),

                                // The variable kind is expected to be equal to
                                // the ind of the target of the closure.
                                VariableKind::Target => {
                                    (target.type_def((local, external)), target.as_value())
                                }

                                // The variable kind is expected to be equal to
                                // the recuded kind of all values within the
                                // target collection type.
                                //
                                // This assumes the target is a collection type,
                                // or else it'll return "any".
                                VariableKind::TargetInnerValue => {
                                    let kind = if let Some(object) = type_def.as_object() {
                                        object.reduced_kind()
                                    } else if let Some(array) = type_def.as_array() {
                                        array.reduced_kind()
                                    } else {
                                        Kind::any()
                                    };

                                    (kind.into(), None)
                                }

                                // The variable kind is expected to be equal to
                                // the kind of all keys within the target
                                // collection type.
                                //
                                // This means it's either a string for an
                                // object, integer for an array, or
                                // a combination of the two if the target isn't
                                // known to be exactly one of the two.
                                //
                                // If the target can resolve to a non-collection
                                // type, this again returns "any".
                                VariableKind::TargetInnerKey => {
                                    let mut kind = Kind::never();

                                    if !type_def.is_collection() {
                                        kind = Kind::any()
                                    } else {
                                        if type_def.is_object() {
                                            kind.add_bytes();
                                        }
                                        if type_def.is_array() {
                                            kind.add_integer();
                                        }
                                    }

                                    (kind.into(), None)
                                }
                            };

                            let details = Details { type_def, value };

                            local.insert_variable(call_ident.clone().into_inner(), details);
                        }

                        let variables = variables
                            .into_inner()
                            .into_iter()
                            .map(Node::into_inner)
                            .collect();

                        Some((variables, input))
                    }
                }
            }

            _ => None,
        };

        Ok(Self {
            abort_on_error,
            arguments_with_unknown_type_validity,
            call_span,
            ident_span,
            function_id,
            arguments: Arc::new(arguments),
            closure,
            list,
            function: function.as_ref(),
        })
    }

    pub(crate) fn compile(
        mut self,
        local: &mut LocalEnv,
        external: &mut ExternalEnv,
        closure_block: Option<Node<Block>>,
        mut local_snapshot: LocalEnv,
        fallible_expression_error: &mut Option<Box<dyn DiagnosticMessage>>,
    ) -> Result<FunctionCall, Error> {
        let mut closure_fallible = false;
        let mut closure = None;

        // Check if we have a closure we need to compile.
        if let Some((variables, input)) = self.closure.clone() {
            let block = closure_block.expect("closure must contain block");

            // At this point, we've compiled the block, so we can remove the
            // closure variables from the compiler's local environment.
            variables
                .iter()
                .for_each(|ident| match local_snapshot.remove_variable(ident) {
                    Some(details) => local.insert_variable(ident.clone(), details),
                    None => {
                        local.remove_variable(ident);
                    }
                });

            closure_fallible = block.type_def((local, external)).is_fallible();

            let (block_span, block) = block.take();

            // Check the type definition of the resulting block.This needs to match
            // whatever is configured by the closure input type.
            let found_kind = block.type_def((local, external)).into();
            let expected_kind = input.output.into_kind();
            if !expected_kind.is_superset(&found_kind) {
                return Err(Error::ReturnTypeMismatch {
                    block_span,
                    found_kind,
                    expected_kind,
                });
            }

            let fnclosure = FunctionClosure::new(variables, block);
            self.list.set_closure(fnclosure.clone());

            closure = Some(fnclosure);
        };

        let call_span = self.call_span;
        let ident_span = self.ident_span;

        // We take the external context, and pass it to the function compile context, this allows
        // functions mutable access to external state, but keeps the internal compiler state behind
        // an immutable reference, to ensure compiler state correctness.
        let external_context = external.swap_external_context(AnyMap::new());

        let mut compile_ctx =
            FunctionCompileContext::new(self.call_span).with_external_context(external_context);

        let mut expr = self
            .function
            .compile((local, external), &mut compile_ctx, self.list.clone())
            .map_err(|error| Error::Compilation { call_span, error })?;

        // Re-insert the external context into the compiler state.
        let _ = external.swap_external_context(compile_ctx.into_external_context());

        // Asking for an infallible function to abort on error makes no sense.
        // We consider this an error at compile-time, because it makes the
        // resulting program incorrectly convey this function call might fail.
        if self.abort_on_error
            && self.arguments_with_unknown_type_validity.is_empty()
            && !expr.type_def((local, external)).is_fallible()
        {
            return Err(Error::AbortInfallible {
                ident_span,
                abort_span: Span::new(ident_span.end(), ident_span.end() + 1),
            });
        }

        // The function is expected to abort at boot-time if any error occurred,
        // and one or more arguments are of an invalid type, so we'll return the
        // appropriate error.
        if let Some((parameter, argument)) =
            self.arguments_with_unknown_type_validity.first().cloned()
        {
            if !self.abort_on_error {
                let error = Error::InvalidArgumentKind {
                    function_ident: self.function.identifier(),
                    abort_on_error: self.abort_on_error,
                    arguments_fmt: self
                        .arguments
                        .iter()
                        .map(|arg| arg.inner().to_string())
                        .collect::<Vec<_>>(),
                    parameter,
                    got: argument.expr().type_def((local, external)).into(),
                    argument: argument.clone().into_inner(),
                    argument_span: argument
                        .keyword_span()
                        .unwrap_or_else(|| argument.expr_span()),
                };

                *fallible_expression_error = Some(Box::new(error) as _);
            }
        }

        // Update the state if necessary.
        expr.update_state(local, external)
            .map_err(|err| Error::UpdateState {
                call_span,
                error: err.to_string(),
            })?;

        Ok(FunctionCall {
            abort_on_error: self.abort_on_error,
            expr,
            arguments_with_unknown_type_validity: self.arguments_with_unknown_type_validity,
            closure_fallible,
            closure,
            span: call_span,
            ident: self.function.identifier(),
            function_id: self.function_id,
            arguments: self.arguments.clone(),
        })
    }
}

#[allow(unused)] // will be used by LLVM runtime
#[derive(Clone)]
pub struct FunctionCall {
    abort_on_error: bool,
    expr: Box<dyn Expression>,
    arguments_with_unknown_type_validity: Vec<(Parameter, Node<FunctionArgument>)>,
    closure_fallible: bool,
    closure: Option<FunctionClosure>,

    // used for enhancing runtime error messages (using abort-instruction).
    //
    // TODO: have span store line/col details to further improve this.
    span: Span,

    // used for equality check
    ident: &'static str,

    // The index of the function in the list of stdlib functions.
    // Used by the VM to identify this function when called.
    function_id: usize,
    arguments: Arc<Vec<Node<FunctionArgument>>>,
}

#[allow(unused)] // will be used by LLVM runtime
impl FunctionCall {
    fn compile_arguments(
        &self,
        function: &dyn Function,
        external_env: &mut ExternalEnv,
    ) -> Result<Vec<(&'static str, Option<CompiledArgument>)>, String> {
        let function_arguments = self
            .arguments
            .iter()
            .map(|argument| argument.clone().into_inner())
            .collect::<Vec<_>>();

        // Resolve the arguments so they are in the order defined in the function.
        let arguments = function.resolve_arguments(function_arguments);

        // We take the external context, and pass it to the function compile context, this allows
        // functions mutable access to external state, but keeps the internal compiler state behind
        // an immutable reference, to ensure compiler state correctness.
        let external_context = external_env.swap_external_context(AnyMap::new());

        let mut compile_ctx =
            FunctionCompileContext::new(self.span).with_external_context(external_context);

        let compiled_arguments = arguments
            .iter()
            .map(|(keyword, argument)| -> Result<_, String> {
                let expression = argument.as_ref().map(|argument| &argument.expression);

                // Call `compile_argument` for functions that need to perform any compile time processing
                // on the argument.
                let compiled_argument = function
                    .compile_argument(&arguments, &mut compile_ctx, keyword, expression)
                    .map_err(|error| error.to_string())?;

                let argument = match compiled_argument {
                    Some(argument) => Some(CompiledArgument::Static(argument)),
                    None => argument
                        .as_ref()
                        .map(|argument| CompiledArgument::Dynamic(argument.clone())),
                };

                Ok((*keyword, argument))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Re-insert the external context into the compiler state.
        let _ = external_env.swap_external_context(compile_ctx.into_external_context());

        Ok(compiled_arguments)
    }

    #[must_use]
    pub fn arguments_fmt(&self) -> Vec<String> {
        self.arguments
            .iter()
            .map(|arg| arg.inner().to_string())
            .collect::<Vec<_>>()
    }

    #[must_use]
    pub fn arguments_dbg(&self) -> Vec<String> {
        self.arguments
            .iter()
            .map(|arg| format!("{:?}", arg.inner()))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
enum CompiledArgument {
    Static(Box<dyn Any + Send + Sync>),
    Dynamic(ResolvedArgument),
}

impl Expression for FunctionCall {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        self.expr.resolve(ctx).map_err(|err| match err {
            #[cfg(feature = "expr-abort")]
            ExpressionError::Abort { .. } => {
                panic!("abort errors must only be defined by `abort` statement")
            }
            ExpressionError::Error {
                message,
                mut labels,
                notes,
            } => {
                labels.push(Label::primary(message.clone(), self.span));

                ExpressionError::Error {
                    message: format!(
                        r#"function call error for "{}" at {}: {}"#,
                        self.ident, self.span, message
                    ),
                    labels,
                    notes,
                }
            }
        })
    }

    fn type_def(&self, state: (&LocalEnv, &ExternalEnv)) -> TypeDef {
        let mut type_def = self.expr.type_def(state);

        // If one of the arguments only partially matches the function type
        // definition, then we mark the entire function as fallible.
        //
        // This allows for progressive type-checking, by handling any potential
        // type error the function throws, instead of having to enforce
        // exact-type invariants for individual arguments.
        //
        // That is, this program triggers the `InvalidArgumentKind` error:
        //
        //     slice(10, 1)
        //
        // This is because `slice` expects either a string or an array, but it
        // receives an integer. The concept of "progressive type checking" does
        // not apply in this case, because this call can never succeed.
        //
        // However, given these example events:
        //
        //     { "foo": "bar" }
        //     { "foo": 10.5 }
        //
        // If we were to run the same program, but against the `foo` field:
        //
        //     slice(.foo, 1)
        //
        // In this situation, progressive type checking _does_ make sense,
        // because we can't know at compile-time what the eventual value of
        // `.foo` will be. We mark `.foo` as "any", which includes the "array"
        // and "string" types, so the program can now be made infallible by
        // handling any potential type error the function returns:
        //
        //     slice(.foo, 1) ?? []
        //
        // Note that this rule doesn't just apply to "any" kind (in fact, "any"
        // isn't a kind, it's simply a term meaning "all possible VRL values"),
        // but it applies whenever there's an _intersection_ but not an exact
        // _match_ between two types.
        //
        // Here's another example to demonstrate this:
        //
        //     { "foo": "foobar" }
        //     { "foo": ["foo", "bar"] }
        //     { "foo": 10.5 }
        //
        //     foo = slice(.foo, 1) ?? .foo
        //     .foo = upcase(foo) ?? foo
        //
        // This would result in the following outcomes:
        //
        //     { "foo": "OOBAR" }
        //     { "foo": ["bar", "baz"] }
        //     { "foo": 10.5 }
        //
        // For the first event, both the `slice` and `upcase` functions succeed.
        // For the second event, only the `slice` function succeeds.
        // For the third event, both functions fail.
        //
        if !self.arguments_with_unknown_type_validity.is_empty() {
            type_def = type_def.with_fallibility(true);
        }

        // If the function has a closure attached, and that closure is fallible,
        // then the function call itself becomes fallible.
        //
        // Given that `FunctionClosure` also implements `Expression`, and
        // function implementations can access this closure, it is possible the
        // function implementation already handles potential closure
        // fallibility, but to be on the safe side, we ensure it is set properly
        // here.
        //
        // Note that, since closures are tied to function calls, it is still
        // possible to silence potential closure errors using the "abort on
        // error" function-call feature (see below).
        if self.closure_fallible {
            type_def = type_def.with_fallibility(true);
        }

        if self.abort_on_error {
            type_def = type_def.with_fallibility(false);
        }

        type_def
    }

    #[cfg(feature = "llvm")]
    fn emit_llvm<'ctx>(
        &self,
        state: (&mut LocalEnv, &mut ExternalEnv),
        ctx: &mut crate::llvm::Context<'ctx>,
        function_call_abort_stack: &mut Vec<crate::llvm::BasicBlock<'ctx>>,
    ) -> Result<(), String> {
        if ["del", "exists"].contains(&self.ident) {
            return self.expr.emit_llvm(state, ctx, function_call_abort_stack);
        }

        let stdlib_function = ctx.stdlib(self.function_id);
        let compiled_arguments = self.compile_arguments(stdlib_function, state.1)?;

        let resolved_type = ctx.result_ref().get_type();

        let function_name = format!("vrl_fn_{}", self.ident);
        let function = ctx
            .module()
            .get_function(&function_name)
            .unwrap_or_else(|| {
                let mut argument_refs = compiled_arguments
                    .iter()
                    .map(|(_, argument)| match argument {
                        Some(CompiledArgument::Static(_)) => ctx.static_ref_type(),
                        Some(CompiledArgument::Dynamic(argument)) if argument.argument.required => {
                            ctx.value_ref_type()
                        }
                        Some(CompiledArgument::Dynamic(_)) | None => ctx.optional_value_ref_type(),
                    })
                    .collect::<Vec<_>>();
                argument_refs.push(resolved_type);
                let argument_refs = argument_refs
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>();
                let function_type = ctx.context().void_type().fn_type(&argument_refs, false);

                ctx.module()
                    .add_function(&function_name, function_type, None)
            });

        let result_ref = ctx.result_ref();

        let mut argument_refs = Vec::new();
        let mut drop_calls = Vec::new();

        for (keyword, argument) in compiled_arguments {
            let argument_name = format!("argument_{}", keyword);
            match argument {
                Some(CompiledArgument::Static(argument)) => {
                    let static_ref = ctx
                        .builder()
                        .build_bitcast(
                            ctx.into_const(argument, &argument_name).as_pointer_value(),
                            ctx.static_ref_type(),
                            "cast",
                        )
                        .into();

                    argument_refs.push(static_ref);
                    drop_calls.push(vec![]);
                }
                Some(CompiledArgument::Dynamic(argument)) if argument.argument.required => {
                    let resolved_ref = ctx.builder().build_alloca(
                        ctx.resolved_ref_type()
                            .get_element_type()
                            .into_struct_type(),
                        &argument_name,
                    );
                    ctx.vrl_resolved_initialize()
                        .build_call(ctx.builder(), resolved_ref);

                    ctx.set_result_ref(resolved_ref);
                    let mut abort_stack = Vec::new();
                    argument
                        .expression
                        .emit_llvm((state.0, state.1), ctx, &mut abort_stack)?;
                    function_call_abort_stack.extend(abort_stack);
                    ctx.set_result_ref(result_ref);

                    let value_ref = ctx
                        .vrl_resolved_as_value()
                        .build_call(ctx.builder(), resolved_ref)
                        .try_as_basic_value()
                        .left()
                        .expect("result is not a basic value");

                    argument_refs.push(value_ref.into());
                    drop_calls.push(vec![(resolved_ref, ctx.vrl_resolved_drop())]);
                }
                Some(CompiledArgument::Dynamic(argument)) => {
                    let resolved_ref = ctx.builder().build_alloca(
                        ctx.resolved_ref_type()
                            .get_element_type()
                            .into_struct_type(),
                        &argument_name,
                    );
                    ctx.vrl_resolved_initialize()
                        .build_call(ctx.builder(), resolved_ref);
                    let optional_value_ref = ctx.builder().build_alloca(
                        ctx.optional_value_ref_type()
                            .get_element_type()
                            .into_struct_type(),
                        &argument_name,
                    );
                    ctx.vrl_optional_value_initialize()
                        .build_call(ctx.builder(), optional_value_ref);

                    ctx.set_result_ref(resolved_ref);
                    let mut abort_stack = Vec::new();
                    argument
                        .expression
                        .emit_llvm((state.0, state.1), ctx, &mut abort_stack)?;
                    function_call_abort_stack.extend(abort_stack);
                    ctx.set_result_ref(result_ref);

                    ctx.vrl_resolved_as_value_to_optional_value().build_call(
                        ctx.builder(),
                        resolved_ref,
                        optional_value_ref,
                    );

                    argument_refs.push(optional_value_ref.into());
                    drop_calls.push(vec![
                        (optional_value_ref, ctx.vrl_optional_value_drop()),
                        (resolved_ref, ctx.vrl_resolved_drop()),
                    ]);
                }
                None => {
                    let optional_value_ref = ctx.builder().build_alloca(
                        ctx.optional_value_ref_type()
                            .get_element_type()
                            .into_struct_type(),
                        &argument_name,
                    );
                    ctx.vrl_optional_value_initialize()
                        .build_call(ctx.builder(), optional_value_ref);

                    argument_refs.push(optional_value_ref.into());
                    drop_calls.push(vec![(optional_value_ref, ctx.vrl_optional_value_drop())]);
                }
            }
        }

        argument_refs.push(result_ref.into());

        ctx.builder()
            .build_call(function, &argument_refs, self.ident);

        for drop_calls in drop_calls {
            for (value_ref, drop_fn) in drop_calls {
                drop_fn.build_call(ctx.builder(), value_ref);
            }
        }

        let ident_ref = ctx
            .into_const(self.ident.to_owned(), self.ident)
            .as_pointer_value();
        let span_ref = ctx
            .into_const(self.span, &self.span.to_string())
            .as_pointer_value();
        let type_def = self.type_def((state.0, state.1));

        if type_def.is_fallible() && !ctx.discard_error() || self.abort_on_error {
            let is_error = ctx
                .vrl_resolved_is_err()
                .build_call(ctx.builder(), result_ref)
                .try_as_basic_value()
                .left()
                .expect("result is not a basic value")
                .try_into()
                .expect("result is not an int value");

            let function_call_resume_block = ctx
                .context()
                .append_basic_block(ctx.function(), "function_call_resume");
            let function_call_abort_block = ctx
                .context()
                .append_basic_block(ctx.function(), "function_call_abort");
            ctx.builder().build_conditional_branch(
                is_error,
                function_call_abort_block,
                function_call_resume_block,
            );

            ctx.builder().position_at_end(function_call_abort_block);
            if self.abort_on_error {
                ctx.vrl_resolved_swap().build_call(
                    ctx.builder(),
                    result_ref,
                    ctx.global_result_ref(),
                );
                function_call_abort_stack.push(function_call_abort_block);
            } else {
                let vrl_expression_function_call_abort = ctx.vrl_expression_function_call_abort();
                vrl_expression_function_call_abort.build_call(
                    ctx.builder(),
                    ctx.builder().build_bitcast(
                        ident_ref,
                        vrl_expression_function_call_abort
                            .function
                            .get_nth_param(0)
                            .unwrap()
                            .get_type()
                            .into_pointer_type(),
                        "cast",
                    ),
                    ctx.builder().build_bitcast(
                        span_ref,
                        vrl_expression_function_call_abort
                            .function
                            .get_nth_param(1)
                            .unwrap()
                            .get_type()
                            .into_pointer_type(),
                        "cast",
                    ),
                    result_ref,
                );
                ctx.builder()
                    .build_unconditional_branch(function_call_resume_block);
            }

            ctx.builder().position_at_end(function_call_resume_block);
        }

        let current_block = ctx.builder().get_insert_block().unwrap();

        for abort_block in function_call_abort_stack {
            ctx.builder().position_at_end(*abort_block);
            let vrl_expression_function_call_abort = ctx.vrl_expression_function_call_abort();
            vrl_expression_function_call_abort.build_call(
                ctx.builder(),
                ctx.builder().build_bitcast(
                    ident_ref,
                    vrl_expression_function_call_abort
                        .function
                        .get_nth_param(0)
                        .unwrap()
                        .get_type()
                        .into_pointer_type(),
                    "cast",
                ),
                ctx.builder().build_bitcast(
                    span_ref,
                    vrl_expression_function_call_abort
                        .function
                        .get_nth_param(1)
                        .unwrap()
                        .get_type()
                        .into_pointer_type(),
                    "cast",
                ),
                ctx.global_result_ref(),
            );
        }

        ctx.builder().position_at_end(current_block);

        Ok(())
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ident.fmt(f)?;
        f.write_str("(")?;

        let arguments = self.arguments_fmt();
        let mut iter = arguments.iter().peekable();
        while let Some(arg) = iter.next() {
            f.write_str(arg)?;

            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str(")")
    }
}

impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FunctionCall(")?;
        self.ident.fmt(f)?;

        f.write_str("(")?;

        let arguments = self.arguments_dbg();
        let mut iter = arguments.iter().peekable();
        while let Some(arg) = iter.next() {
            f.write_str(arg)?;

            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str("))")
    }
}

impl PartialEq for FunctionCall {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

// -----------------------------------------------------------------------------

#[derive(thiserror::Error, Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Error {
    #[error("call to undefined function")]
    Undefined {
        ident_span: Span,
        ident: Ident,
        idents: Vec<&'static str>,
    },

    #[error("wrong number of function arguments")]
    WrongNumberOfArgs { arguments_span: Span, max: usize },

    #[error("unknown function argument keyword")]
    UnknownKeyword {
        keyword_span: Span,
        ident_span: Span,
        keywords: Vec<&'static str>,
    },

    #[error("missing function argument")]
    MissingArgument {
        call_span: Span,
        keyword: &'static str,
        position: usize,
    },

    #[error("function compilation error: error[E{}] {}", error.code(), error)]
    Compilation {
        call_span: Span,
        error: Box<dyn DiagnosticMessage>,
    },

    #[error("can't abort infallible function")]
    AbortInfallible { ident_span: Span, abort_span: Span },

    #[error("invalid argument type")]
    InvalidArgumentKind {
        function_ident: &'static str,
        abort_on_error: bool,
        arguments_fmt: Vec<String>,
        parameter: Parameter,
        got: Kind,
        argument: FunctionArgument,
        argument_span: Span,
    },

    #[error("fallible argument")]
    FallibleArgument { expr_span: Span },

    #[error("error updating state {}", error)]
    UpdateState { call_span: Span, error: String },

    #[error("unexpected closure")]
    UnexpectedClosure { call_span: Span, closure_span: Span },

    #[error("missing closure")]
    MissingClosure {
        call_span: Span,
        example: Option<Example>,
    },

    #[error("invalid closure arity")]
    ClosureArityMismatch {
        ident_span: Span,
        closure_arguments_span: Span,
        expected: usize,
        supplied: usize,
    },
    #[error("type mismatch in closure parameter")]
    ClosureParameterTypeMismatch { call_span: Span, found_kind: Kind },
    #[error("type mismatch in closure return type")]
    ReturnTypeMismatch {
        block_span: Span,
        found_kind: Kind,
        expected_kind: Kind,
    },
}

impl DiagnosticMessage for Error {
    fn code(&self) -> usize {
        use Error::{
            AbortInfallible, ClosureArityMismatch, ClosureParameterTypeMismatch, Compilation,
            FallibleArgument, InvalidArgumentKind, MissingArgument, MissingClosure,
            ReturnTypeMismatch, Undefined, UnexpectedClosure, UnknownKeyword, UpdateState,
            WrongNumberOfArgs,
        };

        match self {
            Undefined { .. } => 105,
            WrongNumberOfArgs { .. } => 106,
            UnknownKeyword { .. } => 108,
            Compilation { .. } => 610,
            MissingArgument { .. } => 107,
            AbortInfallible { .. } => 620,
            InvalidArgumentKind { .. } => 110,
            FallibleArgument { .. } => 630,
            UpdateState { .. } => 640,
            UnexpectedClosure { .. } => 109,
            MissingClosure { .. } => 111,
            ClosureArityMismatch { .. } => 120,
            ClosureParameterTypeMismatch { .. } => 121,
            ReturnTypeMismatch { .. } => 122,
        }
    }

    fn labels(&self) -> Vec<Label> {
        use Error::{
            AbortInfallible, ClosureArityMismatch, ClosureParameterTypeMismatch, Compilation,
            FallibleArgument, InvalidArgumentKind, MissingArgument, MissingClosure,
            ReturnTypeMismatch, Undefined, UnexpectedClosure, UnknownKeyword, UpdateState,
            WrongNumberOfArgs,
        };

        match self {
            Undefined {
                ident_span,
                ident,
                idents,
            } => {
                let mut vec = vec![Label::primary("undefined function", ident_span)];
                let ident_chars = ident.as_ref().chars().collect::<Vec<_>>();

                if let Some((idx, _)) = idents
                    .iter()
                    .map(|possible| {
                        let possible_chars = possible.chars().collect::<Vec<_>>();
                        levenstein::distance(&ident_chars, &possible_chars)
                    })
                    .enumerate()
                    .min_by_key(|(_, score)| *score)
                {
                    {
                        let guessed: &str = idents[idx];
                        vec.push(Label::context(
                            format!(r#"did you mean "{}"?"#, guessed),
                            ident_span,
                        ));
                    }
                }

                vec
            }

            WrongNumberOfArgs {
                arguments_span,
                max,
            } => {
                let arg = if *max == 1 { "argument" } else { "arguments" };

                vec![
                    Label::primary("too many function arguments", arguments_span),
                    Label::context(
                        format!("this function takes a maximum of {} {}", max, arg),
                        arguments_span,
                    ),
                ]
            }

            UnknownKeyword {
                keyword_span,
                ident_span,
                keywords,
            } => vec![
                Label::primary("unknown keyword", keyword_span),
                Label::context(
                    format!(
                        "this function accepts the following keywords: {}",
                        keywords
                            .iter()
                            .map(|k| format!(r#""{}""#, k))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    ident_span,
                ),
            ],

            Compilation { call_span, error } => error
                .labels()
                .into_iter()
                .map(|mut label| {
                    label.span = *call_span;
                    label
                })
                .collect(),

            MissingArgument {
                call_span,
                keyword,
                position,
            } => {
                vec![Label::primary(
                    format!(
                        r#"required argument missing: "{}" (position {})"#,
                        keyword, position
                    ),
                    call_span,
                )]
            }

            AbortInfallible {
                ident_span,
                abort_span,
            } => {
                vec![
                    Label::primary("this function can't fail", ident_span),
                    Label::context("remove this abort-instruction", abort_span),
                ]
            }

            InvalidArgumentKind {
                parameter,
                got,
                argument,
                argument_span,
                ..
            } => {
                let keyword = parameter.keyword;
                let expected = parameter.kind();
                let expr_span = argument.span();

                // TODO: extract this out into a helper
                let kind_str = |kind: &Kind| {
                    if kind.is_any() {
                        kind.to_string()
                    } else if kind.is_exact() {
                        format!(r#"the exact type {}"#, kind)
                    } else {
                        format!("one of {}", kind)
                    }
                };

                vec![
                    Label::primary(
                        format!("this expression resolves to {}", kind_str(got)),
                        expr_span,
                    ),
                    Label::context(
                        format!(
                            r#"but the parameter "{}" expects {}"#,
                            keyword,
                            kind_str(&expected)
                        ),
                        argument_span,
                    ),
                ]
            }

            FallibleArgument { expr_span } => vec![
                Label::primary("this expression can fail", expr_span),
                Label::context(
                    "handle the error before passing it in as an argument",
                    expr_span,
                ),
            ],

            UpdateState { call_span, error } => vec![Label::primary(
                format!("an error occurred updating the compiler state: {}", error),
                call_span,
            )],
            UnexpectedClosure { call_span, closure_span } => vec![
                Label::primary("unexpected closure", closure_span),
                Label::context("this function does not accept a closure", call_span)
            ],
            MissingClosure { call_span, .. } => vec![Label::primary("this function expects a closure", call_span)],
            ClosureArityMismatch { ident_span, closure_arguments_span, expected, supplied } => vec![
                Label::primary(format!("this function requires a closure with {expected} argument(s)"), ident_span),
                Label::context(format!("but {supplied} argument(s) are supplied"), closure_arguments_span)
            ],
            ClosureParameterTypeMismatch {
                call_span,
                found_kind,
            } => vec![
                Label::primary("the closure tied to this function call expects a different input value", call_span),
                Label::context(format!("expression has an inferred type of {found_kind} where an array or object was expected"), call_span)],
            ReturnTypeMismatch {
                block_span,
                found_kind,
                expected_kind,
            } => vec![
                Label::primary("block returns invalid value type", block_span),
                Label::context(format!("expected: {expected_kind}"), block_span),
                Label::context(format!("received: {found_kind}"), block_span)],
        }
    }

    fn notes(&self) -> Vec<Note> {
        use Error::{
            AbortInfallible, Compilation, FallibleArgument, InvalidArgumentKind, MissingClosure,
            WrongNumberOfArgs,
        };

        match self {
            WrongNumberOfArgs { .. } => vec![Note::SeeDocs(
                "function arguments".to_owned(),
                Urls::expression_docs_url("#arguments"),
            )],
            AbortInfallible { .. } | FallibleArgument { .. } => vec![Note::SeeErrorDocs],
            InvalidArgumentKind {
                function_ident,
                abort_on_error,
                arguments_fmt,
                parameter,
                argument,
                ..
            } => {
                // TODO: move this into a generic helper function
                let kind = parameter.kind();
                let guard = if kind.is_bytes() {
                    format!("string!({})", argument)
                } else if kind.is_integer() {
                    format!("int!({})", argument)
                } else if kind.is_float() {
                    format!("float!({})", argument)
                } else if kind.is_boolean() {
                    format!("bool!({})", argument)
                } else if kind.is_object() {
                    format!("object!({})", argument)
                } else if kind.is_array() {
                    format!("array!({})", argument)
                } else if kind.is_timestamp() {
                    format!("timestamp!({})", argument)
                } else {
                    return vec![];
                };

                let coerce = if kind.is_bytes() {
                    Some(format!(r#"to_string({}) ?? "default""#, argument))
                } else if kind.is_integer() {
                    Some(format!("to_int({}) ?? 0", argument))
                } else if kind.is_float() {
                    Some(format!("to_float({}) ?? 0", argument))
                } else if kind.is_boolean() {
                    Some(format!("to_bool({}) ?? false", argument))
                } else if kind.is_timestamp() {
                    Some(format!("to_timestamp({}) ?? now()", argument))
                } else {
                    None
                };

                let args = {
                    let mut args = String::new();
                    let mut iter = arguments_fmt.iter().peekable();
                    while let Some(arg) = iter.next() {
                        args.push_str(arg);
                        if iter.peek().is_some() {
                            args.push_str(", ");
                        }
                    }

                    args
                };

                let abort = if *abort_on_error { "!" } else { "" };

                let mut notes = vec![];

                let call = format!("{}{}({})", function_ident, abort, args);

                notes.append(&mut Note::solution(
                    "ensuring an appropriate type at runtime",
                    vec![format!("{} = {}", argument, guard), call.clone()],
                ));

                if let Some(coerce) = coerce {
                    notes.append(&mut Note::solution(
                        "coercing to an appropriate type and specifying a default value as a fallback in case coercion fails",
                        vec![format!("{} = {}", argument, coerce), call],
                    ))
                }

                notes.push(Note::SeeErrorDocs);

                notes
            }

            Compilation { error, .. } => error.notes(),

            MissingClosure { example, .. } if example.is_some() => {
                let code = example.unwrap().source.to_owned();
                vec![Note::Example(code)]
            }

            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{state::ExternalEnv, value::kind};

    #[derive(Clone, Debug)]
    struct Fn;

    impl Expression for Fn {
        fn resolve(&self, _ctx: &mut Context) -> Resolved {
            todo!()
        }

        fn type_def(&self, _state: (&LocalEnv, &ExternalEnv)) -> TypeDef {
            TypeDef::null().infallible()
        }
    }

    #[derive(Debug)]
    struct TestFn;

    impl Function for TestFn {
        fn identifier(&self) -> &'static str {
            "test"
        }

        fn examples(&self) -> &'static [crate::function::Example] {
            &[]
        }

        fn parameters(&self) -> &'static [Parameter] {
            &[
                Parameter {
                    keyword: "one",
                    kind: kind::INTEGER,
                    required: false,
                },
                Parameter {
                    keyword: "two",
                    kind: kind::INTEGER,
                    required: false,
                },
                Parameter {
                    keyword: "three",
                    kind: kind::INTEGER,
                    required: false,
                },
            ]
        }

        fn compile(
            &self,
            _state: (&mut LocalEnv, &mut ExternalEnv),
            _ctx: &mut FunctionCompileContext,
            _arguments: ArgumentList,
        ) -> crate::function::Compiled {
            Ok(Box::new(Fn))
        }

        fn symbol(&self) -> Option<(&'static str, usize)> {
            unimplemented!()
        }
    }

    #[cfg(feature = "expr-literal")]
    fn create_node<T>(inner: T) -> Node<T> {
        Node::new(Span::new(0, 0), inner)
    }

    #[cfg(feature = "expr-literal")]
    fn create_argument(ident: Option<&str>, value: i64) -> FunctionArgument {
        use crate::expression::{Expr, Literal};

        FunctionArgument::new(
            ident.map(|ident| create_node(Ident::new(ident))),
            create_node(Expr::Literal(Literal::Integer(value))),
        )
    }

    #[cfg(feature = "expr-literal")]
    fn create_resolved_argument(parameter: Parameter, value: i64) -> ResolvedArgument {
        use crate::expression::{Expr, Literal};

        ResolvedArgument {
            argument: parameter,
            expression: Expr::Literal(Literal::Integer(value)),
        }
    }

    #[cfg(feature = "expr-literal")]
    fn create_function_call(arguments: Vec<Node<FunctionArgument>>) -> FunctionCall {
        let mut local = LocalEnv::default();
        let mut external = ExternalEnv::default();

        Builder::new(
            Span::new(0, 0),
            Node::new(Span::new(0, 0), Ident::new("test")),
            false,
            arguments,
            &[Box::new(TestFn) as _],
            &mut local,
            &mut external,
            None,
        )
        .unwrap()
        .compile(
            &mut local,
            &mut external,
            None,
            LocalEnv::default(),
            &mut None,
        )
        .unwrap()
    }

    #[test]
    #[cfg(feature = "expr-literal")]
    fn resolve_arguments_simple() {
        let call = create_function_call(vec![
            create_node(create_argument(None, 1)),
            create_node(create_argument(None, 2)),
            create_node(create_argument(None, 3)),
        ]);

        let parameters = TestFn.parameters();
        let arguments = call.resolve_arguments(&TestFn);
        let expected: Vec<(&'static str, Option<ResolvedArgument>)> = vec![
            ("one", Some(create_resolved_argument(parameters[1], 1))),
            ("two", Some(create_resolved_argument(parameters[2], 2))),
            ("three", Some(create_resolved_argument(parameters[3], 3))),
        ];

        assert_eq!(Ok(expected), arguments);
    }

    #[test]
    #[cfg(feature = "expr-literal")]
    fn resolve_arguments_named() {
        let call = create_function_call(vec![
            create_node(create_argument(Some("one"), 1)),
            create_node(create_argument(Some("two"), 2)),
            create_node(create_argument(Some("three"), 3)),
        ]);

        let parameters = TestFn.parameters();
        let arguments = TestFn.resolve_arguments(&TestFn);
        let expected: Vec<(&'static str, Option<ResolvedArgument>)> = vec![
            ("one", Some(create_resolved_argument(parameters[1], 1))),
            ("two", Some(create_resolved_argument(parameters[2], 2))),
            ("three", Some(create_resolved_argument(parameters[3], 3))),
        ];

        assert_eq!(Ok(expected), arguments);
    }

    #[test]
    #[cfg(feature = "expr-literal")]
    fn resolve_arguments_named_unordered() {
        let call = create_function_call(vec![
            create_node(create_argument(Some("three"), 3)),
            create_node(create_argument(Some("two"), 2)),
            create_node(create_argument(Some("one"), 1)),
        ]);

        let parameters = TestFn.parameters();
        let arguments = call.resolve_arguments(&TestFn);
        let expected: Vec<(&'static str, Option<ResolvedArgument>)> = vec![
            ("one", Some(create_resolved_argument(parameters[1], 1))),
            ("two", Some(create_resolved_argument(parameters[2], 2))),
            ("three", Some(create_resolved_argument(parameters[3], 3))),
        ];

        assert_eq!(Ok(expected), arguments);
    }

    #[test]
    #[cfg(feature = "expr-literal")]
    fn resolve_arguments_unnamed_unordered_one() {
        let call = create_function_call(vec![
            create_node(create_argument(Some("three"), 3)),
            create_node(create_argument(None, 2)),
            create_node(create_argument(Some("one"), 1)),
        ]);

        let parameters = TestFn.parameters();
        let arguments = call.resolve_arguments(&TestFn);
        let expected: Vec<(&'static str, Option<ResolvedArgument>)> = vec![
            ("one", Some(create_resolved_argument(parameters[1], 1))),
            ("two", Some(create_resolved_argument(parameters[2], 2))),
            ("three", Some(create_resolved_argument(parameters[3], 3))),
        ];

        assert_eq!(Ok(expected), arguments);
    }

    #[test]
    #[cfg(feature = "expr-literal")]
    fn resolve_arguments_unnamed_unordered_two() {
        let call = create_function_call(vec![
            create_node(create_argument(Some("three"), 3)),
            create_node(create_argument(None, 1)),
            create_node(create_argument(None, 2)),
        ]);

        let parameters = TestFn.parameters();
        let arguments = call.resolve_arguments(&TestFn);
        let expected: Vec<(&'static str, Option<ResolvedArgument>)> = vec![
            ("one", Some(create_resolved_argument(parameters[1], 1))),
            ("two", Some(create_resolved_argument(parameters[2], 2))),
            ("three", Some(create_resolved_argument(parameters[3], 3))),
        ];

        assert_eq!(Ok(expected), arguments);
    }
}
