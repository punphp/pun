//! Port of `PhpCsFixer\Fixer\Import\GroupImportFixer`.
use crate::analyzers::analysis::{NamespaceUseAnalysis, NamespaceUseType};
use crate::analyzers::docblock::type_expression::php_spaceship;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::{rtrim_chars, strrpos};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

const ALLOWED_TYPES: &[&str] = &["classy", "functions", "constants"];

struct GroupImportFixer {
    group_types: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(GroupImportFixer {
        group_types: ALLOWED_TYPES.iter().map(|s| s.to_string()).collect(),
    })
}

impl Fixer for GroupImportFixer {
    fn name(&self) -> &'static str {
        "group_import"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_USE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["group_types"])?;
        let types = o.strings("group_types", ALLOWED_TYPES, &[])?;
        if let Some(bad) = types.iter().find(|t| !ALLOWED_TYPES.contains(&t.as_str())) {
            bail!(
                "[group_import] Invalid configuration: Invalid group type: {bad}, allowed types: \"classy\", \"functions\" and \"constants\"."
            );
        }
        self.group_types = types;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let by_type = get_same_namespaces_by_type(tokens);
        let enabled = |type_: NamespaceUseType| {
            let group = match type_ {
                NamespaceUseType::Class => "classy",
                NamespaceUseType::Function => "functions",
                NamespaceUseType::Constant => "constants",
            };
            self.group_types.iter().any(|t| t == group)
        };
        // As a first step we need to remove all the use statements for the enabled import types.
        // We can't add new group imports yet, because we need to operate on previously determined token indices for all types.
        for (type_, uses) in &by_type {
            if enabled(*type_) {
                remove_single_use_statements(uses, tokens);
            }
        }
        for (type_, uses) in &by_type {
            if enabled(*type_) {
                add_group_use_statements(uses, tokens);
            }
        }
    }
}

/// The imports sharing a namespace with another import of the same type, by type (`ksort`ed).
fn get_same_namespaces_by_type(
    tokens: &Tokens,
) -> Vec<(NamespaceUseType, Vec<NamespaceUseAnalysis>)> {
    let declarations = NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, false);
    let key = |u: &NamespaceUseAnalysis| (namespace_name_with_slash(u).to_vec(), u.type_);
    let mut same: Vec<NamespaceUseAnalysis> = (declarations.iter())
        .filter(|d| declarations.iter().filter(|e| key(e) == key(d)).count() > 1)
        .cloned()
        .collect();
    same.sort_by(|a, b| {
        let (na, nb) = (namespace_name_with_slash(a), namespace_name_with_slash(b));
        na.len()
            .cmp(&nb.len())
            .then_with(|| php_spaceship(&a.full_name, &b.full_name))
    });
    let mut by_type: Vec<(NamespaceUseType, Vec<NamespaceUseAnalysis>)> = vec![];
    for analysis in same {
        match by_type.iter_mut().find(|(t, _)| *t == analysis.type_) {
            Some((_, list)) => list.push(analysis),
            None => by_type.push((analysis.type_, vec![analysis])),
        }
    }
    by_type.sort_by_key(|(t, _)| *t as u8);
    by_type
}

const USE_STATEMENT_TOKENS: &[Kind] = &[
    T_USE,
    T_WHITESPACE,
    T_STRING,
    T_NS_SEPARATOR,
    T_AS,
    CT_CONST_IMPORT,
    CT_FUNCTION_IMPORT,
];

fn remove_single_use_statements(statements: &[NamespaceUseAnalysis], tokens: &mut Tokens) {
    for declaration in statements {
        let mut index = declaration.start_index;
        while index != declaration.end_index {
            if tokens[index].is_given_kind(USE_STATEMENT_TOKENS) {
                tokens.clear_at(index);
            }
            index += 1;
        }
        if index < tokens.count() && tokens[index].equals(";", true) {
            tokens.clear_at(index);
        }
        index += 1;
        if index < tokens.count() && tokens[index].is_kind(T_WHITESPACE) {
            tokens.clear_at(index);
        }
    }
}

fn is_cleared(token: &Token) -> bool {
    token.get_id().is_none() && token.get_content().is_empty()
}

