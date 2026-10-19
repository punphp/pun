//! GetNonVirtualPropertyHookReadRule (ClassPropertiesNode) and SetNonVirtualPropertyHookAssignRule
//! (PropertyHookReturnStatementsNode), both level 3: a hook with a body of a non-virtual property that never reads
//! (`get`) or does not always assign (`set`) the property's backing value.

use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::Tri;

use super::Checker;
use super::properties::{PropertyNode, is_virtual};

/// Both rules on a property node of the class-like being walked.
pub fn check(c: &mut Checker, node: &PropertyNode) {
    if c.level < 3 || is_virtual(c, node) {
        return;
    }
    let name = &node.name[1..];
    let k = c.class_scope().expect("a property is walked inside its class-like");
    let class = c.display(k);
    let property = String::from_utf8_lossy(name);
    for hook in node.hooks {
        let PropertyHookBody::Concrete(body) = &hook.body else {
            continue;
        };
        if hook.name.value.eq_ignore_ascii_case(b"get") {
            let mut reads = Reads { name, found: false };
            match body {
                PropertyHookConcreteBody::Expression(e) => reads.walk_expression(e.expression, &mut ()),
                PropertyHookConcreteBody::Block(b) => reads.walk_block(b, &mut ()),
            }
            if !reads.found {
                let message = format!("Get hook for non-virtual property {class}::${property} does not read its value.");
                report(c, hook, "propertyGetHook.noRead", message);
            }
            // only the first `get` hook
            break;
        }
    }
    for hook in node.hooks.iter().filter(|h| h.name.value.eq_ignore_ascii_case(b"set")) {
        // a short `set => expr` body assigns its value
        let PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) = &hook.body else {
            continue;
        };
        let mut flow = Flow { name, ends: Vec::new() };
        let mut end = Some(Tri::No);
        for s in b.statements.iter() {
            let Some(before) = end else {
                break;
            };
            let returns = flow.ends.len();
            end = flow.statement(s, before);
            // a statement that ends the hook counts with its scope, unless it is a `throw` or `exit` itself
            // ponytail: the scope before it, not where inside it the hook ends
            if end.is_none() && flow.ends.len() == returns && !terminates(s) {
                flow.ends.push(before);
            }
        }
        let Some(has) = flow.ends.into_iter().chain(end).reduce(merge) else {
            continue;
        };
        if !has.yes() {
            let always = if has.maybe() { "always " } else { "" };
            let message = format!("Set hook for non-virtual property {class}::${property} does not {always}assign value to it.");
            report(c, hook, "propertySetHook.noAssign", message);
        }
    }
}

/// ClassPropertiesNode and PropertyHookReturnStatementsNode are the class's: a trait's property reports in the
/// using class's file.
fn report(c: &mut Checker, hook: &PropertyHook, identifier: &str, message: String) {
    c.report(hook, identifier, message, None);
    if let Some(t) = &c.trait_context
        && let Some(e) = c.errors.last_mut()
    {
        e.file = t.class_file.clone();
    }
}

/// A `throw` or `exit` statement.
fn terminates(s: &Statement) -> bool {
    matches!(s, Statement::Expression(e) if matches!(e.expression, Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_))))
}

/// A `$this->name` read: any fetch but the target of a plain assignment.
struct Reads<'n> {
    name: &'n [u8],
    found: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Reads<'_> {
    fn walk_assignment(&mut self, a: &'ast Assignment<'arena>, c: &mut ()) {
        if !matches!(a.operator, AssignmentOperator::Assign(_)) || !is_fetch(a.lhs, self.name) {
            self.walk_expression(a.lhs, c);
        }
        self.walk_expression(a.rhs, c);
    }
    fn walk_in_property_access(&mut self, a: &'ast PropertyAccess<'arena>, _: &mut ()) {
        self.found |= is_this_fetch(a, self.name);
    }
}

fn is_fetch(e: &Expression, name: &[u8]) -> bool {
    matches!(e, Expression::Access(Access::Property(a)) if is_this_fetch(a, name))
}

fn is_this_fetch(a: &PropertyAccess, name: &[u8]) -> bool {
    matches!(
        (a.object, &a.property),
        (Expression::Variable(Variable::Direct(v)), ClassLikeMemberSelector::Identifier(p)) if v.name == b"$this" && p.value == name
    )
}

