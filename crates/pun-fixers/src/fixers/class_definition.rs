//! Port of `PhpCsFixer\Fixer\ClassNotation\ClassDefinitionFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct ClassDefinitionFixer {
    multi_line_extends_each_single_line: bool,
    single_item_single_line: bool,
    single_line: bool,
    space_before_parenthesis: bool,
    inline_constructor_arguments: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ClassDefinitionFixer {
        inline_constructor_arguments: true,
        ..Default::default()
    })
}

#[derive(Clone, Copy)]
struct InheritanceInfo {
    start: usize,
    count: usize,
    multi_line: bool,
}

struct ClassyDefinitionInfo {
    classy: usize,
    open: usize,
    extends: Option<InheritanceInfo>,
    implements: Option<InheritanceInfo>,
    anonymous_class: bool,
    final_: Option<usize>,
    abstract_: Option<usize>,
    readonly: Option<usize>,
    start: usize,
}

fn has_nl(t: &Token) -> bool {
    t.content.contains(&b'\n')
}

impl Fixer for ClassDefinitionFixer {
    fn name(&self) -> &'static str {
        "class_definition"
    }
    fn priority(&self) -> i32 {
        36
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &[
                "multi_line_extends_each_single_line",
                "single_item_single_line",
                "single_line",
                "space_before_parenthesis",
                "inline_constructor_arguments",
            ],
        )?;
        self.multi_line_extends_each_single_line =
            c.bool("multi_line_extends_each_single_line", false)?;
        self.single_item_single_line = c.bool("single_item_single_line", false)?;
        self.single_line = c.bool("single_line", false)?;
        self.space_before_parenthesis = c.bool("space_before_parenthesis", false)?;
        self.inline_constructor_arguments = c.bool("inline_constructor_arguments", true)?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // -4, one for count to index, 3 because min. of tokens for a classy location.
        let mut index = tokens.count() as isize - 4;
        while index > 0 {
            if tokens[index as usize].is_classy() {
                self.fix_classy_definition(tokens, index as usize);
            }
            index -= 1;
        }
    }
}

