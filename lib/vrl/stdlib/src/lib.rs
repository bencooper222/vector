#![deny(
    warnings,
    clippy::all,
    clippy::pedantic,
    unreachable_pub,
    unused_allocation,
    unused_extern_crates,
    unused_assignments,
    unused_comparisons
)]
#![allow(
    clippy::cast_possible_truncation, // allowed in initial deny commit
    clippy::cast_precision_loss, // allowed in initial deny commit
    clippy::cast_sign_loss, // allowed in initial deny commit
    clippy::default_trait_access, // allowed in initial deny commit
    clippy::doc_markdown, // allowed in initial deny commit
    clippy::inefficient_to_string, // allowed in initial deny commit
    clippy::match_bool, // allowed in initial deny commit
    clippy::match_same_arms, // allowed in initial deny commit
    clippy::needless_pass_by_value, // allowed in initial deny commit
    clippy::semicolon_if_nothing_returned,  // allowed in initial deny commit
    clippy::similar_names, // allowed in initial deny commit
    clippy::single_match_else, // allowed in initial deny commit
    clippy::struct_excessive_bools,  // allowed in initial deny commit
    clippy::too_many_lines, // allowed in initial deny commit
    clippy::trivially_copy_pass_by_ref, // allowed in initial deny commit
)]

mod util;

