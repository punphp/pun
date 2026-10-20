//! AccessPropertiesRule and AccessPropertiesInAssignRule (AccessPropertiesCheck). Level 0 and 1 run them with
//! `checkThisOnly`, so only `$this->name` is checked, against the class `$this` is.

use std::collections::HashSet;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use mago_word::word;

use super::Checker;
use super::scope::Guard;

pub(super) const TIP: &str =
    "Learn more: https://phpstan.org/blog/solving-phpstan-access-to-undefined-property";

/// What the walk needs to know of a file's property fetches before it reaches them.
#[derive(Default)]
pub struct State {
    /// `isUndefinedExpressionAllowed()`: inside `isset()`, `empty()`, `unset()`, the left of `??` and a list
    /// destructuring target, with the expressions they hold (lookForSetAllowedUndefinedExpressions).
    allowed: HashSet<(u32, u32)>,
    /// `isInExpressionAssign()`: the property fetch an assignment (not a compound one) writes, or whose offset it
    /// writes.
    assigned: HashSet<(u32, u32)>,
    /// `isInExpressionAssign()` as ReadingWriteOnlyPropertiesRule sees it: also the target of `??=`, of a foreach
    /// and the referenced side of `= &`.
    expression_assign: HashSet<(u32, u32)>,
    /// Whether `$this` is defined in each function-like entered (static methods and closures have none).
    this: Vec<bool>,
}

/// `isUndefinedExpressionAllowed()` of the expression at `span`.
pub fn undefined_allowed(c: &Checker, span: Span) -> bool {
    c.property_access.allowed.contains(&key(span))
}

fn key(s: Span) -> (u32, u32) {
    (s.start.offset, s.end.offset)
}

fn unparenthesized<'e, 'arena>(mut e: &'e Expression<'arena>) -> &'e Expression<'arena> {
    while let Expression::Parenthesized(p) = e {
        e = p.expression;
    }
    e
}

struct Contexts<'s> {
    state: &'s mut State,
}

impl Contexts<'_> {
    /// lookForExpressionCallback().
    fn allow(&mut self, e: &Expression) {
        let e = unparenthesized(e);
        if !matches!(e, Expression::ArrayAppend(_)) {
            self.state.allowed.insert(key(e.span()));
        }
        match e {
            Expression::ArrayAccess(a) => self.allow(a.array),
            Expression::ArrayAppend(a) => self.allow(a.array),
            Expression::Access(Access::Property(p)) => self.allow(p.object),
            Expression::Access(Access::NullSafeProperty(p)) => self.allow(p.object),
            Expression::Call(Call::NullSafeMethod(m)) => self.allow(m.object),
            Expression::Access(Access::StaticProperty(p)) if !is_name(p.class) => {
                self.allow(p.class);
            }
            Expression::List(l) => l.elements.iter().for_each(|el| self.element(el)),
            Expression::Array(a) => a.elements.iter().for_each(|el| self.element(el)),
            _ => {}
        }
    }

    fn element(&mut self, el: &ArrayElement) {
        match el {
            ArrayElement::KeyValue(kv) => self.allow(kv.value),
            ArrayElement::Value(v) => self.allow(v.value),
            _ => {}
        }
    }

    /// The root of an assignment target (through its offsets), and a destructuring's items.
    fn assigned(&mut self, e: &Expression) {
        match unparenthesized(e) {
            Expression::ArrayAccess(a) => self.assigned(a.array),
            Expression::ArrayAppend(a) => self.assigned(a.array),
            e @ Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
                self.state.assigned.insert(key(e.span()));
                self.state.expression_assign.insert(key(e.span()));
            }
            Expression::List(l) => l.elements.iter().for_each(|el| self.destructured(el)),
            Expression::Array(a) => a.elements.iter().for_each(|el| self.destructured(el)),
            _ => {}
        }
    }

    /// A target [`State::expression_assign`] adds (through a `&`).
    fn expression_assigned(&mut self, e: &Expression) {
        let mut root = unparenthesized(e);
        loop {
            match root {
                Expression::UnaryPrefix(u) if u.operator.is_reference() => root = unparenthesized(u.operand),
                Expression::ArrayAccess(a) => root = unparenthesized(a.array),
                Expression::ArrayAppend(a) => root = unparenthesized(a.array),
                _ => break,
            }
        }
        match root {
            Expression::List(_) | Expression::Array(_) => {
                let assigned = self.state.assigned.clone();
                self.assigned(root);
                // a destructuring only adds to the expression assigns
                self.state.assigned = assigned;
            }
            _ => {
                self.state.expression_assign.insert(key(root.span()));
            }
        }
    }

    fn destructured(&mut self, el: &ArrayElement) {
        let value = match el {
            ArrayElement::KeyValue(kv) => kv.value,
            ArrayElement::Value(v) => v.value,
            _ => return,
        };
        self.allow(value);
        self.assigned(value);
    }
}