impl ClassDefinitionFixer {
    fn fix_classy_definition(&self, tokens: &mut Tokens, classy_index: usize) {
        let mut def = self.get_classy_definition_info(tokens, classy_index);
        // PSR2 4.1 Lists of implements MAY be split across multiple lines, where each subsequent line is indented once.
        if let Some(implements) = def.implements {
            def.implements =
                Some(self.fix_classy_definition_implements(tokens, def.open, implements));
        }
        if let Some(extends) = def.extends {
            let open = def.implements.map_or(def.open, |i| i.start);
            def.extends = Some(self.fix_classy_definition_extends(tokens, open, extends));
        }
        // PSR2: class definition open curly brace must go on a new line.
        // PSR12: anonymous class curly brace on same line if not multi line implements.
        def.open = self.fix_classy_definition_open_spacing(tokens, &def);
        let mut end = if let Some(i) = def.implements {
            i.start
        } else if let Some(e) = def.extends {
            e.start
        } else {
            tokens.get_prev_non_whitespace(def.open, None).unwrap()
        };
        if def.anonymous_class && !self.inline_constructor_arguments {
            if !tokens[end].equals(")", true) {
                // anonymous class with `extends` and/or `implements`
                let start = tokens.get_prev_meaningful_token(end).unwrap();
                self.make_classy_definition_single_line(tokens, start, end);
                end = start;
            }
            if tokens[end].equals(")", true) {
                // skip constructor arguments of anonymous class
                end = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, end);
            }
        }
        // 4.1 The extends and implements keywords MUST be declared on the same line as the class name.
        self.make_classy_definition_single_line(tokens, def.start, end);
        sort_class_modifiers(tokens, &def);
    }

    fn fix_classy_definition_extends(
        &self,
        tokens: &mut Tokens,
        class_open_index: usize,
        mut info: InheritanceInfo,
    ) -> InheritanceInfo {
        let end_index = tokens
            .get_prev_non_whitespace(class_open_index, None)
            .unwrap();
        if self.single_line || !info.multi_line {
            self.make_classy_definition_single_line(tokens, info.start, end_index);
            info.multi_line = false;
        } else if self.single_item_single_line && info.count == 1 {
            self.make_classy_definition_single_line(tokens, info.start, end_index);
            info.multi_line = false;
        } else if self.multi_line_extends_each_single_line && info.multi_line {
            self.make_classy_inheritance_part_multi_line(tokens, info.start, end_index);
            info.multi_line = true;
        }
        info
    }

    fn fix_classy_definition_implements(
        &self,
        tokens: &mut Tokens,
        class_open_index: usize,
        mut info: InheritanceInfo,
    ) -> InheritanceInfo {
        let end_index = tokens
            .get_prev_non_whitespace(class_open_index, None)
            .unwrap();
        if self.single_line || !info.multi_line {
            self.make_classy_definition_single_line(tokens, info.start, end_index);
            info.multi_line = false;
        } else if self.single_item_single_line && info.count == 1 {
            self.make_classy_definition_single_line(tokens, info.start, end_index);
            info.multi_line = false;
        } else {
            self.make_classy_inheritance_part_multi_line(tokens, info.start, end_index);
            info.multi_line = true;
        }
        info
    }

    fn fix_classy_definition_open_spacing(
        &self,
        tokens: &mut Tokens,
        def: &ClassyDefinitionInfo,
    ) -> usize {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let spacing: &[u8] = if def.anonymous_class {
            if let Some(i) = def.implements {
                if i.multi_line { line_ending } else { b" " }
            } else if let Some(e) = def.extends {
                if e.multi_line { line_ending } else { b" " }
            } else {
                b" "
            }
        } else {
            line_ending
        };
        let open_index = tokens
            .get_next_token_of_kind(def.classy, &["{".into()], true)
            .unwrap();
        if spacing != b" " && has_nl(&tokens[open_index - 1]) {
            return open_index;
        }
        if tokens[open_index - 1].is_whitespace(None) {
            if spacing != b" "
                || !tokens[tokens
                    .get_prev_non_whitespace(open_index - 1, None)
                    .unwrap()]
                .is_comment()
            {
                tokens.set(open_index - 1, Token::new(T_WHITESPACE, spacing));
            }
            return open_index;
        }
        tokens.insert_at(open_index, vec![Token::new(T_WHITESPACE, spacing)]);
        open_index + 1
    }

    fn get_classy_definition_info(
        &self,
        tokens: &Tokens,
        classy_index: usize,
    ) -> ClassyDefinitionInfo {
        let tokens_analyzer = TokensAnalyzer::new(tokens);
        let open_index = tokens
            .get_next_token_of_kind(classy_index, &["{".into()], true)
            .unwrap();
        let mut def = ClassyDefinitionInfo {
            classy: classy_index,
            open: open_index,
            extends: None,
            implements: None,
            anonymous_class: false,
            final_: None,
            abstract_: None,
            readonly: None,
            start: classy_index,
        };
        if !tokens[classy_index].is_kind(T_TRAIT) {
            let extends = tokens.find_given_kind(T_EXTENDS, classy_index, Some(open_index));
            def.extends = extends
                .first()
                .map(|&i| get_classy_inheritance_info(tokens, i));
            if !tokens[classy_index].is_kind(T_INTERFACE) {
                let implements =
                    tokens.find_given_kind(T_IMPLEMENTS, classy_index, Some(open_index));
                def.implements = implements
                    .first()
                    .map(|&i| get_classy_inheritance_info(tokens, i));
                def.anonymous_class = tokens_analyzer.is_anonymous_class(classy_index);
            }
        }
        if def.anonymous_class {
            // go to "new" for anonymous class
            def.start = tokens
                .get_prev_token_of_kind(classy_index, &[T_NEW.into()], true)
                .unwrap();
        } else {
            let modifiers = tokens_analyzer.get_classy_modifiers(classy_index);
            def.final_ = modifiers.final_;
            def.abstract_ = modifiers.abstract_;
            def.readonly = modifiers.readonly;
            for m in [modifiers.final_, modifiers.abstract_, modifiers.readonly]
                .into_iter()
                .flatten()
            {
                def.start = def.start.min(m);
            }
        }
        def
    }

    fn make_classy_definition_single_line(
        &self,
        tokens: &mut Tokens,
        start_index: usize,
        end_index: usize,
    ) {
        let ws = || Token::new(T_WHITESPACE, " ");
        let mut i = end_index as isize;
        while i >= start_index as isize {
            let iu = i as usize;
            i -= 1;
            if tokens[iu].is_whitespace(None) {
                if has_nl(&tokens[iu]) {
                    if tokens[iu - 1].is_kind(CT_ATTRIBUTE_CLOSE)
                        || tokens[iu + 1].is_kind(T_ATTRIBUTE)
                    {
                        continue;
                    }
                    if (tokens[iu - 1].is_comment() && tokens[iu - 1].content.ends_with(b"]"))
                        || (tokens[iu + 1].is_comment()
                            && tokens[iu + 1].content.starts_with(b"#["))
                    {
                        continue;
                    }
                    if tokens[iu - 1].is_kind(T_DOC_COMMENT)
                        || tokens[iu + 1].is_kind(T_DOC_COMMENT)
                    {
                        continue;
                    }
                }
                if tokens[iu - 1].is_comment() {
                    let content = &tokens[iu - 1].content;
                    if !content.starts_with(b"//") && !content.starts_with(b"#") {
                        tokens.set(iu, ws());
                    }
                    continue;
                }
                if tokens[iu + 1].is_comment() {
                    if !tokens[iu + 1].content.starts_with(b"//") {
                        tokens.set(iu, ws());
                    }
                    continue;
                }
                if tokens[iu - 1].is_kind(T_CLASS) && tokens[iu + 1].equals("(", true) {
                    if self.space_before_parenthesis {
                        tokens.set(iu, ws());
                    } else {
                        tokens.clear_at(iu);
                    }
                    continue;
                }
                if (!tokens[iu - 1].equals(",", true)
                    && tokens[iu + 1].equals_any(&[",".into(), ")".into()], true))
                    || tokens[iu - 1].equals("(", true)
                {
                    tokens.clear_at(iu);
                    continue;
                }
                tokens.set(iu, ws());
                continue;
            }
            if tokens[iu].equals(",", true) && !tokens[iu + 1].is_whitespace(None) {
                tokens.insert_at(iu + 1, vec![ws()]);
                continue;
            }
            if self.space_before_parenthesis
                && tokens[iu].is_kind(T_CLASS)
                && !tokens[iu + 1].is_whitespace(None)
            {
                tokens.insert_at(iu + 1, vec![ws()]);
                continue;
            }
            if !tokens[iu].is_comment() {
                continue;
            }
            if !tokens[iu + 1].is_whitespace(None)
                && !tokens[iu + 1].is_comment()
                && !has_nl(&tokens[iu])
            {
                tokens.insert_at(iu + 1, vec![ws()]);
            }
            if !tokens[iu - 1].is_whitespace(None) && !tokens[iu - 1].is_comment() {
                tokens.insert_at(iu, vec![ws()]);
            }
        }
    }

    fn make_classy_inheritance_part_multi_line(
        &self,
        tokens: &mut Tokens,
        start_index: usize,
        end_index: usize,
    ) {
        let nl_indent = [
            self.whitespaces_config.get_line_ending().as_bytes(),
            self.whitespaces_config.get_indent().as_bytes(),
        ]
        .concat();
        let mut i = end_index;
        while i > start_index {
            let previous = tokens
                .get_prev_token_of_kind(
                    i,
                    &[",".into(), T_IMPLEMENTS.into(), T_EXTENDS.into()],
                    true,
                )
                .unwrap();
            let break_at_index = tokens.get_next_meaningful_token(previous).unwrap();
            // make the part of a ',' or 'implements' single line
            self.make_classy_definition_single_line(tokens, break_at_index, i);
            // make sure the part is on its own line
            let is_on_own_line = (previous + 1..=break_at_index)
                .rev()
                .any(|j| has_nl(&tokens[j]));
            if !is_on_own_line {
                if tokens[break_at_index - 1].is_whitespace(None) {
                    tokens.set(
                        break_at_index - 1,
                        Token::new(T_WHITESPACE, nl_indent.clone()),
                    );
                } else {
                    tokens.insert_at(
                        break_at_index,
                        vec![Token::new(T_WHITESPACE, nl_indent.clone())],
                    );
                }
            }
            i = previous + 1;
            i -= 1;
        }
    }
}

