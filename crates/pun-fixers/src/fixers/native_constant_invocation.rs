//! Port of `PhpCsFixer\Fixer\ConstantNotation\NativeConstantInvocationFixer`. `fix_built_in` reads PHP's
//! `get_defined_constants(true)` as the oracle sees it (analyzers/php_constants.rs).
use crate::analyzers::constant_invocation::is_constant_invocation;
use crate::analyzers::fixer_configuration_resolver::{Resolved, resolve};
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use crate::analyzers::php_constants::INTERNAL_CONSTANTS;
use pun_format::Fixer;
use pun_tokens::php_compat::{strtolower, trim};
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;

const CASE_INSENSITIVE: &[&[u8]] = &[b"null", b"false", b"true"];

#[derive(Default)]
struct NativeConstantInvocationFixer {
    fix_built_in: bool,
    /// `include` minus `exclude` (case-sensitive), without the `null`/`false`/`true` spellings.
    include: HashSet<Vec<u8>>,
    exclude: HashSet<Vec<u8>>,
    /// `caseInsensitiveConstantsToEscape`: lowercase `null`/`false`/`true`.
    case_insensitive: HashSet<Vec<u8>>,
    scope_all: bool,
    strict: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NativeConstantInvocationFixer::default())
}

/// The `$constantChecker` allowed-values closure (an empty string passes, unlike native_function_invocation's).
fn trimmed_names(
    c: &Resolved,
    fixer: &str,
    key: &str,
    default: &[&str],
) -> anyhow::Result<Vec<String>> {
    let names = c.strings(key, default, &[])?;
    if names.iter().any(|n| trim(n.as_bytes()) != n.as_bytes()) {
        anyhow::bail!(
            "[{fixer}] Invalid configuration: Each element must be a non-empty, trimmed string, got \"string\" instead."
        );
    }
    Ok(names)
}

impl NativeConstantInvocationFixer {
    /// `isset($this->constantsToEscape[$content])`.
    fn is_constant_to_escape(&self, content: &[u8]) -> bool {
        let built_in = self.fix_built_in
            && std::str::from_utf8(content).is_ok_and(|c| INTERNAL_CONSTANTS.contains(c));
        (built_in || self.include.contains(content))
            && !self.exclude.contains(content)
            && !CASE_INSENSITIVE.contains(&&strtolower(content)[..])
    }

    fn fix_constant_invocations(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let use_constant_declarations: HashSet<Vec<u8>> = NamespaceUsesAnalyzer
            .get_declarations_from_tokens(tokens, false)
            .into_iter()
            .filter(|u| u.is_constant())
            .map(|u| u.short_name)
            .collect();
        let mut index = end_index;
        while index > start_index {
            let i = index;
            index -= 1;
            // test if we are at a constant call
            if !tokens[i].is_kind(T_STRING) || !is_constant_invocation(tokens, i) {
                continue;
            }
            let token_content = tokens[i].get_content();
            let prev_index = tokens.get_prev_meaningful_token(i).unwrap();
            if !self.is_constant_to_escape(token_content)
                && !self.case_insensitive.contains(&strtolower(token_content))
            {
                if !self.strict || !tokens[prev_index].is_kind(T_NS_SEPARATOR) {
                    continue;
                }
                let prev_prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
                if tokens[prev_prev_index].is_kind(T_STRING) {
                    continue;
                }
                tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
                continue;
            }
            if use_constant_declarations.contains(token_content)
                || tokens[prev_index].is_kind(T_NS_SEPARATOR)
            {
                continue;
            }
            tokens.insert_at(i, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
    }
}

impl Fixer for NativeConstantInvocationFixer {
    fn name(&self) -> &'static str {
        "native_constant_invocation"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let name = self.name();
        let c = resolve(
            name,
            options,
            &["fix_built_in", "include", "exclude", "scope", "strict"],
        )?;
        self.fix_built_in = c.bool("fix_built_in", true)?;
        let include = trimmed_names(&c, name, "include", &[])?;
        let exclude = trimmed_names(&c, name, "exclude", &["null", "false", "true"])?;
        self.scope_all = c.string("scope", "all", &["all", "namespaced"])? == "all";
        self.strict = c.bool("strict", true)?;
        self.exclude = exclude.iter().map(|e| e.as_bytes().to_vec()).collect();
        self.include = include.iter().map(|i| i.as_bytes().to_vec()).collect();
        // Case-insensitive constants handling: every `null`/`false`/`true` spelling left after the case-sensitive
        // exclude, lowered, minus the lowered exclude.
        let lowered_exclude: HashSet<Vec<u8>> =
            exclude.iter().map(|e| strtolower(e.as_bytes())).collect();
        let built_ins = INTERNAL_CONSTANTS.iter().filter(|_| self.fix_built_in);
        self.case_insensitive = include
            .iter()
            .map(String::as_str)
            .chain(built_ins.copied())
            .filter(|c| !self.exclude.contains(c.as_bytes()))
            .map(|c| strtolower(c.as_bytes()))
            .filter(|c| CASE_INSENSITIVE.contains(&&c[..]) && !lowered_exclude.contains(c))
            .collect();
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.scope_all {
            let end = tokens.count() - 1;
            self.fix_constant_invocations(tokens, 0, end);
            return;
        }
        // 'scope' is 'namespaced' here
        let namespaces = tokens.namespace_declarations();
        for namespace in namespaces.iter().rev() {
            if namespace.is_global_namespace() {
                continue;
            }
            self.fix_constant_invocations(
                tokens,
                namespace.scope_start_index,
                namespace.scope_end_index,
            );
        }
    }
}