#[cfg(feature = "append")]
mod append;
#[cfg(feature = "array")]
mod array;
#[cfg(feature = "assert")]
mod assert;
#[cfg(feature = "assert_eq")]
mod assert_eq;
#[cfg(feature = "boolean")]
mod boolean;
#[cfg(feature = "ceil")]
mod ceil;
#[cfg(feature = "compact")]
mod compact;
#[cfg(feature = "contains")]
mod contains;
#[cfg(feature = "decode_base64")]
mod decode_base64;
#[cfg(feature = "decode_percent")]
mod decode_percent;
#[cfg(feature = "decrypt")]
mod decrypt;
#[cfg(feature = "del")]
mod del;
#[cfg(feature = "downcase")]
mod downcase;
#[cfg(feature = "encode_base64")]
mod encode_base64;
#[cfg(feature = "encode_json")]
mod encode_json;
#[cfg(feature = "encode_key_value")]
mod encode_key_value;
#[cfg(feature = "encode_logfmt")]
mod encode_logfmt;
#[cfg(feature = "encode_percent")]
mod encode_percent;
#[cfg(feature = "encrypt")]
mod encrypt;
#[cfg(feature = "ends_with")]
mod ends_with;
#[cfg(feature = "exists")]
mod exists;
#[cfg(feature = "filter")]
mod filter;
#[cfg(feature = "find")]
mod find;
#[cfg(feature = "flatten")]
mod flatten;
#[cfg(feature = "float")]
mod float;
#[cfg(feature = "floor")]
mod floor;
#[cfg(feature = "for_each")]
mod for_each;
#[cfg(feature = "format_int")]
mod format_int;
#[cfg(feature = "format_number")]
mod format_number;
#[cfg(feature = "format_timestamp")]
mod format_timestamp;
#[cfg(feature = "get")]
mod get;
#[cfg(feature = "get_env_var")]
mod get_env_var;
#[cfg(feature = "get_hostname")]
mod get_hostname;
#[cfg(feature = "includes")]
mod includes;
#[cfg(feature = "integer")]
mod integer;
#[cfg(feature = "ip_aton")]
mod ip_aton;
#[cfg(feature = "ip_cidr_contains")]
mod ip_cidr_contains;
#[cfg(feature = "ip_ntoa")]
mod ip_ntoa;
#[cfg(feature = "ip_ntop")]
mod ip_ntop;
#[cfg(feature = "ip_pton")]
mod ip_pton;
#[cfg(feature = "ip_subnet")]
mod ip_subnet;
#[cfg(feature = "ip_to_ipv6")]
mod ip_to_ipv6;
#[cfg(feature = "ipv6_to_ipv4")]
mod ipv6_to_ipv4;
#[cfg(feature = "is_array")]
mod is_array;
#[cfg(feature = "is_boolean")]
mod is_boolean;
#[cfg(feature = "is_empty")]
mod is_empty;
#[cfg(feature = "is_float")]
mod is_float;
#[cfg(feature = "is_integer")]
mod is_integer;
#[cfg(feature = "is_json")]
mod is_json;
#[cfg(feature = "is_null")]
mod is_null;
#[cfg(feature = "is_nullish")]
mod is_nullish;
#[cfg(feature = "is_object")]
mod is_object;
#[cfg(feature = "is_regex")]
mod is_regex;
#[cfg(feature = "is_string")]
mod is_string;
#[cfg(feature = "is_timestamp")]
mod is_timestamp;
#[cfg(feature = "join")]
mod join;
#[cfg(feature = "length")]
mod length;
#[cfg(feature = "log")]
mod log;
#[cfg(any(
    feature = "parse_common_log",
    feature = "parse_apache_log",
    feature = "parse_nginx_log"
))]
mod log_util;
#[cfg(feature = "map_keys")]
mod map_keys;
#[cfg(feature = "map_values")]
mod map_values;
#[cfg(feature = "match")]
mod r#match;
#[cfg(feature = "match_any")]
mod match_any;
#[cfg(feature = "match_array")]
mod match_array;
#[cfg(feature = "match_datadog_query")]
mod match_datadog_query;
#[cfg(feature = "md5")]
mod md5;
#[cfg(feature = "merge")]
mod merge;
#[cfg(feature = "now")]
mod now;
#[cfg(feature = "object")]
mod object;
#[cfg(feature = "only_fields")]
mod only_fields;
#[cfg(feature = "parse_apache_log")]
mod parse_apache_log;
#[cfg(feature = "parse_aws_alb_log")]
mod parse_aws_alb_log;
#[cfg(feature = "parse_aws_cloudwatch_log_subscription_message")]
mod parse_aws_cloudwatch_log_subscription_message;
#[cfg(feature = "parse_aws_vpc_flow_log")]
mod parse_aws_vpc_flow_log;
#[cfg(feature = "parse_common_log")]
mod parse_common_log;
#[cfg(feature = "parse_csv")]
mod parse_csv;
#[cfg(feature = "parse_duration")]
mod parse_duration;
#[cfg(feature = "parse_glog")]
mod parse_glog;
#[cfg(feature = "parse_grok")]
mod parse_grok;
#[cfg(feature = "parse_groks")]
mod parse_groks;
#[cfg(feature = "parse_int")]
mod parse_int;
#[cfg(feature = "parse_json")]
mod parse_json;
#[cfg(feature = "parse_key_value")]
mod parse_key_value;
#[cfg(feature = "parse_klog")]
mod parse_klog;
#[cfg(feature = "parse_linux_authorization")]
mod parse_linux_authorization;
#[cfg(feature = "parse_logfmt")]
mod parse_logfmt;
#[cfg(feature = "parse_nginx_log")]
mod parse_nginx_log;
#[cfg(feature = "parse_query_string")]
mod parse_query_string;
#[cfg(feature = "parse_regex")]
mod parse_regex;
#[cfg(feature = "parse_regex_all")]
mod parse_regex_all;
#[cfg(feature = "parse_ruby_hash")]
mod parse_ruby_hash;
#[cfg(feature = "parse_syslog")]
mod parse_syslog;
#[cfg(feature = "parse_timestamp")]
mod parse_timestamp;
#[cfg(feature = "parse_tokens")]
mod parse_tokens;
#[cfg(feature = "parse_url")]
mod parse_url;
#[cfg(feature = "parse_user_agent")]
mod parse_user_agent;
#[cfg(feature = "parse_xml")]
mod parse_xml;
#[cfg(feature = "push")]
mod push;
#[cfg(feature = "random_bytes")]
mod random_bytes;
#[cfg(feature = "redact")]
mod redact;
#[cfg(feature = "remove")]
mod remove;
#[cfg(feature = "replace")]
mod replace;
#[cfg(feature = "reverse_dns")]
mod reverse_dns;
#[cfg(feature = "round")]
mod round;
#[cfg(feature = "set")]
mod set;
#[cfg(feature = "sha1")]
mod sha1;
#[cfg(feature = "sha2")]
mod sha2;
#[cfg(feature = "sha3")]
mod sha3;
#[cfg(feature = "slice")]
mod slice;
#[cfg(feature = "split")]
mod split;
#[cfg(feature = "starts_with")]
mod starts_with;
#[cfg(feature = "string")]
mod string;
#[cfg(feature = "strip_ansi_escape_codes")]
mod strip_ansi_escape_codes;
#[cfg(feature = "strip_whitespace")]
mod strip_whitespace;
#[cfg(feature = "strlen")]
mod strlen;
#[cfg(feature = "tag_types_externally")]
mod tag_types_externally;
#[cfg(feature = "tally")]
mod tally;
#[cfg(feature = "tally_value")]
mod tally_value;
#[cfg(feature = "timestamp")]
mod timestamp;
#[cfg(feature = "to_bool")]
mod to_bool;
#[cfg(feature = "to_float")]
mod to_float;
#[cfg(feature = "to_int")]
mod to_int;
#[cfg(feature = "to_regex")]
mod to_regex;
#[cfg(feature = "to_string")]
mod to_string;
#[cfg(feature = "to_syslog_facility")]
mod to_syslog_facility;
#[cfg(feature = "to_syslog_level")]
mod to_syslog_level;
#[cfg(feature = "to_syslog_severity")]
mod to_syslog_severity;
#[cfg(feature = "to_timestamp")]
mod to_timestamp;
#[cfg(feature = "to_unix_timestamp")]
mod to_unix_timestamp;
#[cfg(feature = "truncate")]
mod truncate;
#[cfg(feature = "type_def")]
mod type_def;
#[cfg(feature = "unique")]
mod unique;
#[cfg(feature = "unnest")]
mod unnest;
#[cfg(feature = "upcase")]
mod upcase;
#[cfg(feature = "uuid_v4")]
mod uuid_v4;

