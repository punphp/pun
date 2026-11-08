//! `lint/style/require-phpdoc`, ported from Mago 1.57.0's `missing-docs` (mago-linter src/rule/clarity/missing_docs.rs).
//!
//! Reports a declaration with no docblock right before it (only whitespace and other comments between): top-level
//! and namespaced functions, classes, interfaces, traits, enums, constants and `static` variables, and the
//! constants, enum cases, methods and properties of a top-level class-like. Each kind has a boolean option, as in
//! Mago: `functions`, `enum-cases`, `constants`, `statics`, `methods` and `properties` default to true, `classes`,
//! `interfaces`, `traits` and `enums` to false. `exclusions` holds a list of regular expressions per kind (Rust's
//! `regex`, as Mago; an invalid one is ignored, as Mago does): a declaration is skipped when every name it declares
//! matches one of them.

use mago_span::HasSpan;
use mago_syntax::comments::docblock::get_docblock_before_position;
use mago_syntax::cst::{
    ClassLikeMember, DirectVariable, Node, NodeKind, Program, Property, Statement,
};
use regex::bytes::Regex;

use super::mago_helpers::{boolean, configure};
use crate::syntax::span;
use crate::{Context, Report, Rule};

/// The kinds, in Mago's order: (option name, checked by default).
const KINDS: [(&str, bool); 10] = [
    ("functions", true),
    ("classes", false),
    ("interfaces", false),
    ("traits", false),
    ("enums", false),
    ("enum-cases", true),
    ("constants", true),
    ("statics", true),
    ("methods", true),
    ("properties", true),
];

const FUNCTIONS: usize = 0;
const CLASSES: usize = 1;
const INTERFACES: usize = 2;
const TRAITS: usize = 3;
const ENUMS: usize = 4;
const ENUM_CASES: usize = 5;
const CONSTANTS: usize = 6;
const STATICS: usize = 7;
const METHODS: usize = 8;
const PROPERTIES: usize = 9;

pub struct RequirePhpdoc {
    /// Whether each kind of [`KINDS`] is checked.
    pub checked: [bool; 10],
    /// The exclusion patterns of each kind.
    pub exclusions: [Vec<Regex>; 10],
}

impl Default for RequirePhpdoc {
    fn default() -> Self {
        RequirePhpdoc {
            checked: KINDS.map(|(_, on)| on),
            exclusions: Default::default(),
        }
    }
}

impl Rule for RequirePhpdoc {
    fn id(&self) -> &'static str {
        "lint/style/require-phpdoc"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            if key == "exclusions" {
                for (kind, patterns) in value.as_object()? {
                    let i = KINDS.iter().position(|(k, _)| k == kind)?;
                    self.exclusions[i] = patterns
                        .as_array()?
                        .iter()
                        .map(|p| p.as_str().map(Regex::new))
                        .collect::<Option<Vec<_>>>()?
                        .into_iter()
                        .filter_map(Result::ok)
                        .collect();
                }
                return Some(());
            }
            let i = KINDS.iter().position(|(k, _)| *k == key)?;
            self.checked[i] = boolean(value)?;
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        for statement in &program.statements {
            self.check_statement(program, statement, out);
        }
    }
}

impl RequirePhpdoc {
    fn check_statement(
        &self,
        program: &Program<'_>,
        statement: &Statement<'_>,
        out: &mut Vec<Report>,
    ) {
        match statement {
            Statement::Function(f) => {
                self.check(program, f, FUNCTIONS, "function", &[f.name.value], out);
            }
            Statement::Namespace(namespace) => {
                for inner in namespace.statements() {
                    self.check_statement(program, inner, out);
                }
            }
            Statement::Class(c) => {
                self.check(program, c, CLASSES, "class", &[c.name.value], out);
                self.check_members(program, c.members.iter(), out);
            }
            Statement::Interface(i) => {
                self.check(program, i, INTERFACES, "interface", &[i.name.value], out);
                self.check_members(program, i.members.iter(), out);
            }
            Statement::Trait(t) => {
                self.check(program, t, TRAITS, "trait", &[t.name.value], out);
                self.check_members(program, t.members.iter(), out);
            }
            Statement::Enum(e) => {
                self.check(program, e, ENUMS, "enum", &[e.name.value], out);
                self.check_members(program, e.members.iter(), out);
            }
            Statement::Constant(c) => {
                let names: Vec<&[u8]> = c.items.iter().map(|i| i.name.value).collect();
                self.check(program, c, CONSTANTS, "constant", &names, out);
            }
            Statement::Static(s) => {
                let names: Vec<&[u8]> = s
                    .items
                    .iter()
                    .map(|i| variable_name(i.variable()))
                    .collect();
                self.check(program, s, STATICS, "static variable", &names, out);
            }
            _ => {}
        }
    }

    fn check_members<'a, 'arena: 'a>(
        &self,
        program: &Program<'_>,
        members: impl Iterator<Item = &'a ClassLikeMember<'arena>>,
        out: &mut Vec<Report>,
    ) {
        for member in members {
            match member {
                ClassLikeMember::Constant(c) => {
                    let names: Vec<&[u8]> = c.items.iter().map(|i| i.name.value).collect();
                    self.check(program, c, CONSTANTS, "class constant", &names, out);
                }
                ClassLikeMember::EnumCase(c) => {
                    self.check(
                        program,
                        c,
                        ENUM_CASES,
                        "enum case",
                        &[c.item.name().value],
                        out,
                    );
                }
                ClassLikeMember::Method(m) => {
                    self.check(program, m, METHODS, "method", &[m.name.value], out);
                }
                ClassLikeMember::Property(p) => {
                    let names = match p {
                        Property::Plain(plain) => plain
                            .items
                            .iter()
                            .map(|i| variable_name(i.variable()))
                            .collect(),
                        Property::Hooked(hooked) => vec![variable_name(hooked.item.variable())],
                    };
                    self.check(program, p, PROPERTIES, "property", &names, out);
                }
                _ => {}
            }
        }
    }

    /// `check_docs`, for a kind that is checked.
    fn check(
        &self,
        program: &Program<'_>,
        node: &impl HasSpan,
        kind: usize,
        subject: &str,
        names: &[&[u8]],
        out: &mut Vec<Report>,
    ) {
        if !self.checked[kind] {
            return;
        }
        let exclusions = &self.exclusions[kind];
        let excluded = !exclusions.is_empty()
            && !names.is_empty()
            && names
                .iter()
                .all(|name| exclusions.iter().any(|p| p.is_match(name)));
        if excluded
            || get_docblock_before_position(program.trivia.as_slice(), node.span().start.offset)
                .is_some()
        {
            return;
        }
        out.push(Report {
            span: span(node.span()),
            message: format!("Missing docblock for {subject}"),
            help: Some(format!("Add a docblock above this {subject}")),
            fix: None,
        });
    }
}

fn variable_name<'arena>(variable: &DirectVariable<'arena>) -> &'arena [u8] {
    variable.name.strip_prefix(b"$").unwrap_or(variable.name)
}