fn add_group_use_statements(statements: &[NamespaceUseAnalysis], tokens: &mut Tokens) {
    let mut current: Option<&NamespaceUseAnalysis> = None;
    let mut insert_index = statements[0].start_index;
    // If group import was inserted in place of removed imports, it may have more tokens than before,
    // and indices stored in imports of another type can be out-of-sync, and can point in the middle of group import.
    // Let's move the pointer to the closest empty token (erased single import).
    while !is_cleared(&tokens[insert_index]) {
        insert_index += 1;
    }
    for (index, declaration) in statements.iter().enumerate() {
        if are_declarations_different(current, Some(declaration)) {
            current = Some(declaration);
            let namespace = rtrim_chars(namespace_name_with_slash(declaration), b"\\").to_vec();
            insert_index += create_new_group(tokens, insert_index, declaration, &namespace);
            continue;
        }
        let mut new_tokens = vec![Token::char(","), Token::new(T_WHITESPACE, " ")];
        if declaration.is_aliased {
            let n = new_tokens.len();
            tokens.insert_at(insert_index, std::mem::take(&mut new_tokens));
            insert_index += n;
            insert_index += insert_to_group_use_with_alias(tokens, insert_index, declaration);
        }
        new_tokens.push(Token::new(T_STRING, declaration.short_name.clone()));
        if are_declarations_different(current, statements.get(index + 1)) {
            new_tokens.push(Token::new(CT_GROUP_IMPORT_BRACE_CLOSE, "}"));
            new_tokens.push(Token::char(";"));
            new_tokens.push(Token::new(T_WHITESPACE, "\n"));
        }
        let n = new_tokens.len();
        tokens.insert_at(insert_index, new_tokens);
        insert_index += n;
    }
}

fn namespace_name_with_slash(declaration: &NamespaceUseAnalysis) -> &[u8] {
    let full = &declaration.full_name[..];
    match strrpos(full, b"\\") {
        None | Some(0) => full,
        Some(position) => &full[..=position],
    }
}

fn insert_to_group_use_with_alias(
    tokens: &mut Tokens,
    insert_index: usize,
    declaration: &NamespaceUseAnalysis,
) -> usize {
    let full = &declaration.full_name[..];
    // `(int) strripos(...) + 1`: a name without `\` starts at 1.
    let start = strrpos(full, b"\\").unwrap_or(0) + 1;
    let new_tokens = vec![
        Token::new(T_STRING, &full[start.min(full.len())..]),
        Token::new(T_WHITESPACE, " "),
        Token::new(T_AS, "as"),
        Token::new(T_WHITESPACE, " "),
    ];
    let n = new_tokens.len();
    tokens.insert_at(insert_index, new_tokens);
    n
}

fn create_new_group(
    tokens: &mut Tokens,
    mut insert_index: usize,
    declaration: &NamespaceUseAnalysis,
    namespace: &[u8],
) -> usize {
    let mut new_tokens = vec![Token::new(T_USE, "use"), Token::new(T_WHITESPACE, " ")];
    if declaration.is_function() {
        new_tokens.push(Token::new(CT_FUNCTION_IMPORT, "function"));
        new_tokens.push(Token::new(T_WHITESPACE, " "));
    } else if declaration.is_constant() {
        new_tokens.push(Token::new(CT_CONST_IMPORT, "const"));
        new_tokens.push(Token::new(T_WHITESPACE, " "));
    }
    for part in namespace.split(|&c| c == b'\\') {
        new_tokens.push(Token::new(T_STRING, part));
        new_tokens.push(Token::new(T_NS_SEPARATOR, "\\"));
    }
    new_tokens.push(Token::new(CT_GROUP_IMPORT_BRACE_OPEN, "{"));
    let n = new_tokens.len();
    tokens.insert_at(insert_index, new_tokens);
    let mut inserted = n;
    insert_index += n;
    if declaration.is_aliased {
        let alias = insert_to_group_use_with_alias(tokens, insert_index + 1, declaration) + 1;
        inserted += alias;
        insert_index += alias;
    }
    tokens.insert_at(
        insert_index,
        vec![Token::new(T_STRING, declaration.short_name.clone())],
    );
    inserted + 1
}

fn are_declarations_different(
    a: Option<&NamespaceUseAnalysis>,
    b: Option<&NamespaceUseAnalysis>,
) -> bool {
    let (Some(a), Some(b)) = (a, b) else {
        return true;
    };
    namespace_name_with_slash(a) != namespace_name_with_slash(b) || a.type_ != b.type_
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GroupImportFixerTest::provideInvalidConfigurationCases: the messages the real fixer's configure() throws.
    #[test]
    fn invalid_configuration() {
        let raw = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/group_import.json"
        ))
        .unwrap();
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let test = (doc["tests"].as_array().unwrap().iter())
            .find(|t| t["method"] == "testInvalidConfiguration")
            .unwrap();
        for case in test["cases"].as_array().unwrap() {
            let (message, config) = (case["args"][0].as_str().unwrap(), &case["args"][1]);
            let error = fixer().configure(config).unwrap_err().to_string();
            assert_eq!(error, message, "{config}");
        }
    }
}
