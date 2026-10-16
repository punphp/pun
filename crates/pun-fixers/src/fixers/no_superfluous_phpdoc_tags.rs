//! Port of `PhpCsFixer\Fixer\Phpdoc\NoSuperfluousPhpdocTagsFixer`.
use crate::analyzers::analysis::TypeAnalysis;
use crate::analyzers::docblock::{Annotation, DocBlock, type_expression::REGEX_TYPES};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use crate::analyzers::namespaces_analyzer::NamespacesAnalyzer;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

const SYMBOL_KINDS: &[Kind] = &[T_CLASS, T_INTERFACE, T_ENUM];
const MODIFIER_KINDS: &[Kind] =
    &[T_PRIVATE, T_PROTECTED, T_PUBLIC, T_ABSTRACT, T_FINAL, T_STATIC, T_READONLY, T_PRIVATE_SET, T_PROTECTED_SET, T_PUBLIC_SET];
const TYPE_KINDS: &[Kind] = &[CT_NULLABLE_TYPE, CT_ARRAY_TYPEHINT, CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION, T_STRING, T_NS_SEPARATOR];

/// `array{types: list<string>, allows_null: bool}`.
#[derive(Clone)]
struct TypeInfo {
    types: Vec<Vec<u8>>,
    allows_null: bool,
}

fn no_type_info() -> TypeInfo {
    TypeInfo { types: vec![], allows_null: true }
}

#[derive(PartialEq)]
enum ElementType {
    Classy,
    Function,
    Property,
}

struct Element {
    index: usize,
    type_: ElementType,
    /// Token index => kind (PHP keeps the tokens; only their kinds are read).
    modifiers: Vec<Kind>,
    first_type_index: Option<usize>,
}

/// `$shortNames`: lower-cased short name => lower-cased full name.
type ShortNames = BTreeMap<Vec<u8>, Vec<u8>>;

#[derive(Default)]
struct NoSuperfluousPhpdocTagsFixer {
    allow_mixed: bool,
    remove_inheritdoc: bool,
    allow_hidden_params: bool,
    allow_unused_params: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSuperfluousPhpdocTagsFixer::default())
}

impl Fixer for NoSuperfluousPhpdocTagsFixer {
    fn name(&self) -> &'static str {
        "no_superfluous_phpdoc_tags"
    }
    fn priority(&self) -> i32 {
        6
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let keys = ["allow_mixed", "remove_inheritdoc", "allow_hidden_params", "allow_unused_params"];
        let r = resolve(self.name(), options, &keys)?;
        self.allow_mixed = r.bool("allow_mixed", false)?;
        self.remove_inheritdoc = r.bool("remove_inheritdoc", false)?;
        // Future::getV4OrV3(true, false)
        self.allow_hidden_params = r.bool("allow_hidden_params", false)?;
        self.allow_unused_params = r.bool("allow_unused_params", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut short_names = ShortNames::new();
        for u in NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, false) {
            short_names.insert(strtolower(&u.short_name), strtolower(&u.full_name));
        }
        // Only doc comment contents change below, so the namespaces are the same for every getNamespaceAt().
        let namespaces = NamespacesAnalyzer.get_declarations(tokens);
        let mut current_symbol: Option<Vec<u8>> = None;
        let mut current_symbol_end_index = None;

        for index in 0..tokens.count() {
            if Some(index) == current_symbol_end_index {
                current_symbol = None;
                current_symbol_end_index = None;
                continue;
            }
            let token = &tokens[index];
            if token.is_kind(T_CLASS) && TokensAnalyzer::new(tokens).is_anonymous_class(index) {
                continue;
            }
            if token.is_given_kind(SYMBOL_KINDS) {
                current_symbol = Some(tokens[tokens.get_next_meaningful_token(index).unwrap()].content.clone());
                let open = tokens.get_next_token_of_kind(index, &["{".into()], true).unwrap();
                current_symbol_end_index = Some(tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open));
                continue;
            }
            if !token.is_kind(T_DOC_COMMENT) {
                continue;
            }
            let Some(element) = find_documented_element(tokens, index) else { continue };
            let initial_content = tokens[index].content.clone();
            let mut content = initial_content.clone();
            if self.remove_inheritdoc {
                content = remove_superfluous_inherit_doc(&content);
            }
            let namespace = namespaces
                .iter()
                .find(|ns| ns.scope_start_index <= index && ns.scope_end_index >= index)
                .unwrap_or_else(|| panic!("Unable to get the namespace at index {index}."))
                .full_name
                .clone();
            let namespace = (!namespace.is_empty()).then_some(namespace);
            let ctx = Ctx { namespace: namespace.as_deref(), current_symbol: current_symbol.as_deref(), short_names: &short_names };
            content = match element.type_ {
                ElementType::Function => self.fix_function_doc_comment(&content, tokens, &element, &ctx),
                ElementType::Property => self.fix_property_doc_comment(&content, tokens, &element, &ctx),
                ElementType::Classy => fix_class_doc_comment(&content, &element),
            };
            if content.is_empty() {
                content = b"/**  */".to_vec();
            }
            if content != initial_content {
                tokens.set(index, Token::new(T_DOC_COMMENT, content));
            }
        }
    }
}

