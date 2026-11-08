//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitAttributesFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, get_doc_block_index, is_candidate};
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::docblock::{Annotation, DocBlock};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::{Captures, Preg};

use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{explode, ltrim_chars, rtrim_chars, str_replace, strpos, trim};
use pun_tokens::transformers::tokenize_name;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Copy)]
enum Fix {
    WithoutParameters,
    WithSingleStringValue,
    WithEnabledDisabledValue,
    Covers,
    DataProvider,
    Depends,
    Requires,
    TestWith,
    Uses,
}

/// `prepareFixingMap`.
fn fixing_map(annotation_name: &[u8]) -> Option<Fix> {
    Some(match annotation_name {
        // annotations that map to attribute without parameters
        b"after"
        | b"afterClass"
        | b"before"
        | b"beforeClass"
        | b"coversNothing"
        | b"doesNotPerformAssertions"
        | b"large"
        | b"medium"
        | b"runInSeparateProcess"
        | b"runTestsInSeparateProcesses"
        | b"small"
        | b"test"
        | b"preCondition"
        | b"postCondition" => Fix::WithoutParameters,
        // annotations that map to attribute with single string value
        b"group" | b"testDox" | b"ticket" => Fix::WithSingleStringValue,
        // annotations that map from 'enabled'/'disabled' value to attribute with boolean value
        b"backupGlobals" | b"backupStaticAttributes" | b"preserveGlobalState" => {
            Fix::WithEnabledDisabledValue
        }
        // annotations that has custom mapping function
        b"covers" => Fix::Covers,
        b"dataProvider" => Fix::DataProvider,
        b"depends" => Fix::Depends,
        b"requires" => Fix::Requires,
        b"testWith" => Fix::TestWith,
        b"uses" => Fix::Uses,
        _ => return None,
    })
}

struct PhpUnitAttributesFixer {
    keep_annotations: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitAttributesFixer {
        keep_annotations: false,
    })
}

impl Fixer for PhpUnitAttributesFixer {
    fn name(&self) -> &'static str {
        "php_unit_attributes"
    }
    fn priority(&self) -> i32 {
        8
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["keep_annotations"])?;
        self.keep_annotations = c.bool("keep_annotations", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            self.apply_php_unit_class_fix(tokens, start_index, end_index);
        });
    }
}

impl PhpUnitAttributesFixer {
    fn apply_php_unit_class_fix(
        &self,
        tokens: &mut Tokens,
        mut start_index: usize,
        end_index: usize,
    ) {
        let class_index = tokens
            .get_prev_token_of_kind(start_index, &[T_CLASS.into()], true)
            .unwrap();
        let doc_block_index = get_doc_block_index(tokens, class_index);
        if tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
            start_index = doc_block_index;
        }
        for index in (start_index..=end_index).rev() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let target_index = tokens
                .get_token_not_of_kinds_sibling(
                    index,
                    1,
                    &[
                        T_ABSTRACT,
                        T_COMMENT,
                        T_FINAL,
                        T_FUNCTION,
                        T_PRIVATE,
                        T_PROTECTED,
                        T_PUBLIC,
                        T_STATIC,
                        T_WHITESPACE,
                    ],
                )
                .unwrap();
            let class_scope = tokens[target_index].is_kind(T_CLASS);
            let doc_block = DocBlock::new(tokens[index].get_content());
            let mut present_attributes: HashMap<Vec<u8>, bool> = HashMap::new();
            for annotation in doc_block.get_annotations().iter().rev() {
                let annotation_name = annotation.get_tag().get_name().to_vec();
                let Some(fix) = fixing_map(&annotation_name) else {
                    continue;
                };
                if !should_be_fixed(&annotation_name, class_scope) {
                    continue;
                }
                let tokens_to_insert = fix_annotation(fix, tokens, index, annotation);
                let present = *present_attributes
                    .entry(annotation_name)
                    .or_insert_with(|| {
                        is_attribute_already_present(tokens, index, &tokens_to_insert)
                    });
                if tokens_to_insert.is_empty() {
                    continue;
                }
                if !present {
                    tokens.insert_slices(BTreeMap::from([(index + 1, tokens_to_insert)]));
                }
                if !self.keep_annotations {
                    annotation.remove();
                }
            }
            let content = doc_block.get_content();
            if content.is_empty() {
                tokens.clear_token_and_merge_surrounding_whitespace(index);
            } else {
                tokens.set(index, Token::new(T_DOC_COMMENT, content));
            }
        }
    }
}