fn is_name(e: &Expression) -> bool {
    matches!(
        e,
        Expression::Identifier(_)
            | Expression::Self_(_)
            | Expression::Static(_)
            | Expression::Parent(_)
    )
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Contexts<'_> {
    fn walk_in_isset_construct(&mut self, i: &'ast IssetConstruct<'arena>, _: &mut ()) {
        for v in i.values.iter() {
            self.allow(v);
        }
    }
    fn walk_in_empty_construct(&mut self, e: &'ast EmptyConstruct<'arena>, _: &mut ()) {
        self.allow(e.value);
    }
    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        for v in u.values.iter() {
            self.allow(v);
        }
    }
    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        if b.operator.is_null_coalesce() {
            self.allow(b.lhs);
        }
    }
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if matches!(a.operator, AssignmentOperator::Assign(_)) {
            self.assigned(a.lhs);
            if let Expression::UnaryPrefix(u) = unparenthesized(a.rhs)
                && u.operator.is_reference()
            {
                self.expression_assigned(u.operand);
            }
        } else if matches!(a.operator, AssignmentOperator::Coalesce(_)) {
            self.expression_assigned(a.lhs);
        }
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        if let ForeachTarget::KeyValue(t) = &f.target {
            self.expression_assigned(t.key);
        }
        self.expression_assigned(f.target.value());
    }
}

/// The [`State`] of `program`.
pub fn state(program: &Program) -> State {
    let mut state = State::default();
    Contexts { state: &mut state }.walk_program(program, &mut ());
    state
}

/// The guard of `$variable->name` with a written name (`$this` included).
fn fetch_guard(e: &Expression) -> Option<Guard> {
    let Expression::Access(Access::Property(p)) = unparenthesized(e) else {
        return None;
    };
    let (Expression::Variable(Variable::Direct(v)), ClassLikeMemberSelector::Identifier(id)) =
        (unparenthesized(p.object), &p.property)
    else {
        return None;
    };
    Some(var_guard(v.name, id.value))
}

fn var_guard(var: &[u8], name: &[u8]) -> Guard {
    Guard::Expr(format!("{}->{}", String::from_utf8_lossy(var), String::from_utf8_lossy(name)))
}

/// The key of a variable an `isset()` of its offset gives a HasOffsetType.
pub fn offset_guard(var: &[u8]) -> Guard {
    Guard::Expr(format!("{}[]", String::from_utf8_lossy(var)))
}

