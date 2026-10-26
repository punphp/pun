//! The configurable bans of Mago 1.57.0's linter (mago-linter src/rule/security/):
//!
//! - `lint/security/no-restricted-functions` (`disallowed-functions`, disallowed_functions.rs): a call to one of the
//!   `functions`, or to a function of one of the `extensions` (the extension names compared ignoring case, each
//!   extension's functions as Mago lists them, mago_extension_functions.rs). The first listed function that matches
//!   wins; otherwise the first function of the first listed extension in Mago's order.
//! - `lint/security/no-restricted-instantiation` (`disallowed-type-instantiation`,
//!   disallowed_type_instantiation.rs): `new` of one of the `types`, compared with the resolved class name ignoring
//!   case.
//!
//! Each entry is a name, or a table with `name`, an optional `help` that replaces the report's help and an optional
//! `level`. Pun takes a rule's level from pun.toml, so an entry's `level` is checked (Mago's level names) and then
//! not used. Both rules do nothing until configured.

use mago_syntax::cst::{Expression, Node, NodeKind};

use super::lint_b_names::{function_call_matches, resolve_class};
use super::mago_extension_functions::EXTENSION_FUNCTIONS;
use super::mago_helpers::configure;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// A `DisallowedEntry`: the name and the custom help.
#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub help: Option<String>,
}

/// The entries of a list option, Mago's untagged `DisallowedEntry`: keys other than `name`, `help` and `level` are
/// ignored, as serde does.
fn entries(value: &serde_json::Value) -> Option<Vec<Entry>> {
    value
        .as_array()?
        .iter()
        .map(|item| match item {
            serde_json::Value::String(name) => Some(Entry {
                name: name.clone(),
                help: None,
            }),
            serde_json::Value::Object(map) => {
                let name = map.get("name")?.as_str()?.to_string();
                let help = match map.get("help") {
                    None | Some(serde_json::Value::Null) => None,
                    Some(help) => Some(help.as_str()?.to_string()),
                };
                match map.get("level") {
                    None | Some(serde_json::Value::Null) => {}
                    Some(level) => {
                        let level = level.as_str()?;
                        [
                            "Note", "note", "Help", "help", "Warning", "warning", "warn", "Error",
                            "error", "err",
                        ]
                        .contains(&level)
                        .then_some(())?;
                    }
                }
                Some(Entry { name, help })
            }
            _ => None,
        })
        .collect()
}

#[derive(Default)]
pub struct NoRestrictedFunctions {
    pub functions: Vec<Entry>,
    pub extensions: Vec<Entry>,
}

impl Rule for NoRestrictedFunctions {
    fn id(&self) -> &'static str {
        "lint/security/no-restricted-functions"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "functions" => self.functions = entries(value)?,
                "extensions" => self.extensions = entries(value)?,
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        for entry in &self.functions {
            if !function_call_matches(ancestors, call, &entry.name) {
                continue;
            }
            out.push(Report {
                span: node_span(call),
                message: format!("Function `{}` is disallowed.", entry.name),
                help: Some(entry.help.clone().unwrap_or_else(|| {
                    "Use an alternative function or update your configuration if this restriction is no longer needed."
                        .into()
                })),
                fix: None,
            });
            return;
        }
        for (extension, functions) in &EXTENSION_FUNCTIONS {
            let Some(entry) = self
                .extensions
                .iter()
                .find(|e| e.name.eq_ignore_ascii_case(extension))
            else {
                continue;
            };
            let Some(function) = functions
                .iter()
                .find(|f| function_call_matches(ancestors, call, f))
            else {
                continue;
            };
            out.push(Report {
                span: node_span(call),
                message: format!("Function `{function}` from the `{extension}` extension is disallowed."),
                help: Some(entry.help.clone().unwrap_or_else(|| {
                    "Avoid using this extension or update your configuration if exceptions are acceptable.".into()
                })),
                fix: None,
            });
            return;
        }
    }
}

#[derive(Default)]
pub struct NoRestrictedInstantiation {
    pub types: Vec<Entry>,
}

impl Rule for NoRestrictedInstantiation {
    fn id(&self) -> &'static str {
        "lint/security/no-restricted-instantiation"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Instantiation])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "types" => self.types = entries(value)?,
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Instantiation(new) = node else {
            return;
        };
        let Expression::Identifier(identifier) = new.class else {
            return;
        };
        let class = resolve_class(ancestors, identifier);
        let Some(entry) = self
            .types
            .iter()
            .find(|e| class.eq_ignore_ascii_case(e.name.as_bytes()))
        else {
            return;
        };
        out.push(Report {
            span: node_span(new),
            message: format!("Direct instantiation of type `{}` is disallowed.", entry.name),
            help: Some(entry.help.clone().unwrap_or_else(|| {
                "Use an alternative factory or provider pattern, or update your configuration if this restriction is no longer needed."
                    .into()
            })),
            fix: None,
        });
    }
}
