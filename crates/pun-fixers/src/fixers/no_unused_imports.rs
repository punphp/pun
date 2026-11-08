//! Port of `PhpCsFixer\Fixer\Import\NoUnusedImportsFixer`.
use crate::analyzers::analysis::{NamespaceUseAnalysis, NamespaceUseType};
use pun_tokens::NamespaceAnalysis;
use crate::analyzers::constant_invocation::is_constant_invocation;
use crate::analyzers::goto_label_analyzer::GotoLabelAnalyzer;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_tokens::NamespacesAnalyzer;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::php_compat::{ltrim_chars, rtrim_chars, strcasecmp};
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

const TOKENS_NOT_BEFORE_FUNCTION_CALL: &[Kind] = &[Kind::T_NEW, Kind::T_ATTRIBUTE];

struct NoUnusedImportsFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnusedImportsFixer)
}

impl Fixer for NoUnusedImportsFixer {
    fn name(&self) -> &'static str {
        "no_unused_imports"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_USE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // Upstream returns early when there are none, but a candidate's T_USE is always an import (closure and
        // trait `use` are retokenized), so there is at least one.
        let use_declarations = NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, true);
        for namespace in NamespacesAnalyzer.get_declarations(tokens) {
            let current: Vec<&NamespaceUseAnalysis> = use_declarations
                .iter()
                .filter(|u| {
                    u.start_index >= namespace.scope_start_index
                        && u.end_index <= namespace.scope_end_index
                })
                .collect();
            // `$ignoredIndices` (use statement start => end) as a dense jump table: looked up for every token of
            // the namespace once per import.
            let mut indices = vec![0usize; tokens.count()];
            for u in &current {
                indices[u.start_index] = u.end_index + 1;
            }
            for u in &current {
                if !is_import_used(tokens, &namespace, u, &indices) {
                    remove_use_declaration(tokens, u, false);
                }
            }
            remove_uses_in_same_namespace(tokens, &current, &namespace);
        }
    }
}

fn is_import_used(
    tokens: &Tokens,
    namespace: &NamespaceAnalysis,
    import: &NamespaceUseAnalysis,
    ignored: &[usize],
) -> bool {
    let end = namespace.scope_end_index;
    let mut in_attribute = false;
    let mut index = namespace.scope_start_index;
    let comment_pattern = [
        &br"/(?<![[:alnum:]\$_])(?<!\\)"[..],
        &import.short_name,
        br"(?![[:alnum:]_])/i",
    ]
    .concat();
    while index <= end {
        let token = &tokens[index];
        if token.is_kind(Kind::T_ATTRIBUTE) {
            in_attribute = true;
            index += 1;
            continue;
        }
        if token.is_kind(Kind::CT_ATTRIBUTE_CLOSE) {
            in_attribute = false;
            index += 1;
            continue;
        }
        if ignored[index] != 0 {
            index = ignored[index];
            continue;
        }
        if token.is_kind(Kind::T_STRING) {
            if strcasecmp(&import.short_name, token.get_content()) != 0 {
                index += 1;
                continue;
            }
            let prev = &tokens[tokens.get_prev_meaningful_token(index).unwrap()];
            if prev.is_kind(Kind::T_NAMESPACE) {
                index = tokens
                    .get_next_token_of_kind(
                        index,
                        &[";".into(), "{".into(), Kind::T_CLOSE_TAG.into()],
                        true,
                    )
                    .unwrap()
                    + 1;
                continue;
            }
            if prev.is_given_kind(&[Kind::T_NS_SEPARATOR, Kind::T_FUNCTION, Kind::T_DOUBLE_COLON])
                || prev.is_object_operator()
            {
                index += 1;
                continue;
            }
            if prev.is_kind(Kind::T_CONST)
                && tokens[tokens.get_next_meaningful_token(index).unwrap()].equals("=", true)
            {
                index += 1;
                continue;
            }
            if in_attribute {
                return true;
            }
            let next_index = tokens.get_next_meaningful_token(index).unwrap();
            if GotoLabelAnalyzer.belongs_to_go_to_label(tokens, next_index) {
                index += 1;
                continue;
            }
            let ty = if is_constant_invocation(tokens, index) {
                NamespaceUseType::Constant
            } else if tokens[next_index].equals("(", true)
                && !prev.is_given_kind(TOKENS_NOT_BEFORE_FUNCTION_CALL)
            {
                NamespaceUseType::Function
            } else {
                NamespaceUseType::Class
            };
            if import.type_ == ty {
                return true;
            }
            index += 1;
            continue;
        }
        if token.is_comment()
            && may_mention(token.get_content(), &import.short_name)
            && Preg::r#match(&comment_pattern, token.get_content())
        {
            return true;
        }
        index += 1;
    }
    false
}

