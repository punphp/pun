//! `lint/complexity/no-boolean-flag-parameter`, ported from Mago 1.57.0's `no-boolean-flag-parameter`
//! (mago-linter src/rule/maintainability/no_boolean_flag_parameter.rs).
//!
//! Reports a parameter typed `bool` (not promoted) that the function branches on: it appears in the condition of an
//! `if`, `elseif` or ternary of the body. A parameter of a function-like without a body (an abstract method, an
//! arrow function) is always reported. Options: `exclude-constructors` (default true) and `exclude-setters`
//! (default false, methods whose name starts with `set` and is longer than that).

use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::mago_helpers::{boolean, configure, function_like_scope};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoBooleanFlagParameter {
    pub exclude_setters: bool,
    pub exclude_constructors: bool,
}

impl Default for NoBooleanFlagParameter {
    fn default() -> Self {
        NoBooleanFlagParameter {
            exclude_setters: false,
            exclude_constructors: true,
        }
    }
}

impl Rule for NoBooleanFlagParameter {
    fn id(&self) -> &'static str {
        "lint/complexity/no-boolean-flag-parameter"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionLikeParameter])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "exclude-setters" => self.exclude_setters = boolean(value)?,
                "exclude-constructors" => self.exclude_constructors = boolean(value)?,
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
        let Node::FunctionLikeParameter(parameter) = node else {
            return;
        };
        if parameter.is_promoted_property() || !matches!(parameter.hint, Some(Hint::Bool(_))) {
            return;
        }
        let scope = function_like_scope(ancestors);
        if let Some(Node::Method(method)) = scope {
            let name = method.name.value;
            if self.exclude_constructors && name.eq_ignore_ascii_case(b"__construct") {
                return;
            }
            if self.exclude_setters && name.len() > 3 && name[..3].eq_ignore_ascii_case(b"set") {
                return;
            }
        }
        let body = match scope {
            Some(Node::Function(f)) => Some(&f.body),
            Some(Node::Method(m)) => match &m.body {
                MethodBody::Concrete(b) => Some(b),
                MethodBody::Abstract(_) => None,
            },
            Some(Node::Closure(c)) => Some(&c.body),
            _ => None,
        };
        if let Some(body) = body {
            let mut walker = FlagWalker {
                target: parameter.variable.name,
                used_as_flag: false,
            };
            walker.walk_block(body, &mut ());
            if !walker.used_as_flag {
                return;
            }
        }
        out.push(Report {
            span: node_span(&parameter.variable),
            message: "Avoid boolean flag parameters.".into(),
            help: Some(
                "Refactor by splitting the function into two separate methods, each with a clear, descriptive name."
                    .into(),
            ),
            fix: None,
        });
    }
}

/// `FlagParameterWalker`: whether the parameter appears in a branching condition.
struct FlagWalker<'t> {
    target: &'t [u8],
    used_as_flag: bool,
}

impl FlagWalker<'_> {
    fn check_condition(&mut self, condition: &Expression<'_>) {
        if self.used_as_flag {
            return;
        }
        let mut finder = VariableFinder {
            target: self.target,
            found: false,
        };
        finder.walk_expression(condition, &mut ());
        self.used_as_flag = finder.found;
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for FlagWalker<'_> {
    fn walk_in_if(&mut self, r#if: &'ast If<'arena>, _: &mut ()) {
        self.check_condition(r#if.condition);
    }

    fn walk_in_if_statement_body_else_if_clause(
        &mut self,
        clause: &'ast IfStatementBodyElseIfClause<'arena>,
        _: &mut (),
    ) {
        self.check_condition(clause.condition);
    }

    fn walk_in_if_colon_delimited_body_else_if_clause(
        &mut self,
        clause: &'ast IfColonDelimitedBodyElseIfClause<'arena>,
        _: &mut (),
    ) {
        self.check_condition(clause.condition);
    }

    fn walk_in_conditional(&mut self, conditional: &'ast Conditional<'arena>, _: &mut ()) {
        self.check_condition(conditional.condition);
    }
}

struct VariableFinder<'t> {
    target: &'t [u8],
    found: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for VariableFinder<'_> {
    fn walk_in_direct_variable(&mut self, variable: &'ast DirectVariable<'arena>, _: &mut ()) {
        if variable.name == self.target {
            self.found = true;
        }
    }
}
