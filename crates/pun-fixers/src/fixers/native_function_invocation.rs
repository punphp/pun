//! Port of `PhpCsFixer\Fixer\FunctionNotation\NativeFunctionInvocationFixer`. `@internal` is PHP's
//! `get_defined_functions()['internal']` as the oracle sees it (analyzers/php_functions.rs).
use crate::analyzers::fixer_configuration_resolver::{Resolved, resolve};
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::php_functions::is_internal_function;
use pun_format::Fixer;
use pun_tokens::php_compat::{strtolower, trim};
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

const SET_ALL: &str = "@all";
const SET_INTERNAL: &str = "@internal";
const SET_COMPILER_OPTIMIZED: &str = "@compiler_optimized";

/// `getAllCompilerOptimizedFunctionsNormalized`.
const COMPILER_OPTIMIZED: &[&str] = &[
    // @see https://github.com/php/php-src/blob/PHP-7.4/Zend/zend_compile.c "zend_try_compile_special_func"
    "array_key_exists",
    "array_slice",
    "assert",
    "boolval",
    "call_user_func",
    "call_user_func_array",
    "chr",
    "count",
    "defined",
    "doubleval",
    "floatval",
    "func_get_args",
    "func_num_args",
    "get_called_class",
    "get_class",
    "gettype",
    "in_array",
    "intval",
    "is_array",
    "is_bool",
    "is_double",
    "is_float",
    "is_int",
    "is_integer",
    "is_long",
    "is_null",
    "is_object",
    "is_real",
    "is_resource",
    "is_scalar",
    "is_string",
    "ord",
    "sizeof",
    "sprintf",
    "strlen",
    "strval",
    // @see https://github.com/php/php-src/blob/php-7.2.6/ext/opcache/Optimizer/pass1_5.c
    // @see https://github.com/php/php-src/blob/PHP-8.1.2/Zend/Optimizer/block_pass.c
    // @see https://github.com/php/php-src/blob/php-8.1.3/Zend/Optimizer/zend_optimizer.c
    "constant",
    "define",
    "dirname",
    "extension_loaded",
    "function_exists",
    "is_callable",
    "ini_get",
];

/// The resolved `include` base set (`getFunctionFilter`).
#[derive(Default)]
enum Base {
    All,
    Internal,
    CompilerOptimized,
    #[default]
    None,
}

#[derive(Default)]
struct NativeFunctionInvocationFixer {
    base: Base,
    include: HashSet<Vec<u8>>,
    exclude: HashSet<Vec<u8>>,
    scope_all: bool,
    strict: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NativeFunctionInvocationFixer::default())
}

/// The allowed-values closure shared by `exclude` and `include`.
fn trimmed_names(
    c: &Resolved,
    fixer: &str,
    key: &str,
    default: &[&str],
) -> anyhow::Result<Vec<String>> {
    let names = c.strings(key, default, &[])?;
    for name in &names {
        if name.is_empty() || trim(name.as_bytes()) != name.as_bytes() {
            anyhow::bail!(
                "[{fixer}] Invalid configuration: Each element must be a non-empty, trimmed string, got \"string\" instead."
            );
        }
    }
    Ok(names)
}

impl NativeFunctionInvocationFixer {
    /// The `getFunctionFilter` closure.
    fn function_filter(&self, function_name: &[u8]) -> bool {
        let name = strtolower(function_name);
        let included = match self.base {
            Base::All => true,
            Base::Internal => is_internal_function(&name),
            Base::CompilerOptimized => COMPILER_OPTIMIZED.iter().any(|f| f.as_bytes() == name),
            Base::None => false,
        };
        (included || self.include.contains(&name)) && !self.exclude.contains(&name)
    }

    fn fix_function_calls(
        &self,
        tokens: &mut Tokens,
        start: usize,
        end: usize,
        try_to_remove: bool,
    ) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let mut tokens_to_insert = BTreeMap::new();
        for index in start..end {
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !self.function_filter(tokens[index].get_content()) || try_to_remove {
                if self.strict && tokens[prev_index].is_kind(T_NS_SEPARATOR) {
                    tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
                }
                continue;
            }
            if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
                continue; // do not bother if previous token is already namespace separator
            }
            tokens_to_insert.insert(index, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
        tokens.insert_slices(tokens_to_insert);
    }
}

impl Fixer for NativeFunctionInvocationFixer {
    fn name(&self) -> &'static str {
        "native_function_invocation"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let name = self.name();
        let c = resolve(name, options, &["exclude", "include", "scope", "strict"])?;
        let exclude = trimmed_names(&c, name, "exclude", &[])?;
        let include = trimmed_names(&c, name, "include", &[SET_COMPILER_OPTIMIZED])?;
        let sets = [SET_ALL, SET_INTERNAL, SET_COMPILER_OPTIMIZED];
        if let Some(unknown) = include
            .iter()
            .find(|f| f.starts_with('@') && !sets.contains(&f.as_str()))
        {
            anyhow::bail!(
                "[{name}] Invalid configuration: Unknown set \"{unknown}\", known sets are \"@all\", \"@internal\" and \"@compiler_optimized\"."
            );
        }
        self.scope_all = c.string("scope", "all", &["all", "namespaced"])? == "all";
        self.strict = c.bool("strict", true)?;
        let has = |set: &str| include.iter().any(|f| f == set);
        self.base = if has(SET_ALL) {
            Base::All
        } else if has(SET_INTERNAL) {
            Base::Internal
        } else if has(SET_COMPILER_OPTIMIZED) {
            Base::CompilerOptimized
        } else {
            Base::None
        };
        self.include = include
            .iter()
            .filter(|f| !f.starts_with('@'))
            .map(|f| strtolower(f.as_bytes()))
            .collect();
        self.exclude = exclude.iter().map(|f| strtolower(f.as_bytes())).collect();
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.scope_all {
            let end = tokens.count() - 1;
            self.fix_function_calls(tokens, 0, end, false);
            return;
        }
        // 'scope' is 'namespaced' here
        let namespaces = tokens.namespace_declarations();
        for namespace in namespaces.iter().rev() {
            self.fix_function_calls(
                tokens,
                namespace.scope_start_index,
                namespace.scope_end_index,
                namespace.is_global_namespace(),
            );
        }
    }
}
