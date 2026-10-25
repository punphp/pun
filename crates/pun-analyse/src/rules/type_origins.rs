//! Source edges for information Mago erases: PHPDoc shape order and explicit mixed. These are not inferred types.
//! Only straight-line local copies, declarations and inline call arguments carry an origin. Replacements,
//! reference escapes and control-flow joins forget it. Offset unsets preserve source metadata, not values;
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

use super::super::{doc, function_checks};

pub(in super::super) type Function = (Word, Word);

pub(in super::super) struct Call {
    pub name: String,
    /// An instance receiver, whose actual inferred type must resolve the method before its docs are used.
    pub receiver: Option<Span>,
    /// A written static class name, or the lexical `self`/`static`/`parent` keyword. Neither field: a function.
    pub class: Option<Word>,
    pub args: Vec<(Option<String>, Span)>,
}

pub(in super::super) struct MethodCallGuard {
    pub receiver: Span,
    pub name: String,
}

#[derive(Clone)]
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
    /// Conversion must resolve the actual receiver and prove its native signature cannot pass references.
    AfterMethodCall(Arc<MethodCallGuard>, Arc<Origin>),
}

pub(in super::super) type Origins = HashMap<Span, Origin>;

fn branching(e: &Expression) -> bool {
    matches!(
        e,
        Expression::Conditional(_)
            | Expression::Match(_)
            | Expression::Call(mago_syntax::cst::Call::NullSafeMethod(_))
            | Expression::Access(Access::NullSafeProperty(_))
    ) || matches!(e, Expression::Binary(b) if b.operator.is_logical() || b.operator.is_null_coalesce())
}

pub(super) fn collect(
    file: &File,
    program: &Program,
    names: &ResolvedNames,
    codebase: &CodebaseMetadata,
) -> Origins {
    let mut walker = Sources {
        file,
        names,
        codebase,
        functions: codebase
            .function_likes
            .iter()
            .filter(|(_, f)| f.span.file_id == file.id)
            .map(|(id, f)| (f.span, *id))
            .collect(),
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
            Some(Origin::Unset(array, keys)) => std::iter::once(*array).chain(keys.iter().copied()).collect(),
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
        let origin = target
            .filter(|_| !arguments.arguments.iter().any(Argument::is_unpacked))
            .map(|mut call| {
                call.args = arguments
                    .arguments
                    .iter()
                    .map(|arg| {
                        let name = match arg {
                            Argument::Named(n) => {
                                Some(String::from_utf8_lossy(n.name.value).into_owned())
                            }
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
    fn walk_in_function(&mut self, f: &'ast mago_syntax::cst::Function<'arena>, _: &mut ()) {
        self.enter(f.span(), &f.parameter_list);
    }
    fn walk_out_function(&mut self, _: &'ast mago_syntax::cst::Function<'arena>, _: &mut ()) {
        self.leave();
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        self.enter(m.span(), &m.parameter_list);
    }
    fn walk_out_method(&mut self, _: &'ast Method<'arena>, _: &mut ()) {
        self.leave();
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        self.closure(f.span(), &f.parameter_list);
    }
    fn walk_out_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        self.leave();
        if f.use_clause
            .as_ref()
            .is_some_and(|u| u.variables.iter().any(|v| v.ampersand.is_some()))
        {
            self.locals.clear();
        }
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.closure(f.span(), &f.parameter_list);
    }
    fn walk_out_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.leave();
    }

    fn walk_in_statement(&mut self, s: &'ast Statement<'arena>, _: &mut ()) {
        if matches!(
            s,
            Statement::If(_)
                | Statement::For(_)
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
    fn walk_out_statement(&mut self, s: &'ast Statement<'arena>, _: &mut ()) {
        if matches!(
            s,
            Statement::If(_)
                | Statement::For(_)
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
    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        self.unset_targets
            .extend(u.values.iter().rev().map(HasSpan::span));
    }
    fn walk_array_access(&mut self, a: &'ast ArrayAccess<'arena>, ctx: &mut ()) {
        // ArrayDimFetchHandler evaluates the index first. It can replace the receiver's value and
        // source edge; syntax order must not attach the old declaration to that new value.
        self.walk_expression(a.index, ctx);
        self.walk_expression(a.array, ctx);
    }
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
        if branching(e) {
            self.locals.clear();
            self.control_depth += 1;
        }
        if self.control_depth == 0
            && let Expression::Variable(Variable::Direct(v)) = e
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
    fn walk_out_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
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
            && let Some(items) = elements.iter().map(|element| match element {
                ArrayElement::KeyValue(kv) => Some((Some(kv.key.span()), kv.value.span())),
                ArrayElement::Value(value) => Some((None, value.value.span())),
                _ => None,
            }).collect::<Option<Vec<_>>>()
        {
            self.found.insert(e.span(), Origin::Array(items));
        }
        if let Expression::ArrayAccess(a) = e {
            self.found.insert(e.span(), Origin::Offset(a.array.span(), a.index.span()));
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
                        self.locals.insert(v.name.to_vec(), Origin::Unset(root.span(), keys));
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
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
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
    fn walk_out_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
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
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
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
    fn walk_out_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.calls.pop();
        if let ClassLikeMemberSelector::Identifier(name) = &call.method {
            let call = Arc::new(MethodCallGuard {
                receiver: call.object.span(),
                name: String::from_utf8_lossy(name.value).into_owned(),
            });
            for origin in self.locals.values_mut() {
                *origin = Origin::AfterMethodCall(Arc::clone(&call), Arc::new(origin.clone()));
            }
        } else {
            self.locals.clear();
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
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
    fn walk_out_static_method_call(&mut self, _: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.calls.pop();
        self.locals.clear();
    }
    fn walk_out_instantiation(&mut self, _: &'ast Instantiation<'arena>, _: &mut ()) {
        self.locals.clear();
    }

    fn walk_out_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if self.control_depth > 0 || !matches!(a.operator, AssignmentOperator::Assign(_)) {
            self.locals.clear();
            return;
        }
        let Expression::Variable(Variable::Direct(v)) = a.lhs else {
            self.locals.clear();
            return;
        };
        let raw = doc::doc_before(&self.file.contents, a.span().start.offset as usize);
        let has_doc = raw.is_some_and(|raw| !doc::Doc::parse(raw).vars.is_empty());
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
