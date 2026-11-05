//! WrongVariableNameInVarTagRule (level 2): the `@var` tags of a statement's doc comments must name the variables
//! the statement assigns (or ones that exist), and a function-like's or class-like's doc has no `@var`.
//! ponytail: `$scope->hasVariableType()` is a syntactic approximation: a variable is defined once anything before
//! the statement (or anywhere in an enclosing loop) writes it, even on a path that always terminates or after an
//! `unset()`, and an unknown callee (a method on an object Mago has no type for) takes no argument by reference.
//! VarTagTypeRuleHelper (`varTag.nativeType`,
//! `varTag.type`, `phpstanApi.varTagAssumption`) is not ported.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::statements::FrameKind;
use super::{Checker, assign};

pub const IDENTIFIERS: &[&str] = &[
    "varTag.differentVariable",
    "varTag.misplaced",
    "varTag.multipleTags",
    "varTag.noVariable",
    "varTag.variableNotFound",
];

/// Scope::SUPERGLOBAL_VARIABLES
const SUPERGLOBALS: &[&[u8]] = &[
    b"$GLOBALS", b"$_SERVER", b"$_GET", b"$_POST", b"$_FILES", b"$_COOKIE", b"$_SESSION", b"$_REQUEST", b"$_ENV",
];

/// The variables a function-like's body can have: its parameters and `use`s, and what its body writes where.
#[derive(Default)]
pub struct Defined {
    names: Vec<Vec<u8>>,
    writes: Vec<(Vec<u8>, u32)>,
    loops: Vec<(u32, u32)>,
    /// Where `extract()` is first called (`afterExtractCall()`: any variable can exist after it).
    extract: Option<u32>,
}

impl Defined {
    fn has(&self, name: &[u8], at: u32) -> bool {
        let looped = |w: u32| self.loops.iter().any(|&(s, e)| s <= w && w < e && s <= at && at < e);
        self.names.iter().any(|n| n == name)
            || self.extract.is_some_and(|e| e < at)
            || self.writes.iter().any(|(n, w)| n == name && (*w < at || looped(*w)))
    }
}

struct Writes<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    d: Defined,
}

impl Writes<'_, '_, '_> {
    /// A written expression's variables: a variable, the root of an offset write, a destructuring's items.
    fn target(&mut self, e: &Expression, at: u32) {
        match e {
            Expression::Variable(Variable::Direct(v)) => self.d.writes.push((v.name.to_vec(), at)),
            Expression::ArrayAccess(a) => self.target(a.array, at),
            Expression::ArrayAppend(a) => self.target(a.array, at),
            Expression::UnaryPrefix(u) if u.operator.is_reference() => self.target(u.operand, at),
            Expression::List(List { elements, .. }) | Expression::Array(Array { elements, .. }) => {
                for el in elements.iter() {
                    match el {
                        ArrayElement::KeyValue(kv) => self.target(kv.value, at),
                        ArrayElement::Value(v) => self.target(v.value, at),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn arguments(&mut self, args: &ArgumentList, by_ref: crate::bitset::BitSet, at: u32) {
        for (i, a) in args.arguments.iter().enumerate() {
            if by_ref.get(i) {
                self.target(a.value(), at);
            }
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Writes<'_, '_, 'arena> {
    fn walk_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        for u in f.use_clause.iter().flat_map(|u| u.variables.iter()) {
            if u.ampersand.is_some() {
                self.d.writes.push((u.variable.name.to_vec(), f.span().start.offset));
            }
        }
    }
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        self.target(a.lhs, a.span().start.offset);
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        let at = f.span().start.offset;
        if let ForeachTarget::KeyValue(kv) = &f.target {
            self.target(kv.key, at);
        }
        self.target(f.target.value(), at);
        self.d.loops.push((at, f.span().end.offset));
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        self.d.loops.push((f.span().start.offset, f.span().end.offset));
    }
    fn walk_in_while(&mut self, w: &'ast While<'arena>, _: &mut ()) {
        self.d.loops.push((w.span().start.offset, w.span().end.offset));
    }
    fn walk_in_do_while(&mut self, w: &'ast DoWhile<'arena>, _: &mut ()) {
        self.d.loops.push((w.span().start.offset, w.span().end.offset));
    }
    fn walk_in_static(&mut self, s: &'ast Static<'arena>, _: &mut ()) {
        for item in s.items.iter() {
            self.d.writes.push((item.variable().name.to_vec(), s.span().start.offset));
        }
    }
    fn walk_in_global(&mut self, g: &'ast Global<'arena>, _: &mut ()) {
        for v in g.variables.iter() {
            if let Variable::Direct(v) = v {
                self.d.writes.push((v.name.to_vec(), g.span().start.offset));
            }
        }
    }
    fn walk_in_try_catch_clause(&mut self, t: &'ast TryCatchClause<'arena>, _: &mut ()) {
        if let Some(v) = &t.variable {
            self.d.writes.push((v.name.to_vec(), t.span().start.offset));
        }
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if u.operator.is_increment_or_decrement() {
            self.target(u.operand, u.span().start.offset);
        }
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, _: &mut ()) {
        self.target(u.operand, u.span().start.offset);
    }
    fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
        let at = f.span().start.offset;
        if let Expression::Identifier(id) = f.function
            && id.value().rsplit(|b| *b == b'\\').next().is_some_and(|n| n.eq_ignore_ascii_case(b"extract"))
        {
            self.d.extract.get_or_insert(at);
        }
        let by_ref = assign::by_ref_arguments(self.c, f);
        self.arguments(&f.argument_list, by_ref, at);
    }
    fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
        let by_ref = assign::by_ref_call_arguments(self.c, assign::Callee::Method(m), &m.argument_list);
        self.arguments(&m.argument_list, by_ref, m.span().start.offset);
    }
    fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        let by_ref = assign::by_ref_call_arguments(self.c, assign::Callee::StaticMethod(m), &m.argument_list);
        self.arguments(&m.argument_list, by_ref, m.span().start.offset);
    }
}

/// What a function-like's parameters (and `use`s) `names` and its body define (see [`Defined`]).
pub fn defined<'n>(c: &Checker, names: impl IntoIterator<Item = &'n [u8]>, body: &Block) -> Box<Defined> {
    let mut w = Writes { c, d: Defined::default() };
    w.d.names = names.into_iter().map(<[u8]>::to_vec).collect();
    w.walk_block(body, &mut ());
    Box::new(w.d)
}

