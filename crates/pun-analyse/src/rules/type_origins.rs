//! Source edges for information Mago erases: PHPDoc shape order and explicit mixed. These are not inferred types.
//! Local copies, declarations and inline call arguments carry an origin. If/ternary/short-circuit branches
//! read incoming state and join common origins or the upstream-selected equal mixed declaration. Other flow
//! joins, replacements and reference escapes forget origins. Offset unsets preserve source metadata, not values;
//! conversion still requires Mago's actual expression type, including its post-unset narrowing.

use std::collections::HashMap;
use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_database::file::File;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use mago_word::Word;

use super::super::function_checks;

pub(in super::super) type Function = (Word, Word);

#[derive(PartialEq, Eq)]
pub(in super::super) struct Call {
    pub name: String,
    /// An instance receiver, whose actual inferred type must resolve the method before its docs are used.
    pub receiver: Option<Span>,
    /// A written static class name, or the lexical `self`/`static`/`parent` keyword. Neither field: a function.
    pub class: Option<Word>,
    pub args: Vec<(Option<String>, Span)>,
}

#[derive(PartialEq, Eq)]
pub(in super::super) struct MethodCallGuard {
    pub receiver: Span,
    pub name: String,
    pub has_arguments: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub(in super::super) enum Origin {
    Parameter(Function, usize),
    Annotation(Span, String, Option<Function>),
    Copy(Span),
    /// A literal's key/value expressions, in PHP insertion order. No unknown keys or unpacks are invented.
    Array(Vec<(Option<Span>, Span)>),
    Offset(Span, Span),
    /// Ordered, explicit offset unsets: virtual write-back can append a previously absent parent key.
    Unset(Span, Vec<Span>),
    Call(Arc<Call>),
    Callback(Arc<Call>, usize, usize),
    /// Calls with arguments must resolve the receiver and prove its native signature cannot pass references.
    AfterMethodCall(Arc<MethodCallGuard>, Arc<Origin>),
}

pub(in super::super) type Origins = HashMap<Span, Origin>;

fn branching(e: &Expression) -> bool {
    matches!(
        e,
        Expression::Match(_)
            | Expression::Call(mago_syntax::cst::Call::NullSafeMethod(_))
            | Expression::Access(Access::NullSafeProperty(_))
    )
}

/// `functions` are the file's function-likes ([`super::super::driver::Checks`]'s table).
pub(super) fn collect(
    file: &File,
    comments: &super::super::Comments,
    functions: &[(Span, Function)],
    program: &Program,
    names: &ResolvedNames,
    codebase: &CodebaseMetadata,
) -> Origins {
    let mut walker = Sources {
        file,
        comments,
        names,
        codebase,
        functions: functions.iter().copied().collect(),
        stack: Vec::new(),
        locals: HashMap::new(),
        current: None,
        calls: Vec::new(),
        unset_targets: Vec::new(),
        control_depth: 0,
        found: HashMap::new(),
    };
    walker.walk_program(program, &mut ());
    walker.found
}

/// Keep dependency artifacts, but do not cause unrelated declarations to be analysed.
pub(super) fn dependencies(found: &Origins, wanted: &[Span]) -> Vec<Span> {
    let mut seen: std::collections::HashSet<Span> = wanted.iter().copied().collect();
    let mut pending = wanted.to_vec();
    let mut out = Vec::new();
    while let Some(span) = pending.pop() {
        let mut origin = found.get(&span);
        let mut more = Vec::new();
        while let Some(Origin::AfterMethodCall(call, previous)) = origin {
            more.push(call.receiver);
            origin = Some(previous);
        }
        more.extend(match origin {
            Some(Origin::Copy(s)) => vec![*s],
            Some(Origin::Array(items)) => items
                .iter()
                .flat_map(|(key, value)| key.iter().copied().chain(std::iter::once(*value)))
                .collect(),
            Some(Origin::Offset(array, key)) => vec![*array, *key],
            Some(Origin::Unset(array, keys)) => std::iter::once(*array)
                .chain(keys.iter().copied())
                .collect(),
            Some(Origin::Call(call) | Origin::Callback(call, ..)) => call
                .args
                .iter()
                .map(|(_, s)| *s)
                .chain(call.receiver)
                .collect(),
            _ => Vec::new(),
        });
        for span in more {
            if seen.insert(span) {
                out.push(span);
                pending.push(span);
            }
        }
    }
    out
}

struct Sources<'a, 'arena> {
    file: &'a File,
    comments: &'a super::super::Comments,
    names: &'a ResolvedNames<'arena>,
    codebase: &'a CodebaseMetadata,
    functions: HashMap<Span, Function>,
    stack: Vec<(HashMap<Vec<u8>, Origin>, Option<Function>)>,
    locals: HashMap<Vec<u8>, Origin>,
    current: Option<Function>,
    calls: Vec<Option<Arc<Call>>>,
    unset_targets: Vec<Span>,
    control_depth: usize,
    found: Origins,
}

impl Sources<'_, '_> {
    fn enter(&mut self, span: Span, params: &FunctionLikeParameterList) {
        self.stack
            .push((std::mem::take(&mut self.locals), self.current));
        self.current = self.functions.get(&span).copied();
        for (i, p) in params.parameters.iter().enumerate() {
            if let Some(function) = self.current {
                self.locals
                    .insert(p.variable.name.to_vec(), Origin::Parameter(function, i));
            }
        }
    }

