//! UnusedPrivateConstantRule, UnusedPrivateMethodRule and UnusedPrivatePropertyRule (ClassConstantsNode,
//! ClassMethodsNode, ClassPropertiesNode), over what ClassStatementsGatherer collects in the class's scope: its own
//! members and the methods of the traits it uses, without nested anonymous classes.
//! ponytail: `$classType->isSuperTypeOf($calledOnType)` is a class-relation check on Mago's types of the analysed
//! file's expressions (an expression in a trait's file is a maybe); no AlwaysUsed*/ReadWriteProperties extensions,
//! `propertyAlways*Tags` or `checkUninitializedProperties`; by-reference arguments write a property only through
//! functions of the class's own file.

use std::collections::HashMap;
use std::sync::Arc;

use mago_allocator::LocalArena;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_database::file::File;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::{Checker, assign};
use crate::codebase::parse;

/// `$classType->isSuperTypeOf($calledOnType)`, without its yes/maybe distinction.
fn related(c: &Checker, k: &ClassLikeMetadata, name: &[u8]) -> bool {
    let Some(other) = c.codebase.get_class_like(name) else {
        return true;
    };
    if c.codebase
        .is_instance_of(other.name.as_bytes(), k.name.as_bytes())
        || c.codebase
            .is_instance_of(k.name.as_bytes(), other.name.as_bytes())
    {
        return true;
    }
    // a subclass of k may implement the interface
    other.kind == SymbolKind::Interface && k.kind == SymbolKind::Class && !k.flags.is_final()
}

/// A member fetch's `$calledOnType` relation to the class.
enum On<'e, 'arena> {
    /// `$this`, `self`, `static` or the class's own name: the class itself.
    Class,
    Name(Vec<u8>),
    Expr(&'e Expression<'arena>),
}

struct Prop {
    line_at: Span,
    is_static: bool,
    promoted: bool,
    read: bool,
    written: bool,
    true_read: bool,
    only_readable: bool,
    only_writable: bool,
}

struct Gatherer<'g, 'a, 'arena> {
    c: &'g Checker<'a, 'arena>,
    k: &'g ClassLikeMetadata,
    names: &'g ResolvedNames<'g>,
    /// The class's own file (Mago's types are the checker's).
    own: bool,
    methods: HashMap<String, (String, Span, bool)>,
    constants: HashMap<String, i64>,
    properties: HashMap<String, Prop>,
    /// The method (or `$p::get` hook) the walk is in.
    in_method: Option<String>,
    /// The hooked property and whether the hook is `set`.
    in_hook: Option<(String, bool)>,
    closures: usize,
    /// The constructor's promoted parameters, when walking `__construct`.
    promoted: Vec<String>,
    /// Inside an argument of `new <this class>(...)` for this constructor parameter (isPropertySelfWrite).
    self_write: Option<String>,
    /// Array callables `[$this, 'm']`: (method, in method).
    arrays: Vec<(String, Option<String>)>,
    /// The parameters of the constructor the class declares.
    constructor: Vec<String>,
}

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