fn remove_use_declaration(
    tokens: &mut Tokens,
    u: &NamespaceUseAnalysis,
    force_complete_removal: bool,
) {
    let (start, end) = if u.is_in_multi && !force_complete_removal {
        (u.chunk_start_index.unwrap(), u.chunk_end_index.unwrap())
    } else {
        (u.start_index, u.end_index)
    };
    let loop_start = if u.is_in_multi || force_complete_removal {
        end
    } else {
        end - 1
    };
    let mut index = loop_start as isize;
    while index >= start as isize {
        let i = index as usize;
        index -= 1;
        if tokens[i].is_comment() {
            continue;
        }
        if !tokens[i].is_whitespace(None) || !tokens[i].get_content().contains(&b'\n') {
            tokens.clear_at(i);
            continue;
        }
        let prev = tokens.get_prev_non_whitespace(i, None).unwrap();
        if tokens[prev].is_comment() {
            let content = tokens[i].get_content();
            let at = content.iter().rposition(|&c| c == b'\n').unwrap();
            let t = Token::new(Kind::T_WHITESPACE, content[at..].to_vec());
            tokens.set(i, t);
        } else {
            tokens.clear_token_and_merge_surrounding_whitespace(i);
        }
    }
    if !force_complete_removal && u.is_in_multi {
        clean_up_after_import_chunk_removal(tokens, u);
        return;
    }
    if tokens[u.end_index].equals(";", true) {
        tokens.clear_at(u.end_index);
    }
    clean_up_surrounding_new_lines(tokens, u);
}

fn remove_uses_in_same_namespace(
    tokens: &mut Tokens,
    uses: &[&NamespaceUseAnalysis],
    ns: &NamespaceAnalysis,
) {
    let prefix = [&ns.full_name[..], b"\\"].concat();
    for u in uses {
        if u.is_aliased {
            continue;
        }
        let full = ltrim_chars(&u.full_name, b"\\");
        if !full.starts_with(&prefix) {
            continue;
        }
        if !full[prefix.len()..].contains(&b'\\') {
            remove_use_declaration(tokens, u, false);
        }
    }
}

fn clean_up_after_import_chunk_removal(tokens: &mut Tokens, u: &NamespaceUseAnalysis) {
    let before = tokens
        .get_prev_meaningful_token(u.chunk_start_index.unwrap())
        .unwrap();
    let after = tokens
        .get_next_meaningful_token(u.chunk_end_index.unwrap())
        .unwrap();
    let has_before = scan_for_non_empty_tokens_until_new_line_found(tokens, after, -1);
    let has_after = scan_for_non_empty_tokens_until_new_line_found(tokens, after, 1);
    let safe = has_before.1 || has_after.1;
    let clear = |tokens: &mut Tokens, i: usize| {
        if safe {
            tokens.clear_token_and_merge_surrounding_whitespace(i);
        } else {
            tokens.clear_at(i);
        }
    };
    if tokens[after].equals(",", true) {
        clear(tokens, after);
    } else if tokens[before].equals(",", true) {
        clear(tokens, before);
    }
    for index in before..=after {
        if tokens[index].get_id().is_none() || !tokens[index].is_whitespace(Some(b" ")) {
            continue;
        }
        let next = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[next].equals(",", true)
            || tokens[next].equals(";", true)
            || tokens[next].is_kind(Kind::CT_GROUP_IMPORT_BRACE_CLOSE)
        {
            tokens.clear_at(index);
        } else {
            tokens.set(index, Token::new(Kind::T_WHITESPACE, b" ".to_vec()));
        }
        let prev = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev].is_kind(Kind::CT_GROUP_IMPORT_BRACE_OPEN) {
            tokens.clear_at(index);
        }
    }
    remove_line_if_empty(tokens, u);
    remove_import_statement_if_empty(tokens, u);
}