    fn leave(&mut self) {
        (self.locals, self.current) = self.stack.pop().unwrap();
    }

    fn explicit_mixed_parameter(&self, origin: &Origin) -> bool {
        match origin {
            Origin::Parameter(function, index) => self
                .codebase
                .function_likes
                .get(function)
                .map(|f| &f.parameters[*index])
                .is_some_and(|p| {
                    p.type_declaration_metadata
                        .as_ref()
                        .is_none_or(|t| t.type_union.is_mixed())
                        && p.type_metadata
                            .as_ref()
                            .is_some_and(|t| t.type_union.is_mixed())
                }),
            Origin::Copy(span) => self
                .found
                .get(span)
                .is_some_and(|o| self.explicit_mixed_parameter(o)),
            _ => false,
        }
    }

    fn merge(&mut self, mut paths: Vec<HashMap<Vec<u8>, Origin>>) {
        let last = paths.pop().unwrap_or_default();
        self.locals.clear();
        for (name, origin) in last {
            if !paths.iter().all(|p| p.contains_key(&name)) {
                continue;
            }
            // ExpressionTypeHolder::and() keeps its left type when MixedType::equals() ignores the
            // explicit flag. Callers order paths as the upstream merge, not by origin preference.
            if self.explicit_mixed_parameter(&origin)
                || paths.iter().all(|p| p.get(&name) == Some(&origin))
            {
                self.locals.insert(name, origin);
            }
        }
    }

    fn closure(&mut self, span: Span, params: &FunctionLikeParameterList) {
        let callback = self.calls.last().and_then(|call| {
            let call = call.as_ref()?;
            Some((
                Arc::clone(call),
                call.args.iter().position(|(_, s)| *s == span)?,
            ))
        });
        self.enter(span, params);
        if let Some((call, arg)) = callback {
            for (i, p) in params.parameters.iter().enumerate() {
                self.locals.insert(
                    p.variable.name.to_vec(),
                    Origin::Callback(Arc::clone(&call), arg, i),
                );
            }
        }
    }

