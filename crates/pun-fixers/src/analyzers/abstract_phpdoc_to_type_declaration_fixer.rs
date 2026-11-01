//! Port of `PhpCsFixer\AbstractPhpdocToTypeDeclarationFixer`, shared by phpdoc_to_param_type,
//! phpdoc_to_property_type and phpdoc_to_return_type, with the parts of `PhpCsFixer\DocBlock\TypeExpression` it calls
//! that know the namespace (`getCommonType`, `allowsNull`, `getParentType`, `normalize`).
//!
//! The oracle runs on PHP 8.5, so every `\PHP_VERSION_ID` check is true.
use crate::analyzers::analysis::NamespaceUseAnalysis;
use crate::analyzers::docblock::type_expression::REGEX_IDENTIFIER;
use crate::analyzers::docblock::{Annotation, DocBlock, TypeExpression};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_tokens::Kind::*;
use pun_tokens::namespaces_analyzer::{NamespaceAnalysis, NamespacesAnalyzer};
use pun_tokens::php_compat::trim;
use pun_tokens::{Token, Tokens};
use serde_json::Value;

/// Types newer than PHP 7.0 (`versionSpecificTypes`); all are available on PHP 8.5.
const VERSION_SPECIFIC_TYPES: &[&[u8]] = &[b"void", b"iterable", b"object", b"mixed", b"never"];
const SCALAR_TYPES: &[&[u8]] = &[b"bool", b"float", b"int", b"string"];

/// `TypeExpression::ALIASES`.
fn alias(t: &[u8]) -> Option<&'static [u8]> {
    Some(match t {
        b"boolean" | b"false" | b"true" => b"bool",
        b"callback" => b"callable",
        b"double" | b"real" => b"float",
        b"integer" => b"int",
        b"list" => b"array",
        _ => return None,
    })
}

#[derive(Default)]
pub(crate) struct Options {
    pub scalar_types: bool,
    pub union_types: bool,
    pub types_map: Vec<(Vec<u8>, Vec<u8>)>,
}

impl Options {
    pub(crate) fn configure(name: &'static str, options: &Value) -> Result<Options> {
        let o = resolve(name, options, &["scalar_types", "union_types", "types_map"])?;
        // `array<string, string>`, which the resolver checks as `string[]`.
        o.check_types("types_map", &["string[]"])?;
        let types_map = match o.get("types_map") {
            Some(Value::Object(m)) => m
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_bytes().to_vec(),
                        v.as_str().unwrap().as_bytes().to_vec(),
                    )
                })
                .collect(),
            _ => Vec::new(),
        };
        Ok(Options {
            scalar_types: o.bool("scalar_types", true)?,
            union_types: o.bool("union_types", true)?,
            types_map,
        })
    }
}

/// `{commonType, isNullable}`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct TypeInfo {
    pub common_type: Vec<u8>,
    pub is_nullable: bool,
}

/// The namespace a docblock sits in and its imports, which `TypeExpression::normalize` resolves names with.
pub(crate) struct Scope {
    namespace: NamespaceAnalysis,
    uses: Vec<NamespaceUseAnalysis>,
}

pub(crate) struct Common<'a> {
    pub options: &'a Options,
    pub is_skipped_type: fn(&[u8]) -> bool,
}

/// `findFunctionDocComment`.
pub(crate) fn find_function_doc_comment(tokens: &Tokens, mut index: usize) -> Option<usize> {
    loop {
        index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if !tokens[index].is_given_kind(&[
            T_COMMENT,
            T_ABSTRACT,
            T_FINAL,
            T_PRIVATE,
            T_PROTECTED,
            T_PUBLIC,
            T_STATIC,
        ]) {
            break;
        }
    }
    tokens[index].is_kind(T_DOC_COMMENT).then_some(index)
}

/// `getAnnotationsFromDocComment`: the docblock (its annotations are `doc.get_annotations_of_type(&[name])`) and the
/// scope its types resolve in.
pub(crate) fn doc_comment(tokens: &Tokens, doc_comment_index: usize) -> (DocBlock, Scope) {
    let namespace = NamespacesAnalyzer.get_namespace_at(tokens, doc_comment_index);
    let uses = NamespaceUsesAnalyzer.get_declarations_in_namespace(tokens, &namespace, false);
    (
        DocBlock::new(&tokens[doc_comment_index].get_content()),
        Scope { namespace, uses },
    )
}