struct Ctx<'a> {
    namespace: Option<&'a [u8]>,
    current_symbol: Option<&'a [u8]>,
    short_names: &'a ShortNames,
}

fn find_documented_element(tokens: &Tokens, doc_comment_index: usize) -> Option<Element> {
    let mut element = Element { index: 0, type_: ElementType::Classy, modifiers: vec![], first_type_index: None };
    let mut index = tokens.get_next_meaningful_token(doc_comment_index);
    let is_attribute = |i: Option<usize>| i.is_some_and(|i| tokens[i].is_kind(T_ATTRIBUTE));
    if is_attribute(index) {
        loop {
            index = tokens.get_next_meaningful_token(tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index.unwrap()));
            if !is_attribute(index) {
                break;
            }
        }
    }
    while let Some(i) = index {
        let t = &tokens[i];
        element.index = i;
        if t.is_classy() {
            element.type_ = ElementType::Classy;
            return Some(element);
        }
        if t.is_given_kind(&[T_FUNCTION, T_FN]) {
            element.type_ = ElementType::Function;
            return Some(element);
        }
        if t.is_kind(T_VARIABLE) {
            element.type_ = ElementType::Property;
            return Some(element);
        }
        if t.is_given_kind(MODIFIER_KINDS) {
            element.modifiers.push(t.id.unwrap());
        } else if t.is_given_kind(TYPE_KINDS) {
            element.first_type_index.get_or_insert(i);
        } else {
            break;
        }
        index = tokens.get_next_meaningful_token(i);
    }
    None
}

impl NoSuperfluousPhpdocTagsFixer {
    fn fix_function_doc_comment(&self, content: &[u8], tokens: &Tokens, element: &Element, ctx: &Ctx) -> Vec<u8> {
        let doc_block = DocBlock::new(content);
        let opening = tokens.get_next_token_of_kind(element.index, &["(".into()], true).unwrap();
        let closing = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, opening);
        let arguments_info = self.get_arguments_info(tokens, opening + 1, closing - 1);

        for annotation in doc_block.get_annotations_of_type(&["param"]) {
            let Some(argument_name) = annotation.get_variable_name() else {
                if self.annotation_is_superfluous(annotation, &no_type_info(), ctx) {
                    annotation.remove();
                }
                continue;
            };
            let info = arguments_info.get(&argument_name);
            if info.is_none() && self.allow_unused_params {
                continue;
            }
            if info.is_none_or(|info| self.annotation_is_superfluous(annotation, info, ctx)) {
                annotation.remove();
            }
        }