    fn call(&mut self, span: Span, arguments: &ArgumentList, target: Option<Call>) {
        // Larastan's LiteralExtension reads one unpacked argument's array, marked `...`, and
        // ModelOnlyDynamicMethodReturnTypeExtension each argument's array, unpacked or not
        let literal = |call: &Call| {
            call.receiver.is_none()
                && call.class.is_none()
                && call.name.eq_ignore_ascii_case("literal")
                && arguments.arguments.len() == 1
                || call.receiver.is_some() && call.name.eq_ignore_ascii_case("only")
        };
        let origin = target
            .filter(|call| !arguments.arguments.iter().any(Argument::is_unpacked) || literal(call))
            .map(|mut call| {
                call.args = arguments
                    .arguments
                    .iter()
                    .map(|arg| {
                        let name = match arg {
                            Argument::Named(n) => {
                                Some(String::from_utf8_lossy(n.name.value).into_owned())
                            }
                            arg if arg.is_unpacked() => Some("...".to_string()),
                            _ => None,
                        };
                        (name, arg.value().span())
                    })
                    .collect();
                Arc::new(call)
            });
        if let Some(origin) = &origin {
            self.found.insert(span, Origin::Call(Arc::clone(origin)));
        }
        self.calls.push(origin);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Sources<'_, 'arena> {
    fn walk_in_function(&mut self, f: &'ast mago_syntax::cst::Function<'arena>, (): &mut ()) {
        self.enter(f.span(), &f.parameter_list);
    }
    fn walk_out_function(&mut self, _: &'ast mago_syntax::cst::Function<'arena>, (): &mut ()) {
        self.leave();
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, (): &mut ()) {
        self.enter(m.span(), &m.parameter_list);
    }
    fn walk_out_method(&mut self, _: &'ast Method<'arena>, (): &mut ()) {
        self.leave();
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, (): &mut ()) {
        self.closure(f.span(), &f.parameter_list);
    }
    fn walk_out_closure(&mut self, f: &'ast Closure<'arena>, (): &mut ()) {
        self.leave();
        if f.use_clause
            .as_ref()
            .is_some_and(|u| u.variables.iter().any(|v| v.ampersand.is_some()))
        {
            self.locals.clear();
        }
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, (): &mut ()) {
        self.closure(f.span(), &f.parameter_list);
    }
    fn walk_out_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, (): &mut ()) {
        self.leave();
    }

    fn walk_in_statement(&mut self, s: &'ast Statement<'arena>, (): &mut ()) {
        if matches!(
            s,
            Statement::For(_)
                | Statement::Foreach(_)
                | Statement::While(_)
                | Statement::DoWhile(_)
                | Statement::Switch(_)
                | Statement::Try(_)
                | Statement::Global(_)
                | Statement::Static(_)
        ) {
            self.locals.clear();
            self.control_depth += 1;
        }
    }
    fn walk_out_statement(&mut self, s: &'ast Statement<'arena>, (): &mut ()) {
        if matches!(
            s,
            Statement::For(_)
                | Statement::Foreach(_)
                | Statement::While(_)
                | Statement::DoWhile(_)
                | Statement::Switch(_)
                | Statement::Try(_)
                | Statement::Global(_)
                | Statement::Static(_)
        ) {
            self.locals.clear();
            self.control_depth -= 1;
        }
    }

    #[expect(
        clippy::assigning_clones,
        reason = "`next` was moved out on the previous pass"
    )]
    fn walk_if(&mut self, i: &'ast If<'arena>, ctx: &mut ()) {
        self.walk_expression(i.condition, ctx);
        let mut branches: Vec<(Option<&Expression>, Vec<&Statement>)> = Vec::new();
        let has_else = match &i.body {
            IfBody::Statement(b) => {
                branches.push((None, vec![b.statement]));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), vec![e.statement])),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, vec![e.statement]));
                }
                b.else_clause.is_some()
            }
            IfBody::ColonDelimited(b) => {
                branches.push((None, b.statements.iter().collect()));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), e.statements.iter().collect())),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, e.statements.iter().collect()));
                }
                b.else_clause.is_some()
            }
        };
        let mut next = self.locals.clone();
        let mut paths = Vec::new();
        for (condition, statements) in branches {
            self.locals = next;
            if let Some(condition) = condition {
                self.walk_expression(condition, ctx);
            }
            next = self.locals.clone();
            for statement in &statements {
                self.walk_statement(statement, ctx);
            }
            if !super::super::scope::statements_terminate(statements.into_iter()) {
                paths.push(self.locals.clone());
            }
        }
        if !has_else {
            paths.push(next);
        }
        self.merge(paths);
    }

    fn walk_conditional(&mut self, c: &'ast Conditional<'arena>, ctx: &mut ()) {
        self.walk_expression(c.condition, ctx);
        let entry = self.locals.clone();
        if let Some(then) = c.then {
            self.walk_expression(then, ctx);
        }
        let yes = std::mem::replace(&mut self.locals, entry);
        self.walk_expression(c.r#else, ctx);
        self.merge(vec![self.locals.clone(), yes]);
    }

    fn walk_binary(&mut self, b: &'ast Binary<'arena>, ctx: &mut ()) {
        self.walk_expression(b.lhs, ctx);
        let skipped = self.locals.clone();
        self.walk_expression(b.rhs, ctx);
        if matches!(
            b.operator,
            BinaryOperator::And(_)
                | BinaryOperator::LowAnd(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::LowOr(_)
        ) || b.operator.is_null_coalesce()
        {
            self.merge(vec![self.locals.clone(), skipped]);
        }
    }

    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, (): &mut ()) {
        self.unset_targets
            .extend(u.values.iter().rev().map(HasSpan::span));
    }
    fn walk_array_access(&mut self, a: &'ast ArrayAccess<'arena>, ctx: &mut ()) {
        // ArrayDimFetchHandler evaluates the index first. It can replace the receiver's value and
        // source edge; syntax order must not attach the old declaration to that new value.
        self.walk_expression(a.index, ctx);
        self.walk_expression(a.array, ctx);
    }
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, (): &mut ()) {
        if branching(e) {
            self.locals.clear();
            self.control_depth += 1;
        }
        if let Expression::Variable(Variable::Direct(v)) = e
            && let Some(origin) = self.locals.get(v.name)
        {
            self.found.insert(e.span(), origin.clone());
        }
        // References and mutations are not straight-line value copies. Forget, rather than attach stale docs.
        if matches!(e, Expression::UnaryPrefix(p) if p.operator.is_reference() || p.operator.is_increment_or_decrement())
            || matches!(e, Expression::UnaryPostfix(_))
        {
            self.locals.clear();
        }
    }
    fn walk_out_expression(&mut self, e: &'ast Expression<'arena>, (): &mut ()) {
        if branching(e) {
            self.locals.clear();
            self.control_depth -= 1;
        }
        if let Expression::Parenthesized(p) = e {
            self.found
                .insert(e.span(), Origin::Copy(p.expression.span()));
        }
        let elements = match e {
            Expression::Array(a) => Some(&a.elements),
            Expression::LegacyArray(a) => Some(&a.elements),
            _ => None,
        };
        if let Some(elements) = elements
            && elements.len() <= 256
            && let Some(items) = elements
                .iter()
                .map(|element| match element {
                    ArrayElement::KeyValue(kv) => Some((Some(kv.key.span()), kv.value.span())),
                    ArrayElement::Value(value) => Some((None, value.value.span())),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
        {
            self.found.insert(e.span(), Origin::Array(items));
        }
        if let Expression::ArrayAccess(a) = e {
            self.found
                .insert(e.span(), Origin::Offset(a.array.span(), a.index.span()));
        }
        // Keep the operation's source edge, not a cached inferred value. Parent offsets written back
        // by nested unsets retain existing slots or append new ones in this operation's real order.
        if self.unset_targets.last() == Some(&e.span()) {
            self.unset_targets.pop();
            let mut root = super::super::types::flow::unparen(e);
            let mut keys = Vec::new();
            while let Expression::ArrayAccess(a) = root {
                keys.push(a.index.span());
                root = super::super::types::flow::unparen(a.array);
            }
            match root {
                Expression::Variable(Variable::Direct(v)) if !keys.is_empty() => {
                    keys.reverse();
                    if self.locals.contains_key(v.name) {
                        self.locals
                            .insert(v.name.to_vec(), Origin::Unset(root.span(), keys));
                    }
                }
                Expression::Variable(Variable::Direct(v)) => {
                    self.locals.remove(v.name);
                }
                Expression::Variable(_) => self.locals.clear(),
                _ => {}
            }
        }
    }
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, (): &mut ()) {
        let target = match call.function {
            Expression::Identifier(id) => Some(Call {
                name: function_checks::function_name(self.names, self.codebase, id),
                receiver: None,
                class: None,
                args: Vec::new(),
            }),
            _ => None,
        };
        self.call(call.span(), &call.argument_list, target);
    }
    fn walk_out_function_call(&mut self, call: &'ast FunctionCall<'arena>, (): &mut ()) {
        self.calls.pop();
        let f = match call.function {
            Expression::Identifier(id) => self.codebase.get_function(
                function_checks::function_name(self.names, self.codebase, id).as_bytes(),
            ),
            _ => None,
        };
        if f.is_none_or(|f| f.parameters.iter().any(|p| p.flags.is_by_reference())) {
            self.locals.clear();
        }
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, (): &mut ()) {
        let target = match &call.method {
            ClassLikeMemberSelector::Identifier(name) => Some(Call {
                name: String::from_utf8_lossy(name.value).into_owned(),
                receiver: Some(call.object.span()),
                class: None,
                args: Vec::new(),
            }),
            _ => None,
        };
        self.call(call.span(), &call.argument_list, target);
    }
    fn walk_out_method_call(&mut self, call: &'ast MethodCall<'arena>, (): &mut ()) {
        self.calls.pop();
        if let ClassLikeMemberSelector::Identifier(name) = &call.method {
            let call = Arc::new(MethodCallGuard {
                receiver: call.object.span(),
                name: String::from_utf8_lossy(name.value).into_owned(),
                has_arguments: !call.argument_list.arguments.is_empty(),
            });
            for origin in self.locals.values_mut() {
                *origin = Origin::AfterMethodCall(Arc::clone(&call), Arc::new(origin.clone()));
            }
        } else {
            self.locals.clear();
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, (): &mut ()) {
        let class = match call.class {
            Expression::Identifier(id) => Some(mago_word::word(self.names.get(&id.span()))),
            Expression::Self_(_) => Some(mago_word::word(b"self")),
            Expression::Static(_) => Some(mago_word::word(b"static")),
            Expression::Parent(_) => Some(mago_word::word(b"parent")),
            _ => None,
        };
        let target = match (&call.method, class) {
            (ClassLikeMemberSelector::Identifier(name), Some(class)) => Some(Call {
                name: String::from_utf8_lossy(name.value).into_owned(),
                receiver: None,
                class: Some(class),
                args: Vec::new(),
            }),
            _ => None,
        };
        self.call(call.span(), &call.argument_list, target);
    }
    fn walk_out_static_method_call(&mut self, _: &'ast StaticMethodCall<'arena>, (): &mut ()) {
        self.calls.pop();
        self.locals.clear();
    }
    fn walk_out_instantiation(&mut self, _: &'ast Instantiation<'arena>, (): &mut ()) {
        self.locals.clear();
    }

    fn walk_out_assignment(&mut self, a: &'ast Assignment<'arena>, (): &mut ()) {
        if self.control_depth > 0 || !matches!(a.operator, AssignmentOperator::Assign(_)) {
            self.locals.clear();
            return;
        }
        let Expression::Variable(Variable::Direct(v)) = a.lhs else {
            self.locals.clear();
            return;
        };
        let has_doc = self
            .comments
            .doc_before(&self.file.contents, a.span().start.offset as usize)
            .is_some_and(|(_, entry)| !entry.doc.vars.is_empty());
        let origin = if has_doc {
            Origin::Annotation(
                a.span(),
                String::from_utf8_lossy(&v.name[1..]).into_owned(),
                self.current,
            )
        } else {
            Origin::Copy(a.rhs.span())
        };
        self.locals.insert(v.name.to_vec(), origin);
    }
}
