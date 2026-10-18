//! Port of `PhpCsFixer\Fixer\ClassNotation\ClassAttributesSeparationFixer`.
use crate::analyzers::fixer_configuration_resolver::{PhpKey, resolve};
use crate::analyzers::preg::Preg;
use anyhow::bail;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::{self, *};
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spacing {
    None,
    One,
    OnlyIfMeta,
}

#[derive(Default)]
struct ClassAttributesSeparationFixer {
    /// `$classElementTypes`: element type => spacing, in configuration order.
    class_element_types: Vec<(ClassyElementType, Spacing)>,
    whitespaces_config: WhitespacesFixerConfig,
}

const DEFAULT_ELEMENTS: [(ClassyElementType, Spacing); 5] = [
    (ClassyElementType::Const, Spacing::One),
    (ClassyElementType::Method, Spacing::One),
    (ClassyElementType::Property, Spacing::One),
    (ClassyElementType::TraitImport, Spacing::None),
    (ClassyElementType::Case, Spacing::None),
];

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(ClassAttributesSeparationFixer::default())
}

const MODIFIER_TYPES: &[Kind] = &[
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_ABSTRACT,
    T_FINAL,
    T_STATIC,
    T_STRING,
    T_NS_SEPARATOR,
    T_VAR,
    CT_NULLABLE_TYPE,
    CT_ARRAY_TYPEHINT,
    CT_TYPE_ALTERNATION,
    CT_TYPE_INTERSECTION,
    CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
    CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
];

struct Element {
    type_: ClassyElementType,
    start: usize,
    end: usize,
}

struct Class {
    index: usize,
    open: usize,
    close: usize,
    elements: Vec<Element>,
}

fn substr_count_nl(t: &Token) -> usize {
    t.content.iter().filter(|&&c| c == b'\n').count()
}

impl Fixer for ClassAttributesSeparationFixer {
    fn name(&self) -> &'static str {
        "class_attributes_separation"
    }
    fn priority(&self) -> i32 {
        55
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["elements"])?;
        c.check_types("elements", &["string[]"])?; // setAllowedTypes(['array<string, string>'])
        let Some(elements) = c.php_array("elements") else {
            self.class_element_types = DEFAULT_ELEMENTS.to_vec();
            return Ok(());
        };
        let mut out = Vec::new();
        for (type_, spacing) in elements {
            let spacing = spacing.as_str().unwrap();
            let Some(&(t, _)) = DEFAULT_ELEMENTS
                .iter()
                .find(|(t, _)| matches!(type_, PhpKey::Str(s) if s == t.as_str()))
            else {
                bail!(
                    "[class_attributes_separation] Invalid configuration: Unexpected element type, expected any of \"const\", \"method\", \"property\", \"trait_import\" and \"case\", got \"{}\".",
                    type_.typed()
                );
            };
            let s = match spacing {
                "none" => Spacing::None,
                "one" => Spacing::One,
                "only_if_meta" => Spacing::OnlyIfMeta,
                _ => bail!(
                    "[class_attributes_separation] Invalid configuration: Unexpected spacing for element type \"{spacing}\", expected any of \"none\", \"one\" and \"only_if_meta\", got \"string#{spacing}\"."
                ),
            };
            out.push((t, s));
        }
        self.class_element_types = out;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // getElementsByClass is a generator: each class group's start/end/open/close is computed
        // after the previously yielded groups were fixed.
        let all: Vec<(usize, usize, ClassyElementType)> = TokensAnalyzer::new(tokens)
            .get_classy_elements()
            .into_iter()
            .rev()
            .map(|(index, e)| (index, e.class_index, e.kind))
            .collect();
        let mut pos = 0;
        while pos < all.len() {
            let class_index = all[pos].1;
            let class_open = tokens
                .get_next_token_of_kind(class_index, &["{".into()], true)
                .unwrap();
            let class_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open);
            let mut class = Class {
                index: class_index,
                open: class_open,
                close: class_end,
                elements: Vec::new(),
            };
            while pos < all.len() && all[pos].1 == class_index {
                let (index, _, type_) = all[pos];
                let start = get_first_token_index_of_class_element(tokens, index);
                let end = get_last_token_index_of_class_element(tokens, class.index, index, type_);
                class.elements.push(Element { type_, start, end });
                pos += 1;
            }
            self.fix_class(tokens, &class);
        }
    }
}

impl ClassAttributesSeparationFixer {
    fn spacing(&self, type_: ClassyElementType) -> Option<Spacing> {
        self.class_element_types
            .iter()
            .find(|(t, _)| *t == type_)
            .map(|(_, s)| *s)
    }

    fn fix_class(&self, tokens: &mut Tokens, class: &Class) {
        if self.spacing(class.elements[0].type_).is_some() {
            self.fix_space_below_class_element(tokens, class);
        }
        for index in 0..class.elements.len() {
            if self.spacing(class.elements[index].type_).is_some() {
                self.fix_space_above_class_element(tokens, class, index);
            }
        }
    }