fn is_this(e: &Expression) -> bool {
    matches!(e, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

fn unparen<'e, 'arena>(mut e: &'e Expression<'arena>) -> &'e Expression<'arena> {
    while let Expression::Parenthesized(p) = e {
        e = p.expression;
    }
    e
}

impl<'g, 'a, 'arena> Gatherer<'g, 'a, 'arena> {
    fn on<'e, 'x>(&self, e: &'e Expression<'x>) -> On<'e, 'x> {
        match unparen(e) {
            e if is_this(e) => On::Class,
            Expression::Self_(_) | Expression::Static(_) => On::Class,
            Expression::Parent(_) => On::Name(
                self.k
                    .direct_parent_class
                    .as_ref()
                    .map_or(Vec::new(), |p| p.as_bytes().to_vec()),
            ),
            Expression::Identifier(id) => {
                let name = self.names.get(&id.span());
                if name.eq_ignore_ascii_case(self.k.name.as_bytes()) {
                    On::Class
                } else {
                    On::Name(name.to_vec())
                }
            }
            e => On::Expr(e),
        }
    }

    /// Not `$classType->isSuperTypeOf($on)->no()`.
    fn maybe(&self, on: &On) -> bool {
        match on {
            On::Class => true,
            On::Name(name) => related(self.c, self.k, name),
            On::Expr(e) => {
                let Some(t) = self.own.then(|| self.c.types.get(&e.span())).flatten() else {
                    return true;
                };
                t.types.iter().any(|a| match a {
                    TAtomic::Object(TObject::Named(n)) => {
                        related(self.c, self.k, n.name.as_bytes())
                    }
                    TAtomic::Object(TObject::Enum(n)) => related(self.c, self.k, n.name.as_bytes()),
                    TAtomic::Null
                    | TAtomic::Void
                    | TAtomic::Never
                    | TAtomic::Array(_)
                    | TAtomic::Resource(_) => false,
                    TAtomic::Scalar(s) => !(s.is_int() || s.is_float() || s.is_bool()),
                    _ => true,
                })
            }
        }
    }

    /// The constant strings of `span`'s type, when it has only those.
    fn strings(&self, span: Span) -> Option<Vec<String>> {
        let t = self.own.then(|| self.c.types.get(&span)).flatten()?;
        t.types
            .iter()
            .map(|a| a.get_literal_string_value().map(text))
            .collect()
    }

    /// A member name: written, or the constant strings of its expression (`None`: any name).
    fn names(&self, s: &ClassLikeMemberSelector) -> Option<Vec<String>> {
        match s {
            ClassLikeMemberSelector::Identifier(id) => Some(vec![text(id.value)]),
            ClassLikeMemberSelector::Expression(e) => match unparen(e.expression) {
                Expression::Literal(Literal::String(s)) => {
                    Some(s.value.map(text).into_iter().collect())
                }
                e => self.strings(e.span()),
            },
            s => self.strings(s.span()),
        }
    }

    fn method_call(&mut self, on: On, method: &ClassLikeMemberSelector) {
        let Some(names) = self.names(method) else {
            // a dynamic name may be any of them
            self.methods.clear();
            return;
        };
        if self.in_method.is_none() || !self.maybe(&on) {
            return;
        }
        for name in names {
            // a recursive call does not use it (the method is found, declared in the class)
            if matches!(on, On::Class) && self.in_method.as_deref() == Some(&*name) {
                continue;
            }
            self.methods.remove(&name.to_ascii_lowercase());
        }
    }

    fn constant(&mut self, on: On, constant: &ClassLikeConstantSelector) {
        let maybe = self.maybe(&on);
        match constant {
            ClassLikeConstantSelector::Identifier(id) if maybe => {
                self.constants.remove(&text(id.value));
            }
            ClassLikeConstantSelector::Expression(_) if maybe => self.constants.clear(),
            _ => {}
        }
    }

    /// A PropertyRead (`write` false) or PropertyWrite of `$on->name` / `On::$name`.
    fn property(&mut self, on: On, names: Option<Vec<String>>, write: bool) {
        let Some(names) = names else {
            self.properties.clear();
            return;
        };
        for name in names {
            self.property_named(&on, name, write);
        }
    }

    fn property_named(&mut self, on: &On, name: String, write: bool) {
        if matches!(on, On::Class)
            && let Some((hooked, set)) = &self.in_hook
            && *hooked == name
            && (*set || !write)
        {
            return;
        }
        if !self.properties.contains_key(&name) || !self.maybe(on) {
            return;
        }
        let self_write = self.self_write.as_deref() == Some(&*name);
        let p = self.properties.get_mut(&name).expect("checked");
        if write {
            p.written = true;
        } else {
            p.read = true;
            if !(self_write && p.promoted) {
                p.true_read = true;
            }
        }
    }

    fn fetch(&mut self, e: &Expression, write: bool) {
        match unparen(e) {
            Expression::Access(Access::Property(p)) => {
                let names = self.names(&p.property);
                let on = self.on(p.object);
                self.property(on, names, write);
            }
            Expression::Access(Access::NullSafeProperty(p)) => {
                let names = self.names(&p.property);
                let on = self.on(p.object);
                self.property(on, names, write);
            }
            Expression::Access(Access::StaticProperty(p)) => {
                let name = match &p.property {
                    Variable::Direct(v) => Some(vec![text(&v.name[1..])]),
                    v => dynamic_name(v).and_then(|name| self.strings(name)),
                };
                let on = self.on(p.class);
                self.property(on, name, write);
            }
            _ => {}
        }
    }

    /// The property an assignment target writes, through its dimensions: (fetch, via offset).
    fn target<'e, 'x>(e: &'e Expression<'x>) -> (&'e Expression<'x>, bool) {
        let mut e = unparen(e);
        let mut offset = false;
        while let Expression::ArrayAccess(ArrayAccess { array, .. })
        | Expression::ArrayAppend(ArrayAppend { array, .. }) = e
        {
            e = unparen(array);
            offset = true;
        }
        (e, offset)
    }

    /// processAssignVar(): the writes of an assignment target, then its own sub-expressions.
    fn assign_var(&mut self, var: &Expression) {
        let var = unparen(var);
        if let Expression::Array(_) | Expression::LegacyArray(_) | Expression::List(_) = var {
            for child in Node::Expression(var).children() {
                self.assign_child(child);
            }
            return;
        }
        let (fetch, offset) = Self::target(var);
        if matches!(
            fetch,
            Expression::Access(Access::Property(_) | Access::StaticProperty(_))
        ) {
            // an offset write reads the property when it may hold an object (ArrayAccess)
            if offset {
                self.fetch(fetch, false);
            }
            self.fetch(fetch, true);
            for child in Node::Expression(fetch).children() {
                self.node(child);
            }
            self.dims(var);
            return;
        }
        for child in Node::Expression(var).children() {
            self.node(child);
        }
    }

    /// The dimensions of `var` down to its fetch.
    fn dims(&mut self, var: &Expression) {
        let mut e = unparen(var);
        while let Expression::ArrayAccess(ArrayAccess { array, .. })
        | Expression::ArrayAppend(ArrayAppend { array, .. }) = e
        {
            if let Expression::ArrayAccess(a) = e {
                self.node(Node::Expression(a.index));
            }
            e = unparen(array);
        }
    }

    fn assign_child(&mut self, n: Node) {
        match n {
            Node::Array(_)
            | Node::LegacyArray(_)
            | Node::List(_)
            | Node::ArrayElement(_)
            | Node::ValueArrayElement(_) => {
                for child in n.children() {
                    self.assign_child(child);
                }
            }
            Node::KeyValueArrayElement(e) => {
                self.node(Node::Expression(e.key));
                self.assign_var(e.value);
            }
            Node::Expression(e) => self.assign_var(e),
            n => self.node(n),
        }
    }

    fn new_args(&mut self, new: &Instantiation) {
        let to_self = matches!(self.on(new.class), On::Class);
        let Some(args) = &new.argument_list else {
            return;
        };
        let params = if to_self {
            self.constructor.clone()
        } else {
            Vec::new()
        };
        for (i, arg) in args.arguments.iter().enumerate() {
            let param = match arg {
                Argument::Positional(_) => params.get(i).cloned(),
                Argument::Named(n) => params
                    .iter()
                    .find(|p| p.as_bytes() == n.name.value)
                    .cloned(),
            };
            let saved = std::mem::replace(&mut self.self_write, param);
            self.node(Node::Expression(arg.value()));
            self.self_write = saved;
        }
    }

    fn node(&mut self, n: Node) {
        match n {
            // other scopes
            Node::Class(_)
            | Node::Interface(_)
            | Node::Trait(_)
            | Node::Enum(_)
            | Node::Function(_) => {}
            Node::AnonymousClass(a) => {
                if let Some(args) = &a.argument_list {
                    self.node(Node::PartialArgumentList(args));
                }
            }
            Node::Method(m) => {
                let name = text(m.name.value);
                let constructor = name.eq_ignore_ascii_case("__construct");
                let promoted = if constructor {
                    m.parameter_list
                        .parameters
                        .iter()
                        .filter(|p| p.is_promoted_property())
                        .map(|p| text(p.variable.name))
                        .collect()
                } else {
                    Vec::new()
                };
                let saved = (
                    self.in_method.replace(name),
                    std::mem::replace(&mut self.promoted, promoted),
                );
                self.children(n);
                (self.in_method, self.promoted) = saved;
            }
            Node::PropertyHook(h) => {
                let property = self
                    .in_hook
                    .as_ref()
                    .map(|(p, _)| p.clone())
                    .unwrap_or_default();
                let set = h.name.value.eq_ignore_ascii_case(b"set");
                let saved = (
                    self.in_method
                        .replace(format!("${property}::{}", text(h.name.value))),
                    self.in_hook.replace((property, set)),
                );
                self.children(n);
                (self.in_method, self.in_hook) = saved;
            }
            Node::HookedProperty(p) => {
                let name = text(&p.item.variable().name[1..]);
                let saved = self.in_hook.replace((name, false));
                self.children(n);
                self.in_hook = saved;
            }
            // the parameter itself is no read
            Node::FunctionLikeParameter(_) => {
                for child in n.children() {
                    if !matches!(child, Node::DirectVariable(_)) {
                        self.node(child);
                    }
                }
            }
            Node::Closure(_) | Node::ArrowFunction(_) => {
                self.closures += 1;
                self.children(n);
                self.closures -= 1;
            }
            Node::MethodCall(m) => {
                let on = self.on(m.object);
                self.method_call(on, &m.method);
                self.call_children(n);
                self.by_ref(assign::Callee::Method(m), &m.argument_list);
            }
            Node::NullSafeMethodCall(m) => {
                let on = self.on(m.object);
                self.method_call(on, &m.method);
                self.call_children(n);
            }
            Node::StaticMethodCall(m) => {
                let on = self.on(m.class);
                self.method_call(on, &m.method);
                self.call_children(n);
                self.by_ref(assign::Callee::StaticMethod(m), &m.argument_list);
            }
            Node::MethodPartialApplication(m) => {
                let on = self.on(m.object);
                self.method_call(on, &m.method);
                self.call_children(n);
            }
            Node::StaticMethodPartialApplication(m) => {
                let on = self.on(m.class);
                self.method_call(on, &m.method);
                self.call_children(n);
            }
            Node::ClassConstantAccess(a) => {
                let on = self.on(a.class);
                self.constant(on, &a.constant);
                self.children(n);
            }
            Node::Array(a) => {
                self.array_callable(a.elements.as_slice());
                self.children(n);
            }
            Node::LegacyArray(a) => {
                self.array_callable(a.elements.as_slice());
                self.children(n);
            }
            Node::Instantiation(new) => {
                self.node(Node::Expression(new.class));
                self.new_args(new);
            }
            Node::FunctionCall(f) => self.function_call(f),
            Node::Assignment(a) => {
                let by_ref = match (&a.operator, unparen(a.rhs)) {
                    (AssignmentOperator::Assign(_), Expression::UnaryPrefix(u))
                        if u.operator.is_reference() =>
                    {
                        Some(u.operand)
                    }
                    _ => None,
                };
                match &a.operator {
                    AssignmentOperator::Assign(_) => {}
                    // the compound operators read the target first
                    _ => self.fetch(Self::target(a.lhs).0, false),
                }
                self.assign_var(a.lhs);
                match by_ref {
                    // `$a = &$this->p` reads and writes it
                    Some(operand)
                        if matches!(
                            unparen(operand),
                            Expression::Access(Access::Property(_) | Access::StaticProperty(_))
                        ) =>
                    {
                        self.fetch(operand, false);
                        self.fetch(operand, true);
                        for child in Node::Expression(unparen(operand)).children() {
                            self.node(child);
                        }
                    }
                    _ => self.node(Node::Expression(a.rhs)),
                }
            }
            Node::UnaryPrefix(u) if u.operator.is_increment_or_decrement() => {
                self.fetch(Self::target(u.operand).0, false);
                self.assign_var(u.operand);
            }
            Node::UnaryPostfix(u) => {
                self.fetch(Self::target(u.operand).0, false);
                self.assign_var(u.operand);
            }
            Node::ForeachKeyValueTarget(t) => {
                self.assign_var(t.key);
                self.assign_var(t.value);
            }
            Node::ForeachValueTarget(t) => self.assign_var(t.value),
            Node::Expression(e)
                if matches!(
                    e,
                    Expression::Access(
                        Access::Property(_)
                            | Access::NullSafeProperty(_)
                            | Access::StaticProperty(_)
                    ) | Expression::ArrayAccess(_)
                ) =>
            {
                let (fetch, _) = Self::target(e);
                self.fetch(fetch, false);
                self.children(n);
            }
            Node::DirectVariable(v) => {
                let name = text(v.name);
                if self.closures == 0 && self.promoted.contains(&name) {
                    self.property(On::Class, Some(vec![text(&v.name[1..])]), false);
                }
            }
            n => self.children(n),
        }
    }

    fn children(&mut self, n: Node) {
        for child in n.children() {
            self.node(child);
        }
    }

    /// A call's children, outside any constructor argument (the call stack's last call is this one).
    fn call_children(&mut self, n: Node) {
        let saved = self.self_write.take();
        self.children(n);
        self.self_write = saved;
    }

    fn function_call(&mut self, f: &FunctionCall) {
        let name = match f.function {
            Expression::Identifier(id) => Some(self.names.get(&id.span())),
            _ => None,
        };
        // ClassStatementsGatherer::tryToApplyPropertyReads()
        let enumerating = name.is_some_and(|n| {
            let n = n.strip_prefix(b"\\").unwrap_or(n);
            n.eq_ignore_ascii_case(b"get_object_vars") || n.eq_ignore_ascii_case(b"array_walk")
        });
        if enumerating
            && f.argument_list
                .arguments
                .first()
                .is_some_and(|a| is_this(a.value()))
        {
            let names: Vec<String> = self
                .properties
                .iter()
                .filter(|(_, p)| !p.is_static)
                .map(|(n, _)| n.clone())
                .collect();
            self.property(On::Class, Some(names), false);
        }
        self.call_children(Node::FunctionCall(f));
        if self.own {
            // processVirtualAssign() of the arguments passed by reference
            let by_ref: Vec<bool> = assign::by_ref_arguments(self.c, f);
            let values: Vec<&Expression> = f
                .argument_list
                .arguments
                .iter()
                .zip(by_ref)
                .filter(|(_, r)| *r)
                .map(|(a, _)| a.value())
                .chain(assign::virtual_assigned(self.c, f))
                .collect();
            for value in values {
                let (fetch, _) = Self::target(value);
                self.fetch(fetch, true);
            }
        }
    }

    /// processVirtualAssign() of the arguments a method or static call takes by reference.
    fn by_ref(&mut self, call: assign::Callee, args: &ArgumentList) {
        if !self.own {
            return;
        }
        let by_ref = assign::by_ref_call_arguments(self.c, call, args);
        for (arg, by_ref) in args.arguments.iter().zip(by_ref) {
            if by_ref {
                self.fetch(Self::target(arg.value()).0, true);
            }
        }
    }

    /// A two-item array: `[$this, 'm']`, `[self::class, 'm']` (findTypeAndMethodNames()).
    fn array_callable(&mut self, elements: &[ArrayElement]) {
        let [ArrayElement::Value(first), ArrayElement::Value(second)] = elements else {
            return;
        };
        let methods = match unparen(second.value) {
            Expression::Literal(Literal::String(method)) => {
                method.value.map(text).into_iter().collect()
            }
            e => self.strings(e.span()).unwrap_or_default(),
        };
        let first = unparen(first.value);
        let on_class = match first {
            Expression::Access(Access::ClassConstant(a)) if matches!(&a.constant, ClassLikeConstantSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"class")) =>
            {
                matches!(self.on(a.class), On::Class)
            }
            e => is_this(e),
        };
        if on_class {
            for method in methods {
                self.arrays.push((method, self.in_method.clone()));
            }
        }
    }
}

/// The traits `k` uses, with their own traits.
fn traits<'c>(c: &'c Checker, k: &ClassLikeMetadata) -> Vec<&'c ClassLikeMetadata> {
    let mut out: Vec<&ClassLikeMetadata> = Vec::new();
    let mut pending = vec![k];
    while let Some(t) = pending.pop() {
        for name in t.used_traits.iter() {
            if let Some(t) = c.codebase.get_class_like(name.as_bytes())
                && t.kind == SymbolKind::Trait
                && !out.iter().any(|x| x.name == t.name)
            {
                out.push(t);
                pending.push(t);
            }
        }
    }
    out
}