/// `getTypesToExclude`.
pub(crate) fn get_types_to_exclude(content: &[u8]) -> Vec<Vec<u8>> {
    let mut types_to_exclude = Vec::new();
    let doc = DocBlock::new(content);
    for annotation in doc.get_annotations_of_type(&["phpstan-type", "psalm-type"]) {
        types_to_exclude.extend(
            annotation
                .get_type_expression()
                .map(|t| t.to_string().to_vec()),
        );
    }
    for annotation in doc.get_annotations_of_type(&["phpstan-import-type", "psalm-import-type"]) {
        let content = annotation.get_content();
        let pattern = format!(r"/\bas\s+({REGEX_IDENTIFIER})$/");
        if let Some(m) = Preg::match_groups(&pattern, trim(&content)) {
            types_to_exclude.push(m.str(1).to_vec());
            continue;
        }
        types_to_exclude.extend(
            annotation
                .get_type_expression()
                .map(|t| t.to_string().to_vec()),
        );
    }
    types_to_exclude
}

/// `isValidSyntax`: `Tokens::fromCode` throws a ParseError on code PHP's grammar rejects.
pub(crate) fn is_valid_syntax(code: &str) -> bool {
    pun_format::lint(code.as_bytes()).is_ok()
}

/// `createTokensFromRawType`: `template` with the type, minus `clear_start` tokens at the start and `clear_end` at the
/// end.
pub(crate) fn tokens_from_raw_type(code: &str, clear_start: usize, clear_end: usize) -> Vec<Token> {
    let mut type_tokens = Tokens::from_code(code.as_bytes());
    let count = type_tokens.count();
    type_tokens.clear_range(0, clear_start);
    type_tokens.clear_range(count - clear_end, count - 1);
    type_tokens.clear_empty_tokens();
    type_tokens.iter().cloned().collect()
}

/// `createTypeDeclarationTokens`.
pub(crate) fn create_type_declaration_tokens(
    raw_type_tokens: Vec<Token>,
    type_: &[u8],
    is_nullable: bool,
) -> Vec<Token> {
    let mut new_tokens = Vec::new();
    if is_nullable && type_ != b"mixed" {
        new_tokens.push(Token::new(CT_NULLABLE_TYPE, "?"));
    }
    new_tokens.extend(raw_type_tokens);
    // 'scalar's, 'void', 'iterable' and 'object' must be unqualified
    let mut keep = vec![true; new_tokens.len()];
    for i in 1..new_tokens.len() {
        let content = new_tokens[i].get_content();
        if new_tokens[i].is_kind(T_STRING)
            && (SCALAR_TYPES.contains(&content) || VERSION_SPECIFIC_TYPES.contains(&content))
            && new_tokens[i - 1].get_content() == b"\\"
        {
            keep[i - 1] = false;
        }
    }
    new_tokens
        .into_iter()
        .zip(keep)
        .filter_map(|(t, k)| k.then_some(t))
        .collect()
}

impl Common<'_> {
    /// `getCommonTypeInfo`; `scope` is `None` for a `new TypeExpression($type, null, [])`.
    pub(crate) fn get_common_type_info(
        &self,
        types_expression: &TypeExpression,
        scope: Option<&Scope>,
        is_return_type: bool,
    ) -> Option<TypeInfo> {
        let mut common_type = get_common_type(types_expression, scope)?;
        let is_nullable = allows_null(types_expression);
        if is_nullable && common_type == b"void" {
            return None;
        }
        if common_type == b"static" && !is_return_type {
            common_type = b"self".to_vec();
        }
        if (self.is_skipped_type)(&common_type) {
            return None;
        }
        if let Some((_, to)) = self
            .options
            .types_map
            .iter()
            .find(|(from, _)| *from == common_type)
        {
            common_type = to.clone();
        }
        if SCALAR_TYPES.contains(&&common_type[..]) {
            if !self.options.scalar_types {
                return None;
            }
        } else if !Preg::r#match(
            format!(r"/^(?:\\?+{REGEX_IDENTIFIER}(\\{REGEX_IDENTIFIER})*+)$/"),
            &common_type,
        ) {
            return None;
        }
        Some(TypeInfo {
            common_type,
            is_nullable,
        })
    }

    /// `getUnionTypes`.
    pub(crate) fn get_union_types(
        &self,
        types_expression: &TypeExpression,
        is_return_type: bool,
    ) -> Option<Vec<u8>> {
        if !types_expression.is_union_type() || !self.options.union_types {
            return None;
        }
        let mut is_nullable = allows_null(types_expression);
        let mut union_types: Vec<Vec<u8>> = Vec::new();
        let mut contains_other_than_iterable_type = false;
        let mut contains_other_than_empty_type = false;
        for type_ in types_expression.get_types() {
            if type_ == b"null" {
                continue;
            }
            if (self.is_skipped_type)(&type_) {
                return None;
            }
            let info =
                self.get_common_type_info(&TypeExpression::new(&type_), None, is_return_type)?;
            let common_type = info.common_type;
            if ![&b"array"[..], b"Traversable", b"iterable"].contains(&&common_type[..]) {
                contains_other_than_iterable_type = true;
            }
            if is_return_type && ![&b"null"[..], b"void", b"never"].contains(&&common_type[..]) {
                contains_other_than_empty_type = true;
            }
            if info.is_nullable {
                is_nullable = true;
            }
            union_types.push(common_type);
        }
        if !contains_other_than_iterable_type || (is_return_type && !contains_other_than_empty_type)
        {
            return None;
        }
        if is_nullable {
            union_types.push(b"null".to_vec());
        }
        let mut unique: Vec<Vec<u8>> = Vec::new();
        for t in union_types {
            if !unique.contains(&t) {
                unique.push(t);
            }
        }
        Some(unique.join(&[types_expression.get_types_glue().unwrap()][..]))
    }

    /// The type of one annotation: its common type, else its union type (not nullable), as the three fixers read it.
    pub(crate) fn type_info(
        &self,
        annotation: &Annotation,
        scope: &Scope,
        is_return_type: bool,
    ) -> Option<TypeInfo> {
        let types_expression = annotation.get_type_expression()?;
        if let Some(info) =
            self.get_common_type_info(&types_expression, Some(scope), is_return_type)
        {
            return Some(info);
        }
        self.get_union_types(&types_expression, is_return_type)
            .map(|common_type| TypeInfo {
                common_type,
                is_nullable: false,
            })
    }
}