/// `shouldBeFixed`.
fn should_be_fixed(annotation_name: &[u8], class_scope: bool) -> bool {
    let method_only: &[&[u8]] = &[
        b"covers",
        b"large",
        b"medium",
        b"runTestsInSeparateProcesses",
        b"small",
        b"uses",
    ];
    let class_only: &[&[u8]] = &[
        b"after",
        b"afterClass",
        b"before",
        b"beforeClass",
        b"dataProvider",
        b"depends",
        b"postCondition",
        b"preCondition",
        b"runInSeparateProcess",
        b"test",
        b"testWith",
    ];
    !(if class_scope { class_only } else { method_only }).contains(&annotation_name)
}

/// `isAttributeAlreadyPresent`.
fn is_attribute_already_present(tokens: &Tokens, index: usize, tokens_to_insert: &[Token]) -> bool {
    let attribute_index = tokens.get_next_meaningful_token(index).unwrap();
    if !tokens[attribute_index].is_kind(T_ATTRIBUTE) {
        return false;
    }
    let mut inserted_class_name = Vec::new();
    for token in tokens_to_insert.iter().skip(3) {
        if token.equals("(", true) || token.is_kind(CT_ATTRIBUTE_CLOSE) {
            break;
        }
        inserted_class_name.extend_from_slice(token.get_content());
    }
    AttributeAnalyzer::collect(tokens, attribute_index)
        .iter()
        .any(|analysis| {
            analysis.attributes.iter().any(|attribute| {
                let full_name = AttributeAnalyzer::determine_attribute_fully_qualified_name(
                    tokens,
                    &attribute.name,
                    attribute.start,
                );
                let class_name = ltrim_chars(&full_name, b"\\");
                inserted_class_name == class_name
                    || (inserted_class_name == b"PHPUnit\\Framework\\Attributes\\TestWithJson"
                        && class_name == b"PHPUnit\\Framework\\Attributes\\TestWith")
            })
        })
}

fn fix_annotation(fix: Fix, tokens: &Tokens, index: usize, annotation: &Annotation) -> Vec<Token> {
    let name = annotation.get_tag().get_name();
    let content = annotation.get_content();
    match fix {
        Fix::WithoutParameters => {
            create_attribute_tokens(tokens, index, &attribute_name_for_annotation(name), vec![])
        }
        Fix::WithSingleStringValue => {
            let pattern = [b"/@".as_slice(), name, br"\s+(.*\S)(?:\R|\s*\*+\/$)/"].concat();
            let Some(matches) = Preg::match_groups(&pattern, &content) else {
                return vec![];
            };
            create_attribute_tokens(
                tokens,
                index,
                &attribute_name_for_annotation(name),
                vec![create_escaped_string_token(matches.str(1))],
            )
        }
        Fix::WithEnabledDisabledValue => {
            let Some(matches) = get_matches(name, &content) else {
                return vec![];
            };
            let value: &[u8] = if matches.str(1) == b"enabled" {
                b"true"
            } else {
                b"false"
            };
            create_attribute_tokens(
                tokens,
                index,
                &attribute_name_for_annotation(name),
                vec![Token::new(T_STRING, value.to_vec())],
            )
        }
        Fix::Covers => fix_covers(tokens, index, get_matches(name, &content)),
        Fix::DataProvider => fix_data_provider(tokens, index, get_matches(name, &content)),
        Fix::Depends => fix_depends(tokens, index, get_matches(name, &content)),
        Fix::Requires => fix_requires(tokens, index, get_matches(name, &content)),
        Fix::TestWith => fix_test_with(tokens, index, &content),
        Fix::Uses => fix_uses(tokens, index, get_matches(name, &content)),
    }
}

/// `fixCovers`.
fn fix_covers(tokens: &Tokens, index: usize, matches: Option<Captures>) -> Vec<Token> {
    // Upstream's `explode('::', null)` throws when the annotation has no value.
    let value = matches
        .as_ref()
        .map(|m| m.str(1))
        .expect("explode(): Argument #2 ($string) must be of type string, null given");
    let split: Vec<&[u8]> = explode(b"::", value);
    match split.len() {
        1 => {
            // naive guessing if it's a Trait or Class, as we do not have access to codebase to make sure, but it's
            // better than nothing
            let name = if split[0].ends_with(b"Trait") {
                b"CoversTrait".as_slice()
            } else {
                b"CoversClass"
            };
            create_attribute_tokens(tokens, index, name, to_class_constant(split[0]))
        }
        2 if split[0].is_empty() => create_attribute_tokens(
            tokens,
            index,
            b"CoversFunction",
            vec![create_escaped_string_token(split[1])],
        ),
        2 if !split[1].is_empty() => {
            let mut attribute_tokens = to_class_constant(split[0]);
            attribute_tokens.extend(separator());
            attribute_tokens.push(fix_name_and_create_escaped_string_token(split[1]));
            create_attribute_tokens(tokens, index, b"CoversMethod", attribute_tokens)
        }
        // unexpected format, do not attempt to fix
        _ => vec![],
    }
}