/// MutatingScope::mergeWith() of PropertyInitializationExpr's certainty.
fn merge(a: Tri, b: Tri) -> Tri {
    if a == b { a } else { Tri::Maybe }
}

/// Whether `$this->name` is assigned at each way out of a set hook's body: its `return` statements and its end.
/// ponytail: branches and loops (zero or more runs) merge; only `throw` and `exit` terminate a path, not calls to
/// `never` functions; `try` and `switch` count an assignment anywhere inside as certain.
struct Flow<'n> {
    name: &'n [u8],
    ends: Vec<Tri>,
}

impl Flow<'_> {
    /// The state after `statements` from `state` (`None`: unreachable).
    fn statements(&mut self, statements: &[Statement], mut state: Option<Tri>) -> Option<Tri> {
        for s in statements {
            state = self.statement(s, state?);
        }
        state
    }

    fn body(&mut self, s: &Statement, state: Tri) -> Option<Tri> {
        self.statements(std::slice::from_ref(s), Some(state))
    }

    fn statement(&mut self, s: &Statement, state: Tri) -> Option<Tri> {
        match s {
            Statement::Return(_) => {
                // ReturnStatement's scope is before its expression, the execution end's after it
                self.ends.push(state);
                if self.assigns(s) {
                    self.ends.push(Tri::Yes);
                }
                None
            }
            s if terminates(s) => None,
            Statement::Block(b) => self.statements(b.statements.as_slice(), Some(state)),
            Statement::If(i) => {
                let state = if self.assigns_expression(i.condition) { Tri::Yes } else { state };
                let mut branches: Vec<Option<Tri>> = Vec::new();
                let has_else = match &i.body {
                    IfBody::Statement(b) => {
                        branches.push(self.body(b.statement, state));
                        for e in b.else_if_clauses.iter() {
                            branches.push(self.body(e.statement, state));
                        }
                        b.else_clause.as_ref().map(|e| branches.push(self.body(e.statement, state))).is_some()
                    }
                    IfBody::ColonDelimited(b) => {
                        branches.push(self.statements(b.statements.as_slice(), Some(state)));
                        for e in b.else_if_clauses.iter() {
                            branches.push(self.statements(e.statements.as_slice(), Some(state)));
                        }
                        b.else_clause.as_ref().map(|e| branches.push(self.statements(e.statements.as_slice(), Some(state)))).is_some()
                    }
                };
                if !has_else {
                    branches.push(Some(state));
                }
                branches.into_iter().flatten().reduce(merge)
            }
            // a loop body runs zero or more times, a `do`-`while` body at least once
            Statement::While(w) => {
                let after = match &w.body {
                    WhileBody::Statement(b) => self.body(b, state),
                    WhileBody::ColonDelimited(b) => self.statements(b.statements.as_slice(), Some(state)),
                };
                Some(after.map_or(state, |a| merge(a, state)))
            }
            Statement::For(l) => {
                let after = match &l.body {
                    ForBody::Statement(b) => self.body(b, state),
                    ForBody::ColonDelimited(b) => self.statements(b.statements.as_slice(), Some(state)),
                };
                Some(after.map_or(state, |a| merge(a, state)))
            }
            Statement::Foreach(l) => {
                let after = match &l.body {
                    ForeachBody::Statement(b) => self.body(b, state),
                    ForeachBody::ColonDelimited(b) => self.statements(b.statements.as_slice(), Some(state)),
                };
                Some(after.map_or(state, |a| merge(a, state)))
            }
            Statement::DoWhile(d) => self.body(d.statement, state),
            s => Some(if self.assigns(s) { Tri::Yes } else { state }),
        }
    }

    fn assigns(&self, s: &Statement) -> bool {
        let mut a = Assigns { name: self.name, found: false };
        a.walk_statement(s, &mut ());
        a.found
    }

    fn assigns_expression(&self, e: &Expression) -> bool {
        let mut a = Assigns { name: self.name, found: false };
        a.walk_expression(e, &mut ());
        a.found
    }
}

/// An assignment to `$this->name`.
struct Assigns<'n> {
    name: &'n [u8],
    found: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Assigns<'_> {
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        self.found |= is_fetch(a.lhs, self.name);
    }
}