fn this_name<'a>(p: &'a PropertyAccess) -> Option<&'a [u8]> {
    let is_this = matches!(unparenthesized(p.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    match &p.property {
        ClassLikeMemberSelector::Identifier(id) if is_this => Some(id.value),
        _ => None,
    }
}

/// The expression key of `$this->name` that guards remember.
fn guard(name: &[u8]) -> Guard {
    var_guard(b"$this", name)
}

/// The `$this->name` fetches a condition specifies a type for (TypeSpecifier), as guards.
pub fn narrowed(
    e: &Expression,
    truthy: bool,
    resolve: &dyn Fn(&Identifier) -> String,
    out: &mut Vec<Guard>,
) {
    let e = unparenthesized(e);
    if let Some(g) = fetch_guard(e) {
        out.push(g);
        return;
    }
    if let Some(g) = static_fetch(e, resolve) {
        out.push(g);
        return;
    }
    match e {
        // a true `isset()` and a false `empty()` give the fetches a type
        Expression::Construct(Construct::Isset(i)) if truthy => {
            for v in i.values.iter() {
                chain(v, resolve, out);
            }
        }
        Expression::Construct(Construct::Empty(x)) if !truthy => chain(x.value, resolve, out),
        Expression::Binary(b) if b.operator.is_comparison() || b.operator.is_instanceof() => {
            narrowed(b.lhs, truthy, resolve, out);
            narrowed(b.rhs, truthy, resolve, out);
        }
        Expression::Call(Call::Function(f)) => {
            if truthy && let Some(g) = property_exists(f, resolve) {
                out.push(g);
                return;
            }
            if let Some(first) = f.argument_list.arguments.first() {
                narrowed(first.value(), truthy, resolve, out);
            }
        }
        _ => {}
    }
}

/// `isset()` specifies every fetch of the chain.
fn chain(e: &Expression, resolve: &dyn Fn(&Identifier) -> String, out: &mut Vec<Guard>) {
    let e = unparenthesized(e);
    if let Some(g) = fetch_guard(e) {
        out.push(g);
    }
    if let Some(g) = static_fetch(e, resolve) {
        out.push(g);
    }
    match e {
        Expression::ArrayAccess(a) => {
            // a variable's offset: the variable is an intersection with HasOffsetType
            if let Expression::Variable(Variable::Direct(v)) = unparenthesized(a.array) {
                out.push(offset_guard(v.name));
            }
            chain(a.array, resolve, out)
        }
        Expression::Access(Access::Property(p)) => chain(p.object, resolve, out),
        Expression::Access(Access::NullSafeProperty(p)) => chain(p.object, resolve, out),
        _ => {}
    }
}

/// How a static fetch's class is written in a scope key: `self`, `static`, `parent` or the resolved name.
fn class_key(e: &Expression, resolve: &dyn Fn(&Identifier) -> String) -> Option<String> {
    Some(match unparenthesized(e) {
        Expression::Self_(_) => "self".to_string(),
        Expression::Static(_) => "static".to_string(),
        Expression::Parent(_) => "parent".to_string(),
        Expression::Identifier(id) => resolve(id).to_ascii_lowercase(),
        _ => return None,
    })
}

/// The scope key of `C::$name`.
fn static_fetch(e: &Expression, resolve: &dyn Fn(&Identifier) -> String) -> Option<Guard> {
    let Expression::Access(Access::StaticProperty(p)) = e else {
        return None;
    };
    static_key(p, resolve)
}

fn static_key(p: &StaticPropertyAccess, resolve: &dyn Fn(&Identifier) -> String) -> Option<Guard> {
    let Variable::Direct(v) = &p.property else {
        return None;
    };
    let class = class_key(p.class, resolve)?;
    Some(Guard::Expr(format!(
        "{class}::{}",
        String::from_utf8_lossy(v.name)
    )))
}

/// The scope key of a true `property_exists(C::class, 'name')`.
fn exists_key(class: &str, name: &str) -> Guard {
    Guard::Expr(format!("property_exists({class}::class, '{name}')"))
}

/// PropertyExistsTypeSpecifyingExtension: a true `property_exists($this, 'x')` gives `$this` the property, and
/// a true `property_exists(C::class, 'x')` is remembered for AccessStaticPropertiesCheck.
fn property_exists(f: &FunctionCall, resolve: &dyn Fn(&Identifier) -> String) -> Option<Guard> {
    let Expression::Identifier(id) = f.function else {
        return None;
    };
    // an unqualified call falls back to the global function
    let function = match id {
        Identifier::Local(l) => String::from_utf8_lossy(l.value).into_owned(),
        _ => resolve(id),
    };
    if !function
        .trim_start_matches('\\')
        .eq_ignore_ascii_case("property_exists")
    {
        return None;
    }
    let mut args = f.argument_list.arguments.iter();
    let (object, name) = (args.next()?, args.next()?);
    let Expression::Literal(Literal::String(s)) = unparenthesized(name.value()) else {
        return None;
    };
    let name = String::from_utf8_lossy(s.value?).into_owned();
    match unparenthesized(object.value()) {
        Expression::Variable(Variable::Direct(v)) => Some(var_guard(v.name, name.as_bytes())),
        Expression::Access(Access::ClassConstant(cc)) => match &cc.constant {
            ClassLikeConstantSelector::Identifier(c) if c.value.eq_ignore_ascii_case(b"class") => {
                Some(exists_key(&class_key(cc.class, resolve)?, &name))
            }
            _ => None,
        },
        _ => None,
    }
}

impl Checker<'_, '_> {
    pub(super) fn enter_this(&mut self, defined: bool) {
        self.property_access.this.push(defined);
    }

    /// A closure or arrow function: `$this` as in its parent unless it is static.
    pub(super) fn enter_closure_this(&mut self, is_static: bool) {
        let parent = self.property_access.this.last().copied().unwrap_or(false);
        self.property_access.this.push(parent && !is_static);
    }

    /// `$this++`/`--$this` leave `$this` a non-object type, so no later `$this->x` is checked (oracle-checked).
    // ponytail: the rest of the function, not just the paths after it; and `$this[] = ...` on a class without
    // ArrayAccess, after which PHPStan describes `$this` as `ArrayAccess&C`
    pub(super) fn incremented(&mut self, operand: &Expression) {
        if matches!(unparenthesized(operand), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            && let Some(this) = self.property_access.this.last_mut()
        {
            *this = false;
        }
    }

    /// Whether `$this` is defined where the walk is.
    pub(super) fn this_defined(&self) -> bool {
        self.property_access.this.last().copied().unwrap_or(false)
    }

    pub(super) fn allows_dynamic_properties(&self, k: &ClassLikeMetadata) -> bool {
        allows_dynamic_properties(self, k)
    }

    pub(super) fn leave_this(&mut self) {
        self.property_access.this.pop();
    }

    fn has_expression_type(&self, name: &[u8]) -> bool {
        let g = guard(name);
        self.guards.contains(&g)
    }

    /// `hasExpressionType()` of `$object->name` (a variable's).
    pub(super) fn has_fetch_type(&self, object: &Expression, name: &str) -> bool {
        match unparenthesized(object) {
            Expression::Variable(Variable::Direct(v)) => self.guards.contains(&var_guard(v.name, name.as_bytes())),
            _ => false,
        }
    }
}

/// `$this->name = ...` (a compound assignment and a destructuring too) leaves `$this->name` an expression type.
pub fn assigned(c: &mut Checker, a: &Assignment) {
    remember(c, a.lhs);
}

fn remember(c: &mut Checker, e: &Expression) {
    let e = unparenthesized(e);
    if let Some(g) = fetch_guard(e) {
        c.guards.push(g);
        return;
    }
    let items = match e {
        Expression::ArrayAccess(x) => return remember(c, x.array),
        Expression::ArrayAppend(x) => return remember(c, x.array),
        Expression::List(l) => &l.elements,
        Expression::Array(a) => &a.elements,
        _ => return,
    };
    for el in items.iter() {
        match el {
            ArrayElement::KeyValue(kv) => remember(c, kv.value),
            ArrayElement::Value(v) => remember(c, v.value),
            _ => {}
        }
    }
}

/// AccessPropertiesRule on one property fetch.
pub fn read(c: &mut Checker, p: &PropertyAccess) {
    if c.property_access.assigned.contains(&key(p.span())) {
        return;
    }
    check(c, p, false);
}

/// AccessPropertiesInAssignRule: an assignment (not a compound one) to a property, one of its offsets, or a
/// destructuring item.
pub fn assign(c: &mut Checker, a: &Assignment) {
    if matches!(a.operator, AssignmentOperator::Assign(_)) {
        written(c, a.lhs);
    }
}

fn written(c: &mut Checker, e: &Expression) {
    match unparenthesized(e) {
        Expression::ArrayAccess(x) => written(c, x.array),
        Expression::ArrayAppend(x) => written(c, x.array),
        Expression::Access(Access::Property(p)) => check(c, p, true),
        Expression::Access(Access::StaticProperty(p)) => static_check(c, p),
        Expression::List(l) => l.elements.iter().for_each(|el| written_item(c, el)),
        Expression::Array(a) => a.elements.iter().for_each(|el| written_item(c, el)),
        _ => {}
    }
}

fn written_item(c: &mut Checker, el: &ArrayElement) {
    match el {
        ArrayElement::KeyValue(kv) => written(c, kv.value),
        ArrayElement::Value(v) => written(c, v.value),
        _ => {}
    }
}

/// `ClassReflection::allowsDynamicProperties()`.
pub(super) fn allows_dynamic_properties(c: &Checker, k: &ClassLikeMetadata) -> bool {
    if k.kind == SymbolKind::Enum {
        return false;
    }
    // PhpVersion::deprecatesDynamicProperties()
    if c.php_version < 80200 {
        return true;
    }
    if ["__get", "__set", "__isset"]
        .iter()
        .any(|m| has_native_method(c, k, m))
    {
        return true;
    }
    if k.flags.is_readonly() {
        return false;
    }
    // the universal object crates (`stdClass` and its children)
    if c.codebase.is_instance_of(k.name.as_bytes(), b"stdClass") {
        return true;
    }
    std::iter::once(&k.name)
        .chain(k.all_parent_classes.iter())
        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
        .any(|p| {
            p.attributes.iter().any(|a| {
                a.name
                    .as_bytes()
                    .eq_ignore_ascii_case(b"AllowDynamicProperties")
            })
        })
}

/// `hasNativeMethod()`.
fn has_native_method(c: &Checker, k: &ClassLikeMetadata, name: &str) -> bool {
    super::members::method(c, k, name.as_bytes()).is_some()
}

/// A native property of `k` by name (`$` included) as BetterReflection's `hasProperty()` sees it: declared by the
/// class, its traits or a parent (Mago leaves a parent's private one out of `declaring_property_ids`); `Some(is_static)`.
pub(super) fn native_property(c: &Checker, k: &ClassLikeMetadata, property: &[u8]) -> Option<bool> {
    let p = c
        .codebase
        .get_declaring_property(k.name.as_bytes(), property)?;
    Some(p.flags.is_static() || is_static_hooked(c, p))
}

/// Mago drops `static` from a hooked property (PHP refuses one), while PHPStan's reflection keeps it: read the
/// modifiers from the declaration's source.
fn is_static_hooked(c: &Checker, p: &mago_codex::metadata::property::PropertyMetadata) -> bool {
    let (Some(span), Some(name)) = (p.span, p.name_span) else {
        return false;
    };
    if p.hooks.is_empty() {
        return false;
    }
    c.file_contents(span.file_id).is_some_and(|contents| {
        let head = &contents[span.start.offset as usize..name.start.offset as usize];
        // past any attribute group; a property type cannot be `static`
        let modifiers = head.rsplit(|&b| b == b']').next().unwrap_or(head);
        modifiers
            .split(|b| b.is_ascii_whitespace())
            .any(|w| w.eq_ignore_ascii_case(b"static"))
    })
}

/// `ClassReflection::hasInstanceProperty()`: a non-static native property, else (when dynamic properties are
/// allowed) an `@property` tag.
pub(super) fn has_instance_property(c: &Checker, k: &ClassLikeMetadata, property: &[u8]) -> bool {
    if native_property(c, k, property) == Some(false) {
        return true;
    }
    if k.kind == SymbolKind::Enum {
        return false;
    }
    // ponytail: the mixin and require-extends extensions (and Larastan's) are not consulted
    allows_dynamic_properties(c, k)
        && (k.magic_property_ids.contains_key(&word(property)) || is_universal_object_crate(k))
}

/// UniversalObjectCratesClassReflectionExtension with the default `universalObjectCratesClasses: [stdClass]`.
fn is_universal_object_crate(k: &ClassLikeMetadata) -> bool {
    k.name.as_bytes() == b"stdclass" || k.all_parent_classes.contains(&word(b"stdclass"))
}

fn check(c: &mut Checker, p: &PropertyAccess, write: bool) {
    let Some(name) = this_name(p) else {
        let allowed = !write && c.property_access.allowed.contains(&key(p.span()));
        super::types::property_fetch(c, p.span(), p.object, &p.property, false, allowed);
        return;
    };
    if !c.property_access.this.last().copied().unwrap_or(false) {
        return;
    }
    let Some(class) = c.class_scope() else {
        return;
    };
    let mut property = Vec::with_capacity(name.len() + 1);
    property.push(b'$');
    property.extend_from_slice(name);
    // the write check runs in the scope before the assignment, where no undefined expression is allowed
    let allowed = !write && c.property_access.allowed.contains(&key(p.span()));
    let has = has_instance_property(c, class, &property);
    let has_static = native_property(c, class, &property) == Some(true);
    if has {
        return; // ponytail: asymmetric visibility (`private(set)`) on a parent's property
    }
    // `isFinal()`: the keyword or `@final` (Mago's flag holds both); an anonymous class is final
    let is_final = class.flags.is_final()
        || class
            .original_name
            .as_bytes()
            .starts_with(b"{anonymous-class:");
    let maybe = allows_dynamic_properties(c, class) || !is_final;
    if maybe && !has_static && allowed {
        return;
    }
    if c.has_expression_type(name) {
        return;
    }
    // !reportMagicProperties (level 0); ponytail: from level 1 a class with `__get` needs the property extensions
    // (Larastan's model properties), so it stays unchecked
    if has_native_method(c, class, "__get") || has_native_method(c, class, "__set") {
        return;
    }
    // this_name() took the name from an identifier
    let name_line = p.property.span();
    let display_name = String::from_utf8_lossy(name).into_owned();
    let mut parent = class
        .direct_parent_class
        .and_then(|n| c.codebase.get_class_like(n.as_bytes()));
    while let Some(k) = parent {
        if has_instance_property(c, k, &property) {
            // `canReadProperty()`/`canWriteProperty()` from the class: only a parent's private property is refused
            if k.properties
                .get(&word(&property))
                .is_some_and(|x| x.read_visibility.is_private())
            {
                let message = format!(
                    "Access to private property ${display_name} of parent class {}.",
                    c.display(k)
                );
                c.report(name_line, "property.private", message, None);
            }
            return;
        }
        parent = k
            .direct_parent_class
            .and_then(|n| c.codebase.get_class_like(n.as_bytes()));
    }
    if has_static {
        let declaring = c
            .codebase
            .get_declaring_property_class(class.name.as_bytes(), &property)
            .and_then(|d| c.codebase.get_class_like(d.as_bytes()))
            .map_or_else(|| c.display(class), |d| c.display(d));
        c.report(
            name_line,
            "staticProperty.nonStaticAccess",
            format!("Non-static access to static property {declaring}::${display_name}."),
            None,
        );
        return;
    }
    let described = c.describe_object(class);
    c.report(
        p.span(),
        "property.notFound",
        format!("Access to an undefined property {described}::${display_name}."),
        Some(TIP),
    );
}

/// AccessPropertiesRule on a nullsafe fetch, which it sees as a fetch on the object without `null`.
pub fn null_safe_read(c: &mut Checker, p: &NullSafePropertyAccess) {
    let allowed = c.property_access.allowed.contains(&key(p.span()));
    super::types::property_fetch(c, p.span(), p.object, &p.property, true, allowed);
}

/// AccessStaticPropertiesRule on one static property fetch.
pub fn static_read(c: &mut Checker, p: &StaticPropertyAccess) {
    if c.property_access.assigned.contains(&key(p.span())) {
        return;
    }
    static_check(c, p);
}

/// The class a static property fetch names: `None` when nothing is left to check.
enum Holder<'a> {
    /// `self`/`static` (`static` describes as `static(C)`), or a named class.
    Class(&'a ClassLikeMetadata, bool),
    Done,
}

/// `ClassReflection::hasStaticProperty()`.
pub(super) fn has_static_property(c: &Checker, k: &ClassLikeMetadata, property: &[u8]) -> bool {
    native_property(c, k, property) == Some(true)
}

/// AccessStaticPropertiesCheck::processSingleProperty().
fn static_check(c: &mut Checker, p: &StaticPropertyAccess) {
    let Variable::Direct(v) = &p.property else {
        return; // ponytail: a dynamic name (`Foo::$$name`) needs its constant strings
    };
    let property = v.name;
    let name = String::from_utf8_lossy(&property[1..]).into_owned();
    let at = p.property.span();
    let holder = match (unparenthesized(p.class), c.class_scope()) {
        (e @ (Expression::Self_(_) | Expression::Static(_)), scope) => {
            let written = if matches!(e, Expression::Self_(_)) {
                "self"
            } else {
                "static"
            };
            let Some(k) = scope else {
                c.report(
                    at,
                    &format!("outOfClass.{written}"),
                    format!(
                        "Accessing {}::${name} outside of class scope.",
                        c.written(e.span())
                    ),
                    None,
                );
                return;
            };
            Holder::Class(k, written == "static")
        }
        (e @ Expression::Parent(_), scope) => {
            let Some(scope) = scope else {
                c.report(
                    at,
                    "outOfClass.parent",
                    format!(
                        "Accessing {}::${name} outside of class scope.",
                        c.written(e.span())
                    ),
                    None,
                );
                return;
            };
            // an unknown parent is no parent (`getParentClass()`)
            let parent = scope
                .direct_parent_class
                .and_then(|p| c.codebase.get_class_like(p.as_bytes()));
            match parent {
                None => {
                    let action = format!(
                        "{}::{}() accesses parent::${name}",
                        c.display(scope),
                        c.function_name()
                    );
                    c.report_no_parent(at, scope, &action);
                    return;
                }
                Some(k) => Holder::Class(k, false),
            }
        }
        (Expression::Identifier(id), _) => {
            let class = c.resolved(id);
            match c.class_like(&class) {
                super::reflection::Lookup::NotFound => {
                    if !c.in_class_exists(&class) {
                        c.report(
                            at,
                            "class.notFound",
                            format!(
                                "Access to static property ${name} on an unknown class {class}."
                            ),
                            Some(super::DISCOVERING_SYMBOLS_TIP),
                        );
                    }
                    return;
                }
                super::reflection::Lookup::Opaque => {
                    super::class_names::check(
                        c,
                        &[super::class_names::Pair::name(class, id.span())],
                        true,
                    );
                    Holder::Done
                }
                super::reflection::Lookup::Found(k) => {
                    let member = super::internal::static_property_member(c, k, name.as_bytes());
                    let location = super::class_names::Location::StaticPropertyAccess(member);
                    super::class_names::check_at(
                        c,
                        &[super::class_names::Pair::name(class, id.span())],
                        Some(&location),
                        true,
                    );
                    Holder::Class(k, false)
                }
            }
        }
        _ => return super::types::static_property_fetch(c, p, property),
    };
    let Holder::Class(class, is_static) = holder else {
        return;
    };
    // PHPStan 2.3.1 reports `isset(self::$x)` and a read after `self::$x = ...` alike (oracle-checked), so neither
    // the allowed-undefined contexts nor assignments exempt a static fetch
    let declared = c
        .codebase
        .get_declaring_property(class.name.as_bytes(), property)
        .filter(|_| has_static_property(c, class, property));
    let Some(declared) = declared else {
        let resolve = |id: &Identifier| c.resolved(id);
        if static_key(p, &resolve).is_some_and(|g| c.guards.contains(&g)) {
            return;
        }
        let mut parent = class
            .direct_parent_class
            .and_then(|n| c.codebase.get_class_like(n.as_bytes()));
        while let Some(k) = parent {
            if has_static_property(c, k, property) {
                if k.properties
                    .get(&word(property))
                    .is_some_and(|x| x.read_visibility.is_private())
                {
                    let message = format!(
                        "Access to private static property ${name} of parent class {}.",
                        c.display(k)
                    );
                    c.report(at, "staticProperty.private", message, None);
                }
                return;
            }
            parent = k
                .direct_parent_class
                .and_then(|n| c.codebase.get_class_like(n.as_bytes()));
        }
        if has_instance_property(c, class, property) {
            let declaring = c
                .codebase
                .get_declaring_property_class(class.name.as_bytes(), property)
                .and_then(|d| c.codebase.get_class_like(d.as_bytes()))
                .map_or_else(|| c.display(class), |d| c.display(d));
            c.report(
                at,
                "property.staticAccess",
                format!("Static access to instance property {declaring}::${name}."),
                None,
            );
            return;
        }
        let exists = class_key(p.class, &resolve).map(|k| exists_key(&k, &name));
        if exists.is_some_and(|g| c.guards.contains(&g)) {
            return;
        }
        let described = if is_static {
            format!("static({})", c.display(class))
        } else {
            c.display(class)
        };
        c.report(
            at,
            "staticProperty.notFound",
            format!("Access to an undefined static property {described}::${name}."),
            None,
        );
        return;
    };
    let visibility = declared.read_visibility;
    let declaring = c
        .codebase
        .get_declaring_property_class(class.name.as_bytes(), property)
        .and_then(|d| c.codebase.get_class_like(d.as_bytes()))
        .filter(|d| d.kind != SymbolKind::Trait)
        .unwrap_or(class);
    let declaring_name = String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned();
    if super::calls::can_call(c, c.class_scope(), &declaring_name, visibility) {
        return; // ponytail: asymmetric set visibility on static properties (PHP 8.5)
    }
    let v = super::restricted(visibility);
    let message = format!(
        "Access to {v} property ${name} of class {}.",
        c.display(declaring)
    );
    c.report(at, &format!("staticProperty.{v}"), message, None);
}

/// `isInExpressionAssign()` of a property fetch, for ReadingWriteOnlyPropertiesRule.
pub(super) fn in_expression_assign(c: &Checker, e: Span) -> bool {
    c.property_access.expression_assign.contains(&key(e))
}
