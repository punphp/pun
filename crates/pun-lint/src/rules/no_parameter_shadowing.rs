//! `lint/style/no-parameter-shadowing`, ported from Mago 1.57.0's linter rule `no-parameter-shadowing`
//! (rule/best_practices/no_parameter_shadowing.rs).
//!
//! A parameter of a function, concrete method or closure that its body reuses as a `foreach` key or value, a `catch`
//! variable, a `global` or a `static` variable is reported once, on the parameter. Nested functions, closures, arrow
//! functions, methods and property hooks have their own parameters and are not looked into.

use std::collections::HashMap;

use mago_span::Span;
use mago_syntax::cst::{
    ArrowFunction, Closure, Expression, Foreach, ForeachTarget, Function, Global, Method,
    MethodBody, Node, NodeKind, PropertyHook, Static, TryCatchClause, Variable,
};
use mago_syntax::walker::MutWalker;

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoParameterShadowing;

impl Rule for NoParameterShadowing {
    fn id(&self) -> &'static str {
        "lint/style/no-parameter-shadowing"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (parameter_list, body) = match node {
            Node::Function(f) => (&f.parameter_list, &f.body),
            Node::Method(Method {
                parameter_list,
                body: MethodBody::Concrete(body),
                ..
            }) => (parameter_list, body),
            _ => {
                let Node::Closure(c) = node else { return };
                (&c.parameter_list, &c.body)
            }
        };
        let params: HashMap<&[u8], Span> = parameter_list
            .parameters
            .iter()
            .map(|p| (p.variable.name, p.variable.span))
            .collect();
        let mut collector = ShadowCollector {
            params: &params,
            shadows: Vec::new(),
        };
        collector.walk_block(body, &mut ());
        for name in collector.shadows {
            let name = String::from_utf8_lossy(name);
            out.push(Report {
                span: span(params[name.as_bytes()]),
                message: format!(
                    "Parameter `{name}` is shadowed by a loop, catch, `global`, or `static` variable."
                ),
                help: Some(format!(
                    "Rename the variable to avoid shadowing the `{name}` parameter."
                )),
                fix: None,
            });
        }
    }
}

struct ShadowCollector<'rule, 'arena> {
    params: &'rule HashMap<&'arena [u8], Span>,
    /// The shadowed parameters, each once.
    shadows: Vec<&'arena [u8]>,
}

impl<'arena> ShadowCollector<'_, 'arena> {
    fn check_variable(&mut self, name: &'arena [u8]) {
        if self.params.contains_key(name) && !self.shadows.contains(&name) {
            self.shadows.push(name);
        }
    }

    fn check_expression(&mut self, expr: &Expression<'arena>) {
        let Expression::Variable(Variable::Direct(var)) = expr else {
            return;
        };
        self.check_variable(var.name);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ShadowCollector<'_, 'arena> {
    fn walk_in_foreach(&mut self, foreach: &'ast Foreach<'arena>, (): &mut ()) {
        match &foreach.target {
            ForeachTarget::Value(v) => self.check_expression(v.value),
            ForeachTarget::KeyValue(kv) => {
                self.check_expression(kv.key);
                self.check_expression(kv.value);
            }
        }
    }

    fn walk_in_try_catch_clause(&mut self, clause: &'ast TryCatchClause<'arena>, (): &mut ()) {
        if let Some(var) = &clause.variable {
            self.check_variable(var.name);
        }
    }

    fn walk_in_global(&mut self, global: &'ast Global<'arena>, (): &mut ()) {
        for variable in &global.variables {
            if let Variable::Direct(var) = variable {
                self.check_variable(var.name);
            }
        }
    }

    fn walk_in_static(&mut self, r#static: &'ast Static<'arena>, (): &mut ()) {
        for item in &r#static.items {
            self.check_variable(item.variable().name);
        }
    }

    fn walk_function(&mut self, _: &'ast Function<'arena>, (): &mut ()) {}
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, (): &mut ()) {}
    fn walk_method(&mut self, _: &'ast Method<'arena>, (): &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, (): &mut ()) {}
    fn walk_property_hook(&mut self, _: &'ast PropertyHook<'arena>, (): &mut ()) {}
}
