//! Port of `PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationSpacesFixer`.
use crate::analyzers::doctrine_annotation::{self as doctrine, *};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::T_DOC_COMMENT;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

struct DoctrineAnnotationSpacesFixer {
    ignored_tags: Vec<String>,
    around_parentheses: bool,
    around_commas: bool,
    before_argument_assignments: Option<bool>,
    after_argument_assignments: Option<bool>,
    before_array_assignments_equals: Option<bool>,
    after_array_assignments_equals: Option<bool>,
    before_array_assignments_colon: Option<bool>,
    after_array_assignments_colon: Option<bool>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(DoctrineAnnotationSpacesFixer {
        ignored_tags: vec![],
        around_parentheses: true,
        around_commas: true,
        before_argument_assignments: None,
        after_argument_assignments: None,
        before_array_assignments_equals: None,
        after_array_assignments_equals: None,
        before_array_assignments_colon: None,
        after_array_assignments_colon: None,
    })
}

impl Fixer for DoctrineAnnotationSpacesFixer {
    fn name(&self) -> &'static str {
        "doctrine_annotation_spaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &[
                "ignored_tags",
                "around_parentheses",
                "around_commas",
                "before_argument_assignments",
                "after_argument_assignments",
                "before_array_assignments_equals",
                "after_array_assignments_equals",
                "before_array_assignments_colon",
                "after_array_assignments_colon",
            ],
        )?;
        self.ignored_tags = doctrine::ignored_tags(&o)?;
        self.around_parentheses = o.bool("around_parentheses", true)?;
        self.around_commas = o.bool("around_commas", true)?;
        self.before_argument_assignments =
            o.opt_bool("before_argument_assignments", Some(false))?;
        self.after_argument_assignments = o.opt_bool("after_argument_assignments", Some(false))?;
        self.before_array_assignments_equals =
            o.opt_bool("before_array_assignments_equals", Some(true))?;
        self.after_array_assignments_equals =
            o.opt_bool("after_array_assignments_equals", Some(true))?;
        self.before_array_assignments_colon =
            o.opt_bool("before_array_assignments_colon", Some(true))?;
        self.after_array_assignments_colon =
            o.opt_bool("after_array_assignments_colon", Some(true))?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        doctrine::apply_fix(tokens, &self.ignored_tags, |doc| {
            if self.around_parentheses {
                fix_spaces_around_parentheses(doc);
            }
            if self.around_commas {
                fix_spaces_around_commas(doc);
            }
            if [
                self.before_argument_assignments,
                self.after_argument_assignments,
                self.before_array_assignments_equals,
                self.after_array_assignments_equals,
                self.before_array_assignments_colon,
                self.after_array_assignments_colon,
            ]
            .iter()
            .any(Option::is_some)
            {
                self.fix_around_assignments(doc);
            }
        });
    }
}

fn fix_spaces_around_parentheses(tokens: &mut DocTokens) {
    let mut in_annotation_until = None;
    for index in 0..tokens.len() {
        if let Some(until) = in_annotation_until {
            if index == until {
                in_annotation_until = None;
                continue;
            }
        } else if tokens[index].kind == T_AT {
            in_annotation_until = Some(tokens.get_annotation_end(index) + 1);
            continue;
        }
        if in_annotation_until.is_none()
            || !tokens[index].is_type(&[T_OPEN_PARENTHESIS, T_CLOSE_PARENTHESIS])
        {
            continue;
        }
        let target = if tokens[index].kind == T_OPEN_PARENTHESIS {
            if tokens[index - 1].kind == T_NONE {
                tokens[index - 1].clear();
            }
            index + 1
        } else {
            index - 1
        };
        let token = &mut tokens[target];
        if token.kind == T_NONE && !token.content.contains(&b'\n') {
            token.clear();
        }
    }
}

fn fix_spaces_around_commas(tokens: &mut DocTokens) {
    let mut in_annotation_until = None;
    // `foreach` over the live list: a space inserted after a comma is visited too.
    let mut index = 0;
    while index < tokens.len() {
        let current = index;
        index += 1;
        if let Some(until) = in_annotation_until {
            if current == until {
                in_annotation_until = None;
                continue;
            }
        } else if tokens[current].kind == T_AT {
            in_annotation_until = Some(tokens.get_annotation_end(current));
            continue;
        }
        if in_annotation_until.is_none() || tokens[current].kind != T_COMMA {
            continue;
        }
        if tokens[current - 1].kind == T_NONE {
            tokens[current - 1].clear();
        }
        if current < tokens.len() - 1 && !Preg::r#match(r"/^\s/", &tokens[current + 1].content) {
            tokens.insert_at(current + 1, DocToken::new(T_NONE, " "));
        }
    }
}

impl DoctrineAnnotationSpacesFixer {
    fn fix_around_assignments(&self, tokens: &mut DocTokens) {
        let mut scopes: Vec<u8> = vec![];
        // `foreach` over the live list: a space inserted before the operator moves it to the next index, so the
        // operator is visited again.
        let mut index = 0;
        while index < tokens.len() {
            let current = index;
            index += 1;
            let end_scope_type = scopes.last().copied();
            let kind = tokens[current].kind;
            if end_scope_type == Some(kind) {
                scopes.pop();
                continue;
            }
            if kind == T_AT {
                scopes.push(T_CLOSE_PARENTHESIS);
                continue;
            }
            if kind == T_OPEN_CURLY_BRACES {
                scopes.push(T_CLOSE_CURLY_BRACES);
                continue;
            }
            let (after, before) = match (end_scope_type, kind) {
                (Some(T_CLOSE_PARENTHESIS), T_EQUALS) => (
                    self.after_argument_assignments,
                    self.before_argument_assignments,
                ),
                (Some(T_CLOSE_CURLY_BRACES), T_EQUALS) => (
                    self.after_array_assignments_equals,
                    self.before_array_assignments_equals,
                ),
                (Some(T_CLOSE_CURLY_BRACES), T_COLON) => (
                    self.after_array_assignments_colon,
                    self.before_array_assignments_colon,
                ),
                _ => continue,
            };
            update_spaces_at(tokens, current + 1, current + 1, after);
            update_spaces_at(tokens, current - 1, current, before);
        }
    }
}

fn update_spaces_at(
    tokens: &mut DocTokens,
    index: usize,
    insert_index: usize,
    insert: Option<bool>,
) {
    let Some(insert) = insert else {
        return;
    };
    if insert {
        if tokens[index].kind == T_NONE {
            tokens[index].content = b" ".to_vec();
        } else {
            tokens.insert_at(insert_index, DocToken::new(T_NONE, " "));
        }
    } else if tokens[index].kind == T_NONE {
        tokens[index].clear();
    }
}