fn clean_up_surrounding_new_lines(tokens: &mut Tokens, u: &NamespaceUseAnalysis) {
    let prev_index = u.start_index - 1;
    if tokens[prev_index].is_whitespace(None) {
        let content = rtrim_chars(tokens[prev_index].get_content(), b" \t").to_vec();
        tokens.ensure_whitespace_at_index(prev_index, 0, &content);
    }
    // upstream returns separately for no token after the statement and for only empty ones
    let next = if u.end_index + 1 < tokens.count() {
        tokens.get_non_empty_sibling(u.end_index, 1)
    } else {
        None
    };
    let Some(next_index) = next else {
        return;
    };
    if tokens[next_index].is_whitespace(None) {
        let content = Preg::replace_limit(
            "#^\r\n|^\n#",
            "",
            ltrim_chars(tokens[next_index].get_content(), b" \t"),
            1,
        )
        .0;
        tokens.ensure_whitespace_at_index(next_index, 0, &content);
    }
    if tokens[prev_index].is_whitespace(None) && tokens[next_index].is_whitespace(None) {
        let content = [tokens[prev_index].get_content(), tokens[next_index].get_content()].concat();
        tokens.ensure_whitespace_at_index(next_index, 0, &content);
        tokens.clear_at(prev_index);
    }
}

fn remove_import_statement_if_empty(tokens: &mut Tokens, u: &NamespaceUseAnalysis) {
    let ending = tokens.get_prev_token_of_kind(
        u.end_index,
        &[Kind::CT_GROUP_IMPORT_BRACE_CLOSE.into()],
        true,
    );
    if let Some(e) = ending.filter(|&e| e > u.start_index) {
        let opening = tokens.get_prev_meaningful_token(e).unwrap();
        if tokens[opening].is_kind(Kind::CT_GROUP_IMPORT_BRACE_OPEN) {
            remove_use_declaration(tokens, u, true);
        }
    }
    let before_semicolon = tokens.get_prev_meaningful_token(u.end_index).unwrap();
    let t = &tokens[before_semicolon];
    if t.is_kind(Kind::T_USE) || t.get_content() == b"function" || t.get_content() == b"const" {
        remove_use_declaration(tokens, u, true);
    }
}

/// Upstream first returns for non-multi imports; the only caller handles multi-import chunks.
fn remove_line_if_empty(tokens: &mut Tokens, u: &NamespaceUseAnalysis) {
    let before =
        scan_for_non_empty_tokens_until_new_line_found(tokens, u.chunk_start_index.unwrap(), -1);
    let after =
        scan_for_non_empty_tokens_until_new_line_found(tokens, u.chunk_end_index.unwrap(), 1);
    if let (Some(b), false, Some(a), false) = (before.0, before.1, after.0, after.1) {
        tokens.clear_range(b, a - 1);
    }
}

fn scan_for_non_empty_tokens_until_new_line_found(
    tokens: &Tokens,
    index: usize,
    direction: isize,
) -> (Option<usize>, bool) {
    let mut has_non_empty = false;
    let mut index = Some(index);
    while let Some(i) = index {
        index = tokens.get_non_empty_sibling(i, direction);
        let Some(i) = index else { continue };
        if tokens[i].get_id().is_none() {
            continue;
        }
        if !tokens[i].is_whitespace(None) {
            has_non_empty = true;
        } else if tokens[i].get_content().starts_with(b"\n") {
            return (Some(i), has_non_empty);
        }
    }
    (None, has_non_empty)
}

/// False only when the `/i` comment pattern cannot match: an all-ASCII comment without the name in any ASCII case.
/// (Non-ASCII text goes to the regex, whose `/iu` folding also matches e.g. the Kelvin sign for `k`.)
fn may_mention(comment: &[u8], name: &[u8]) -> bool {
    !comment.is_ascii()
        || name.is_empty()
        || comment
            .windows(name.len())
            .any(|w| w[0].eq_ignore_ascii_case(&name[0]) && w.eq_ignore_ascii_case(name))
}