fn get_classy_inheritance_info(tokens: &Tokens, mut start_index: usize) -> InheritanceInfo {
    let mut info = InheritanceInfo {
        start: start_index,
        count: 1,
        multi_line: false,
    };
    start_index += 1;
    let mut end_index = tokens
        .get_next_token_of_kind(
            start_index,
            &["{".into(), T_IMPLEMENTS.into(), T_EXTENDS.into()],
            true,
        )
        .unwrap();
    if tokens[end_index].equals("{", true) {
        end_index = tokens.get_prev_non_whitespace(end_index, None).unwrap();
    }
    for i in start_index..end_index {
        if tokens[i].equals(",", true) {
            info.count += 1;
            continue;
        }
        if !info.multi_line && has_nl(&tokens[i]) {
            info.multi_line = true;
        }
    }
    info
}

fn sort_class_modifiers(tokens: &mut Tokens, def: &ClassyDefinitionInfo) {
    let Some(readonly_index) = def.readonly else {
        return;
    };
    for access_modifier in [def.final_, def.abstract_] {
        let Some(access_modifier_index) = access_modifier else {
            continue;
        };
        if access_modifier_index < readonly_index {
            continue;
        }
        let readonly_token = tokens[readonly_index].clone();
        let access_token = tokens[access_modifier_index].clone();
        tokens.set(readonly_index, access_token);
        tokens.set(access_modifier_index, readonly_token);
        break;
    }
}