        let return_type_info = get_return_type_info(tokens, closing);
        for annotation in doc_block.get_annotations_of_type(&["return"]) {
            if self.annotation_is_superfluous(annotation, &return_type_info, ctx) {
                annotation.remove();
            }
        }
        remove_superfluous_modifier_annotation(&doc_block, element);
        doc_block.get_content()
    }

    fn fix_property_doc_comment(&self, content: &[u8], tokens: &Tokens, element: &Element, ctx: &Ctx) -> Vec<u8> {
        let property_type_info = match element.first_type_index {
            Some(i) => parse_type_hint(tokens, i),
            None => no_type_info(),
        };
        let doc_block = DocBlock::new(content);
        for annotation in doc_block.get_annotations_of_type(&["var"]) {
            if self.annotation_is_superfluous(annotation, &property_type_info, ctx) {
                annotation.remove();
            }
        }
        doc_block.get_content()
    }

    fn get_arguments_info(&self, tokens: &Tokens, start: usize, end: usize) -> BTreeMap<Vec<u8>, TypeInfo> {
        let mut arguments_info = BTreeMap::new();
        for index in start..=end {
            let token = &tokens[index];
            if !token.is_kind(T_VARIABLE) {
                continue;
            }
            let before_argument_index =
                tokens.get_prev_token_of_kind(index, &["(".into(), ",".into(), CT_ATTRIBUTE_CLOSE.into()], true).unwrap();
            let type_index = tokens.get_next_meaningful_token(before_argument_index).unwrap();
            let mut info = if type_index != index { parse_type_hint(tokens, type_index) } else { no_type_info() };
            if !info.allows_null {
                let next_index = tokens.get_next_meaningful_token(index).unwrap();
                if tokens[next_index].equals("=", true)
                    && tokens[tokens.get_next_meaningful_token(next_index).unwrap()].equals((T_STRING, "null"), false)
                {
                    info.allows_null = true;
                }
            }
            arguments_info.insert(token.content.clone(), info);
        }
        // virtualise "hidden params" as if they would be regular ones
        if self.allow_hidden_params {
            let params_string = tokens.generate_partial_code(start, end);
            for comment in Preg::match_all(r"~/\*.*?\*/~s", &params_string, 0) {
                for hidden_param in Preg::match_all(r"~\$\w+~", comment.str(0), 0) {
                    arguments_info.insert(hidden_param.str(0).to_vec(), no_type_info());
                }
            }
        }
        arguments_info
    }

    fn annotation_is_superfluous(&self, annotation: &Annotation, info: &TypeInfo, ctx: &Ctx) -> bool {
        let tag = annotation.get_tag().get_name();
        static PARAM: LazyLock<String> = LazyLock::new(|| {
            format!(r"{{\*\h*@param(?:\h+{REGEX_TYPES})?(?!\S)(?:\h+(?:\&\h*)?(?:\.{{3}}\h*)?\$\S+)?(?:\s+(?<description>(?!\*+\/)\S+))?}}s")
        });
        static VAR: LazyLock<String> =
            LazyLock::new(|| format!(r"{{\*\h*@var(?:\h+{REGEX_TYPES})?(?!\S)(?:\h+\$\S+)?(?:\s+(?<description>(?!\*\/)\S+))?}}s"));
        static RETURN: LazyLock<String> =
            LazyLock::new(|| format!(r"{{\*\h*@return(?:\h+{REGEX_TYPES})?(?!\S)(?:\s+(?<description>(?!\*\/)\S+))?}}s"));
        let regex: &str = if tag == b"param" {
            &PARAM
        } else if tag == b"var" {
            &VAR
        } else {
            &RETURN
        };
        let content = annotation.get_content();
        let Some(matches) = Preg::match_groups(regex, &content, 0) else {
            // Unable to match the annotation, it must be malformed or has unsupported format.
            return false;
        };
        if matches.name_exists("description") {
            return false;
        }
        if matches.name("types").is_none_or(|t| t.is_empty()) {
            // If there's no type info in the annotation, further checks make no sense, exit early.
            return true;
        }
        let mut annotation_types = to_comparable_names(annotation.get_types(), ctx);
        if annotation_types == [b"null"] && info.types != [b"null"] {
            return false;
        }
        if annotation_types == [b"mixed"] && info.types.is_empty() {
            return !self.allow_mixed;
        }
        let mut actual_types = info.types.clone();
        if info.allows_null {
            actual_types.push(b"null".to_vec());
        }
        let actual_types = to_comparable_names(actual_types, ctx);
        if annotation_types == actual_types {
            return true;
        }
        // retry comparison with annotation type unioned with null
        // phpstan implies the null presence from the native type
        annotation_types.push(b"null".to_vec());
        annotation_types.sort();
        actual_types == annotation_types
    }
}

fn get_return_type_info(tokens: &Tokens, closing_parenthesis_index: usize) -> TypeInfo {
    let colon_index = tokens.get_next_meaningful_token(closing_parenthesis_index).unwrap();
    if tokens[colon_index].is_kind(CT_TYPE_COLON) {
        parse_type_hint(tokens, tokens.get_next_meaningful_token(colon_index).unwrap())
    } else {
        no_type_info()
    }
}

