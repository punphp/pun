//! IssetRule and NullCoalesceRule through IssetCheck::check() (`checkAdvancedIsset` from level 4) on isset
//! operands and the left side of `??` and `??=`. Certainty and `hasExpressionType()` come from Mago's block context (see
//! [`super::super::infer`]).

use std::collections::{HashMap, HashSet};

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::property::PropertyMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{ConstArray, Level, Tri, Type, is_super_type_of, union};

use super::super::{Checker, declared, types::offset_accessible};
use super::no_discard::unparen;

const SUPERGLOBALS: [&[u8]; 9] =
    [b"$GLOBALS", b"$_SERVER", b"$_GET", b"$_POST", b"$_FILES", b"$_COOKIE", b"$_SESSION", b"$_REQUEST", b"$_ENV"];

/// Isset operands and coalescing left-side chains (level 1): the expressions whose type the check reads,
/// and the variables and fetches whose certainty it reads.
pub(in super::super) fn coalesce_types(program: &Program, level: u8) -> (Vec<Span>, Vec<Span>) {
    let mut chains = Chains::default();
    if level >= 1 {
        chains.walk_program(program, &mut ());
    }
    (chains.typed, chains.watched)
}

#[derive(Default)]
struct Chains {
    typed: Vec<Span>,
    watched: Vec<Span>,
}

impl Chains {
    fn chain(&mut self, e: &Expression) {
        let e = unparen(e);
        self.typed.push(e.span());
        match e {
            Expression::Variable(Variable::Direct(_)) => self.watched.push(e.span()),
            Expression::ArrayAccess(a) => {
                self.watched.push(e.span());
                self.typed.push(a.index.span());
                self.chain(a.array);
            }
            Expression::Access(Access::Property(p)) => {
                self.watched.push(e.span());
                self.chain(p.object);
            }
            Expression::Access(Access::NullSafeProperty(p)) => self.chain(p.object),
            Expression::Access(Access::StaticProperty(p)) => self.chain(p.class),
            _ => {}
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Chains {
    fn walk_in_isset_construct(&mut self, i: &'ast IssetConstruct<'arena>, _: &mut ()) {
        for value in &i.values {
            self.chain(value);
        }
    }

    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        if b.operator.is_null_coalesce() {
            self.chain(b.lhs);
        }
    }

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if matches!(a.operator, AssignmentOperator::Coalesce(_)) {
            self.chain(a.lhs);
        }
    }
}

/// IssetRule invokes the shared check for every operand, including later multi-isset subjects.
pub(in super::super) fn isset(c: &mut Checker, construct: &IssetConstruct) {
    if c.level < 1 {
        return;
    }
    for value in &construct.values {
        let check = IssetCheck {
            c,
            operator: "in isset()".to_string(),
            identifier: "isset",
            advanced: c.level >= 4,
        };
        match check.check(value, None) {
            Some((identifier, message)) => {
                // Errors without an explicit line inherit the IssetExpressionNode's line upstream.
                let line = c.line_of(construct.span());
                c.condition_error(value.span(), line, &identifier, true, message, None);
            }
            None => c.no_condition(value.span(), "isset.expr"),
        }
    }
}

/// The `$this->name` fetches of initialized properties: in a method, after a statement of its body assigns it; in the
/// constructor, the promoted ones; in the other methods, what the constructor's statements assign or promotes
/// (rememberConstructorScope(), unless the class serializes itself).
// ponytail: only a body's own statements assign for certain, and a constructor's calls initialize nothing
pub(in super::super) fn initialized(program: &Program) -> HashMap<Span, bool> {
    let mut found = Initialized::default();
    found.walk_program(program, &mut ());
    found.spans
}

#[derive(Default)]
struct Initialized {
    /// Each fetch, and whether the constructor's initialization is remembered for it.
    spans: HashMap<Span, bool>,
    /// What each enclosing class's constructor initializes.
    classes: Vec<HashSet<Vec<u8>>>,
    /// Each enclosing function-like: what is initialized from its start (remembered from the constructor or not),
    /// and where its statements assign.
    functions: Vec<(HashSet<Vec<u8>>, bool, Vec<(u32, Vec<u8>)>)>,
}

/// The name of a `$this->name` fetch.
fn this_name<'e>(p: &'e PropertyAccess) -> Option<&'e [u8]> {
    let ClassLikeMemberSelector::Identifier(name) = &p.property else { return None };
    matches!(p.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this").then_some(name.value)
}