/// `fixDataProvider`.
fn fix_data_provider(tokens: &Tokens, index: usize, matches: Option<Captures>) -> Vec<Token> {
    let Some(matches) = matches else {
        return vec![];
    };
    let value = matches.str(1);
    if strpos(value, b"::", 0).is_some() {
        let parts = explode(b"::", value);
        let mut attribute_tokens = to_class_constant(parts[0]);
        attribute_tokens.extend(separator());
        attribute_tokens.push(fix_name_and_create_escaped_string_token(parts[1]));
        return create_attribute_tokens(tokens, index, b"DataProviderExternal", attribute_tokens);
    }
    create_attribute_tokens(
        tokens,
        index,
        b"DataProvider",
        vec![fix_name_and_create_escaped_string_token(value)],
    )
}

/// `fixNameAndCreateEscapedStringToken`.
fn fix_name_and_create_escaped_string_token(method_name: &[u8]) -> Token {
    create_escaped_string_token(rtrim_chars(method_name, b" ()\n\r\t\x0B\0"))
}

/// `fixDepends`.
fn fix_depends(tokens: &Tokens, index: usize, matches: Option<Captures>) -> Vec<Token> {
    let Some(matches) = matches else {
        return vec![];
    };
    let mut name_suffix: Vec<u8> = Vec::new();
    let mut depended = matches.str(1);
    if isset(&matches, 2) {
        if matches.str(1) == b"clone" {
            name_suffix = b"UsingDeepClone".to_vec();
            depended = matches.str(2);
        } else if matches.str(1) == b"shallowClone" {
            name_suffix = b"UsingShallowClone".to_vec();
            depended = matches.str(2);
        }
    }
    let mut class = None;
    let mut method = Some(depended);
    if strpos(depended, b"::", 0).is_some() {
        let parts = explode(b"::", depended);
        class = Some(parts[0]);
        method = Some(parts[1]);
        if parts[1] == b"class" {
            method = None;
            name_suffix = [b"OnClass".as_slice(), &name_suffix].concat();
        } else {
            name_suffix = [b"External".as_slice(), &name_suffix].concat();
        }
    }
    let mut attribute_tokens = class.map(to_class_constant).unwrap_or_default();
    if let Some(method) = method {
        if !attribute_tokens.is_empty() {
            attribute_tokens.extend(separator());
        }
        attribute_tokens.push(create_escaped_string_token(method));
    }
    create_attribute_tokens(
        tokens,
        index,
        &[b"Depends".as_slice(), &name_suffix].concat(),
        attribute_tokens,
    )
}

/// `fixRequires`.
fn fix_requires(tokens: &Tokens, index: usize, matches: Option<Captures>) -> Vec<Token> {
    let Some(matches) = matches else {
        return vec![];
    };
    let attribute_name: &[u8] = match matches.str(1) {
        b"extension" => b"RequiresPhpExtension",
        b"function" => b"RequiresFunction",
        b"PHP" => b"RequiresPhp",
        b"PHPUnit" => b"RequiresPhpunit",
        b"OS" => b"RequiresOperatingSystem",
        b"OSFAMILY" => b"RequiresOperatingSystemFamily",
        b"setting" => b"RequiresSetting",
        _ => return vec![],
    };
    if !isset(&matches, 2) {
        return vec![];
    }
    let is_version = attribute_name == b"RequiresPhp" || attribute_name == b"RequiresPhpunit";
    let mut attribute_name = attribute_name;
    let mut attribute_tokens =
        if attribute_name == b"RequiresFunction" && strpos(matches.str(2), b"::", 0).is_some() {
            let parts = explode(b"::", matches.str(2));
            attribute_name = b"RequiresMethod";
            let mut attribute_tokens = to_class_constant(parts[0]);
            attribute_tokens.extend(separator());
            attribute_tokens.push(create_escaped_string_token(parts[1]));
            attribute_tokens
        } else if is_version && isset(&matches, 3) {
            vec![create_escaped_string_token(
                &[matches.str(2), b" ", matches.str(3)].concat(),
            )]
        } else {
            vec![create_escaped_string_token(&fix_version_constraint(
                matches.str(2),
            ))]
        };
    if isset(&matches, 3) && !is_version {
        attribute_tokens.extend(separator());
        attribute_tokens.push(create_escaped_string_token(&fix_version_constraint(
            matches.str(3),
        )));
    }
    create_attribute_tokens(tokens, index, attribute_name, attribute_tokens)
}