// -----------------------------------------------------------------------------

#[cfg(feature = "append")]
pub use append::{vrl_fn_append, Append};
#[cfg(feature = "assert")]
pub use assert::{vrl_fn_assert, Assert};
#[cfg(feature = "assert_eq")]
pub use assert_eq::{vrl_fn_assert_eq, AssertEq};
#[cfg(feature = "boolean")]
pub use boolean::{vrl_fn_boolean, Boolean};
#[cfg(feature = "ceil")]
pub use ceil::{vrl_fn_ceil, Ceil};
#[cfg(feature = "compact")]
pub use compact::{vrl_fn_compact, Compact};
#[cfg(feature = "contains")]
pub use contains::{vrl_fn_contains, Contains};
#[cfg(feature = "decode_base64")]
pub use decode_base64::{vrl_fn_decode_base64, DecodeBase64};
#[cfg(feature = "decode_percent")]
pub use decode_percent::{vrl_fn_decode_percent, DecodePercent};
#[cfg(feature = "decrypt")]
pub use decrypt::{vrl_fn_decrypt, Decrypt};
#[cfg(feature = "del")]
pub use del::Del;
#[cfg(feature = "downcase")]
pub use downcase::{vrl_fn_downcase, Downcase};
#[cfg(feature = "encode_base64")]
pub use encode_base64::{vrl_fn_encode_base64, EncodeBase64};
#[cfg(feature = "encode_json")]
pub use encode_json::{vrl_fn_encode_json, EncodeJson};
#[cfg(feature = "encode_key_value")]
pub use encode_key_value::{vrl_fn_encode_key_value, EncodeKeyValue};
#[cfg(feature = "encode_logfmt")]
pub use encode_logfmt::{vrl_fn_encode_logfmt, EncodeLogfmt};
#[cfg(feature = "encode_percent")]
pub use encode_percent::{vrl_fn_encode_percent, EncodePercent};
#[cfg(feature = "encrypt")]
pub use encrypt::{vrl_fn_encrypt, Encrypt};
#[cfg(feature = "ends_with")]
pub use ends_with::{vrl_fn_ends_with, EndsWith};
#[cfg(feature = "exists")]
pub use exists::Exists;
#[cfg(feature = "filter")]
pub use filter::Filter;
#[cfg(feature = "find")]
pub use find::{vrl_fn_find, Find};
#[cfg(feature = "flatten")]
pub use flatten::{vrl_fn_flatten, Flatten};
#[cfg(feature = "float")]
pub use float::{vrl_fn_float, Float};
#[cfg(feature = "floor")]
pub use floor::{vrl_fn_floor, Floor};
#[cfg(feature = "for_each")]
pub use for_each::{vrl_fn_for_each, ForEach};
#[cfg(feature = "format_int")]
pub use format_int::{vrl_fn_format_int, FormatInt};
#[cfg(feature = "format_number")]
pub use format_number::{vrl_fn_format_number, FormatNumber};
#[cfg(feature = "format_timestamp")]
pub use format_timestamp::{vrl_fn_format_timestamp, FormatTimestamp};
#[cfg(feature = "get")]
pub use get::{vrl_fn_get, Get};
#[cfg(feature = "get_env_var")]
pub use get_env_var::{vrl_fn_get_env_var, GetEnvVar};
#[cfg(feature = "get_hostname")]
pub use get_hostname::{vrl_fn_get_hostname, GetHostname};
#[cfg(feature = "includes")]
pub use includes::{vrl_fn_includes, Includes};
#[cfg(feature = "integer")]
pub use integer::{vrl_fn_integer, Integer};
#[cfg(feature = "ip_aton")]
pub use ip_aton::{vrl_fn_ip_aton, IpAton};
#[cfg(feature = "ip_cidr_contains")]
pub use ip_cidr_contains::{vrl_fn_ip_cidr_contains, IpCidrContains};
#[cfg(feature = "ip_ntoa")]
pub use ip_ntoa::{vrl_fn_ip_ntoa, IpNtoa};
#[cfg(feature = "ip_ntop")]
pub use ip_ntop::{vrl_fn_ip_ntop, IpNtop};
#[cfg(feature = "ip_pton")]
pub use ip_pton::{vrl_fn_ip_pton, IpPton};
#[cfg(feature = "ip_subnet")]
pub use ip_subnet::{vrl_fn_ip_subnet, IpSubnet};
#[cfg(feature = "ip_to_ipv6")]
pub use ip_to_ipv6::{vrl_fn_ip_to_ipv6, IpToIpv6};
#[cfg(feature = "ipv6_to_ipv4")]
pub use ipv6_to_ipv4::{vrl_fn_ipv6_to_ipv4, Ipv6ToIpV4};
#[cfg(feature = "is_array")]
pub use is_array::{vrl_fn_is_array, IsArray};
#[cfg(feature = "is_boolean")]
pub use is_boolean::{vrl_fn_is_boolean, IsBoolean};
#[cfg(feature = "is_empty")]
pub use is_empty::{vrl_fn_is_empty, IsEmpty};
#[cfg(feature = "is_float")]
pub use is_float::{vrl_fn_is_float, IsFloat};
#[cfg(feature = "is_integer")]
pub use is_integer::{vrl_fn_is_integer, IsInteger};
#[cfg(feature = "is_json")]
pub use is_json::{vrl_fn_is_json, IsJson};
#[cfg(feature = "is_null")]
pub use is_null::{vrl_fn_is_null, IsNull};
#[cfg(feature = "is_nullish")]
pub use is_nullish::{vrl_fn_is_nullish, IsNullish};
#[cfg(feature = "is_object")]
pub use is_object::{vrl_fn_is_object, IsObject};
#[cfg(feature = "is_regex")]
pub use is_regex::{vrl_fn_is_regex, IsRegex};
#[cfg(feature = "is_string")]
pub use is_string::{vrl_fn_is_string, IsString};
#[cfg(feature = "is_timestamp")]
pub use is_timestamp::{vrl_fn_is_timestamp, IsTimestamp};
#[cfg(feature = "join")]
pub use join::{vrl_fn_join, Join};
#[cfg(feature = "length")]
pub use length::{vrl_fn_length, Length};
#[cfg(feature = "log")]
pub use log::{vrl_fn_log, Log};
#[cfg(feature = "map_keys")]
pub use map_keys::{vrl_fn_map_keys, MapKeys};
#[cfg(feature = "map_values")]
pub use map_values::{vrl_fn_map_values, MapValues};
#[cfg(feature = "match_any")]
pub use match_any::{vrl_fn_match_any, MatchAny};
#[cfg(feature = "match_array")]
pub use match_array::{vrl_fn_match_array, MatchArray};
#[cfg(feature = "match_datadog_query")]
pub use match_datadog_query::{vrl_fn_match_datadog_query, MatchDatadogQuery};
#[cfg(feature = "merge")]
pub use merge::{vrl_fn_merge, Merge};
#[cfg(feature = "now")]
pub use now::{vrl_fn_now, Now};
#[cfg(feature = "object")]
pub use object::{vrl_fn_object, Object};
#[cfg(feature = "only_fields")]
pub use only_fields::{vrl_fn_only_fields, OnlyFields};
#[cfg(feature = "parse_apache_log")]
pub use parse_apache_log::{vrl_fn_parse_apache_log, ParseApacheLog};
#[cfg(feature = "parse_aws_alb_log")]
pub use parse_aws_alb_log::{vrl_fn_parse_aws_alb_log, ParseAwsAlbLog};
#[cfg(feature = "parse_aws_cloudwatch_log_subscription_message")]
pub use parse_aws_cloudwatch_log_subscription_message::{
    vrl_fn_parse_aws_cloudwatch_log_subscription_message, ParseAwsCloudWatchLogSubscriptionMessage,
};
#[cfg(feature = "parse_aws_vpc_flow_log")]
pub use parse_aws_vpc_flow_log::{vrl_fn_parse_aws_vpc_flow_log, ParseAwsVpcFlowLog};
#[cfg(feature = "parse_common_log")]
pub use parse_common_log::{vrl_fn_parse_common_log, ParseCommonLog};
#[cfg(feature = "parse_csv")]
pub use parse_csv::{vrl_fn_parse_csv, ParseCsv};
#[cfg(feature = "parse_duration")]
pub use parse_duration::{vrl_fn_parse_duration, ParseDuration};
#[cfg(feature = "parse_glog")]
pub use parse_glog::{vrl_fn_parse_glog, ParseGlog};
#[cfg(feature = "parse_grok")]
pub use parse_grok::{vrl_fn_parse_grok, ParseGrok};
#[cfg(feature = "parse_groks")]
pub use parse_groks::{vrl_fn_parse_groks, ParseGroks};
#[cfg(feature = "parse_int")]
pub use parse_int::{vrl_fn_parse_int, ParseInt};
#[cfg(feature = "parse_json")]
pub use parse_json::{vrl_fn_parse_json, ParseJson};
#[cfg(feature = "parse_key_value")]
pub use parse_key_value::{vrl_fn_parse_key_value, ParseKeyValue};
#[cfg(feature = "parse_klog")]
pub use parse_klog::{vrl_fn_parse_klog, ParseKlog};
#[cfg(feature = "parse_linux_authorization")]
pub use parse_linux_authorization::ParseLinuxAuthorization;
#[cfg(feature = "parse_logfmt")]
pub use parse_logfmt::ParseLogFmt;
#[cfg(feature = "parse_nginx_log")]
pub use parse_nginx_log::{vrl_fn_parse_nginx_log, ParseNginxLog};
#[cfg(feature = "parse_query_string")]
pub use parse_query_string::{vrl_fn_parse_query_string, ParseQueryString};
#[cfg(feature = "parse_regex")]
pub use parse_regex::{vrl_fn_parse_regex, ParseRegex};
#[cfg(feature = "parse_regex_all")]
pub use parse_regex_all::{vrl_fn_parse_regex_all, ParseRegexAll};
#[cfg(feature = "parse_ruby_hash")]
pub use parse_ruby_hash::{vrl_fn_parse_ruby_hash, ParseRubyHash};
#[cfg(feature = "parse_syslog")]
pub use parse_syslog::{vrl_fn_parse_syslog, ParseSyslog};
#[cfg(feature = "parse_timestamp")]
pub use parse_timestamp::{vrl_fn_parse_timestamp, ParseTimestamp};
#[cfg(feature = "parse_tokens")]
pub use parse_tokens::{vrl_fn_parse_tokens, ParseTokens};
#[cfg(feature = "parse_url")]
pub use parse_url::{vrl_fn_parse_url, ParseUrl};
#[cfg(feature = "parse_user_agent")]
pub use parse_user_agent::{vrl_fn_parse_user_agent, ParseUserAgent};
#[cfg(feature = "parse_xml")]
pub use parse_xml::{vrl_fn_parse_xml, ParseXml};
#[cfg(feature = "push")]
pub use push::{vrl_fn_push, Push};
#[cfg(feature = "match")]
pub use r#match::{vrl_fn_match, Match};
#[cfg(feature = "random_bytes")]
pub use random_bytes::{vrl_fn_random_bytes, RandomBytes};
#[cfg(feature = "redact")]
pub use redact::{vrl_fn_redact, Redact};
#[cfg(feature = "remove")]
pub use remove::{vrl_fn_remove, Remove};
#[cfg(feature = "replace")]
pub use replace::{vrl_fn_replace, Replace};
#[cfg(feature = "reverse_dns")]
pub use reverse_dns::{vrl_fn_reverse_dns, ReverseDns};
#[cfg(feature = "round")]
pub use round::{vrl_fn_round, Round};
#[cfg(feature = "set")]
pub use set::{vrl_fn_set, Set};
#[cfg(feature = "sha2")]
pub use sha2::{vrl_fn_sha2, Sha2};
#[cfg(feature = "sha3")]
pub use sha3::{vrl_fn_sha3, Sha3};
#[cfg(feature = "slice")]
pub use slice::{vrl_fn_slice, Slice};
#[cfg(feature = "split")]
pub use split::{vrl_fn_split, Split};
#[cfg(feature = "starts_with")]
pub use starts_with::{vrl_fn_starts_with, StartsWith};
#[cfg(feature = "string")]
pub use string::{vrl_fn_string, String};
#[cfg(feature = "strip_ansi_escape_codes")]
pub use strip_ansi_escape_codes::{vrl_fn_strip_ansi_escape_codes, StripAnsiEscapeCodes};
#[cfg(feature = "strip_whitespace")]
pub use strip_whitespace::{vrl_fn_strip_whitespace, StripWhitespace};
#[cfg(feature = "strlen")]
pub use strlen::{vrl_fn_strlen, Strlen};
#[cfg(feature = "tag_types_externally")]
pub use tag_types_externally::{vrl_fn_tag_types_externally, TagTypesExternally};
#[cfg(feature = "tally")]
pub use tally::{vrl_fn_tally, Tally};
#[cfg(feature = "tally_value")]
pub use tally_value::{vrl_fn_tally_value, TallyValue};
#[cfg(feature = "timestamp")]
pub use timestamp::{vrl_fn_timestamp, Timestamp};
#[cfg(feature = "to_bool")]
pub use to_bool::{vrl_fn_to_bool, ToBool};
#[cfg(feature = "to_float")]
pub use to_float::{vrl_fn_to_float, ToFloat};
#[cfg(feature = "to_int")]
pub use to_int::{vrl_fn_to_int, ToInt};
#[cfg(feature = "to_regex")]
pub use to_regex::{vrl_fn_to_regex, ToRegex};
#[cfg(feature = "to_string")]
pub use to_string::{vrl_fn_to_string, ToString};
#[cfg(feature = "to_syslog_facility")]
pub use to_syslog_facility::{vrl_fn_to_syslog_facility, ToSyslogFacility};
#[cfg(feature = "to_syslog_level")]
pub use to_syslog_level::{vrl_fn_to_syslog_level, ToSyslogLevel};
#[cfg(feature = "to_syslog_severity")]
pub use to_syslog_severity::{vrl_fn_to_syslog_severity, ToSyslogSeverity};
#[cfg(feature = "to_timestamp")]
pub use to_timestamp::{vrl_fn_to_timestamp, ToTimestamp};
#[cfg(feature = "to_unix_timestamp")]
pub use to_unix_timestamp::{vrl_fn_to_unix_timestamp, ToUnixTimestamp};
#[cfg(feature = "truncate")]
pub use truncate::{vrl_fn_truncate, Truncate};
#[cfg(feature = "type_def")]
pub use type_def::{vrl_fn_type_def, TypeDef};
#[cfg(feature = "unique")]
pub use unique::{vrl_fn_unique, Unique};
#[cfg(feature = "unnest")]
pub use unnest::{vrl_fn_unnest, Unnest};
#[cfg(feature = "upcase")]
pub use upcase::{vrl_fn_upcase, Upcase};
#[cfg(feature = "uuid_v4")]
pub use uuid_v4::{vrl_fn_uuid_v4, UuidV4};

