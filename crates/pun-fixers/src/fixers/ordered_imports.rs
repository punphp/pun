//! Port of `PhpCsFixer\Fixer\Import\OrderedImportsFixer`.
use crate::analyzers::analysis::NamespaceUseType;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::natural_language_join;
use anyhow::{Result, bail};
use indexmap::IndexMap;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::php_compat::{str_replace, strcasecmp, trim};
use pun_tokens::{Kind, Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::cmp::Ordering;
use std::path::Path;

const SUPPORTED_SORT_TYPES: &[&str] = &["class", "const", "function"];

#[derive(Clone, Copy, PartialEq)]
enum Sort {
    Alpha,
    Length,
    None,
}

struct OrderedImportsFixer {
    sort_algorithm: Sort,
    imports_order: Option<Vec<NamespaceUseType>>,
    case_sensitive: bool,
    ws: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    // `configure` sets the options.
    crate::configured(OrderedImportsFixer {
        sort_algorithm: Sort::None,
        imports_order: None,
        case_sensitive: false,
        ws: WhitespacesFixerConfig::default(),
    })
}

#[derive(Clone)]
struct Item {
    namespace: Vec<u8>,
    start_index: usize,
    end_index: usize,
    import_type: NamespaceUseType,
    group: bool,
}

impl Fixer for OrderedImportsFixer {
    fn name(&self) -> &'static str {
        "ordered_imports"
    }
    fn priority(&self) -> i32 {
        -30
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.ws = config.clone();
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["sort_algorithm", "imports_order", "case_sensitive"],
        )?;
        self.sort_algorithm = r.choice(
            "sort_algorithm",
            "alpha",
            &[
                ("alpha", Sort::Alpha),
                ("length", Sort::Length),
                ("none", Sort::None),
            ],
        )?;
        self.imports_order = match r.get("imports_order") {
            None | Some(Value::Null) => None,
            Some(_) => {
                let v = r.strings("imports_order", &[], &[])?;
                let missing: Vec<&str> = SUPPORTED_SORT_TYPES
                    .iter()
                    .copied()
                    .filter(|t| !v.iter().any(|x| x == t))
                    .collect();
                if !missing.is_empty() {
                    bail!(
                        "[ordered_imports] Invalid configuration: Missing sort {} {}.",
                        if missing.len() == 1 { "type" } else { "types" },
                        natural_language_join(&missing)
                    );
                }
                let unknown: Vec<&str> = v
                    .iter()
                    .map(String::as_str)
                    .filter(|x| !SUPPORTED_SORT_TYPES.contains(x))
                    .collect();
                if !unknown.is_empty() {
                    bail!(
                        "[ordered_imports] Invalid configuration: Unknown sort {} {}.",
                        if unknown.len() == 1 { "type" } else { "types" },
                        natural_language_join(&unknown)
                    );
                }
                let types = [
                    NamespaceUseType::Class,
                    NamespaceUseType::Function,
                    NamespaceUseType::Constant,
                ];
                Some(
                    v.iter()
                        .map(|x| *types.iter().find(|t| t.name() == x).unwrap())
                        .collect(),
                )
            }
        };
        self.case_sensitive = r.bool("case_sensitive", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_USE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let namespaces_imports = TokensAnalyzer::new(tokens).get_import_use_indexes(true);
        for uses in namespaces_imports.values().rev() {
            let count = uses.len();
            if count == 1 {
                let order = self.get_new_order(uses, tokens);
                self.set_new_order(tokens, order);
                continue;
            }
            let mut group_uses: Vec<Vec<usize>> = vec![vec![uses[0]]];
            for index in 0..count - 1 {
                let mut next_group_use = tokens
                    .get_next_token_of_kind(
                        uses[index],
                        &[";".into(), Kind::T_CLOSE_TAG.into()],
                        true,
                    )
                    .unwrap();
                if tokens[next_group_use].is_kind(Kind::T_CLOSE_TAG) {
                    next_group_use = tokens
                        .get_next_token_of_kind(uses[index], &[Kind::T_OPEN_TAG.into()], true)
                        .unwrap();
                }
                let next_group_use = tokens.get_next_meaningful_token(next_group_use);
                if next_group_use != Some(uses[index + 1]) {
                    group_uses.push(vec![]);
                }
                group_uses.last_mut().unwrap().push(uses[index + 1]);
            }
            for g in group_uses.iter().rev() {
                let order = self.get_new_order(g, tokens);
                self.set_new_order(tokens, order);
            }
        }
    }
}

impl OrderedImportsFixer {
    fn cmp_str(&self, a: &[u8], b: &[u8]) -> Ordering {
        if self.case_sensitive {
            a.cmp(b)
        } else {
            strcasecmp(a, b).cmp(&0)
        }
    }