/// The trait declarations at `spans` in `program`.
fn traits_in<'p>(statements: &'p [Statement<'p>], spans: &[Span], out: &mut Vec<&'p Trait<'p>>) {
    for s in statements {
        match s {
            Statement::Trait(t) if spans.contains(&t.span()) => out.push(t),
            Statement::Namespace(n) => traits_in(n.statements().as_slice(), spans, out),
            _ => {}
        }
    }
}

fn is_private(modifiers: &Sequence<Modifier>) -> bool {
    modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
}

/// The rules on class or enum `k` declared with `members`.
pub(in super::super) fn class(c: &mut Checker, k: &ClassLikeMetadata, members: &[ClassLikeMember]) {
    if c.level < 4 || !matches!(k.kind, SymbolKind::Class | SymbolKind::Enum) {
        return;
    }
    let display = c
        .anonymous
        .get(k.name.as_bytes())
        .cloned()
        .unwrap_or_else(|| c.display(k));
    let mut methods = HashMap::new();
    let mut constants = HashMap::new();
    let mut properties = HashMap::new();
    for member in members {
        match member {
            ClassLikeMember::Constant(k) if is_private(&k.modifiers) => {
                for item in k.items.iter() {
                    constants.insert(text(item.name.value), c.line_of(item.span()));
                }
            }
            ClassLikeMember::Method(m) if is_private(&m.modifiers) => {
                let name = text(m.name.value);
                let lower = name.to_ascii_lowercase();
                if lower == "__construct" || lower == "__clone" {
                    continue;
                }
                let is_static = m.modifiers.iter().any(|m| matches!(m, Modifier::Static(_)));
                methods.insert(lower, (name, m.span(), is_static));
            }
            ClassLikeMember::Property(p)
                if is_private(p.modifiers()) && k.kind == SymbolKind::Class =>
            {
                let is_static = p
                    .modifiers()
                    .iter()
                    .any(|m| matches!(m, Modifier::Static(_)));
                let items: Vec<&PropertyItem> = match p {
                    Property::Plain(plain) => plain.items.iter().collect(),
                    Property::Hooked(h) => vec![&h.item],
                };
                let hooks: Vec<&[u8]> = match p {
                    Property::Hooked(h) => h.hook_list.hooks.iter().map(|h| h.name.value).collect(),
                    Property::Plain(_) => Vec::new(),
                };
                for item in items {
                    let name = &item.variable().name[1..];
                    let (readable, writable) = rw(c, k, name, &hooks);
                    properties.insert(
                        text(name),
                        Prop {
                            line_at: p.span(),
                            is_static,
                            promoted: false,
                            read: !readable,
                            written: !writable || matches!(item, PropertyItem::Concrete(_)),
                            true_read: !readable,
                            only_readable: readable && !writable,
                            only_writable: writable && !readable,
                        },
                    );
                }
            }
            _ => {}
        }
    }
    // promoted constructor parameters, after the properties
    for member in members {
        let ClassLikeMember::Method(m) = member else {
            continue;
        };
        if !m.name.value.eq_ignore_ascii_case(b"__construct") || k.kind != SymbolKind::Class {
            continue;
        }
        for p in m.parameter_list.parameters.iter() {
            if !is_private(&p.modifiers) {
                continue;
            }
            let hooks: Vec<&[u8]> = p
                .hooks
                .as_ref()
                .map(|h| h.hooks.iter().map(|h| h.name.value).collect())
                .unwrap_or_default();
            let name = &p.variable.name[1..];
            let (readable, writable) = rw(c, k, name, &hooks);
            properties.insert(
                text(name),
                Prop {
                    line_at: p.span(),
                    is_static: false,
                    promoted: true,
                    read: !readable,
                    // the promotion writes it
                    written: true,
                    true_read: !readable,
                    only_readable: readable && !writable,
                    only_writable: writable && !readable,
                },
            );
        }
    }
    // the traits' files, parsed: their constants are the class's
    let traits = traits(c, k);
    let mut files: Vec<Arc<File>> = Vec::new();
    for t in &traits {
        if let Some(file) = c.files.get(&t.span.file_id).and_then(|p| c.sources.get(p))
            && !files.iter().any(|f| f.id == file.id)
        {
            files.push(file.clone());
        }
    }
    let arenas: Vec<LocalArena> = files.iter().map(|_| LocalArena::new()).collect();
    let parsed: Vec<_> = files
        .iter()
        .zip(&arenas)
        .map(|(f, a)| parse(a, f))
        .collect();
    let mut found: Vec<(usize, &Trait)> = Vec::new();
    for (i, (file, (program, _))) in files.iter().zip(&parsed).enumerate() {
        let spans: Vec<Span> = traits
            .iter()
            .filter(|t| t.span.file_id == file.id)
            .map(|t| t.span)
            .collect();
        let mut in_file = Vec::new();
        traits_in(program.statements.as_slice(), &spans, &mut in_file);
        found.extend(in_file.into_iter().map(|t| (i, t)));
    }
    for (i, t) in &found {
        for member in t.members.iter() {
            if let ClassLikeMember::Constant(k) = member
                && is_private(&k.modifiers)
            {
                for item in k.items.iter() {
                    let line = i64::from(files[*i].line_number(item.span().start.offset)) + 1;
                    constants.entry(text(item.name.value)).or_insert(line);
                }
            }
        }
    }
    if methods.is_empty() && constants.is_empty() && properties.is_empty() {
        return;
    }
    let constructor: Vec<String> = members
        .iter()
        .find_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                Some(m)
            }
            _ => None,
        })
        .map(|m| {
            m.parameter_list
                .parameters
                .iter()
                .map(|p| text(&p.variable.name[1..]))
                .collect()
        })
        .unwrap_or_default();
    let mut g = Gatherer {
        c,
        k,
        names: c.names,
        own: true,
        methods,
        constants,
        properties,
        in_method: None,
        in_hook: None,
        closures: 0,
        promoted: Vec::new(),
        self_write: None,
        arrays: Vec::new(),
        constructor: constructor.clone(),
    };
    for member in members {
        g.node(Node::ClassLikeMember(member));
    }
    let mut state = (g.methods, g.constants, g.properties, g.arrays);
    // the traits' members, in the class's scope
    for (i, (_, names)) in parsed.iter().enumerate() {
        let mut g = Gatherer {
            c,
            k,
            names,
            own: false,
            methods: state.0,
            constants: state.1,
            properties: state.2,
            in_method: None,
            in_hook: None,
            closures: 0,
            promoted: Vec::new(),
            self_write: None,
            arrays: state.3,
            constructor: constructor.clone(),
        };
        for (_, t) in found.iter().filter(|(j, _)| *j == i) {
            for member in t.members.iter() {
                g.node(Node::ClassLikeMember(member));
            }
        }
        state = (g.methods, g.constants, g.properties, g.arrays);
    }
    let (mut methods, constants, properties, arrays) = state;
    for (method, in_method) in arrays {
        if in_method.is_some() && in_method.as_deref() != Some(method.as_str()) {
            methods.remove(&method.to_ascii_lowercase());
        }
    }
    report(c, &display, methods, constants, properties);
}