/// `fixVersionConstraint`.
fn fix_version_constraint(version: &[u8]) -> Vec<u8> {
    if Preg::r#match(r"/^[\d\.-]+(dev|(RC|alpha|beta)[\d\.])?$/", version) {
        return [b">= ".as_slice(), version].concat();
    }
    version.to_vec()
}

/// `fixTestWith`.
fn fix_test_with(tokens: &Tokens, index: usize, content: &[u8]) -> Vec<Token> {
    let content = Preg::replace(r"/@testWith\s+/", "", content);
    let content = Preg::replace(r"/(^|\R)\s+\**\s*/", "\n", &content);
    let content = trim(&content);
    if content.is_empty() {
        return vec![];
    }
    let mut attribute_tokens = Vec::new();
    for json in explode(b"\n", content) {
        attribute_tokens.extend(create_attribute_tokens(
            tokens,
            index,
            b"TestWithJson",
            vec![create_escaped_string_token(json)],
        ));
    }
    attribute_tokens
}

/// `fixUses`.
fn fix_uses(tokens: &Tokens, index: usize, matches: Option<Captures>) -> Vec<Token> {
    let Some(matches) = matches else {
        return vec![];
    };
    let value = matches.str(1);
    let (attribute_name, attribute_tokens): (&[u8], _) =
        if let Some(function) = value.strip_prefix(b"::") {
            (b"UsesFunction", vec![create_escaped_string_token(function)])
        } else if Preg::r#match(r"/^[a-zA-Z\d\\]+$/", value) {
            (b"UsesClass", to_class_constant(value))
        } else {
            return vec![];
        };
    create_attribute_tokens(tokens, index, attribute_name, attribute_tokens)
}

/// `getMatches`; `None` when the pattern does not match (an empty `$matches`).
fn get_matches<'s>(name: &[u8], content: &'s [u8]) -> Option<Captures<'s>> {
    let pattern = [
        b"/@".as_slice(),
        name,
        br"\s+(\S+)(?:\s+(\S+))?(?:\s+(.+\S))?\s*(?:\R|\*+\/$)/",
    ]
    .concat();
    Preg::match_groups(&pattern, content)
}

/// PHP's `isset($matches[$i])`: unmatched groups before the last matched one are set (to `""`).
fn isset(matches: &Captures, i: usize) -> bool {
    i < matches.len()
}

/// `getAttributeNameForAnnotation`.
fn attribute_name_for_annotation(annotation_name: &[u8]) -> Vec<u8> {
    if annotation_name == b"backupStaticAttributes" {
        return b"BackupStaticProperties".to_vec();
    }
    let mut name = annotation_name.to_vec();
    name[0] = name[0].to_ascii_uppercase();
    name
}

/// `createAttributeTokens`.
fn create_attribute_tokens(
    tokens: &Tokens,
    index: usize,
    class_name: &[u8],
    attribute_tokens: Vec<Token>,
) -> Vec<Token> {
    let mut out = vec![
        tokens[index + 1].clone(),
        Token::new(T_ATTRIBUTE, b"#[".to_vec()),
        Token::new(T_NS_SEPARATOR, b"\\".to_vec()),
        Token::new(T_STRING, b"PHPUnit".to_vec()),
        Token::new(T_NS_SEPARATOR, b"\\".to_vec()),
        Token::new(T_STRING, b"Framework".to_vec()),
        Token::new(T_NS_SEPARATOR, b"\\".to_vec()),
        Token::new(T_STRING, b"Attributes".to_vec()),
        Token::new(T_NS_SEPARATOR, b"\\".to_vec()),
        Token::new(T_STRING, class_name.to_vec()),
    ];
    if !attribute_tokens.is_empty() {
        out.push(Token::char(b"("));
        out.extend(attribute_tokens);
        out.push(Token::char(b")"));
    }
    out.push(Token::new(CT_ATTRIBUTE_CLOSE, b"]".to_vec()));
    out
}

/// `toClassConstant`.
fn to_class_constant(name: &[u8]) -> Vec<Token> {
    let mut out = tokenize_name(name);
    out.push(Token::new(T_DOUBLE_COLON, b"::".to_vec()));
    out.push(Token::new(CT_CLASS_CONSTANT, b"class".to_vec()));
    out
}

/// `, ` between two attribute arguments.
fn separator() -> [Token; 2] {
    [Token::char(b","), Token::new(T_WHITESPACE, b" ".to_vec())]
}

/// `createEscapedStringToken`.
fn create_escaped_string_token(value: &[u8]) -> Token {
    Token::new(
        T_CONSTANT_ENCAPSED_STRING,
        [b"'".as_slice(), &str_replace(b"'", b"\\'", value), b"'"].concat(),
    )
}
