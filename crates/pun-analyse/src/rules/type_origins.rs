//! Source edges for information Mago erases: PHPDoc shape order and explicit mixed. These are not inferred types.
//! Only straight-line local copies, declarations and inline function arguments carry an origin. A mutation,
//! reference escape or control-flow join forgets it; conversion still requires Mago's actual expression type.

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
    pub args: Vec<(Option<String>, Span)>,
}

#[derive(Clone)]
pub(in super::super) enum Origin {
    Parameter(Function, usize),
    Annotation(Span, String, Option<Function>),
    Copy(Span),
    Call(Arc<Call>),
    Callback(Arc<Call>, usize, usize),
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
        let more: Vec<Span> = match found.get(&span) {
            Some(Origin::Copy(s)) => vec![*s],
            Some(Origin::Call(call) | Origin::Callback(call, ..)) => {
                call.args.iter().map(|(_, s)| *s).collect()
            }
            _ => Vec::new(),
        };
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
                | Statement::Unset(_)
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
                | Statement::Unset(_)
                | Statement::Global(_)
                | Statement::Static(_)
        ) {
            self.locals.clear();
            self.control_depth -= 1;
        }
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
    }
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        let mut origin = None;
        if let Expression::Identifier(id) = call.function
            && !call
                .argument_list
                .arguments
                .iter()
                .any(Argument::is_unpacked)
        {
            let name = function_checks::function_name(self.names, self.codebase, id);
            let args = call
                .argument_list
                .arguments
                .iter()
                .map(|a| {
                    let name = match a {
                        Argument::Named(n) => {
                            Some(String::from_utf8_lossy(n.name.value).into_owned())
                        }
                        _ => None,
                    };
                    (name, a.value().span())
                })
                .collect();
            origin = Some(Arc::new(Call { name, args }));
        }
        if let Some(origin) = &origin {
            self.found
                .insert(call.span(), Origin::Call(Arc::clone(origin)));
        }
        self.calls.push(origin);
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
    fn walk_out_method_call(&mut self, _: &'ast MethodCall<'arena>, _: &mut ()) {
        self.locals.clear();
    }
    fn walk_out_static_method_call(&mut self, _: &'ast StaticMethodCall<'arena>, _: &mut ()) {
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