/// A parameter list's names.
pub fn params<'p>(list: &'p FunctionLikeParameterList) -> impl Iterator<Item = &'p [u8]> {
    list.parameters.iter().map(|p| p.variable.name)
}

/// `!$scope->hasVariableType($name)->no()` before `at`.
fn has(c: &Checker, name: &str, at: u32) -> bool {
    let name = format!("${name}");
    if SUPERGLOBALS.contains(&name.as_bytes()) {
        return true;
    }
    let frame = c.frames.iter().rev().find(|f| f.kind != FrameKind::Param).expect("the file's frame");
    match (&frame.kind, &frame.defined) {
        (FrameKind::Named | FrameKind::Anonymous, Some(d)) => {
            if name == "$this" { c.this_defined() } else { d.has(name.as_bytes(), at) }
        }
        _ => true,
    }
}

/// The `@var` tags of every doc comment before `at` (php-parser's `getComments()`): the named ones in order, and
/// how many unnamed ones.
fn tags(c: &Checker, at: Span) -> (Vec<String>, usize) {
    let contents = &c.file.contents;
    let mut docs = Vec::new();
    let mut offset = at.start.offset;
    while let Some((raw, entry)) = c.doc_before(c.file.id, offset) {
        offset = (raw.as_ptr() as usize - contents.as_ptr() as usize) as u32;
        docs.push(entry);
    }
    let (mut named, mut unnamed) = (Vec::<String>::new(), 0);
    if docs.is_empty() {
        return (named, unnamed);
    }
    let scope = super::var_tags::scope(c, at);
    for doc in docs.into_iter().rev() {
        let vars = super::declared::with_classes(c, || doc.doc.var_types(&scope));
        let count = vars.iter().filter(|(n, _)| n.is_empty()).count();
        unnamed = unnamed.max(count);
        for (n, _) in vars {
            if !n.is_empty() && !named.contains(&n) {
                named.push(n);
            }
        }
    }
    (named, unnamed)
}

fn dollars(names: &[String]) -> String {
    names.iter().map(|n| format!("${n}")).collect::<Vec<_>>().join(", ")
}

