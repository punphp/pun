//! Port of `PhpCsFixer\Fixer\ClassNotation\OrderedTypesFixer`.
use crate::analyzers::analysis::TypeAnalysis;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{ltrim_chars, rtrim_chars, strcasecmp, strtolower};
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Kind, Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NullAdjustment {
    AlwaysFirst,
    AlwaysLast,
    None,
}

struct OrderedTypesFixer {
    alpha: bool,
    null_adjustment: NullAdjustment,
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(OrderedTypesFixer {
        alpha: true,
        // `Future::getV4OrV3('always_last', 'always_first')`: Pint runs without future mode.
        null_adjustment: NullAdjustment::AlwaysFirst,
        case_sensitive: false,
    })
}

const PROPERTY_MODIFIERS: [Kind; 9] = [
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_STATIC,
    T_VAR,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Element {
    Method,
    Property,
    Catch,
}

/// One member of a union / intersection type; a DNF group `(A&B)` is a list.
#[derive(Clone, PartialEq, Eq)]
enum Type {
    Single(Vec<u8>),
    Group(Vec<Vec<u8>>),
}

impl Type {
    fn joined(&self) -> Vec<u8> {
        match self {
            Type::Single(s) => s.clone(),
            Type::Group(g) => g.join(&b'&'),
        }
    }
}

impl Fixer for OrderedTypesFixer {
    fn name(&self) -> &'static str {
        "ordered_types"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &["sort_algorithm", "null_adjustment", "case_sensitive"],
        )?;
        self.alpha = c.string("sort_algorithm", "alpha", &["alpha", "none"])? == "alpha";
        self.null_adjustment = match c
            .string(
                "null_adjustment",
                "always_first",
                &["always_first", "always_last", "none"],
            )?
            .as_str()
        {
            "always_first" => NullAdjustment::AlwaysFirst,
            "always_last" => NullAdjustment::AlwaysLast,
            _ => NullAdjustment::None,
        };
        self.case_sensitive = c.bool("case_sensitive", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for (index, element) in get_elements(tokens) {
            match element {
                Element::Catch => self.fix_catch_argument_type(tokens, index),
                Element::Property => self.fix_property_type(tokens, index),
                Element::Method => {
                    self.fix_method_argument_type(&functions_analyzer, tokens, index);
                    self.fix_method_return_type(&functions_analyzer, tokens, index);
                }
            }
        }
    }
}

/// index => element type, in PHP array order: the classy methods and properties (by index), then every
/// other `catch`, `fn` and `function` in token order.
fn get_elements(tokens: &Tokens) -> Vec<(usize, Element)> {
    let mut elements: Vec<(usize, Element)> = TokensAnalyzer::new(tokens)
        .get_classy_elements()
        .into_iter()
        .filter_map(|(index, element)| match element.kind {
            ClassyElementType::Method => Some((index, Element::Method)),
            ClassyElementType::Property => Some((index, Element::Property)),
            _ => None,
        })
        .collect();
    let classy: HashSet<usize> = elements.iter().map(|e| e.0).collect();
    for (index, token) in tokens.iter().enumerate() {
        if token.is_kind(T_CATCH) {
            elements.push((index, Element::Catch));
        } else if token.is_kind(T_FN) || (token.is_kind(T_FUNCTION) && !classy.contains(&index)) {
            elements.push((index, Element::Method));
        }
    }
    elements
}