/// `TypeExpression::getCommonType`.
fn get_common_type(types_expression: &TypeExpression, scope: Option<&Scope>) -> Option<Vec<u8>> {
    let mut main_type: Option<Vec<u8>> = None;
    for type_ in types_expression.get_types() {
        if type_ == b"null" {
            continue;
        }
        let mut t = type_.strip_prefix(b"?").unwrap_or(&type_).to_vec();
        if Preg::r#match(r"/\[\h*\]$/", &t) {
            t = b"array".to_vec();
        } else if let Some(m) = Preg::match_groups(r"/^(.+?)\h*[<{(]/", &t) {
            t = m.str(1).to_vec();
        }
        if let Some(a) = alias(&t) {
            t = a.to_vec();
        }
        match &main_type {
            None => main_type = Some(t),
            Some(main) if *main == t => {}
            Some(main) => main_type = Some(get_parent_type(&t, main, scope)?),
        }
    }
    main_type
}

/// `TypeExpression::allowsNull`.
fn allows_null(types_expression: &TypeExpression) -> bool {
    types_expression
        .get_types()
        .iter()
        .any(|t| t == b"null" || t == b"mixed" || t.starts_with(b"?"))
}

/// `TypeExpression::getParentType`: the two normalized types in natural case-insensitive order, looked up.
fn get_parent_type(type1: &[u8], type2: &[u8], scope: Option<&Scope>) -> Option<Vec<u8>> {
    let pair = [normalize(type1, scope), normalize(type2, scope)];
    let is = |a: &[u8], b: &[u8]| (pair[0] == a && pair[1] == b) || (pair[0] == b && pair[1] == a);
    if is(b"array", b"Traversable") || is(b"array", b"iterable") || is(b"iterable", b"Traversable")
    {
        return Some(b"iterable".to_vec());
    }
    is(b"self", b"static").then(|| b"self".to_vec())
}

/// `TypeExpression::normalize`. Its alias, `[]` and `<` branches never apply here: `getCommonType` has already
/// resolved aliases and cut `[]` and `<...>` off both types.
fn normalize(type_: &[u8], scope: Option<&Scope>) -> Vec<u8> {
    const BUILT_IN: &[&[u8]] = &[
        b"array",
        b"bool",
        b"callable",
        b"false",
        b"float",
        b"int",
        b"iterable",
        b"mixed",
        b"never",
        b"null",
        b"object",
        b"resource",
        b"string",
        b"true",
        b"void",
    ];
    if BUILT_IN.contains(&type_) {
        return type_.to_vec();
    }
    if let Some(rest) = type_.strip_prefix(b"\\") {
        return rest.to_vec();
    }
    if let Some(u) = scope
        .into_iter()
        .flat_map(|s| &s.uses)
        .find(|u| u.short_name == type_)
    {
        return u.full_name.clone();
    }
    match scope {
        Some(s) if !s.namespace.is_global_namespace() => {
            [&s.namespace.full_name[..], b"\\", type_].concat()
        }
        _ => type_.to_vec(),
    }
}