#[cfg(feature = "array")]
pub use crate::array::{vrl_fn_array, Array};
#[cfg(feature = "md5")]
pub use crate::md5::{vrl_fn_md5, Md5};
#[cfg(feature = "sha1")]
pub use crate::sha1::{vrl_fn_sha1, Sha1};

#[must_use]
pub fn all() -> Vec<Box<dyn vrl::Function>> {
    vec![
        #[cfg(feature = "append")]
        Box::new(Append),
        #[cfg(feature = "array")]
        Box::new(Array),
        #[cfg(feature = "assert")]
        Box::new(Assert),
        #[cfg(feature = "assert_eq")]
        Box::new(AssertEq),
        #[cfg(feature = "boolean")]
        Box::new(Boolean),
        #[cfg(feature = "ceil")]
        Box::new(Ceil),
        #[cfg(feature = "compact")]
        Box::new(Compact),
        #[cfg(feature = "contains")]
        Box::new(Contains),
        #[cfg(feature = "decode_base64")]
        Box::new(DecodeBase64),
        #[cfg(feature = "decode_percent")]
        Box::new(DecodePercent),
        #[cfg(feature = "decrypt")]
        Box::new(Decrypt),
        #[cfg(feature = "del")]
        Box::new(Del),
        #[cfg(feature = "downcase")]
        Box::new(Downcase),
        #[cfg(feature = "encode_base64")]
        Box::new(EncodeBase64),
        #[cfg(feature = "encode_json")]
        Box::new(EncodeJson),
        #[cfg(feature = "encode_key_value")]
        Box::new(EncodeKeyValue),
        #[cfg(feature = "encode_logfmt")]
        Box::new(EncodeLogfmt),
        #[cfg(feature = "encode_percent")]
        Box::new(EncodePercent),
        #[cfg(feature = "encrypt")]
        Box::new(Encrypt),
        #[cfg(feature = "ends_with")]
        Box::new(EndsWith),
        #[cfg(feature = "exists")]
        Box::new(Exists),
        #[cfg(feature = "filter")]
        Box::new(Filter),
        #[cfg(feature = "find")]
        Box::new(Find),
        #[cfg(feature = "flatten")]
        Box::new(Flatten),
        #[cfg(feature = "float")]
        Box::new(Float),
        #[cfg(feature = "floor")]
        Box::new(Floor),
        #[cfg(feature = "for_each")]
        Box::new(ForEach),
        #[cfg(feature = "format_int")]
        Box::new(FormatInt),
        #[cfg(feature = "format_number")]
        Box::new(FormatNumber),
        #[cfg(feature = "format_timestamp")]
        Box::new(FormatTimestamp),
        #[cfg(feature = "get")]
        Box::new(Get),
        #[cfg(feature = "get_env_var")]
        Box::new(GetEnvVar),
        #[cfg(feature = "get_hostname")]
        Box::new(GetHostname),
        #[cfg(feature = "includes")]
        Box::new(Includes),
        #[cfg(feature = "integer")]
        Box::new(Integer),
        #[cfg(feature = "ip_aton")]
        Box::new(IpAton),
        #[cfg(feature = "ip_cidr_contains")]
        Box::new(IpCidrContains),
        #[cfg(feature = "ip_ntoa")]
        Box::new(IpNtoa),
        #[cfg(feature = "ip_ntop")]
        Box::new(IpNtop),
        #[cfg(feature = "ip_pton")]
        Box::new(IpPton),
        #[cfg(feature = "ip_subnet")]
        Box::new(IpSubnet),
        #[cfg(feature = "ip_to_ipv6")]
        Box::new(IpToIpv6),
        #[cfg(feature = "ipv6_to_ipv4")]
        Box::new(Ipv6ToIpV4),
        #[cfg(feature = "is_array")]
        Box::new(IsArray),
        #[cfg(feature = "is_boolean")]
        Box::new(IsBoolean),
        #[cfg(feature = "is_empty")]
        Box::new(IsEmpty),
        #[cfg(feature = "is_float")]
        Box::new(IsFloat),
        #[cfg(feature = "is_integer")]
        Box::new(IsInteger),
        #[cfg(feature = "is_json")]
        Box::new(IsJson),
        #[cfg(feature = "is_null")]
        Box::new(IsNull),
        #[cfg(feature = "is_nullish")]
        Box::new(IsNullish),
        #[cfg(feature = "is_object")]
        Box::new(IsObject),
        #[cfg(feature = "is_regex")]
        Box::new(IsRegex),
        #[cfg(feature = "is_string")]
        Box::new(IsString),
        #[cfg(feature = "is_timestamp")]
        Box::new(IsTimestamp),
        #[cfg(feature = "join")]
        Box::new(Join),
        #[cfg(feature = "length")]
        Box::new(Length),
        #[cfg(feature = "log")]
        Box::new(Log),
        #[cfg(feature = "map_keys")]
        Box::new(MapKeys),
        #[cfg(feature = "map_values")]
        Box::new(MapValues),
        #[cfg(feature = "match")]
        Box::new(Match),
        #[cfg(feature = "match_any")]
        Box::new(MatchAny),
        #[cfg(feature = "match_array")]
        Box::new(MatchArray),
        #[cfg(feature = "match_datadog_query")]
        Box::new(MatchDatadogQuery),
        #[cfg(feature = "md5")]
        Box::new(Md5),
        #[cfg(feature = "merge")]
        Box::new(Merge),
        #[cfg(feature = "now")]
        Box::new(Now),
        // We are not sure if this is the way we want to expose this functionality yet
        // https://github.com/vectordotdev/vector/issues/5607
        //#[cfg(feature = "only_fields")]
        //Box::new(OnlyFields),
        #[cfg(feature = "object")]
        Box::new(Object),
        #[cfg(feature = "parse_apache_log")]
        Box::new(ParseApacheLog),
        #[cfg(feature = "parse_aws_alb_log")]
        Box::new(ParseAwsAlbLog),
        #[cfg(feature = "parse_aws_cloudwatch_log_subscription_message")]
        Box::new(ParseAwsCloudWatchLogSubscriptionMessage),
        #[cfg(feature = "parse_aws_vpc_flow_log")]
        Box::new(ParseAwsVpcFlowLog),
        #[cfg(feature = "parse_common_log")]
        Box::new(ParseCommonLog),
        #[cfg(feature = "parse_csv")]
        Box::new(ParseCsv),
        #[cfg(feature = "parse_duration")]
        Box::new(ParseDuration),
        #[cfg(feature = "parse_glog")]
        Box::new(ParseGlog),
        #[cfg(feature = "parse_grok")]
        Box::new(ParseGrok),
        #[cfg(feature = "parse_groks")]
        Box::new(ParseGroks),
        #[cfg(feature = "parse_int")]
        Box::new(ParseInt),
        #[cfg(feature = "parse_json")]
        Box::new(ParseJson),
        #[cfg(feature = "parse_key_value")]
        Box::new(ParseKeyValue),
        #[cfg(feature = "parse_klog")]
        Box::new(ParseKlog),
        #[cfg(feature = "parse_linux_authorization")]
        Box::new(ParseLinuxAuthorization),
        #[cfg(feature = "parse_logfmt")]
        Box::new(ParseLogFmt),
        #[cfg(feature = "parse_nginx_log")]
        Box::new(ParseNginxLog),
        #[cfg(feature = "parse_query_string")]
        Box::new(ParseQueryString),
        #[cfg(feature = "parse_regex")]
        Box::new(ParseRegex),
        #[cfg(feature = "parse_regex_all")]
        Box::new(ParseRegexAll),
        #[cfg(feature = "parse_ruby_hash")]
        Box::new(ParseRubyHash),
        #[cfg(feature = "parse_syslog")]
        Box::new(ParseSyslog),
        #[cfg(feature = "parse_timestamp")]
        Box::new(ParseTimestamp),
        #[cfg(feature = "parse_tokens")]
        Box::new(ParseTokens),
        #[cfg(feature = "parse_url")]
        Box::new(ParseUrl),
        #[cfg(feature = "parse_user_agent")]
        Box::new(ParseUserAgent),
        #[cfg(feature = "parse_xml")]
        Box::new(ParseXml),
        #[cfg(feature = "push")]
        Box::new(Push),
        #[cfg(feature = "random_bytes")]
        Box::new(RandomBytes),
        #[cfg(feature = "redact")]
        Box::new(Redact),
        #[cfg(feature = "remove")]
        Box::new(Remove),
        #[cfg(feature = "replace")]
        Box::new(Replace),
        #[cfg(feature = "reverse_dns")]
        Box::new(ReverseDns),
        #[cfg(feature = "round")]
        Box::new(Round),
        #[cfg(feature = "set")]
        Box::new(Set),
        #[cfg(feature = "sha1")]
        Box::new(Sha1),
        #[cfg(feature = "sha2")]
        Box::new(Sha2),
        #[cfg(feature = "sha3")]
        Box::new(Sha3),
        #[cfg(feature = "slice")]
        Box::new(Slice),
        #[cfg(feature = "split")]
        Box::new(Split),
        #[cfg(feature = "starts_with")]
        Box::new(StartsWith),
        #[cfg(feature = "string")]
        Box::new(String),
        #[cfg(feature = "strip_ansi_escape_codes")]
        Box::new(StripAnsiEscapeCodes),
        #[cfg(feature = "strip_whitespace")]
        Box::new(StripWhitespace),
        #[cfg(feature = "strlen")]
        Box::new(Strlen),
        #[cfg(feature = "tally")]
        Box::new(Tally),
        #[cfg(feature = "tally_value")]
        Box::new(TallyValue),
        #[cfg(feature = "tag_types_externally")]
        Box::new(TagTypesExternally),
        #[cfg(feature = "timestamp")]
        Box::new(Timestamp),
        #[cfg(feature = "to_bool")]
        Box::new(ToBool),
        #[cfg(feature = "to_float")]
        Box::new(ToFloat),
        #[cfg(feature = "to_int")]
        Box::new(ToInt),
        #[cfg(feature = "to_regex")]
        Box::new(ToRegex),
        #[cfg(feature = "to_string")]
        Box::new(ToString),
        #[cfg(feature = "to_syslog_facility")]
        Box::new(ToSyslogFacility),
        #[cfg(feature = "to_syslog_level")]
        Box::new(ToSyslogLevel),
        #[cfg(feature = "to_syslog_severity")]
        Box::new(ToSyslogSeverity),
        #[cfg(feature = "to_timestamp")]
        Box::new(ToTimestamp),
        #[cfg(feature = "to_unix_timestamp")]
        Box::new(ToUnixTimestamp),
        #[cfg(feature = "truncate")]
        Box::new(Truncate),
        #[cfg(feature = "type_def")]
        Box::new(TypeDef),
        #[cfg(feature = "unique")]
        Box::new(Unique),
        #[cfg(feature = "unnest")]
        Box::new(Unnest),
        #[cfg(feature = "upcase")]
        Box::new(Upcase),
        #[cfg(feature = "uuid_v4")]
        Box::new(UuidV4),
    ]
}