/// isReadable() and isWritable() of a native property: a virtual one only through its hooks.
fn rw(c: &Checker, k: &ClassLikeMetadata, name: &[u8], hooks: &[&[u8]]) -> (bool, bool) {
    let mut variable = b"$".to_vec();
    variable.extend_from_slice(name);
    let virtual_ = c
        .codebase
        .get_declaring_property(k.name.as_bytes(), &variable)
        .is_some_and(|p| c.property_virtual(k, p, &variable));
    if !virtual_ {
        return (true, true);
    }
    (
        hooks.iter().any(|h| h.eq_ignore_ascii_case(b"get")),
        hooks.iter().any(|h| h.eq_ignore_ascii_case(b"set")),
    )
}

fn report(
    c: &mut Checker,
    display: &str,
    methods: HashMap<String, (String, Span, bool)>,
    constants: HashMap<String, i64>,
    properties: HashMap<String, Prop>,
) {
    for (name, line) in constants {
        c.report_at_line(
            line,
            "classConstant.unused",
            format!("Constant {display}::{name} is unused."),
            Some("See: https://phpstan.org/developing-extensions/always-used-class-constants"),
        );
    }
    for (_, (name, at, is_static)) in methods {
        let kind = if is_static { "Static method" } else { "Method" };
        c.report(
            at,
            "method.unused",
            format!("{kind} {display}::{name}() is unused."),
            None,
        );
    }
    let tip = Some("See: https://phpstan.org/developing-extensions/always-read-written-properties");
    for (name, p) in properties {
        let read = p.read && (p.true_read || !p.promoted);
        let subject = if p.is_static {
            format!("Static property {display}::${name}")
        } else {
            format!("Property {display}::${name}")
        };
        let lower = format!("{}{}", subject[..1].to_ascii_lowercase(), &subject[1..]);
        if !read {
            if !p.written {
                c.report(
                    p.line_at,
                    "property.unused",
                    format!("{subject} is unused."),
                    tip,
                );
            } else if p.only_readable {
                c.report(
                    p.line_at,
                    "property.neverRead",
                    format!("Readable {lower} is never read."),
                    None,
                );
            } else {
                c.report(
                    p.line_at,
                    "property.onlyWritten",
                    format!("{subject} is never read, only written."),
                    tip,
                );
            }
        } else if !p.written {
            if p.only_writable {
                c.report(
                    p.line_at,
                    "property.neverWritten",
                    format!("Writable {lower} is never written."),
                    None,
                );
            } else {
                c.report(
                    p.line_at,
                    "property.onlyRead",
                    format!("{subject} is never written, only read."),
                    tip,
                );
            }
        }
    }
}