    fn fix_space_above_class_element(
        &self,
        tokens: &mut Tokens,
        class: &Class,
        element_index: usize,
    ) {
        let element = &class.elements[element_index];
        let element_above_end = class.elements.get(element_index + 1).map_or(0, |e| e.end);
        let non_white_above = tokens.get_prev_non_whitespace(element.start, None).unwrap();
        if non_white_above == class.open {
            self.correct_line_breaks(tokens, non_white_above, element.start, 1);
            return;
        }
        if tokens[non_white_above].is_kind(T_COMMENT) {
            if element_above_end == non_white_above {
                let n = self.determine_required_line_count(tokens, class, element_index);
                self.correct_line_breaks(tokens, non_white_above, element.start, n);
                return;
            }
            // more than one line break, always bring it back to 2 line breaks between the element start and what is above it
            if tokens[non_white_above + 1].is_whitespace(None)
                && substr_count_nl(&tokens[non_white_above + 1]) > 1
            {
                self.correct_line_breaks(tokens, non_white_above, element.start, 2);
                return;
            }
            if element.start - non_white_above == 1
                || (tokens[non_white_above - 1].is_whitespace(None)
                    && substr_count_nl(&tokens[non_white_above - 1]) > 0)
                || (tokens[non_white_above + 1].is_whitespace(None)
                    && substr_count_nl(&tokens[non_white_above + 1]) > 0)
            {
                // 1. The comment is meant for the element (although not a PHPDoc),
                //    make sure there is one line break between the element and the comment...
                self.correct_line_breaks(tokens, non_white_above, element.start, 1);
                //    ... and make sure there is blank line above the comment (with the exception when it is directly after a class opening)
                let non_white_above =
                    find_comment_block_start(tokens, non_white_above, element_above_end);
                let non_white_above_comment = tokens
                    .get_prev_non_whitespace(non_white_above, None)
                    .unwrap();
                if non_white_above_comment == class.open {
                    if tokens[non_white_above_comment - 1].is_whitespace(None)
                        && substr_count_nl(&tokens[non_white_above_comment - 1]) > 0
                    {
                        self.correct_line_breaks(
                            tokens,
                            non_white_above_comment,
                            non_white_above,
                            1,
                        );
                    }
                } else {
                    self.correct_line_breaks(tokens, non_white_above_comment, non_white_above, 2);
                }
            } else {
                // 2. The comment belongs to the code above the element,
                //    make sure there is a blank line above the element (i.e. 2 line breaks)
                self.correct_line_breaks(tokens, non_white_above, element.start, 2);
            }
            return;
        }
        if tokens[non_white_above].is_given_kind(&[T_DOC_COMMENT, CT_ATTRIBUTE_CLOSE]) {
            self.correct_line_breaks(tokens, non_white_above, element.start, 1);
            let non_white_above =
                find_comment_block_start(tokens, non_white_above, element_above_end);
            let non_white_above_comment = tokens
                .get_prev_non_whitespace(non_white_above, None)
                .unwrap();
            let n = if non_white_above_comment == class.open {
                1
            } else {
                2
            };
            self.correct_line_breaks(tokens, non_white_above_comment, non_white_above, n);
            return;
        }
        let n = self.determine_required_line_count(tokens, class, element_index);
        self.correct_line_breaks(tokens, non_white_above, element.start, n);
    }

    fn determine_required_line_count(
        &self,
        tokens: &Tokens,
        class: &Class,
        element_index: usize,
    ) -> usize {
        let type_ = class.elements[element_index].type_;
        let spacing = self.spacing(type_).unwrap();
        if spacing == Spacing::One {
            return 2;
        }
        let meta_above = |start: usize| {
            let i = tokens.get_prev_non_whitespace(start, None).unwrap();
            if tokens[i].is_given_kind(&[T_DOC_COMMENT, CT_ATTRIBUTE_CLOSE]) {
                2
            } else {
                1
            }
        };
        if spacing == Spacing::None {
            let above_element = &class.elements[element_index + 1];
            if above_element.type_ != type_ {
                return 2;
            }
            return meta_above(above_element.start);
        }
        meta_above(class.elements[element_index].start)
    }

    fn fix_space_below_class_element(&self, tokens: &mut Tokens, class: &Class) {
        let element = &class.elements[0];
        if tokens.get_next_non_whitespace(element.end, None) == Some(class.close) {
            self.correct_line_breaks(tokens, element.end, class.close, 1);
        }
    }