fn parse_type_hint(tokens: &Tokens, mut index: usize) -> TypeInfo {
    let mut allows_null = false;
    let mut types = vec![];
    loop {
        let mut type_ = Vec::new();
        if tokens[index].is_given_kind(&[T_READONLY, T_PRIVATE_SET, T_PROTECTED_SET, T_PUBLIC_SET]) {
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
        if tokens[index].is_given_kind(&[
            CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
            CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
            CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
        ]) {
            index = tokens.get_next_meaningful_token(index).unwrap();
            continue;
        }
        if tokens[index].is_kind(CT_NULLABLE_TYPE) {
            allows_null = true;
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
        while tokens[index].is_given_kind(&[T_NS_SEPARATOR, T_STATIC, T_STRING, CT_ARRAY_TYPEHINT, T_CALLABLE]) {
            type_.extend_from_slice(&tokens[index].content);
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
        if type_.is_empty() {
            break;
        }
        types.push(type_);
        if !tokens[index].is_given_kind(&[CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION]) {
            break;
        }
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    TypeInfo { types, allows_null }
}

fn to_comparable_names(mut types: Vec<Vec<u8>>, ctx: &Ctx) -> Vec<Vec<u8>> {
    if types.first().and_then(|t| t.first()) == Some(&b'?') {
        types = vec![types[0][1..].to_vec(), b"null".to_vec()];
    }
    let mut normalized: Vec<Vec<u8>> = types
        .into_iter()
        .map(|mut type_| {
            if type_.contains(&b'&') {
                let intersects = type_.split(|&c| c == b'&').map(<[u8]>::to_vec).collect();
                return to_comparable_names(intersects, ctx).join(&b'&');
            }
            if type_ == b"self" {
                if let Some(s) = ctx.current_symbol {
                    type_ = s.to_vec();
                }
            }
            let mut type_ = strtolower(&type_);
            if let Some(full) = ctx.short_names.get(&type_) {
                return full.clone(); // always FQCN /wo leading backslash and in lower-case
            }
            if let Some(rest) = type_.strip_prefix(b"\\") {
                return rest.to_vec();
            }
            if let Some(ns) = ctx.namespace {
                if !TypeAnalysis::new(&type_, None, None).is_reserved_type() {
                    type_ = [&strtolower(ns)[..], b"\\", &type_].concat();
                }
            }
            type_
        })
        .collect();
    // ponytail: PHP's sort() compares numeric strings numerically; both sides of every comparison are sorted
    // the same way and native types are never numeric, so plain byte order gives the same equality result.
    normalized.sort();
    normalized
}

fn fix_class_doc_comment(content: &[u8], element: &Element) -> Vec<u8> {
    let doc_block = DocBlock::new(content);
    remove_superfluous_modifier_annotation(&doc_block, element);
    doc_block.get_content()
}

fn remove_superfluous_inherit_doc(doc_comment: &[u8]) -> Vec<u8> {
    // The pattern needs the tag; without it the replacement returns the comment unchanged.
    if !doc_comment.windows(10).any(|w| w.eq_ignore_ascii_case(b"inheritdoc")) {
        return doc_comment.to_vec();
    }
    Preg::replace(
        r"~
            # $1: before @inheritDoc tag
            (
                # beginning of comment or a PHPDoc tag
                (?:
                    ^/\*\*
                    (?:
                        \R
                        [ \t]*(?:\*[ \t]*)?
                    )*?
                    |
                    @\N+
                )

                # empty comment lines
                (?:
                    \R
                    [ \t]*(?:\*[ \t]*?)?
                )*
            )

            # spaces before @inheritDoc tag
            [ \t]*

            # @inheritDoc tag
            (?:@inheritDocs?|\{@inheritDocs?\})

            # $2: after @inheritDoc tag
            (
                # empty comment lines
                (?:
                    \R
                    [ \t]*(?:\*[ \t]*)?
                )*

                # a PHPDoc tag or end of comment
                (?:
                    @\N+
                    |
                    (?:
                        \R
                        [ \t]*(?:\*[ \t]*)?
                    )*
                    [ \t]*\*/$
                )
            )
        ~ix",
        b"$1$2",
        doc_comment,
    )
}

fn remove_superfluous_modifier_annotation(doc_block: &DocBlock, element: &Element) {
    for (annotation_type, modifier_token) in [("abstract", T_ABSTRACT), ("final", T_FINAL)] {
        let annotations = doc_block.get_annotations_of_type(&[annotation_type]);
        for &kind in &element.modifiers {
            if kind == modifier_token {
                for annotation in &annotations {
                    annotation.remove();
                }
            }
        }
    }
}