/// The name expression of `Foo::$$name` or `Foo::${expr}`.
fn dynamic_name(v: &Variable) -> Option<Span> {
    match v {
        Variable::Direct(_) => None,
        Variable::Indirect(v) => Some(v.expression.span()),
        Variable::Nested(v) => Some(v.variable.span()),
    }
}

/// The expressions whose type decides a member fetch in a class body.
pub(super) fn typed(program: &Program) -> Vec<Span> {
    struct Wanted {
        depth: usize,
        spans: Vec<Span>,
    }
    impl Wanted {
        fn selector(&mut self, s: &ClassLikeMemberSelector) {
            match s {
                ClassLikeMemberSelector::Variable(v) if self.depth > 0 => self.spans.push(v.span()),
                ClassLikeMemberSelector::Expression(e) if self.depth > 0 => {
                    self.spans.push(e.expression.span())
                }
                _ => {}
            }
        }
        fn push(&mut self, e: &Expression) {
            if self.depth > 0
                && !is_this(e)
                && !matches!(
                    e,
                    Expression::Identifier(_)
                        | Expression::Self_(_)
                        | Expression::Static(_)
                        | Expression::Parent(_)
                )
            {
                self.spans.push(e.span());
            }
        }
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {
            self.depth += 1;
        }
        fn walk_out_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {
            self.depth -= 1;
        }
        fn walk_in_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {
            self.depth += 1;
        }
        fn walk_out_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {
            self.depth -= 1;
        }
        fn walk_in_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {
            self.depth += 1;
        }
        fn walk_out_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {
            self.depth -= 1;
        }
        fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
            self.push(m.object);
            self.selector(&m.method);
        }
        fn walk_in_array(&mut self, a: &'ast Array<'arena>, _: &mut ()) {
            if let [ArrayElement::Value(_), ArrayElement::Value(second)] = a.elements.as_slice()
                && self.depth > 0
            {
                self.spans.push(second.value.span());
            }
        }
        fn walk_in_legacy_array(&mut self, a: &'ast LegacyArray<'arena>, _: &mut ()) {
            if let [ArrayElement::Value(_), ArrayElement::Value(second)] = a.elements.as_slice()
                && self.depth > 0
            {
                self.spans.push(second.value.span());
            }
        }
        fn walk_in_null_safe_method_call(
            &mut self,
            m: &'ast NullSafeMethodCall<'arena>,
            _: &mut (),
        ) {
            self.push(m.object);
            self.selector(&m.method);
        }
        fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
            self.push(m.class);
            self.selector(&m.method);
        }
        fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
            self.push(p.object);
            self.selector(&p.property);
        }
        fn walk_in_null_safe_property_access(
            &mut self,
            p: &'ast NullSafePropertyAccess<'arena>,
            _: &mut (),
        ) {
            self.push(p.object);
            self.selector(&p.property);
        }
        fn walk_in_static_property_access(
            &mut self,
            p: &'ast StaticPropertyAccess<'arena>,
            _: &mut (),
        ) {
            self.push(p.class);
            if let Some(name) = dynamic_name(&p.property).filter(|_| self.depth > 0) {
                self.spans.push(name);
            }
        }
        fn walk_in_class_constant_access(
            &mut self,
            a: &'ast ClassConstantAccess<'arena>,
            _: &mut (),
        ) {
            self.push(a.class);
        }
    }
    let mut wanted = Wanted {
        depth: 0,
        spans: Vec::new(),
    };
    wanted.walk_program(program, &mut ());
    wanted.spans
}