fn this_property<'e>(e: &'e Expression) -> Option<&'e [u8]> {
    let Expression::Access(Access::Property(p)) = unparen(e) else { return None };
    this_name(p)
}

/// The properties a statement assigns for certain: its own `$this->name = ...`, a block's, what every branch of an
/// `if` with an `else` assigns.
// ponytail: an `if` with a colon-delimited body, loops and early exits are not followed
fn assigns(s: &Statement) -> HashSet<Vec<u8>> {
    match s {
        Statement::Expression(e) => match e.expression {
            Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_)) => {
                this_property(a.lhs).map(|n| n.to_vec()).into_iter().collect()
            }
            _ => HashSet::new(),
        },
        Statement::Block(b) => b.statements.iter().flat_map(assigns).collect(),
        Statement::If(If { body: IfBody::Statement(b), .. }) if b.else_clause.is_some() => {
            let branches = std::iter::once(b.statement)
                .chain(b.else_if_clauses.iter().map(|c| c.statement))
                .chain(b.else_clause.iter().map(|c| c.statement));
            branches.map(assigns).reduce(|a, b| a.intersection(&b).cloned().collect()).unwrap_or_default()
        }
        _ => HashSet::new(),
    }
}

/// A body's statements assigning `$this->name`: where they end and the name.
fn assigned(statements: &[Statement]) -> Vec<(u32, Vec<u8>)> {
    statements.iter().flat_map(|s| assigns(s).into_iter().map(|n| (s.span().end.offset, n))).collect()
}

fn constructor<'m, 'arena>(members: &'m [ClassLikeMember<'arena>]) -> Option<&'m Method<'arena>> {
    members.iter().find_map(|m| match m {
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => Some(m),
        _ => None,
    })
}

fn promoted(m: &Method) -> HashSet<Vec<u8>> {
    m.parameter_list.parameters.iter().filter(|p| p.is_promoted_property()).map(|p| p.variable.name[1..].to_vec()).collect()
}

fn statements<'m, 'arena>(m: &'m Method<'arena>) -> &'m [Statement<'arena>] {
    match &m.body {
        MethodBody::Concrete(b) => b.statements.as_slice(),
        MethodBody::Abstract(_) => &[],
    }
}

impl Initialized {
    fn enter_class(&mut self, members: &[ClassLikeMember]) {
        let remembered = match constructor(members) {
            Some(m) => {
                let mut names = promoted(m);
                names.extend(assigned(statements(m)).into_iter().map(|(_, n)| n));
                names
            }
            _ => HashSet::new(),
        };
        self.classes.push(remembered);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Initialized {
    fn walk_in_class(&mut self, k: &'ast Class<'arena>, _: &mut ()) {
        self.enter_class(k.members.as_slice());
    }

    fn walk_out_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {
        self.classes.pop();
    }

    fn walk_in_anonymous_class(&mut self, k: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.enter_class(k.members.as_slice());
    }

    fn walk_out_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.classes.pop();
    }

    // (a trait's methods are a class's: nothing is known for them)
    fn walk_in_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {
        self.classes.push(HashSet::new());
    }

    fn walk_out_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {
        self.classes.pop();
    }

    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        let constructor = m.name.value.eq_ignore_ascii_case(b"__construct");
        let from_start = if constructor { promoted(m) } else { self.classes.last().cloned().unwrap_or_default() };
        self.functions.push((from_start, !constructor, assigned(statements(m))));
    }

    fn walk_out_method(&mut self, _: &'ast Method<'arena>, _: &mut ()) {
        self.functions.pop();
    }

    fn walk_in_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {
        self.functions.push(Default::default());
    }

    fn walk_out_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {
        self.functions.pop();
    }

    fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        let Some((from_start, remembered, assignments)) = self.functions.last() else { return };
        let Some(name) = this_name(p) else { return };
        let start = p.span().start.offset;
        if assignments.iter().any(|(end, n)| *end <= start && n == name) {
            self.spans.insert(p.span(), false);
        } else if from_start.contains(name) {
            self.spans.insert(p.span(), *remembered);
        }
    }
}

/// NullCoalesceRule on `??` (`lhs` of a binary) and `??=` (the target of an assignment).
pub(in super::super) fn coalesce(c: &mut Checker, left: &Expression, right: &Expression, operator: &str) {
    if c.level < 1 {
        return;
    }
    let check = IssetCheck {
        c,
        operator: format!("on left side of {operator}"),
        identifier: "nullCoalesce",
        advanced: c.level >= 4,
    };
    // in a trait, per class context (ConstantConditionInTraitHelper)
    let found = check.check(left, None).or_else(|| {
        // checkUnnecessaryNullCoalesce() (bleeding edge)
        // ponytail: a right side other than `null` itself (pure, without writes) is not considered
        let null = matches!(unparen(right), Expression::Literal(Literal::Null(_)))
            || matches!(unparen(right), Expression::ConstantAccess(a) if a.name.value().eq_ignore_ascii_case(b"null"));
        (c.bleeding_edge && null && check.is_set(left)).then(|| {
            let message = format!(
                "Coalesce operator {operator} is unnecessary because the left side is always set and the right side is null."
            );
            ("nullCoalesce.unnecessary".to_string(), message)
        })
    });
    match found {
        Some((identifier, message)) => {
            let line = c.line_of(left.span());
            c.condition_error(left.span(), line, &identifier, true, message, None);
        }
        None => c.no_condition(left.span(), "nullCoalesce.expr"),
    }
}

/// An error: its identifier and message.
type Found = Option<(String, String)>;

struct IssetCheck<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    operator: String,
    identifier: &'static str,
    advanced: bool,
}

/// The declaring class and metadata of a property a fetch finds (`findPropertyReflectionFromNode()`).
struct Property<'a> {
    declaring: &'a ClassLikeMetadata,
    metadata: &'a PropertyMetadata,
}

/// NullCoalesceRule's message for the type on the left side.
fn type_message(t: &Type) -> Option<&'static str> {
    match t.is_null() {
        Tri::Maybe => None,
        Tri::Yes => Some("is always null"),
        Tri::No => Some("is not nullable"),
    }
}

/// `hasOffsetValueType()`: arrays coerce their keys, while strings inspect the raw offset type.
fn has_offset(t: &Type, offset: &Type) -> Tri {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => {
            let all: Vec<Tri> = ts.iter().map(|t| has_offset(t, offset)).collect();
            if all.iter().all(|r| *r == all[0]) { all[0] } else { Tri::Maybe }
        }
        Type::Intersection(ts) => {
            // a non-empty list has offset 0
            if t.is_list().yes() && t.is_iterable_at_least_once().yes() && array_key(offset) == Type::ConstInt(0) {
                return Tri::Yes;
            }
            ts.iter().map(|t| has_offset(t, offset)).fold(Tri::Maybe, |a, b| if a.yes() || b.yes() { Tri::Yes } else if b.no() { Tri::No } else { a })
        }
        Type::ConstArray(a) => has_array_offset(a, &array_key(offset)),
        Type::Array(key, _) => {
            let offset = &array_key(offset);
            if is_super_type_of(key, offset).no() && (offset.is_string().no() || is_constant(offset)) {
                Tri::No
            } else {
                Tri::Maybe
            }
        }
        // a constant string's offsets in range
        Type::ConstString(v) if is_super_type_of(&Type::Int, offset).yes() => {
            let len = v.len() as i64;
            is_super_type_of(&Type::IntRange(Some(-len), Some(len - 1)), offset)
        }
        _ if t.is_string().yes() => if is_super_type_of(&Type::Int, offset).no() { Tri::No } else { Tri::Maybe },
        Type::Null => Tri::No,
        _ => if offset_accessible(t).no() { Tri::No } else { Tri::Maybe },
    }
}

