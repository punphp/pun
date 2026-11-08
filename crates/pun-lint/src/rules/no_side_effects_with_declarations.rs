//! `lint/style/no-side-effects-with-declarations`, ported from Mago 1.57.0's `no-side-effects-with-declarations`
//! (mago-linter src/rule/best_practices/no_side_effects_with_declarations.rs).
//!
//! Reports a file that both declares symbols (classes, interfaces, traits, enums, functions, constants) and has
//! side-effects, once, on the first side-effect. Every statement outside symbol bodies, closures, arrow functions and
//! anonymous classes counts, nested ones included (the body of an `if` or a loop): tags, `use`, `declare`,
//! `namespace`, blocks and `;` are neutral. Options, as in Mago: `allow-conditional-declarations` (default true) makes
//! an `if` itself neutral (its body still counts), `allow-class-alias` and `allow-class-exists` (default true) make
//! a `class_alias()` or `class_exists()` call statement neutral.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::lint_b_names::function_call_matches;
use super::mago_helpers::{boolean, configure};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoSideEffectsWithDeclarations {
    pub allow_conditional_declarations: bool,
    pub allow_class_alias: bool,
    pub allow_class_exists: bool,
}

impl Default for NoSideEffectsWithDeclarations {
    fn default() -> Self {
        NoSideEffectsWithDeclarations {
            allow_conditional_declarations: true,
            allow_class_alias: true,
            allow_class_exists: true,
        }
    }
}

impl Rule for NoSideEffectsWithDeclarations {
    fn id(&self) -> &'static str {
        "lint/style/no-side-effects-with-declarations"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "allow-conditional-declarations" => {
                    self.allow_conditional_declarations = boolean(value)?;
                }
                "allow-class-alias" => self.allow_class_alias = boolean(value)?,
                "allow-class-exists" => self.allow_class_exists = boolean(value)?,
                _ => return None,
            }
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
        let mut collector = Collector {
            rule: self,
            scope: vec![node],
            has_declaration: false,
            first_side_effect: None,
        };
        collector.walk_program(program, &mut ());
        if let (true, Some(at)) = (collector.has_declaration, collector.first_side_effect) {
            out.push(Report {
                span: span(at),
                message: "File mixes symbol declarations and side-effects.".into(),
                help: Some(
                    "Move the side-effects into a separate file (e.g. a bootstrap or entry-point script).".into(),
                ),
                fix: None,
            });
        }
    }
}

/// `StatementCollector`. `scope` is the program and the namespace being walked: the ancestors name resolution needs.
struct Collector<'r, 'ast, 'arena> {
    rule: &'r NoSideEffectsWithDeclarations,
    scope: Vec<Node<'ast, 'arena>>,
    has_declaration: bool,
    first_side_effect: Option<mago_span::Span>,
}

impl<'arena> Collector<'_, '_, 'arena> {
    fn is_allowed_expression(&self, statement: &ExpressionStatement<'arena>) -> bool {
        let Expression::Call(Call::Function(call)) = statement.expression else {
            return false;
        };
        (self.rule.allow_class_alias && function_call_matches(&self.scope, call, "class_alias"))
            || (self.rule.allow_class_exists
                && function_call_matches(&self.scope, call, "class_exists"))
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Collector<'_, 'ast, 'arena> {
    fn walk_in_namespace(&mut self, namespace: &'ast Namespace<'arena>, (): &mut ()) {
        self.scope.push(Node::Namespace(namespace));
    }

    fn walk_out_namespace(&mut self, _: &'ast Namespace<'arena>, (): &mut ()) {
        self.scope.pop();
    }

    fn walk_in_statement(&mut self, statement: &'ast Statement<'arena>, (): &mut ()) {
        match statement {
            _ if statement.is_declaration()
                && !matches!(statement, Statement::Declare(_) | Statement::Namespace(_)) =>
            {
                self.has_declaration = true;
            }
            Statement::OpeningTag(_)
            | Statement::ClosingTag(_)
            | Statement::Use(_)
            | Statement::Declare(_)
            | Statement::Namespace(_)
            | Statement::Block(_)
            | Statement::Noop(_) => {}
            Statement::If(_) if self.rule.allow_conditional_declarations => {}
            Statement::Expression(e) if self.is_allowed_expression(e) => {}
            _ => {
                self.first_side_effect.get_or_insert(statement.span());
            }
        }
    }

    fn walk_class(&mut self, _: &'ast Class<'arena>, (): &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, (): &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, (): &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, (): &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, (): &mut ()) {}
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, (): &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, (): &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, (): &mut ()) {}
}