fn collect_type_analysis(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Option<TypeAnalysis> {
    let mut r#type = Vec::new();
    let type_start_index = tokens.get_next_meaningful_token(start_index).unwrap();
    let mut type_end_index = type_start_index;
    for i in type_start_index..end_index {
        if tokens[i].is_whitespace(None) || tokens[i].is_comment() {
            continue;
        }
        r#type.extend_from_slice(tokens[i].get_content());
        type_end_index = i;
    }
    (!r#type.is_empty())
        .then(|| TypeAnalysis::new(&r#type, Some(type_start_index), Some(type_end_index)))
}

fn is_type_sortable(r#type: &TypeAnalysis) -> bool {
    r#type.name.contains(&b'|') || r#type.name.contains(&b'&')
}

impl OrderedTypesFixer {
    fn fix_catch_argument_type(&self, tokens: &mut Tokens, index: usize) {
        let catch_start = tokens
            .get_next_token_of_kind(index, &[Proto::Char(b"(")], true)
            .unwrap();
        let catch_end = tokens
            .get_next_token_of_kind(
                catch_start,
                &[Proto::Char(b")"), Proto::Kind(T_VARIABLE)],
                true,
            )
            .unwrap();
        if let Some(r#type) = collect_type_analysis(tokens, catch_start, catch_end)
            && is_type_sortable(&r#type)
        {
            self.sort_types(&r#type, tokens);
        }
    }

    fn fix_property_type(&self, tokens: &mut Tokens, mut index: usize) {
        let property_index = index;
        loop {
            index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[index].is_given_kind(&PROPERTY_MODIFIERS) {
                break;
            }
        }
        if let Some(r#type) = collect_type_analysis(tokens, index, property_index)
            && is_type_sortable(&r#type)
        {
            self.sort_types(&r#type, tokens);
        }
    }

    fn fix_method_argument_type(
        &self,
        functions_analyzer: &FunctionsAnalyzer,
        tokens: &mut Tokens,
        index: usize,
    ) {
        for (_, argument_info) in functions_analyzer.get_function_arguments(tokens, index) {
            if let Some(r#type) = argument_info.type_analysis
                && is_type_sortable(&r#type)
            {
                self.sort_types(&r#type, tokens);
            }
        }
    }

    fn fix_method_return_type(
        &self,
        functions_analyzer: &FunctionsAnalyzer,
        tokens: &mut Tokens,
        index: usize,
    ) {
        if let Some(r#type) = functions_analyzer.get_function_return_type(tokens, index)
            && is_type_sortable(&r#type)
        {
            self.sort_types(&r#type, tokens);
        }
    }

    fn sort_types(&self, type_analysis: &TypeAnalysis, tokens: &mut Tokens) {
        let name = &type_analysis.name;
        let (original_types, glue) = if name.contains(&b'|') && name.contains(&b'&') {
            // a DNF type of the form (A&B)|C, available as of PHP 8.2
            collect_disjunctive_normal_form_types(name)
        } else {
            collect_union_or_intersection_types(name)
        };
        let sorted_types: Vec<Type> = original_types
            .iter()
            .map(|sub_type| match sub_type {
                Type::Group(g) => Type::Group(
                    self.run_types_through_sorting_algorithm(
                        g.iter().cloned().map(Type::Single).collect(),
                    )
                    .into_iter()
                    .map(|t| t.joined())
                    .collect(),
                ),
                t => t.clone(),
            })
            .collect();
        let sorted_types = self.run_types_through_sorting_algorithm(sorted_types);
        if sorted_types == original_types {
            return;
        }
        tokens.override_range(
            type_analysis.get_start_index(),
            type_analysis.get_end_index(),
            create_type_declaration_tokens(&sorted_types, glue, false),
        );
    }

    fn run_types_through_sorting_algorithm(&self, mut types: Vec<Type>) -> Vec<Type> {
        let normalize_type = |t: &Type| Preg::replace(r"/^\\?/", "", t.joined());
        types.sort_by(|a, b| {
            let a = normalize_type(a);
            let b = normalize_type(b);
            let lower_case_a = strtolower(&a);
            let lower_case_b = strtolower(&b);
            if self.null_adjustment != NullAdjustment::None {
                let last = self.null_adjustment == NullAdjustment::AlwaysLast;
                if lower_case_a == b"null" && lower_case_b != b"null" {
                    return if last {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                }
                if lower_case_a != b"null" && lower_case_b == b"null" {
                    return if last {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }
            }
            if self.alpha {
                return if self.case_sensitive {
                    a.cmp(&b)
                } else {
                    strcasecmp(&a, &b).cmp(&0)
                };
            }
            Ordering::Equal
        });
        types
    }
}

fn collect_disjunctive_normal_form_types(r#type: &[u8]) -> (Vec<Type>, u8) {
    let types = r#type
        .split(|&c| c == b'|')
        .map(|sub_type| {
            if sub_type.starts_with(b"(") {
                let trimmed = rtrim_chars(ltrim_chars(sub_type, b"()"), b"()");
                Type::Group(trimmed.split(|&c| c == b'&').map(<[u8]>::to_vec).collect())
            } else {
                Type::Single(sub_type.to_vec())
            }
        })
        .collect();
    (types, b'|')
}

fn collect_union_or_intersection_types(r#type: &[u8]) -> (Vec<Type>, u8) {
    let mut glue = b'|';
    if !r#type.contains(&b'|') {
        glue = b'&';
    }
    let types = r#type
        .split(|&c| c == glue)
        .map(|t| Type::Single(t.to_vec()))
        .collect();
    (types, glue)
}

fn create_type_declaration_tokens(types: &[Type], glue: u8, is_disjunctive: bool) -> Vec<Token> {
    let count = types.len();
    let mut new_tokens = Vec::new();
    for (i, r#type) in types.iter().enumerate() {
        match r#type {
            Type::Group(g) => {
                let group: Vec<Type> = g.iter().cloned().map(Type::Single).collect();
                new_tokens.extend(create_type_declaration_tokens(&group, b'&', true));
            }
            Type::Single(t) => match t.as_slice() {
                b"array" => new_tokens.push(Token::new(CT_ARRAY_TYPEHINT, "array")),
                b"callable" => new_tokens.push(Token::new(T_CALLABLE, "callable")),
                b"static" => new_tokens.push(Token::new(T_STATIC, "static")),
                _ => {
                    for (ns_index, value) in t.split(|&c| c == b'\\').enumerate() {
                        if ns_index == 0 && value.is_empty() {
                            continue;
                        }
                        if ns_index > 0 {
                            new_tokens.push(Token::new(T_NS_SEPARATOR, "\\"));
                        }
                        new_tokens.push(Token::new(T_STRING, value));
                    }
                }
            },
        }
        if i + 2 <= count {
            new_tokens.push(if glue == b'|' {
                Token::new(CT_TYPE_ALTERNATION, "|")
            } else {
                Token::new(CT_TYPE_INTERSECTION, "&")
            });
        }
    }
    if is_disjunctive {
        new_tokens.insert(
            0,
            Token::new(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN, "("),
        );
        new_tokens.push(Token::new(
            CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
            ")",
        ));
    }
    new_tokens
}