/// ConstantArrayType::recursiveHasOffsetValueType(), after the one outer toArrayKey() conversion.
fn has_array_offset(a: &ConstArray, offset: &Type) -> Tri {
    // IntegerRangeType::getFiniteTypes() uses CALCULATE_SCALARS_LIMIT (128) on max - min.
    let alternatives = match offset {
        Type::Union(ts) | Type::Benevolent(ts) => Some(ts.clone()),
        Type::IntRange(Some(min), Some(max)) if i128::from(*max) - i128::from(*min) <= 128 => {
            Some((*min..=*max).map(Type::ConstInt).collect())
        }
        _ => None,
    };
    if let Some(alternatives) = alternatives {
        let all: Vec<Tri> = alternatives.iter().map(|offset| has_array_offset(a, offset)).collect();
        return if all.iter().all(|r| *r == all[0]) { all[0] } else { Tri::Maybe };
    }
    let mut result = Tri::No;
    for (i, key) in a.keys.iter().enumerate() {
        if matches!(key, Type::ConstInt(_)) && !offset.is_string().no() && !is_constant(offset)
            && !offset.is_decimal_int_string().no()
        {
            return Tri::Maybe;
        }
        let has = is_super_type_of(key, offset);
        if has.yes() {
            return if a.optional.contains(&i) { Tri::Maybe } else { Tri::Yes };
        }
        result = if has.maybe() { Tri::Maybe } else { result };
    }
    let unsealed = a.unsealed.as_ref().is_some_and(|(k, _)| !is_super_type_of(k, offset).no());
    if result.no() && unsealed { Tri::Maybe } else { result }
}

/// `getOffsetValueType()` of an offset `t` has.
fn offset_value(t: &Type, offset: &Type) -> Type {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => {
            union(ts.iter().filter(|t| !has_offset(t, offset).no()).map(|t| offset_value(t, offset)).collect())
        }
        Type::ConstArray(a) => {
            let offset = array_key(offset);
            // getOffsetValueType() includes only matching slots, not unrelated nullable fields.
            let mut values: Vec<Type> = a.keys.iter().zip(&a.values)
                .filter(|(key, _)| !is_super_type_of(key, &offset).no())
                .map(|(_, value)| value.clone()).collect();
            if let Some((key, value)) = &a.unsealed
                && !is_super_type_of(&union(a.keys.clone()), &offset).yes()
                && !is_super_type_of(key, &offset).no()
            {
                values.push(value.clone());
            }
            // A nonconstant string can hit integer keys without a definite type match.
            if values.is_empty() { t.iterable_value_type() } else { union(values) }
        }
        t if t.is_string().yes() => Type::String,
        t => t.iterable_value_type(),
    }
}

fn is_constant(t: &Type) -> bool {
    matches!(t, Type::ConstInt(_) | Type::ConstString(_) | Type::ConstBool(_) | Type::ConstFloat(_) | Type::Null)
}

/// The scalar and refined-string cases of `toArrayKey()`, preserving template identity.
fn array_key(t: &Type) -> Type {
    match t {
        Type::Union(ts) => union(ts.iter().map(array_key).collect()),
        Type::Accessory(_) | Type::Intersection(_) if t.is_decimal_int_string().yes() => Type::Int,
        Type::Accessory(_) | Type::Intersection(_) if t.is_numeric_string().yes() => union(vec![Type::Int, t.clone()]),
        Type::ConstString(s) if std::str::from_utf8(s).ok().and_then(|s| s.parse::<i64>().ok()).is_some_and(|i| i.to_string().as_bytes() == s) => Type::ConstInt(std::str::from_utf8(s).unwrap().parse().unwrap_or(0)),
        Type::Bool => union(vec![Type::ConstInt(0), Type::ConstInt(1)]),
        Type::Float => Type::Int,
        Type::ConstBool(b) => Type::ConstInt(i64::from(*b)),
        Type::ConstFloat(f) => Type::ConstInt(*f as i64),
        Type::Null => Type::ConstString(Vec::new()),
        t => t.clone(),
    }
}

