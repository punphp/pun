//! IssetRule and NullCoalesceRule through IssetCheck::check() (`checkAdvancedIsset` from level 4) on isset
//! operands and the left side of `??` and `??=`. Certainty and `hasExpressionType()` come from Mago's block context (see
//! [`super::super::infer`]).

use mago_analyzer::property_initialization::{
    PropertyInitializationCertainty, PropertyInitializationRequest,
};
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::property::PropertyMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Accessory, ConstArray, Level, Tri, Type, is_super_type_of, union};

use super::super::types::flow::unparen;
use super::super::{Checker, declared, types::offset_accessible};

const SUPERGLOBALS: [&[u8]; 9] = [
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

/// Isset operands and coalescing left-side chains (level 1): the expressions whose type the check reads,
/// and the variables and fetches whose certainty it reads.
pub(in super::super) fn coalesce_types(
    program: &Program,
    level: u8,
) -> (
    Vec<Span>,
    Vec<Span>,
    Vec<(Span, PropertyInitializationRequest)>,
) {
    let mut chains = Chains::default();
    if level >= 1 {
        chains.walk_program(program, &mut ());
    }
    (chains.typed, chains.watched, chains.initializations)
}

#[derive(Default)]
struct Chains {
    typed: Vec<Span>,
    watched: Vec<Span>,
    initializations: Vec<(Span, PropertyInitializationRequest)>,
}

impl Chains {
    fn chain(&mut self, e: &Expression, assignment: Option<Span>) {
        let e = unparen(e);
        self.typed.push(e.span());
        match e {
            Expression::Variable(Variable::Direct(_)) => self.watched.push(e.span()),
            Expression::ArrayAccess(a) => {
                self.watched.push(e.span());
                self.typed.push(a.index.span());
                self.chain(a.array, assignment);
            }
            Expression::Access(Access::Property(p)) => {
                self.watched.push(e.span());
                self.typed.extend(dynamic_name(&p.property));
                // IssetabilityDescriptor reads the virtual marker only for literal `$this->name`.
                // Aliased ThisType writes still create it in the live analyzer, not here.
                if let ClassLikeMemberSelector::Identifier(name) = &p.property
                    && matches!(unparen(p.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
                {
                    // `??=` visits the assignment, not its read target. Its before-position
                    // is the rule's incoming scope; never substitute the post-write target type.
                    self.initializations.push((
                        e.span(),
                        PropertyInitializationRequest {
                            expression: assignment.unwrap_or_else(|| e.span()),
                            property: mago_word::word(name.value),
                        },
                    ));
                }
                self.chain(p.object, assignment);
            }
            Expression::Access(Access::NullSafeProperty(p)) => self.chain(p.object, assignment),
            Expression::Access(Access::StaticProperty(p)) => self.chain(p.class, assignment),
            _ => {}
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Chains {
    fn walk_in_isset_construct(&mut self, i: &'ast IssetConstruct<'arena>, (): &mut ()) {
        for value in &i.values {
            self.chain(value, None);
        }
    }

    fn walk_in_empty_construct(&mut self, e: &'ast EmptyConstruct<'arena>, (): &mut ()) {
        self.chain(e.value, None);
    }

    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, (): &mut ()) {
        if b.operator.is_null_coalesce() {
            self.chain(b.lhs, None);
        }
    }

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, (): &mut ()) {
        if matches!(a.operator, AssignmentOperator::Coalesce(_)) {
            self.chain(a.lhs, Some(a.span()));
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
            advanced: c.on(crate::rules::Switches::CHECK_ADVANCED_ISSET),
            message: type_message,
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

/// EmptyRule on the operand of `empty()`.
pub(in super::super) fn empty(c: &mut Checker, construct: &EmptyConstruct) {
    if c.level < 1 {
        return;
    }
    let check = IssetCheck {
        c,
        operator: "in empty()".to_string(),
        identifier: "empty",
        advanced: c.on(crate::rules::Switches::CHECK_ADVANCED_ISSET),
        message: empty_message,
    };
    match check.check(construct.value, None) {
        Some((identifier, message)) => {
            // Errors without an explicit line inherit the EmptyExpressionNode's line upstream.
            let line = c.line_of(construct.span());
            c.condition_error(
                construct.value.span(),
                line,
                &identifier,
                true,
                message,
                None,
            );
        }
        None => c.no_condition(construct.value.span(), "empty.expr"),
    }
}

/// NullCoalesceRule on `??` (`lhs` of a binary) and `??=` (the target of an assignment).
pub(in super::super) fn coalesce(
    c: &mut Checker,
    left: &Expression,
    right: &Expression,
    operator: &str,
) {
    if c.level < 1 {
        return;
    }
    let check = IssetCheck {
        c,
        operator: format!("on left side of {operator}"),
        identifier: "nullCoalesce",
        advanced: c.on(crate::rules::Switches::CHECK_ADVANCED_ISSET),
        message: type_message,
    };
    // in a trait, per class context (ConstantConditionInTraitHelper)
    let found = check.check(left, None).or_else(|| {
        // checkUnnecessaryNullCoalesce() (bleeding edge)
        // ponytail: a right side other than `null` itself (pure, without writes) is not considered
        let null = matches!(unparen(right), Expression::Literal(Literal::Null(_)))
            || matches!(unparen(right), Expression::ConstantAccess(a) if a.name.value().eq_ignore_ascii_case(b"null"));
        (c.on(crate::rules::Switches::BLEEDING_EDGE) && null && check.is_set(left)).then(|| {
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
    /// The rule's `$typeMessageCallback`.
    message: fn(&Type) -> Option<&'static str>,
}

/// The declaring class and metadata of a property a fetch finds (`findPropertyReflectionFromNode()`).
struct Property<'a> {
    declaring: &'a ClassLikeMetadata,
    metadata: &'a PropertyMetadata,
    holder: Type,
}

/// The expression whose constant strings PropertyReflectionFinder reads for an instance property name.
fn dynamic_name(name: &ClassLikeMemberSelector) -> Option<Span> {
    match name {
        ClassLikeMemberSelector::Variable(v) => Some(v.span()),
        ClassLikeMemberSelector::Expression(e) => Some(e.expression.span()),
        _ => None,
    }
}

/// NullCoalesceRule's message for the type on the left side.
fn type_message(t: &Type) -> Option<&'static str> {
    match t.is_null() {
        Tri::Maybe => None,
        Tri::Yes => Some("is always null"),
        Tri::No => Some("is not nullable"),
    }
}

/// EmptyRule's message for the type of its operand.
fn empty_message(t: &Type) -> Option<&'static str> {
    let is_null = t.is_null();
    let is_falsey = falsy(t);
    if is_null.maybe() || is_falsey.maybe() {
        return None;
    }
    Some(if is_falsey.yes() {
        "is always falsy"
    } else {
        "is not falsy"
    })
}

/// `toBoolean()->isFalse()`.
fn falsy(t: &Type) -> Tri {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => Tri::extreme_identity(ts.iter().map(falsy)),
        // intersectTypes(): a constant member decides
        Type::Intersection(ts) => ts
            .iter()
            .map(falsy)
            .find(|r| !r.maybe())
            .unwrap_or(Tri::Maybe),
        Type::Template(t) => falsy(&t.bound),
        // MixedType::toBoolean(): a mixed without any falsey value
        Type::SubtractedMixed { excluded, .. } => {
            let falsey = union(vec![
                Type::Null,
                Type::ConstBool(false),
                Type::ConstInt(0),
                Type::ConstFloat(0.0),
                Type::ConstString(Vec::new()),
                Type::ConstString(b"0".to_vec()),
                Type::ConstArray(Box::new(ConstArray::empty())),
            ]);
            if is_super_type_of(excluded, &falsey).yes() {
                Tri::No
            } else {
                Tri::Maybe
            }
        }
        Type::Null => Tri::Yes,
        Type::ConstBool(b) => Tri::from(!b),
        Type::ConstInt(i) => Tri::from(*i == 0),
        Type::ConstFloat(f) => Tri::from(*f == 0.0),
        Type::ConstString(s) => Tri::from(s.is_empty() || s == b"0"),
        // (a range of only 0 is the constant 0)
        Type::IntRange(min, max) => {
            if min.is_none_or(|m| m <= 0) && max.is_none_or(|m| m >= 0) {
                Tri::Maybe
            } else {
                Tri::No
            }
        }
        // ConstantArrayType::getArraySize()
        Type::ConstArray(a) if a.keys.len() > a.optional.len() => Tri::No,
        Type::ConstArray(a) => {
            if a.keys.is_empty() && a.unsealed.is_none() {
                Tri::Yes
            } else {
                Tri::Maybe
            }
        }
        Type::Accessory(
            Accessory::NonFalsyString | Accessory::NonEmptyArray | Accessory::HasOffsetValue(..),
        ) => Tri::No,
        // ObjectType::toBoolean()
        Type::Object(name, ..) => {
            let undecided = ["SimpleXMLElement", "BcMath\\Number", "GMP"]
                .iter()
                .any(|n| n.eq_ignore_ascii_case(name));
            if undecided { Tri::Maybe } else { Tri::No }
        }
        Type::SubtractedObject { object, .. } => falsy(object),
        Type::ObjectWithoutClass
        | Type::Static(..)
        | Type::This(_)
        | Type::Closure(_)
        | Type::Callable(_) => Tri::No,
        Type::ObjectShape(_) | Type::EnumCase(..) | Type::Resource => Tri::No,
        _ => Tri::Maybe,
    }
}

/// `hasOffsetValueType()`: arrays coerce their keys, while strings inspect the raw offset type.
fn has_offset(t: &Type, offset: &Type) -> Tri {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => {
            let all: Vec<Tri> = ts.iter().map(|t| has_offset(t, offset)).collect();
            if all.iter().all(|r| *r == all[0]) {
                all[0]
            } else {
                Tri::Maybe
            }
        }
        Type::Intersection(ts) => {
            // a non-empty list has offset 0
            if t.is_list().yes()
                && t.is_iterable_at_least_once().yes()
                && array_key(offset) == Type::ConstInt(0)
            {
                return Tri::Yes;
            }
            ts.iter()
                .map(|t| has_offset(t, offset))
                .fold(Tri::Maybe, |a, b| {
                    if a.yes() || b.yes() {
                        Tri::Yes
                    } else if b.no() {
                        Tri::No
                    } else {
                        a
                    }
                })
        }
        Type::ConstArray(a) => has_array_offset(a, &array_key(offset)),
        Type::Array(key, _) => {
            let offset = &array_key(offset);
            if is_super_type_of(key, offset).no()
                && (offset.is_string().no() || is_constant(offset))
            {
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
        _ if t.is_string().yes() => {
            if is_super_type_of(&Type::Int, offset).no() {
                Tri::No
            } else {
                Tri::Maybe
            }
        }
        Type::Null => Tri::No,
        _ => {
            if offset_accessible(t).no() {
                Tri::No
            } else {
                Tri::Maybe
            }
        }
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
        let all: Vec<Tri> = alternatives
            .iter()
            .map(|offset| has_array_offset(a, offset))
            .collect();
        return if all.iter().all(|r| *r == all[0]) {
            all[0]
        } else {
            Tri::Maybe
        };
    }
    let mut result = Tri::No;
    for (i, key) in a.keys.iter().enumerate() {
        if matches!(key, Type::ConstInt(_))
            && !offset.is_string().no()
            && !is_constant(offset)
            && !offset.is_decimal_int_string().no()
        {
            return Tri::Maybe;
        }
        let has = is_super_type_of(key, offset);
        if has.yes() {
            return if a.optional.contains(&i) {
                Tri::Maybe
            } else {
                Tri::Yes
            };
        }
        result = if has.maybe() { Tri::Maybe } else { result };
    }
    let unsealed = a
        .unsealed
        .as_ref()
        .is_some_and(|(k, _)| !is_super_type_of(k, offset).no());
    if result.no() && unsealed {
        Tri::Maybe
    } else {
        result
    }
}

/// `getOffsetValueType()` of an offset `t` has.
fn offset_value(t: &Type, offset: &Type) -> Type {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => union(
            ts.iter()
                .filter(|t| !has_offset(t, offset).no())
                .map(|t| offset_value(t, offset))
                .collect(),
        ),
        Type::ConstArray(a) => {
            let offset = array_key(offset);
            // getOffsetValueType() includes only matching slots, not unrelated nullable fields.
            let mut values: Vec<Type> = a
                .keys
                .iter()
                .zip(&a.values)
                .filter(|(key, _)| !is_super_type_of(key, &offset).no())
                .map(|(_, value)| value.clone())
                .collect();
            if let Some((key, value)) = &a.unsealed
                && !is_super_type_of(&union(a.keys.clone()), &offset).yes()
                && !is_super_type_of(key, &offset).no()
            {
                values.push(value.clone());
            }
            // A nonconstant string can hit integer keys without a definite type match.
            if values.is_empty() {
                t.iterable_value_type()
            } else {
                union(values)
            }
        }
        t if t.is_string().yes() => Type::String,
        t => t.iterable_value_type(),
    }
}

fn is_constant(t: &Type) -> bool {
    matches!(
        t,
        Type::ConstInt(_)
            | Type::ConstString(_)
            | Type::ConstBool(_)
            | Type::ConstFloat(_)
            | Type::Null
    )
}

/// The scalar and refined-string cases of `toArrayKey()`, preserving template identity.
fn array_key(t: &Type) -> Type {
    match t {
        Type::Union(ts) => union(ts.iter().map(array_key).collect()),
        Type::Accessory(_) | Type::Intersection(_) if t.is_decimal_int_string().yes() => Type::Int,
        Type::Accessory(_) | Type::Intersection(_) if t.is_numeric_string().yes() => {
            union(vec![Type::Int, t.clone()])
        }
        Type::ConstString(s)
            if std::str::from_utf8(s)
                .ok()
                .and_then(|s| s.parse::<i64>().ok())
                .is_some_and(|i| i.to_string().as_bytes() == s) =>
        {
            Type::ConstInt(std::str::from_utf8(s).unwrap().parse().unwrap_or(0))
        }
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
        self.ty_from(
            e,
            !self
                .c
                .on(crate::rules::Switches::TREAT_PHP_DOC_TYPES_AS_CERTAIN),
        )
    }

    fn ty_from(&self, e: &Expression, native: bool) -> Option<Type> {
        let e = unparen(e);
        if native {
            // Only the operand's live read is evidence. Never recover a missing read from full types,
            // source PHPDoc metadata, a later scope, or another evaluation of the expression.
            return self.c.phpstan_type_in(e, &self.c.native_types);
        }
        match e {
            // (Mago types a property fetch on `$this`, not `$this`)
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => Some(Type::This(
                String::from_utf8_lossy(self.c.class_scope()?.original_name.as_bytes())
                    .into_owned(),
            )),
            e => self.c.phpstan_type(e),
        }
    }

    /// The type of an offset's holder: an offset it does not have is an ErrorType (Mago types it null).
    fn holder_type(&self, e: &Expression, native: bool) -> Option<Type> {
        if let Expression::ArrayAccess(a) = unparen(e)
            && has_offset(
                &self.holder_type(a.array, native)?,
                &self.ty_from(a.index, native)?,
            )
            .no()
        {
            return Some(Type::Error);
        }
        self.ty_from(e, native)
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
        let tail = (self.message)(t)?;
        Some((
            format!("{}.{second}", self.identifier),
            format!("{message} {tail}."),
        ))
    }

    fn never_defined(&self, v: &DirectVariable) -> Found {
        let name = String::from_utf8_lossy(v.name);
        Some((
            format!("{}.variable", self.identifier),
            format!("Variable {name} {} is never defined.", self.operator),
        ))
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
        let (class, name, holder) = match e {
            Expression::Access(Access::Property(p)) => {
                let name = match &p.property {
                    ClassLikeMemberSelector::Identifier(name) => name.value.to_vec(),
                    selector => {
                        let names = self
                            .c
                            .phpstan_type_at(dynamic_name(selector)?)?
                            .constant_strings();
                        let [name] = names.as_slice() else {
                            return None;
                        };
                        name.clone()
                    }
                };
                // IssetabilityDescriptor's reflection resolver keeps the ordinary scope, including
                // PHPDoc template arguments and singleton dynamic names, even for a native resolution.
                let holder = self.ty_from(p.object, false)?;
                if matches!(
                    holder.resolve_to_bounds(),
                    Type::Union(_) | Type::Benevolent(_)
                ) {
                    return None;
                }
                let names = holder.object_class_names();
                let [class] = names.as_slice() else {
                    return None;
                };
                (
                    self.c.codebase.get_class_like(class.as_bytes()),
                    name,
                    holder,
                )
            }
            // (a static fetch: the callers pass property fetches only)
            _ => {
                let Expression::Access(Access::StaticProperty(p)) = e else {
                    return None;
                };
                let Variable::Direct(v) = &p.property else {
                    return None;
                };
                let class = match unparen(p.class) {
                    Expression::Identifier(id) => {
                        self.c.codebase.get_class_like(self.c.names.get(id))
                    }
                    Expression::Self_(_) | Expression::Static(_) => self.c.class_scope(),
                    Expression::Parent(_) => self
                        .c
                        .class_scope()
                        .and_then(|k| k.direct_parent_class.as_ref())
                        .and_then(|p| self.c.codebase.get_class_like(p.as_bytes())),
                    class => {
                        let names = self.ty_from(class, false)?.object_class_names();
                        let [class] = names.as_slice() else {
                            return None;
                        };
                        self.c.codebase.get_class_like(class.as_bytes())
                    }
                }?;
                let holder = Self::holder(e)
                    .and_then(|h| self.ty_from(h, false))
                    .unwrap_or_else(|| Type::Object(self.c.display(class), vec![], false));
                (Some(class), v.name[1..].to_vec(), holder)
            }
        };
        let mut variable = b"$".to_vec();
        variable.extend_from_slice(&name);
        let declaring = self
            .c
            .codebase
            .get_declaring_property_class(class?.name.as_bytes(), &variable)?;
        let declaring = self.c.codebase.get_class_like(declaring.as_bytes())?;
        let metadata = declaring.properties.get(&mago_word::word(&variable))?;
        let is_static = matches!(e, Expression::Access(Access::StaticProperty(_)));
        (metadata.flags.is_static() == is_static).then_some(Property {
            declaring,
            metadata,
            holder,
        })
    }

    /// PhpPropertyReflection::getWritableType(): a set hook's parameter takes precedence over the property's
    /// readable type, including when the hook is inherited.
    fn writable_type(&self, p: &Property) -> Type {
        let name = &p.metadata.name.0.as_bytes()[1..];
        let mut property = Some((p.declaring, p.metadata));
        for _ in 0..super::super::overriding::ANCESTRY {
            let Some((declaring, metadata)) = property else {
                break;
            };
            if let Some(parameter) = metadata
                .hooks
                .get(&mago_word::word("set"))
                .and_then(|h| h.parameter.as_ref())
            {
                let native = parameter
                    .type_declaration_metadata
                    .as_ref()
                    .filter(|t| !t.from_docblock)
                    .or(metadata.type_declaration_metadata.as_ref());
                let native = native.map_or_else(pun_phpstan::types::mixed, |t| {
                    declared::from_tunion(self.c, &t.type_union, declaring)
                });
                let phpdoc = declared::property_span(self.c, metadata)
                    .map(|at| declared::property(self.c, None, at, name, declaring))
                    .filter(|t| !t.is_implicit_mixed());
                return pun_phpstan::types::decide_type(native, phpdoc);
            }
            property = declaring
                .direct_parent_class
                .and_then(|n| self.c.codebase.get_class_like(n.as_bytes()))
                .and_then(|k| self.c.declared_property(k, p.metadata.name.0.as_bytes()))
                .map(|(_, origin, metadata)| (origin, metadata));
        }
        declared::property_of(self.c, p.declaring, p.metadata, name)
    }

    /// A reflected declaring class's active template map, not the unparameterized storage declaration.
    fn property_args(&self, p: &Property) -> Vec<Type> {
        let holder = p.holder.resolve_to_bounds();
        let actual = match &holder {
            Type::Object(_, args, _) | Type::Static(_, args) => args.clone(),
            Type::This(name) => self
                .c
                .codebase
                .get_class_like(name.as_bytes())
                .into_iter()
                .flat_map(|k| declared::class_templates(self.c, k))
                .map(|t| Type::Template(Box::new(t)))
                .collect(),
            _ => vec![],
        };
        let class = declared::original_name(p.declaring);
        let names = p.holder.object_class_names();
        let args = declared::with_classes(self.c, || {
            let name = &names[0]; // property() requires exactly one class name.
            if name.eq_ignore_ascii_case(&class) {
                actual
            } else {
                pun_phpstan::symbols::lookup(name)
                    .and_then(|k| pun_phpstan::types::ancestor_args(&k, &class, &actual))
                    .unwrap_or_default()
            }
        });
        declared::class_templates(self.c, p.declaring)
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                args.get(i)
                    .cloned()
                    .unwrap_or_else(|| t.default.as_ref().unwrap_or(&t.bound).clone())
            })
            .collect()
    }

    /// PropertyDescriptor::describeProperty().
    fn describe(&self, p: &Property, e: &Expression) -> String {
        let mut declaring = self.c.display(p.declaring);
        let args = self.property_args(p);
        if !args.is_empty() {
            declaring = format!(
                "{declaring}<{}>",
                args.iter()
                    .map(|t| t.describe(Level::TypeOnly))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
        match e {
            Expression::Access(Access::Property(fetch)) => {
                let name = match &fetch.property {
                    ClassLikeMemberSelector::Identifier(name) => {
                        String::from_utf8_lossy(name.value).into_owned()
                    }
                    ClassLikeMemberSelector::Variable(v) => {
                        self.c.written(v.span())[1..].to_string()
                    }
                    selector => {
                        if let ClassLikeMemberSelector::Expression(e) = selector
                            && let Expression::Variable(Variable::Direct(v)) = unparen(e.expression)
                        {
                            String::from_utf8_lossy(&v.name[1..]).into_owned()
                        } else {
                            String::new()
                        }
                    }
                };
                let on = self
                    .ty_from(fetch.object, false)
                    .expect("a found property's holder is typed");
                let nominal = declared::with_classes(self.c, || {
                    is_super_type_of(
                        &Type::Object(self.c.display(p.declaring), vec![], false),
                        &on,
                    )
                    .yes()
                });
                let class = if nominal {
                    declaring
                } else {
                    on.describe(Level::TypeOnly)
                };
                format!("Property {class}::${name}")
            }
            _ => format!(
                "Static property {declaring}::{}",
                String::from_utf8_lossy(p.metadata.name.0.as_bytes())
            ),
        }
    }

    /// The expression a property fetch reads on (its object, or a static fetch's class expression).
    fn holder<'e, 'arena>(e: &'e Expression<'arena>) -> Option<&'e Expression<'arena>> {
        match e {
            Expression::Access(Access::Property(p)) => Some(p.object),
            Expression::Access(Access::StaticProperty(p))
                if !matches!(
                    unparen(p.class),
                    Expression::Identifier(_)
                        | Expression::Self_(_)
                        | Expression::Static(_)
                        | Expression::Parent(_)
                ) =>
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
                    if !matches!(t, Type::Never(_)) {
                        return self.error(
                            &t,
                            format!("Variable {name} {} always exists and", self.operator),
                            "variable",
                        );
                    }
                }
                self.never_defined(v)
            }
            Expression::ArrayAccess(a) => {
                let holder = self.holder_type(
                    a.array,
                    !self
                        .c
                        .on(crate::rules::Switches::TREAT_PHP_DOC_TYPES_AS_CERTAIN),
                )?;
                if !offset_accessible(&holder).yes() {
                    return error.or_else(|| self.undefined(a.array));
                }
                let offset = self.ty(a.index)?;
                let has = has_offset(&holder, &offset);
                if has.no() {
                    return self
                        .advanced
                        .then(|| self.no_offset(&holder, &offset))
                        .flatten();
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
                let Some(p) = self.property(e).filter(|p| {
                    !p.declaring
                        .magic_properties
                        .contains_key(&p.metadata.name.0)
                }) else {
                    return undefined();
                };
                let m = p.metadata;
                // isInitializedThisProperty()
                if let Some(native) = &m.type_declaration_metadata
                    && !m.flags.is_virtual_property()
                    && self.c.property_initializations.get(&e.span())
                        == Some(&PropertyInitializationCertainty::Yes)
                {
                    let t = declared::from_tunion(self.c, &native.type_union, p.declaring);
                    let tail = match (self.message)(&t)? {
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
                let names = declared::class_templates(self.c, p.declaring)
                    .into_iter()
                    .map(|t| t.name)
                    .collect::<Vec<_>>();
                let t = super::super::types::with_args(
                    &self.writable_type(&p),
                    &format!("class {}", declared::original_name(p.declaring)),
                    &names,
                    &self.property_args(&p),
                );
                let message = format!(
                    "{} ({}) {}",
                    self.describe(&p, e),
                    t.describe(Level::TypeOnly),
                    self.operator
                );
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
                                || matches!(
                                    holder,
                                    Type::Int
                                        | Type::ConstInt(_)
                                        | Type::IntRange(..)
                                        | Type::Float
                                        | Type::ConstFloat(_)
                                        | Type::Bool
                                        | Type::ConstBool(_)
                                )
                                || self.undefined_property(&holder, &p.property)
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
                let Expression::Access(Access::NullSafeProperty(p)) = e else {
                    return None;
                };
                let name = match &p.property {
                    ClassLikeMemberSelector::Identifier(name) => {
                        String::from_utf8_lossy(name.value).into_owned()
                    }
                    _ => "(Expression)".to_string(),
                };
                Some((
                    "nullsafe.neverNull".to_string(),
                    format!(
                        "Using nullsafe property access \"?->{name}\" {} is unnecessary. Use -> instead.",
                        self.operator
                    ),
                ))
            }
        }
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
            Expression::Variable(Variable::Direct(v)) => {
                self.defined(v).yes() && v.name != b"$_SESSION"
            }
            Expression::ArrayAccess(a) => {
                // NullCoalesceRule::checkUnnecessaryNullCoalesce explicitly requests full types.
                let (Some(holder), Some(offset)) = (
                    self.holder_type(a.array, false),
                    self.ty_from(a.index, false),
                ) else {
                    return false;
                };
                offset_accessible(&holder).yes()
                    && has_offset(&holder, &offset).yes()
                    && self.is_set(a.array)
            }
            Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
                let Some(p) = self.property(e).filter(|p| {
                    !p.declaring
                        .magic_properties
                        .contains_key(&p.metadata.name.0)
                }) else {
                    return false;
                };
                !self.maybe_uninitialized(&p, e) && Self::holder(e).is_none_or(|h| self.is_set(h))
            }
            _ => true,
        }
    }

    /// A fetch the holder's class has no property for, declared, `@property` or `__get()`: PHPStan's ErrorType.
    fn undefined_property(&self, holder: &Type, name: &ClassLikeMemberSelector) -> bool {
        let ClassLikeMemberSelector::Identifier(name) = name else {
            return false;
        };
        let names = holder.object_class_names();
        let [class] = names.as_slice() else {
            return false;
        };
        let Some(class) = self.c.codebase.get_class_like(class.as_bytes()) else {
            return false;
        };
        let mut variable = b"$".to_vec();
        variable.extend_from_slice(name.value);
        self.c
            .codebase
            .get_declaring_property_class(class.name.as_bytes(), &variable)
            .is_none()
            && !class
                .magic_properties
                .contains_key(&mago_word::word(&variable))
            && !self
                .c
                .codebase
                .method_exists(class.name.as_bytes(), b"__get")
    }

    /// IssetCheck::checkUndefined().
    fn undefined(&self, e: &Expression) -> Found {
        let e = unparen(e);
        match e {
            Expression::Variable(Variable::Direct(v)) => {
                if self.defined(v).no() {
                    self.never_defined(v)
                } else {
                    None
                }
            }
            Expression::ArrayAccess(a) => {
                let holder = self.holder_type(
                    a.array,
                    !self
                        .c
                        .on(crate::rules::Switches::TREAT_PHP_DOC_TYPES_AS_CERTAIN),
                )?;
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
