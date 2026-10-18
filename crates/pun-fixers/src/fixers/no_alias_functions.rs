//! Port of `PhpCsFixer\Fixer\Alias\NoAliasFunctionsFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

/// alias => (replacement, required argument count for the `[name, count]` form).
type Alias = (&'static str, &'static str, Option<usize>);

#[derive(Default)]
struct NoAliasFunctionsFixer {
    aliases: Vec<Alias>,
}

const ALLOWED_SETS: &[&str] = &[
    "@all",
    "@internal",
    "@exif",
    "@ftp",
    "@IMAP",
    "@ldap",
    "@mbreg",
    "@mysqli",
    "@oci",
    "@odbc",
    "@openssl",
    "@pcntl",
    "@pg",
    "@posix",
    "@snmp",
    "@sodium",
    "@time",
];
const DEFAULT_SETS: &[&str] = &["@internal", "@IMAP", "@pg"];

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoAliasFunctionsFixer::default())
}

impl NoAliasFunctionsFixer {
    /// configurePostNormalisation: array_merge of the chosen sets (string keys: later wins, position kept).
    fn set_sets(&mut self, sets: &[String]) {
        // array_merge: the sets share no alias, and a set listed twice only adds identical, already-fixed entries.
        let mut aliases: Vec<Alias> = Vec::new();
        for set in sets {
            if set == "@all" {
                aliases.clear();
                for (_, items) in SETS {
                    aliases.extend_from_slice(items);
                }
                break;
            }
            if let Some((_, items)) = SETS.iter().find(|(n, _)| n == set) {
                aliases.extend_from_slice(items);
            }
        }
        self.aliases = aliases;
    }
}

impl Fixer for NoAliasFunctionsFixer {
    fn name(&self) -> &'static str {
        "no_alias_functions"
    }
    fn priority(&self) -> i32 {
        40
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["sets"])?;
        let sets = c.strings("sets", DEFAULT_SETS, ALLOWED_SETS)?;
        self.set_sets(&sets);
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        for index in tokens.find_given_kind(T_STRING, 0, None) {
            let token_content = &tokens[index].content;
            let Some(&(_, alias, number_of_arguments)) = self
                .aliases
                .iter()
                .find(|a| a.0.as_bytes().eq_ignore_ascii_case(token_content))
            else {
                continue;
            };
            let open_parenthesis = tokens.get_next_meaningful_token(index).unwrap();
            if !tokens[open_parenthesis].equals("(", true) {
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            if let Some(n) = number_of_arguments {
                let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis);
                if arguments_analyzer.count_arguments(tokens, open_parenthesis, close) != n {
                    continue;
                }
            }
            tokens.set(index, Token::new(T_STRING, alias));
        }
    }
}

const SETS: &[(&str, &[Alias])] = &[
    (
        "@internal",
        &[
            ("diskfreespace", "disk_free_space", None),
            ("dns_check_record", "checkdnsrr", None),
            ("dns_get_mx", "getmxrr", None),
            ("session_commit", "session_write_close", None),
            ("stream_register_wrapper", "stream_wrapper_register", None),
            ("set_file_buffer", "stream_set_write_buffer", None),
            ("socket_set_blocking", "stream_set_blocking", None),
            ("socket_get_status", "stream_get_meta_data", None),
            ("socket_set_timeout", "stream_set_timeout", None),
            ("socket_getopt", "socket_get_option", None),
            ("socket_setopt", "socket_set_option", None),
            ("chop", "rtrim", None),
            ("close", "closedir", None),
            ("doubleval", "floatval", None),
            ("fputs", "fwrite", None),
            ("get_required_files", "get_included_files", None),
            ("ini_alter", "ini_set", None),
            ("is_double", "is_float", None),
            ("is_integer", "is_int", None),
            ("is_long", "is_int", None),
            ("is_real", "is_float", None),
            ("is_writeable", "is_writable", None),
            ("join", "implode", None),
            ("key_exists", "array_key_exists", None),
            ("magic_quotes_runtime", "set_magic_quotes_runtime", None),
            ("pos", "current", None),
            ("show_source", "highlight_file", None),
            ("sizeof", "count", None),
            ("strchr", "strstr", None),
            ("user_error", "trigger_error", None),
        ],
    ),
    (
        "@IMAP",
        &[
            ("imap_create", "imap_createmailbox", None),
            ("imap_fetchtext", "imap_body", None),
            ("imap_header", "imap_headerinfo", None),
            ("imap_listmailbox", "imap_list", None),
            ("imap_listsubscribed", "imap_lsub", None),
            ("imap_rename", "imap_renamemailbox", None),
            ("imap_scan", "imap_listscan", None),
            ("imap_scanmailbox", "imap_listscan", None),
        ],
    ),
    (
        "@ldap",
        &[
            ("ldap_close", "ldap_unbind", None),
            ("ldap_modify", "ldap_mod_replace", None),
        ],
    ),
    (
        "@mysqli",
        &[
            ("mysqli_execute", "mysqli_stmt_execute", None),
            ("mysqli_set_opt", "mysqli_options", None),
            ("mysqli_escape_string", "mysqli_real_escape_string", None),
        ],
    ),
    ("@pg", &[("pg_exec", "pg_query", None)]),
    ("@oci", &[("oci_free_cursor", "oci_free_statement", None)]),
    (
        "@odbc",
        &[
            ("odbc_do", "odbc_exec", None),
            ("odbc_field_precision", "odbc_field_len", None),
        ],
    ),
    (
        "@mbreg",
        &[
            ("mbereg", "mb_ereg", None),
            ("mbereg_match", "mb_ereg_match", None),
            ("mbereg_replace", "mb_ereg_replace", None),
            ("mbereg_search", "mb_ereg_search", None),
            ("mbereg_search_getpos", "mb_ereg_search_getpos", None),
            ("mbereg_search_getregs", "mb_ereg_search_getregs", None),
            ("mbereg_search_init", "mb_ereg_search_init", None),
            ("mbereg_search_pos", "mb_ereg_search_pos", None),
            ("mbereg_search_regs", "mb_ereg_search_regs", None),
            ("mbereg_search_setpos", "mb_ereg_search_setpos", None),
            ("mberegi", "mb_eregi", None),
            ("mberegi_replace", "mb_eregi_replace", None),
            ("mbregex_encoding", "mb_regex_encoding", None),
            ("mbsplit", "mb_split", None),
        ],
    ),
    (
        "@openssl",
        &[
            ("openssl_get_publickey", "openssl_pkey_get_public", None),
            ("openssl_get_privatekey", "openssl_pkey_get_private", None),
        ],
    ),
    (
        "@sodium",
        &[(
            "sodium_crypto_scalarmult_base",
            "sodium_crypto_box_publickey_from_secretkey",
            None,
        )],
    ),
    ("@exif", &[("read_exif_data", "exif_read_data", None)]),
    ("@ftp", &[("ftp_quit", "ftp_close", None)]),
    ("@posix", &[("posix_errno", "posix_get_last_error", None)]),
    ("@pcntl", &[("pcntl_errno", "pcntl_get_last_error", None)]),
    (
        "@time",
        &[("mktime", "time", Some(0)), ("gmmktime", "time", Some(0))],
    ),
];