impl<'a> IssetCheck<'_, 'a, '_> {
    fn ty(&self, e: &Expression) -> Option<Type> {
        let e = unparen(e);
        match e {
            // (Mago types a property fetch on `$this`, not `$this`)
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                Some(Type::This(String::from_utf8_lossy(self.c.class_scope()?.original_name.as_bytes()).into_owned()))
            }
            e => self.c.phpstan_type(e),
        }
    }

    /// The type of an offset's holder: an offset it does not have is an ErrorType (Mago types it null).
    fn holder_type(&self, e: &Expression) -> Option<Type> {
        if let Expression::ArrayAccess(a) = unparen(e)
            && has_offset(&self.holder_type(a.array)?, &self.ty(a.index)?).no()
        {
            return Some(Type::Error);
        }
        self.ty(e)
    }

    /// `$scope->hasVariableType()`.
    fn defined(&self, v: &DirectVariable) -> Tri {
        if SUPERGLOBALS.contains(&v.name) || v.name == b"$this" {
            return Tri::Yes;
        }
        self.c.defined.get(&v.span).copied().unwrap_or(Tri::Maybe)
    }

    /// `$scope->hasExpressionType($expr)->yes()`.
    fn has_expression_type(&self, e: &Expression) -> bool {
        self.c.defined.get(&e.span()) == Some(&Tri::Yes)
    }

    fn error(&self, t: &Type, message: String, second: &str) -> Found {
        let tail = type_message(t)?;
        Some((format!("{}.{second}", self.identifier), format!("{message} {tail}.")))
    }

    fn never_defined(&self, v: &DirectVariable) -> Found {
        let name = String::from_utf8_lossy(v.name);
        Some((format!("{}.variable", self.identifier), format!("Variable {name} {} is never defined.", self.operator)))
    }

    fn no_offset(&self, holder: &Type, offset: &Type) -> Found {
        Some((
            format!("{}.offset", self.identifier),
            format!(
                "Offset {} on {} {} does not exist.",
                offset.describe(Level::Value),
                holder.describe(Level::Value),
                self.operator
            ),
        ))
    }

    /// The property a fetch finds: a declared one of the holder's single class.
    // ponytail: a union holder's properties (UnionTypePropertyReflection) are not looked up
    fn property(&self, e: &Expression) -> Option<Property<'a>> {
        let (class, name): (Option<&'a ClassLikeMetadata>, &[u8]) = match e {
            Expression::Access(Access::Property(p)) => {
                let ClassLikeMemberSelector::Identifier(name) = &p.property else { return None };
                let names = self.ty(p.object)?.object_class_names();
                let [class] = names.as_slice() else { return None };
                (self.c.codebase.get_class_like(class.as_bytes()), name.value)
            }
            // (a static fetch: the callers pass property fetches only)
            _ => {
                let Expression::Access(Access::StaticProperty(p)) = e else { return None };
                let Variable::Direct(v) = &p.property else { return None };
                let class = match unparen(p.class) {
                    Expression::Identifier(id) => self.c.codebase.get_class_like(self.c.names.get(id)),
                    Expression::Self_(_) | Expression::Static(_) => self.c.class_scope(),
                    Expression::Parent(_) => self
                        .c
                        .class_scope()
                        .and_then(|k| k.direct_parent_class.as_ref())
                        .and_then(|p| self.c.codebase.get_class_like(p.as_bytes())),
                    class => {
                        let names = self.ty(class)?.object_class_names();
                        let [class] = names.as_slice() else { return None };
                        self.c.codebase.get_class_like(class.as_bytes())
                    }
                };
                (class, &v.name[1..])
            }
        };
        let mut variable = b"$".to_vec();
        variable.extend_from_slice(name);
        let declaring = self.c.codebase.get_declaring_property_class(class?.name.as_bytes(), &variable)?;
        let declaring = self.c.codebase.get_class_like(declaring.as_bytes())?;
        let metadata = declaring.properties.get(&mago_word::word(&variable))?;
        let is_static = matches!(e, Expression::Access(Access::StaticProperty(_)));
        (metadata.flags.is_static() == is_static).then_some(Property { declaring, metadata })
    }

    /// PropertyDescriptor::describeProperty().
    fn describe(&self, p: &Property, e: &Expression) -> String {
        let declaring = self.c.display(p.declaring);
        match e {
            Expression::Access(Access::Property(fetch)) => {
                let ClassLikeMemberSelector::Identifier(name) = &fetch.property else { unreachable!("a found property") };
                let on = self.ty(fetch.object).expect("a found property's holder is typed");
                let class = if is_super_type_of(&Type::Object(declaring.clone(), vec![], false), &on).yes() { declaring } else { on.describe(Level::TypeOnly) };
                format!("Property {class}::${}", String::from_utf8_lossy(name.value))
            }
            _ => format!("Static property {declaring}::{}", String::from_utf8_lossy(p.metadata.name.0.as_bytes())),
        }
    }

    /// The expression a property fetch reads on (its object, or a static fetch's class expression).
    fn holder<'e, 'arena>(e: &'e Expression<'arena>) -> Option<&'e Expression<'arena>> {
        match e {
            Expression::Access(Access::Property(p)) => Some(p.object),
            Expression::Access(Access::StaticProperty(p))
                if !matches!(unparen(p.class), Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) =>
            {
                Some(p.class)
            }
            _ => None,
        }
    }

    fn check(&self, e: &Expression, error: Found) -> Found {
        let e = unparen(e);
        match e {
            Expression::Variable(Variable::Direct(v)) => {
                let has = self.defined(v);
                if has.maybe() {
                    return None;
                }
                if error.is_some() {
                    return error;
                }
                if has.yes() {
                    if v.name == b"$_SESSION" {
                        return None;
                    }
                    let name = String::from_utf8_lossy(v.name);
                    let t = self.ty(e)?;
                    if !matches!(t, Type::Never(_)) { return self.error(&t, format!("Variable {name} {} always exists and", self.operator), "variable") }
                }
                self.never_defined(v)
            }
            Expression::ArrayAccess(a) => {
                let holder = self.holder_type(a.array)?;
                if !offset_accessible(&holder).yes() {
                    return error.or_else(|| self.undefined(a.array));
                }
                let offset = self.ty(a.index)?;
                let has = has_offset(&holder, &offset);
                if has.no() {
                    return self.advanced.then(|| self.no_offset(&holder, &offset)).flatten();
                }
                if has.yes() || self.has_expression_type(e) {
                    if !self.advanced {
                        return None;
                    }
                    let error = error.or_else(|| {
                        let message = format!(
                            "Offset {} on {} {} always exists and",
                            offset.describe(Level::Value),
                            holder.describe(Level::Value),
                            self.operator
                        );
                        self.error(&offset_value(&holder, &offset), message, "offset")
                    });
                    if error.is_some() {
                        return self.check(a.array, error);
                    }
                }
                None
            }
            Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
                let holder = Self::holder(e);
                let undefined = || holder.and_then(|h| self.undefined(h));
                let Some(p) = self.property(e).filter(|p| !p.declaring.magic_properties.contains_key(&p.metadata.name.0)) else {
                    return undefined();
                };
                let m = p.metadata;
                // isInitializedThisProperty()
                if let Some(native) = &m.type_declaration_metadata
                    && !m.flags.is_virtual_property()
                    && let Some(remembered) = self.c.conditions.initialized.get(&e.span())
                    && let Some(class) = self.c.class_scope().filter(|k| k.name == p.declaring.name)
                    && !(*remembered && self.serializes(class))
                {
                    let t = declared::from_tunion(self.c, &native.type_union, p.declaring);
                    let tail = match type_message(&t)? {
                        tail if tail.starts_with("is not") => format!("{tail} nor uninitialized"),
                        tail => format!("{tail} and initialized"),
                    };
                    return Some((
                        format!("{}.initializedProperty", self.identifier),
                        format!("{} {} {tail}.", self.describe(&p, e), self.operator),
                    ));
                }
                if self.maybe_uninitialized(&p, e) {
                    return None;
                }
                let on = |error: Found| match holder {
                    Some(h) => self.check(h, error),
                    None => error,
                };
                if error.is_some() {
                    return on(error);
                }
                if !self.advanced {
                    return undefined();
                }
                let name = String::from_utf8_lossy(&m.name.0.as_bytes()[1..]).into_owned();
                let t = declared::property_of(self.c, p.declaring, m, name.as_bytes());
                let message = format!("{} ({}) {}", self.describe(&p, e), t.describe(Level::TypeOnly), self.operator);
                let error = self.error(&t, message, "property");
                if error.is_some() { on(error) } else { None }
            }
            _ if error.is_some() => error,
            _ if !self.advanced => None,
            _ => {
                // a property fetch on what is not an object is an ErrorType (Mago types it null)
                let t = match e {
                    Expression::Access(Access::NullSafeProperty(p))
                        if {
                            let holder = self.ty(p.object)?.remove_null();
                            holder.is_string().yes()
                                || holder.is_array().yes()
                                || matches!(holder, Type::Int | Type::ConstInt(_) | Type::IntRange(..) | Type::Float | Type::ConstFloat(_) | Type::Bool | Type::ConstBool(_))
                        } =>
                    {
                        Type::Error
                    }
                    e => self.ty(e)?,
                };
                let error = self.error(&t, format!("Expression {}", self.operator), "expr");
                if error.is_some() {
                    return error;
                }
                let Expression::Access(Access::NullSafeProperty(p)) = e else { return None };
                let name = match &p.property {
                    ClassLikeMemberSelector::Identifier(name) => String::from_utf8_lossy(name.value).into_owned(),
                    _ => "(Expression)".to_string(),
                };
                Some((
                    "nullsafe.neverNull".to_string(),
                    format!("Using nullsafe property access \"?->{name}\" {} is unnecessary. Use -> instead.", self.operator),
                ))
            }
        }
    }

    /// classHasCustomSerialization(): unserialize() rebuilds its objects without the constructor.
    fn serializes(&self, k: &ClassLikeMetadata) -> bool {
        let has = |method: &str| self.c.codebase.method_exists(k.name.as_bytes(), method.as_bytes());
        ["__sleep", "__serialize", "__unserialize"].iter().any(|m| has(m))
            || (self.c.codebase.is_instance_of(k.name.as_bytes(), b"Serializable") && has("unserialize"))
    }

    /// A natively typed property this fetch may read before its initialization: no default value, not assigned in
    /// this scope, and not a promoted one PHP initializes (readonly or hooked).
    fn maybe_uninitialized(&self, p: &Property, e: &Expression) -> bool {
        let m = p.metadata;
        m.type_declaration_metadata.is_some()
            && !m.flags.is_virtual_property()
            && !self.has_expression_type(e)
            && !m.flags.has_default()
            && (!m.flags.is_promoted_property() || (!m.flags.is_readonly() && m.hooks.is_empty()))
    }

    /// `IssetabilityResolution::isSet()` with a callback that is always true.
    fn is_set(&self, e: &Expression) -> bool {
        let e = unparen(e);
        match e {
            Expression::Variable(Variable::Direct(v)) => self.defined(v).yes() && v.name != b"$_SESSION",
            Expression::ArrayAccess(a) => {
                let (Some(holder), Some(offset)) = (self.holder_type(a.array), self.ty(a.index)) else { return false };
                offset_accessible(&holder).yes() && has_offset(&holder, &offset).yes() && self.is_set(a.array)
            }
            Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
                let Some(p) = self.property(e).filter(|p| !p.declaring.magic_properties.contains_key(&p.metadata.name.0)) else {
                    return false;
                };
                !self.maybe_uninitialized(&p, e) && Self::holder(e).is_none_or(|h| self.is_set(h))
            }
            _ => true,
        }
    }

    /// IssetCheck::checkUndefined().
    fn undefined(&self, e: &Expression) -> Found {
        let e = unparen(e);
        match e {
            Expression::Variable(Variable::Direct(v)) => {
                if self.defined(v).no() { self.never_defined(v) } else { None }
            }
            Expression::ArrayAccess(a) => {
                let holder = self.holder_type(a.array)?;
                let offset = self.ty(a.index)?;
                if !offset_accessible(&holder).yes() || !has_offset(&holder, &offset).no() {
                    return self.undefined(a.array);
                }
                self.no_offset(&holder, &offset)
            }
            _ => Self::holder(e).and_then(|h| self.undefined(h)),
        }
    }
}
