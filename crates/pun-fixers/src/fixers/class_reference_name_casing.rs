//! Port of `PhpCsFixer\Fixer\Casing\ClassReferenceNameCasingFixer`.
use pun_tokens::NamespaceAnalysis;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_tokens::NamespacesAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{BlockType, Kind, Proto, Token, Tokens};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

/// Internal classes of the oracle's PHP (`get_declared_classes()` filtered by `ReflectionClass::isInternal`). Regenerate:
/// `php -r '$c=[];foreach(get_declared_classes() as $x){if((new ReflectionClass($x))->isInternal())$c[]=$x;} sort($c); echo implode("\n",$c),"\n";'`
static CLASS_NAMES: LazyLock<HashMap<Vec<u8>, &'static str>> = LazyLock::new(|| {
    include_str!("../analyzers/php_internal_classes.txt")
        .lines()
        .map(|c| (strtolower(c.as_bytes()), c))
        .collect()
});

const NOT_BEFORE_KINDS: [Kind; 13] = [
    CT_USE_TRAIT,
    T_AS,
    T_CASE, // PHP 8.1 trait enum-case
    T_CLASS,
    T_CONST,
    T_DOUBLE_ARROW,
    T_DOUBLE_COLON,
    T_FUNCTION,
    T_INTERFACE,
    T_OBJECT_OPERATOR,
    T_TRAIT,
    T_NULLSAFE_OBJECT_OPERATOR,
    T_ENUM,
];

struct ClassReferenceNameCasingFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ClassReferenceNameCasingFixer)
}

impl Fixer for ClassReferenceNameCasingFixer {
    fn name(&self) -> &'static str {
        "class_reference_name_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for namespace in NamespacesAnalyzer.get_declarations(tokens) {
            let uses: HashSet<Vec<u8>> = NamespaceUsesAnalyzer
                .get_declarations_in_namespace(tokens, &namespace, false)
                .iter()
                .map(|u| strtolower(&u.short_name))
                .collect();
            for reference in get_class_reference(tokens, &namespace) {
                let current = tokens[reference].get_content();
                let lower = strtolower(current);
                if let Some(&name) = CLASS_NAMES.get(&lower)
                    && current != name.as_bytes()
                    && !uses.contains(&lower)
                {
                    tokens.set(reference, Token::new(T_STRING, name));
                }
            }
        }
    }
}

fn get_class_reference(tokens: &Tokens, namespace: &NamespaceAnalysis) -> Vec<usize> {
    let mut before: Vec<Proto> = vec![",".into()];
    let mut after: Vec<Proto> = vec![",".into()];
    for t in BlockType::ALL {
        let (start, end) = Tokens::block_edges(t);
        before.push(start);
        after.push(end);
    }
    let namespace_is_global = namespace.is_global_namespace();
    let mut references = vec![];
    for index in namespace.scope_start_index..namespace.scope_end_index {
        if !tokens[index].is_kind(T_STRING) {
            continue;
        }
        let next_index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[next_index].is_kind(T_NS_SEPARATOR) {
            continue;
        }
        let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        let is_namespace_separator = tokens[prev_index].is_kind(T_NS_SEPARATOR);
        if !is_namespace_separator && !namespace_is_global {
            continue;
        }
        if is_namespace_separator {
            prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
            if tokens[prev_index].is_kind(T_STRING) {
                continue;
            }
        } else if tokens[prev_index].is_given_kind(&NOT_BEFORE_KINDS) {
            continue;
        }
        if tokens[prev_index].equals_any(&before, true)
            && tokens[next_index].equals_any(&after, true)
        {
            continue;
        }
        if !tokens[prev_index].is_kind(T_NEW)
            && tokens[next_index].equals_any(
                &["(".into(), ";".into(), "=".into(), T_CLOSE_TAG.into()],
                true,
            )
        {
            continue;
        }
        references.push(index);
    }
    references
}