/// The names `getAssignedVariables()` finds in an assignment target.
fn assigned(e: &Expression, out: &mut Vec<String>) {
    match e {
        Expression::Variable(Variable::Direct(v)) => out.push(String::from_utf8_lossy(&v.name[1..]).into_owned()),
        Expression::List(List { elements, .. }) | Expression::Array(Array { elements, .. }) => {
            for el in elements.iter() {
                match el {
                    ArrayElement::KeyValue(kv) => assigned(kv.value, out),
                    ArrayElement::Value(v) => assigned(v.value, out),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

type Errors = Vec<(&'static str, String)>;

/// processStmt()
fn other(c: &Checker, at: u32, named: &[String], unnamed: usize, default: bool, errors: &mut Errors) {
    for name in named {
        if !has(c, name, at) {
            errors.push(("varTag.variableNotFound", format!("Variable ${name} in PHPDoc tag @var does not exist.")));
        }
    }
    if unnamed > 0 && (unnamed != 1 || !default) {
        errors.push(("varTag.noVariable", "PHPDoc tag @var does not specify variable name.".into()));
    }
}

/// processAssign()
fn assignment(c: &Checker, at: u32, lhs: &Expression, named: &[String], unnamed: usize, errors: &mut Errors) {
    let mut vars = Vec::new();
    assigned(lhs, &mut vars);
    let total = named.len() + unnamed;
    for i in 0..unnamed {
        if total != 1 {
            if i == 0 {
                errors.push((
                    "varTag.multipleTags",
                    "Multiple PHPDoc @var tags above single variable assignment are not supported.".into(),
                ));
            }
        } else if vars.len() != 1 {
            errors.push(("varTag.noVariable", "PHPDoc tag @var above assignment does not specify variable name.".into()));
        }
    }
    for name in named {
        if has(c, name, at) || vars.contains(name) {
            continue;
        }
        if vars.len() == 1 && total == 1 {
            errors.push((
                "varTag.differentVariable",
                format!("Variable ${name} in PHPDoc tag @var does not match assigned variable ${}.", vars[0]),
            ));
        } else {
            errors.push(("varTag.variableNotFound", format!("Variable ${name} in PHPDoc tag @var does not exist.")));
        }
    }
}

/// processForeach(), processStatic() and processGlobal(): the tags must name one of `vars`.
fn listed(vars: &[String], named: &[String], unnamed: usize, above: &str, what: &str, errors: &mut Errors) {
    for _ in 0..unnamed {
        if vars.len() != 1 {
            errors.push(("varTag.noVariable", format!("PHPDoc tag @var above {above} does not specify variable name.")));
        }
    }
    for name in named.iter().filter(|n| !vars.contains(n)) {
        errors.push((
            "varTag.differentVariable",
            format!("Variable ${name} in PHPDoc tag @var does not match any {what}: {}", dollars(vars)),
        ));
    }
}

fn direct(e: &Expression) -> Option<String> {
    match e {
        Expression::Variable(Variable::Direct(v)) => Some(String::from_utf8_lossy(&v.name[1..]).into_owned()),
        _ => None,
    }
}

/// A php-parser statement (none of a property, constant, class-like, function or method).
pub fn statement(c: &mut Checker, s: &Statement) {
    if c.level < 2 {
        return;
    }
    let (named, unnamed) = tags(c, s.span());
    if named.is_empty() && unnamed == 0 {
        return;
    }
    let at = s.span().start.offset;
    let mut errors = Vec::new();
    match s {
        Statement::Foreach(f) => {
            let mut vars: Vec<String> = direct(f.expression).into_iter().collect();
            // (a list key is no variable name)
            if let ForeachTarget::KeyValue(kv) = &f.target {
                vars.extend(direct(kv.key));
            }
            assigned(f.target.value(), &mut vars);
            for _ in 0..unnamed {
                if vars.len() != 1 {
                    errors.push((
                        "varTag.noVariable",
                        "PHPDoc tag @var above foreach loop does not specify variable name.".to_string(),
                    ));
                }
            }
            for name in named.iter().filter(|n| !vars.contains(n)) {
                errors.push((
                    "varTag.differentVariable",
                    format!(
                        "Variable ${name} in PHPDoc tag @var does not match any variable in the foreach loop: {}",
                        dollars(&vars)
                    ),
                ));
            }
        }
        Statement::Static(st) => {
            let vars: Vec<String> = st
                .items
                .iter()
                .map(|i| String::from_utf8_lossy(&i.variable().name[1..]).into_owned())
                .collect();
            listed(&vars, &named, unnamed, "multiple static variables", "static variable", &mut errors);
        }
        Statement::Global(g) => {
            let mut vars: Vec<String> = Vec::new();
            for v in g.variables.iter() {
                if let Variable::Direct(v) = v {
                    let name = String::from_utf8_lossy(&v.name[1..]).into_owned();
                    if !vars.contains(&name) {
                        vars.push(name);
                    }
                }
            }
            listed(&vars, &named, unnamed, "multiple global variables", "global variable", &mut errors);
        }
        Statement::Expression(e) => match e.expression {
            Expression::Assignment(a) => assignment(c, at, a.lhs, &named, unnamed, &mut errors),
            Expression::Throw(_) => other(c, at, &named, unnamed, true, &mut errors),
            _ => other(c, at, &named, unnamed, false, &mut errors),
        },
        Statement::Return(r) => other(c, at, &named, unnamed, r.value.is_some(), &mut errors),
        _ => other(c, at, &named, unnamed, false, &mut errors),
    }
    for (identifier, message) in errors {
        c.report(s.span(), identifier, message, None);
    }
}

/// A php-parser statement that is no Mago statement, or a `Nop` (see phpdoc_tags.rs): processStmt().
pub fn clause(c: &mut Checker, at: Span) {
    if c.level < 2 {
        return;
    }
    let (named, unnamed) = tags(c, at);
    let mut errors = Vec::new();
    other(c, at.start.offset, &named, unnamed, false, &mut errors);
    for (identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// InClassNode, InClassMethodNode and InFunctionNode: a `@var` there has no effect.
pub fn misplaced(c: &mut Checker, at: Span, description: &str) {
    if c.level < 2 {
        return;
    }
    let (named, unnamed) = tags(c, at);
    if !named.is_empty() || unnamed > 0 {
        c.report(at, "varTag.misplaced", format!("PHPDoc tag @var above {description} has no effect."), None);
    }
}