    /// sortAlphabetically's operand: the prepared namespace with `\\` as a space and `{` dropped.
    fn alphabetical_key(item: &Item) -> Vec<u8> {
        str_replace(b"{", b"", &str_replace(b"\\", b" ", &prepare_namespace(&item.namespace)))
    }

    /// sortByLength's operand: the import type (but `class`), a space, and the prepared namespace.
    fn length_key(item: &Item) -> Vec<u8> {
        let p = if item.import_type == NamespaceUseType::Class {
            vec![]
        } else {
            [item.import_type.name().as_bytes(), b" "].concat()
        };
        [p, prepare_namespace(&item.namespace)].concat()
    }

    fn sort_by_length(&self, a: &[u8], b: &[u8]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| self.cmp_str(a, b))
    }

    /// usort with the algorithm's comparison; each item's operand is computed once instead of per comparison.
    fn sort_by_algorithm(&self, indices: &mut [Item]) {
        let key = match self.sort_algorithm {
            Sort::Alpha => Self::alphabetical_key,
            Sort::Length => Self::length_key,
            Sort::None => return,
        };
        let keys: Vec<Vec<u8>> = indices.iter().map(key).collect();
        let mut order: Vec<usize> = (0..indices.len()).collect();
        order.sort_by(|&a, &b| {
            let (a, b) = (&keys[a], &keys[b]);
            if self.sort_algorithm == Sort::Length {
                self.sort_by_length(a, b)
            } else {
                self.cmp_str(a, b)
            }
        });
        let sorted: Vec<Item> = order.iter().map(|&i| indices[i].clone()).collect();
        indices.clone_from_slice(&sorted);
    }

    /// Returns `(original start index, item)` pairs in upstream's `$usesOrder` order.
    fn get_new_order(&self, uses: &[usize], tokens: &Tokens) -> Vec<(usize, Item)> {
        let mut indices: Vec<Item> = vec![];
        let mut original_indices: Vec<usize> = vec![];
        let line_ending = self.ws.get_line_ending().as_bytes();
        for &use_index in uses {
            let mut start_index = tokens
                .get_token_not_of_kinds_sibling(use_index + 1, 1, &[Kind::T_WHITESPACE])
                .unwrap();
            let end_index = tokens
                .get_next_token_of_kind(start_index, &[";".into(), Kind::T_CLOSE_TAG.into()], true)
                .unwrap();
            let previous = tokens.get_prev_meaningful_token(end_index).unwrap();
            let group = tokens[previous].is_kind(Kind::CT_GROUP_IMPORT_BRACE_CLOSE);
            let (ty, mut index) = if tokens[start_index].is_kind(Kind::CT_CONST_IMPORT) {
                (
                    NamespaceUseType::Constant,
                    tokens.get_next_non_whitespace(start_index, None).unwrap(),
                )
            } else if tokens[start_index].is_kind(Kind::CT_FUNCTION_IMPORT) {
                (
                    NamespaceUseType::Function,
                    tokens.get_next_non_whitespace(start_index, None).unwrap(),
                )
            } else {
                (NamespaceUseType::Class, start_index)
            };
            let mut namespace_tokens: Vec<Token> = vec![];
            while index <= end_index {
                let token = &tokens[index];
                if index == end_index || (!group && token.equals(",", true)) {
                    let namespace = if group && self.sort_algorithm != Sort::None {
                        self.sort_group(&namespace_tokens, line_ending)
                    } else {
                        namespace_tokens
                            .iter()
                            .flat_map(|t| t.content.clone())
                            .collect()
                    };
                    indices.push(Item {
                        namespace,
                        start_index,
                        end_index: index - 1,
                        import_type: ty,
                        group,
                    });
                    original_indices.push(start_index);
                    if index == end_index {
                        break;
                    }
                    namespace_tokens.clear();
                    let next_part_index = tokens
                        .get_token_not_of_kind_sibling(
                            index,
                            1,
                            &[",".into(), Kind::T_WHITESPACE.into()],
                        )
                        .unwrap();
                    start_index = next_part_index;
                    index = next_part_index;
                    continue;
                }
                namespace_tokens.push(token.clone());
                index += 1;
            }
        }

        let indices: Vec<Item> = match &self.imports_order {
            Some(order) => {
                let mut grouped: IndexMap<NamespaceUseType, Vec<Item>> = IndexMap::new();
                for item in indices {
                    grouped.entry(item.import_type).or_default().push(item);
                }
                for v in grouped.values_mut() {
                    self.sort_by_algorithm(v);
                }
                let mut sorted = vec![];
                for t in order {
                    if let Some(v) = grouped.get(t) {
                        sorted.extend(v.iter().cloned());
                    }
                }
                sorted
            }
            None => {
                let mut v = indices;
                self.sort_by_algorithm(&mut v);
                v
            }
        };
        original_indices.into_iter().zip(indices).collect()
    }

    fn sort_group(&self, namespace_tokens: &[Token], line_ending: &[u8]) -> Vec<u8> {
        let count = namespace_tokens.len() as isize - 1;
        let mut namespace = vec![];
        let mut k: isize = 0;
        while k < count {
            let t = &namespace_tokens[k as usize];
            if t.is_kind(Kind::CT_GROUP_IMPORT_BRACE_OPEN) {
                namespace.push(b'{');
                break;
            }
            namespace.extend_from_slice(&t.content);
            k += 1;
        }
        let mut parts: Vec<Vec<u8>> = vec![];
        let mut first_indent: Vec<u8> = vec![];
        let mut separator: Vec<u8> = b", ".to_vec();
        let mut last_indent: Vec<u8> = vec![];
        let mut has_group_trailing_comma = false;
        let mut k1 = k + 1;
        while k1 < count {
            let mut comment = vec![];
            let mut part = vec![];
            let mut k2 = k1;
            loop {
                let t = &namespace_tokens[k2 as usize];
                if t.equals_any(
                    &[",".into(), Proto::Kind(Kind::CT_GROUP_IMPORT_BRACE_CLOSE)],
                    true,
                ) {
                    break;
                }
                if t.is_comment() {
                    comment.extend_from_slice(&t.content);
                    k2 += 1;
                    continue;
                }
                if first_indent.is_empty()
                    && t.is_whitespace(None)
                    && t.content
                        .windows(line_ending.len())
                        .any(|w| w == line_ending)
                {
                    last_indent = line_ending.to_vec();
                    first_indent = [line_ending, self.ws.get_indent().as_bytes()].concat();
                    separator = [&b","[..], &first_indent].concat();
                }
                part.extend_from_slice(&t.content);
                k2 += 1;
            }
            let part = trim(&part).to_vec();
            if part.is_empty() {
                has_group_trailing_comma = true;
                k1 += 1;
                continue;
            }
            let comment = trim(&comment).to_vec();
            let mut part = part;
            if !comment.is_empty() {
                part.push(b' ');
                part.extend(comment);
            }
            parts.push(part);
            k1 = k2 + 1;
        }
        let before = parts.clone();
        parts.sort();
        if before == parts {
            return namespace_tokens
                .iter()
                .flat_map(|t| t.content.clone())
                .collect();
        }
        namespace.extend(first_indent);
        namespace.extend(parts.join(&separator[..]));
        if has_group_trailing_comma {
            namespace.push(b',');
        }
        namespace.extend(last_indent);
        namespace.push(b'}');
        namespace
    }

    fn set_new_order(&self, tokens: &mut Tokens, uses_order: Vec<(usize, Item)>) {
        let map_start_to_end: Vec<(usize, usize)> = uses_order
            .iter()
            .map(|(_, u)| (u.start_index, u.end_index))
            .collect();
        for (index, u) in uses_order.iter().rev() {
            let index = *index;
            let code = [
                &b"<?php use "[..],
                &(if u.import_type == NamespaceUseType::Class {
                    vec![]
                } else {
                    [&b" "[..], u.import_type.name().as_bytes(), b" "].concat()
                }),
                &u.namespace,
                b";",
            ]
            .concat();
            let mut n = 3;
            if u.import_type != NamespaceUseType::Class
                && tokens[tokens.get_prev_meaningful_token(index).unwrap()].equals(",", true)
            {
                n = 5;
            }
            let mut decl = Tokens::from_code(&code);
            decl.clear_range(0, n - 1);
            let last = decl.count() - 1;
            decl.clear_at(last);
            decl.clear_empty_tokens();
            let end = map_start_to_end
                .iter()
                .find(|(s, _)| *s == index)
                .unwrap()
                .1;
            tokens.override_range(index, end, decl.as_slice().to_vec());
            if u.group {
                let prev = tokens.get_prev_meaningful_token(index).unwrap();
                fix_comma_to_use(tokens, prev);
                let close = tokens
                    .get_next_token_of_kind(
                        index,
                        &[Kind::CT_GROUP_IMPORT_BRACE_CLOSE.into()],
                        true,
                    )
                    .unwrap();
                let next = tokens.get_next_meaningful_token(close).unwrap();
                fix_comma_to_use(tokens, next);
            }
        }
    }
}

fn prepare_namespace(ns: &[u8]) -> Vec<u8> {
    trim(&Preg::replace(r"%/\*(.*)\*/%s", "", ns)).to_vec()
}

fn fix_comma_to_use(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals(",", true) {
        return;
    }
    tokens.set(index, Token::char(";"));
    tokens.insert_at(index + 1, vec![Token::new(Kind::T_USE, b"use".to_vec())]);
    if !tokens[index + 2].is_whitespace(None) {
        tokens.insert_at(
            index + 2,
            vec![Token::new(Kind::T_WHITESPACE, b" ".to_vec())],
        );
    }
}