    fn correct_line_breaks(
        &self,
        tokens: &mut Tokens,
        mut start_index: usize,
        end_index: usize,
        req_line_count: usize,
    ) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        start_index += 1;
        let numb_of_white_tokens = end_index as isize - start_index as isize;
        if numb_of_white_tokens == 0 {
            tokens.insert_at(
                start_index,
                vec![Token::new(T_WHITESPACE, line_ending.repeat(req_line_count))],
            );
            return;
        }
        let line_break_count = get_line_break_count(tokens, start_index, end_index);
        if req_line_count == line_break_count {
            return;
        }
        if line_break_count < req_line_count {
            let content = [
                &line_ending.repeat(req_line_count - line_break_count)[..],
                &tokens[start_index].content,
            ]
            .concat();
            tokens.set(start_index, Token::new(T_WHITESPACE, content));
            return;
        }
        // $lineCount = > $reqLineCount : check the one Token case first since this one will be true most of the time
        if numb_of_white_tokens == 1 {
            let (content, _) = Preg::replace_limit(
                "/\\r\\n|\\n/",
                "",
                &tokens[start_index].content,
                (line_break_count - req_line_count) as isize,
            );
            tokens.set(start_index, Token::new(T_WHITESPACE, content));
        }
    }
}

fn get_line_break_count(tokens: &Tokens, start_index: usize, end_index: usize) -> usize {
    (start_index..end_index)
        .map(|i| substr_count_nl(&tokens[i]))
        .sum()
}

fn find_comment_block_start(tokens: &Tokens, mut start: usize, element_above_end: usize) -> usize {
    let mut i = start as isize;
    while i > element_above_end as isize {
        let iu = i as usize;
        if tokens[iu].is_kind(CT_ATTRIBUTE_CLOSE) {
            start = tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, iu);
            i = start as isize - 1;
            continue;
        }
        if tokens[iu].is_comment() {
            start = iu;
            i -= 1;
            continue;
        }
        if !tokens[iu].is_whitespace(None) || get_line_break_count(tokens, iu, iu + 1) > 1 {
            break;
        }
        i -= 1;
    }
    start
}

/// Upstream's `while ($firstElementAttributeIndex > $classOpen)` always holds: it only moves onto modifiers, which sit
/// after the class's `{`.
fn get_first_token_index_of_class_element(tokens: &Tokens, element_index: usize) -> usize {
    let mut first_element_attribute_index = element_index;
    while let Some(non_white_above) = tokens
        .get_prev_meaningful_token(first_element_attribute_index)
        .filter(|&i| tokens[i].is_given_kind(MODIFIER_TYPES))
    {
        first_element_attribute_index = non_white_above;
    }
    first_element_attribute_index
}

fn get_last_token_index_of_class_element(
    tokens: &Tokens,
    class_index: usize,
    element_index: usize,
    element_type: ClassyElementType,
) -> usize {
    let mut element_end_index;
    if element_type == ClassyElementType::Method && !tokens[class_index].is_kind(T_INTERFACE) {
        let attributes = TokensAnalyzer::new(tokens).get_method_attributes(element_index);
        if attributes.abstract_ {
            element_end_index = tokens
                .get_next_token_of_kind(element_index, &[";".into()], true)
                .unwrap();
        } else {
            let open = tokens
                .get_next_token_of_kind(element_index, &["{".into()], true)
                .unwrap();
            element_end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open);
        }
    } else if element_type == ClassyElementType::TraitImport {
        element_end_index = element_index;
        loop {
            element_end_index = tokens.get_next_meaningful_token(element_end_index).unwrap();
            if !(tokens[element_end_index].is_given_kind(&[T_STRING, T_NS_SEPARATOR])
                || tokens[element_end_index].equals(",", true))
            {
                break;
            }
        }
        if !tokens[element_end_index].equals(";", true) {
            let open = tokens
                .get_next_token_of_kind(element_index, &["{".into()], true)
                .unwrap();
            element_end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open);
        }
    } else if element_type == ClassyElementType::Property {
        element_end_index = tokens
            .get_next_token_of_kind(
                element_index,
                &[";".into(), CT_PROPERTY_HOOK_BRACE_OPEN.into()],
                true,
            )
            .unwrap();
        if tokens[element_end_index].is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
            element_end_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, element_end_index);
        }
    } else {
        // 'const', enum-'case', 'promoted_property', or 'method' of an interface
        element_end_index = tokens
            .get_next_token_of_kind(element_index, &[";".into(), "{".into()], true)
            .unwrap();
    }
    let single_line_element =
        !(element_index + 1..element_end_index).any(|i| tokens[i].content.contains(&b'\n'));
    if single_line_element {
        loop {
            let next_token = &tokens[element_end_index + 1];
            if (next_token.is_comment() || next_token.is_whitespace(None))
                && !next_token.content.contains(&b'\n')
            {
                element_end_index += 1;
            } else {
                break;
            }
        }
        if tokens[element_end_index].is_whitespace(None) {
            element_end_index = tokens
                .get_prev_non_whitespace(element_end_index, None)
                .unwrap();
        }
    }
    element_end_index
}
