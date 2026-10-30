//! `lint/php-8.5/no-null-array-key`, ported from Rector 2.6.1's `ArrayKeyExistsNullToEmptyStringRector`
//! (rules/Php85/Rector/FuncCall) and the `NullToStrictStringIntConverter` it calls (rules/Php81/NodeManipulator).
//!
//! PHP 8.5 deprecates `null` as the key of `array_key_exists()`. A `null` key becomes `''`; a key that may be `null`
//! becomes `(string) $key`, or the `else` of a ternary key gets the cast. Rector asks PHPStan for the key's type
//! (`getType`, see [`super::types::rector_type`]) and native type (`getNativeType`), both from Mago here
//! ([`Facts`]), and skips a key whose type is no `mixed`, `null` or union of `string`, `int` and `null`.
//!
//! What Mago does not keep is read from the code:
//! - PHPStan's explicit `mixed` (a `mixed` someone declared) against the implicit one (a missing type): a variable is
//!   explicit when the last assignment to it before the key, in its function, assigns an explicit value, or else
//!   when it is a parameter declared `mixed` (of an arrow function, then of the functions around it); a call is
//!   explicit when the function or method it calls declares a native `mixed` return. Property fetches are implicit,
//!   as PHPStan's native scope gives them after a `!== null` (checked on the oracle).
//! - `Scope::getParentScope()`, which only a closure's scope has (an arrow function keeps the one around it, checked
//!   on the oracle): a variable is an `ErrorType` in the scope around the closure when that scope is a function's and
//!   the variable is not its parameter, not in its `use`, and not written or read there before the closure.
//! - PHPStan's `ErrorType` of a property fetch Mago types otherwise: a property a class of the object's type does not
//!   declare (a `__get` does not count), and a static property of a class PHPStan does not
//!   know or that the class around (`self::`, `static::`) does not declare.
//!
//! `shouldSkipValue`'s `InterpolatedString` check is left out: such a string's type is `string`, which skips it first.
//! A key that is unpacked (`...$keys`) is left alone, where Rector would cast the array.
//!
//! `isValidUnionType` of the parameter `NullToStrictStringIntConverter` reads never refuses here: `array_key_exists`'s
//! `$key` and `$array` are no native union of other types.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::mixed::TMixed;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, Argument, Call, ClassLikeMemberSelector, CompositeString, Expression,
    FunctionLikeParameterList, Hint, Node, NodeKind, NullSafePropertyAccess, PropertyAccess,
    UnaryPrefixOperator, Variable,
};
use pun_phpstan::types::{Tri, Type, sort_types};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::{unpacked, value};
use super::phpdoc_writer::phpstan_type;
use super::rector_code_quality::{Prec, args, is_constant, named, prec, text};
use super::types::facts::{Facts, facts};
use super::types::names::resolve_class_name;
use super::types::rector_type::is_union;
use super::types::{ClassScope, class_scope, no_nullsafe_on_non_null::callee};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNullArrayKey;

impl Rule for NoNullArrayKey {
    fn id(&self) -> &'static str {
        "lint/php-8.5/no-null-array-key"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATE_NULL_ARG_IN_ARRAY_KEY_EXISTS_FUNCTION
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "array_key_exists") {
            return;
        }
        let args = args(c);
        if args.len() != 2 {
            return;
        }
        // `ArgsAnalyzer::resolveArgPosition($args, 'key', 0)`
        let position = (args.iter())
            .position(
                |a| matches!(a, Argument::Named(n) if n.name.value.eq_ignore_ascii_case(b"key")),
            )
            .unwrap_or(0);
        let arg = args[position];
        let key = unparenthesize(value(arg));
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let is_trait = ancestors.iter().any(|a| matches!(a, Node::Trait(_)));
        let at = At {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
            // inside a trait PHPStan does not know `$this`; in code Mago's analysis does not reach (after a `return`)
            // PHPStan still types each value: both give an implicit `mixed` where Mago gives no type
            untyped_mixed: is_trait || facts.native_type(node_span(c)).is_none(),
            is_trait,
        };
        let original = at.get_type(key);
        if original.is_union() && !original.has_null() {
            return;
        }
        let Some(new) = at.convert_if_null(ctx, key, unpacked(arg)) else {
            return;
        };
        out.push(Report {
            span: node_span(key),
            message: "PHP 8.5 deprecates a `null` key in `array_key_exists()`: pass a string."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![new],
            }),
        });
    }
}

/// Where the call is: its file's facts, its ancestors and the class-like around it.
struct At<'a, 'ast, 'arena> {
    facts: &'a Facts,
    ancestors: &'a [Node<'ast, 'arena>],
    scope: Option<&'a ClassScope>,
    is_trait: bool,
    untyped_mixed: bool,
}

/// A type as `NullToStrictStringIntConverter` reads it: the members of a union Rector builds itself, or PHPStan's.
enum Ty<'f> {
    Built(Vec<Option<TUnion>>),
    Scope(Option<&'f TUnion>),
}

impl Ty<'_> {
    fn members(&self) -> Vec<Option<TUnion>> {
        match self {
            Ty::Built(members) => members.clone(),
            Ty::Scope(t) => vec![t.cloned()],
        }
    }

    /// `instanceof UnionType`.
    fn is_union(&self) -> bool {
        match self {
            Ty::Built(_) => true,
            Ty::Scope(t) => t.is_some_and(is_union),
        }
    }

    /// Whether `TypeCombinator::removeNull()` changes the type.
    fn has_null(&self) -> bool {
        (self.members().into_iter().flatten())
            .any(|t| t.types.iter().any(|a| matches!(a, TAtomic::Null)))
    }

    /// `instanceof MixedType`, with the `TMixed`.
    fn mixed(&self) -> Option<&TMixed> {
        match self {
            Ty::Scope(Some(t)) => match t.types.as_ref() {
                [TAtomic::Mixed(m)] => Some(m),
                _ => None,
            },
            _ => None,
        }
    }

    /// `isString()->yes()`.
    fn is_string(&self) -> bool {
        self.all(|t| t.is_string())
    }

    /// `isNull()->yes()`.
    fn is_null(&self) -> bool {
        self.all(|t| t.is_null())
    }

    /// `isValidUnionType`: a union of only strings, integers and `null`.
    fn is_valid_union(&self) -> bool {
        self.is_union()
            && self.all(|t| {
                let valid =
                    |m: &Type| m.is_string().yes() || m.is_integer().yes() || m.is_null().yes();
                Tri::from(match t {
                    Type::Union(types) | Type::Benevolent(types) => types.iter().all(valid),
                    t => valid(t),
                })
            })
    }

    /// Whether `check` says yes of every member (PHPStan's answer on a union).
    fn all(&self, check: impl Fn(&Type) -> Tri) -> bool {
        (self.members().iter()).all(|t| {
            t.as_ref()
                .and_then(phpstan_type)
                .is_some_and(|t| check(&t).yes())
        })
    }
}

impl<'a, 'ast, 'arena> At<'a, 'ast, 'arena> {
    /// `NodeTypeResolver::getType`.
    fn get_type(&self, expr: &Expression<'_>) -> Ty<'a> {
        let expr = unparenthesize(expr);
        let single = |t: &Ty<'a>| !t.is_union();
        let not_null = |t: &Ty<'a>| !t.is_union() && !t.is_null();
        match expr {
            Expression::Conditional(c) => {
                if let Some(then) = c.then {
                    let (first, second) = (self.get_type(then), self.get_type(c.r#else));
                    if single(&first) && not_null(&second) {
                        return Ty::Built([first.members(), second.members()].concat());
                    }
                }
                // a nullable condition: the first member of its union, in PHPStan's order (`UnionTypeHelper::sortTypes`,
                // `null` last)
                if let Ty::Scope(Some(cond)) = self.get_type(c.condition)
                    && is_union(cond)
                    && cond.types.iter().any(|a| matches!(a, TAtomic::Null))
                {
                    let second = self.get_type(c.r#else);
                    if not_null(&second) {
                        let members: Vec<(Type, &TAtomic)> = (cond.types.iter())
                            .filter_map(|a| {
                                phpstan_type(&TUnion::from_atomic(a.clone())).map(|t| (t, a))
                            })
                            .collect();
                        let sorted =
                            sort_types(&members.iter().map(|(t, _)| t.clone()).collect::<Vec<_>>());
                        let first = (members.iter().find(|(t, _)| sorted.first() == Some(t)))
                            .map_or(&cond.types[0], |(_, a)| *a);
                        let first = TUnion::from_atomic(first.clone());
                        return Ty::Built([vec![Some(first)], second.members()].concat());
                    }
                }
            }
            Expression::Binary(b) if b.operator.is_null_coalesce() => {
                let (first, second) = (self.get_type(b.lhs), self.get_type(b.rhs));
                // PHPStan types the left side without `null` there
                let left_single = first.members().into_iter().flatten().all(|t| {
                    t.types
                        .iter()
                        .filter(|a| !matches!(a, TAtomic::Null))
                        .count()
                        <= 1
                }) && !matches!(first, Ty::Built(_));
                if left_single && not_null(&second) {
                    return Ty::Built([first.members(), second.members()].concat());
                }
            }
            _ => {}
        }
        Ty::Scope(self.scope_type(expr))
    }

    /// PHPStan's type of `expr`, an implicit `mixed` where Mago gives none in a trait or in code it does not reach.
    fn scope_type(&self, expr: &Expression<'_>) -> Option<&'a TUnion> {
        if matches!(
            expr,
            Expression::CompositeString(CompositeString::ShellExecute(_))
        ) {
            return Some(&*MIXED);
        }
        self.facts
            .full_type(node_span(expr))
            .or(self.untyped_mixed.then_some(&*MIXED))
    }

    /// `NodeTypeResolver::getNativeType`.
    fn native_type(&self, expr: &Expression<'_>) -> Ty<'a> {
        if matches!(
            expr,
            Expression::CompositeString(CompositeString::ShellExecute(_))
        ) {
            return Ty::Scope(Some(&*MIXED));
        }
        Ty::Scope(
            (self.facts.native_type(node_span(expr))).or(self.untyped_mixed.then_some(&*MIXED)),
        )
    }

    /// `convertIfNull`, the edit it makes.
    fn convert_if_null(
        &self,
        ctx: &Context<'_>,
        key: &Expression<'_>,
        unpacked: bool,
    ) -> Option<Edit> {
        if unpacked {
            return None;
        }
        if is_constant(key, "null") {
            return Some(Edit::new(node_span(key), "''"));
        }
        if self.is_string_casted_ternary_of_mixed_types(key) {
            return None;
        }
        if self.should_skip_value(key) {
            return None;
        }
        if let Expression::Conditional(c) = key {
            let r#else = unparenthesize(c.r#else);
            if !self.should_skip_value(r#else) {
                let new = if is_constant(r#else, "null") {
                    "''".to_string()
                } else {
                    cast(ctx, r#else)
                };
                return Some(Edit::new(node_span(r#else), new));
            }
        }
        Some(Edit::new(node_span(key), cast(ctx, key)))
    }

    /// `shouldSkipValue` for the target type `string`.
    fn should_skip_value(&self, expr: &Expression<'_>) -> bool {
        // (php-parser's `ArrayDimFetch`)
        if matches!(
            expr,
            Expression::ArrayAccess(_) | Expression::ArrayAppend(_)
        ) {
            return true;
        }
        if self.is_property_fetch_on_class_with_magic_get(expr) {
            return true;
        }
        let r#type = self.get_type(expr);
        if r#type.is_string() {
            return true;
        }
        let native = self.native_type(expr);
        if native.is_string() {
            return true;
        }
        if self.is_possible_array_variable_name(&r#type, expr) {
            return true;
        }
        if should_skip_type(&r#type) {
            return true;
        }
        if self.is_an_error_type(expr, &native) {
            return true;
        }
        self.should_skip_trait(expr, &r#type)
    }

    /// `isPropertyFetchOnClassWithMagicGet`: `$object->name` where `$object` is one class (not `$this`), which has a
    /// `__get` method.
    fn is_property_fetch_on_class_with_magic_get(&self, expr: &Expression<'_>) -> bool {
        let Expression::Access(Access::Property(fetch)) = expr else {
            return false;
        };
        let Ty::Scope(Some(t)) = self.get_type(fetch.object) else {
            return false;
        };
        let [TAtomic::Object(TObject::Named(named))] = t.types.as_ref() else {
            return false;
        };
        !named.is_this
            && named.intersection_types.is_none()
            && (self
                .facts
                .codebase
                .get_declaring_method(named.name.as_bytes(), b"__get"))
            .is_some()
    }

    /// `isPossibleArrayVariableName`: a `mixed` variable named like a plural. Its check that the native type may be an
    /// array always holds here: Mago's native type of a value whose type is `mixed` is `mixed` too.
    fn is_possible_array_variable_name(&self, passed: &Ty<'_>, expr: &Expression<'_>) -> bool {
        if passed.mixed().is_none() {
            return false;
        }
        match expr {
            Expression::Variable(Variable::Direct(v)) => {
                let name = &v.name[1..];
                name.len() > 3 && name.ends_with(b"s")
            }
            _ => false,
        }
    }

    /// `isAnErrorType` on the native type.
    fn is_an_error_type(&self, expr: &Expression<'_>, native: &Ty<'_>) -> bool {
        if native.mixed().is_some_and(TMixed::is_error) || self.is_undeclared_property(expr) {
            return true;
        }
        // `getParentScope()`: the scope around the nearest closure; an arrow function keeps the one around it
        let index = (self.ancestors.iter())
            .rposition(|a| matches!(a, Node::Function(_) | Node::Method(_) | Node::Closure(_)));
        if let Some(index) = index
            && let Node::Closure(closure) = self.ancestors[index]
        {
            let outer = &self.ancestors[..index];
            return self.is_undefined_before(outer, expr, node_span(closure).start)
                || self.is_undeclared_property_of_parameter(outer, expr);
        }
        // a variable an arrow function reads that neither it nor the code before it defines (Mago: `mixed`)
        if let Some(arrow) = self.outermost_arrow(index)
            && let Expression::Variable(Variable::Direct(v)) = expr
            && !self.ancestors[arrow..].iter().any(|a| {
                parameters(a)
                    .is_some_and(|l| l.parameters.iter().any(|p| p.variable.name == v.name))
            })
            && self.is_undefined_before(
                &self.ancestors[..arrow],
                expr,
                node_span(&self.ancestors[arrow]).start,
            )
        {
            return true;
        }
        native
            .mixed()
            .is_some_and(|m| is_null_subtracted(m) && !self.is_explicit(expr, self.ancestors))
    }

    /// The index of the outermost arrow function of the chain the code is in, inside the function at `function`.
    fn outermost_arrow(&self, function: Option<usize>) -> Option<usize> {
        let from = function.map_or(0, |i| i + 1);
        (from..self.ancestors.len()).find(|&i| matches!(self.ancestors[i], Node::ArrowFunction(_)))
    }

    /// Whether `expr` fetches a property that a class of its object's type does not declare: PHPStan's native
    /// `ErrorType` (a `__get` does not count there).
    fn is_undeclared_property(&self, expr: &Expression<'_>) -> bool {
        let (classes, name) = match expr {
            Expression::Access(Access::Property(PropertyAccess {
                object, property, ..
            }))
            | Expression::Access(Access::NullSafeProperty(NullSafePropertyAccess {
                object,
                property,
                ..
            })) => {
                let ClassLikeMemberSelector::Identifier(id) = property else {
                    return false;
                };
                let Ty::Scope(Some(t)) = self.get_type(object) else {
                    return false;
                };
                let classes = (t.types.iter())
                    .filter_map(|a| match a {
                        TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() => {
                            Some(n.name.to_string())
                        }
                        _ => None,
                    })
                    .collect();
                (classes, [b"$", id.value].concat())
            }
            Expression::Access(Access::StaticProperty(p)) => {
                let Variable::Direct(v) = &p.property else {
                    return false;
                };
                let class = match p.class {
                    // a class PHPStan does not know: `ErrorType`
                    Expression::Identifier(id) => {
                        let class = resolve_class_name(self.ancestors, id);
                        if self
                            .facts
                            .codebase
                            .get_class_like(class.as_bytes())
                            .is_none()
                        {
                            return true;
                        }
                        class
                    }
                    Expression::Self_(_) | Expression::Static(_)
                        if !self.is_trait
                            && let Some(name) = self.scope.and_then(|s| s.name.clone()) =>
                    {
                        name
                    }
                    _ => return false,
                };
                (vec![class], v.name.to_vec())
            }
            _ => return false,
        };
        self.lacks_property(&classes, &name)
    }

    /// Whether one of `classes` (an existing class, not a trait) does not declare the property `name` (with `$`).
    fn lacks_property(&self, classes: &[String], name: &[u8]) -> bool {
        let codebase = &self.facts.codebase;
        classes.iter().any(|class| {
            codebase
                .get_class_like(class.as_bytes())
                .is_some_and(|c| !c.kind.is_trait())
                && codebase
                    .get_declaring_property(class.as_bytes(), name)
                    .is_none()
        })
    }

    /// The parent scope's `ErrorType` of `$variable->name` in a closure, where `$variable` is a parameter of the
    /// function around it declared with a class (Mago types such a captured variable `mixed` inside the closure).
    fn is_undeclared_property_of_parameter(
        &self,
        outer: &[Node<'_, '_>],
        expr: &Expression<'_>,
    ) -> bool {
        let Expression::Access(Access::Property(PropertyAccess {
            object, property, ..
        })) = expr
        else {
            return false;
        };
        let (Expression::Variable(Variable::Direct(v)), ClassLikeMemberSelector::Identifier(id)) =
            (object, property)
        else {
            return false;
        };
        let Some(Hint::Identifier(class)) = (outer.iter().rev().find(|a| is_function_like(a)))
            .and_then(parameters)
            .and_then(|l| l.parameters.iter().find(|p| p.variable.name == v.name))
            .and_then(|p| p.hint.as_ref())
        else {
            return false;
        };
        self.lacks_property(
            &[resolve_class_name(outer, class)],
            &[b"$", id.value].concat(),
        )
    }

    /// Whether `expr`, a variable, is an `ErrorType` in the scope of `outer` (the ancestors of a closure) at `start`.
    fn is_undefined_before(
        &self,
        outer: &[Node<'_, '_>],
        expr: &Expression<'_>,
        start: usize,
    ) -> bool {
        let Expression::Variable(Variable::Direct(v)) = expr else {
            return false;
        };
        // the global scope may have any variable
        let Some(function) = outer.iter().rev().find(|a| is_function_like(a)) else {
            return false;
        };
        if parameters(function)
            .is_some_and(|list| list.parameters.iter().any(|p| p.variable.name == v.name))
        {
            return false;
        }
        if let Node::Closure(c) = function
            && c.use_clause
                .as_ref()
                .is_some_and(|u| u.variables.iter().any(|u| u.variable.name == v.name))
        {
            return false;
        }
        // written or read in the function before the closure
        let mut stack: Vec<Node<'_, '_>> = function.children();
        while let Some(node) = stack.pop() {
            match node {
                Node::DirectVariable(d)
                    if d.name == v.name && d.span.end.offset as usize <= start =>
                {
                    return false;
                }
                Node::Closure(_) | Node::Function(_) | Node::Class(_) | Node::AnonymousClass(_) => {
                }
                _ => stack.extend(node.children()),
            }
        }
        true
    }

    /// `shouldSkipTrait`.
    fn should_skip_trait(&self, expr: &Expression<'_>, r#type: &Ty<'_>) -> bool {
        if r#type.mixed().is_none() || !self.is_trait || self.is_explicit(expr, self.ancestors) {
            return false;
        }
        match expr {
            Expression::Call(Call::Method(_)) => true,
            // `PropertyFetchAnalyzer::isLocalPropertyFetch` on a trait: `$this->name`, `self::$name`, `static::$name`
            Expression::Access(Access::Property(p)) => {
                matches!(p.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            }
            Expression::Access(Access::StaticProperty(p)) => {
                matches!(p.class, Expression::Self_(_) | Expression::Static(_))
            }
            _ => false,
        }
    }

    /// `isStringCastedTernaryOfMixedTypes`.
    fn is_string_casted_ternary_of_mixed_types(&self, expr: &Expression<'_>) -> bool {
        let Expression::Conditional(c) = expr else {
            return false;
        };
        let Expression::UnaryPrefix(u) = unparenthesize(c.condition) else {
            return false;
        };
        if !matches!(
            u.operator,
            UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..)
        ) {
            return false;
        }
        let Some(then) = c.then else {
            return false;
        };
        let mixed = |e: &Expression<'_>| {
            (self.scope_type(unparenthesize(e)))
                .is_some_and(|t| matches!(t.types.as_ref(), [TAtomic::Mixed(_)]))
        };
        mixed(then) || mixed(c.r#else)
    }

    /// Whether PHPStan's native `mixed` of `expr` is an explicit one; `ancestors` lead to `expr`.
    fn is_explicit(&self, expr: &Expression<'_>, ancestors: &[Node<'_, '_>]) -> bool {
        if self.is_trait {
            return false;
        }
        match unparenthesize(expr) {
            Expression::Variable(Variable::Direct(v)) => {
                self.is_explicit_variable(v.name, node_span(expr).start, ancestors)
            }
            Expression::Call(
                call @ (Call::Function(_) | Call::Method(_) | Call::StaticMethod(_)),
            ) => callee(self.facts, ancestors, self.scope, call).is_some_and(|c| {
                (c.metadata.return_type_declaration_metadata.as_ref())
                    .is_some_and(|t| matches!(t.type_union.types.as_ref(), [TAtomic::Mixed(_)]))
            }),
            _ => false,
        }
    }

    /// Whether the variable `name` read at `at` holds an explicit `mixed`: see the module's documentation.
    fn is_explicit_variable(&self, name: &[u8], at: usize, ancestors: &[Node<'_, '_>]) -> bool {
        // outside a function, the file's own statements
        let index = ancestors.iter().rposition(is_function_like).unwrap_or(0);
        let function = ancestors[index];
        // the last assignment to it before `at`
        let mut last: Option<(usize, &Expression<'_>)> = None;
        let mut stack: Vec<Node<'_, '_>> = function.children();
        while let Some(node) = stack.pop() {
            match node {
                Node::Assignment(a)
                    if a.operator.is_assign()
                        && matches!(a.lhs, Expression::Variable(Variable::Direct(v)) if v.name == name)
                        && node_span(a).end <= at
                        && last.is_none_or(|(end, _)| end < node_span(a).end) =>
                {
                    last = Some((node_span(a).end, a.rhs));
                    stack.extend(node.children());
                }
                Node::Closure(_)
                | Node::ArrowFunction(_)
                | Node::Function(_)
                | Node::Class(_)
                | Node::AnonymousClass(_) => {}
                _ => stack.extend(node.children()),
            }
        }
        if let Some((_, assigned)) = last {
            // outside a function its value is read where it is, inside the namespace around it
            let around = if is_function_like(&function) {
                &ancestors[..=index]
            } else {
                ancestors
            };
            return self.is_explicit(assigned, around);
        }
        if let Some(p) = parameters(&function)
            .and_then(|l| l.parameters.iter().find(|p| p.variable.name == name))
        {
            return matches!(p.hint, Some(Hint::Mixed(_)));
        }
        // an arrow function sees the variables of the functions around it
        matches!(function, Node::ArrowFunction(_))
            && self.is_explicit_variable(name, node_span(&function).start, &ancestors[..index])
    }
}

/// An implicit `mixed`, for the values PHPStan types so inside a trait.
static MIXED: std::sync::LazyLock<TUnion> =
    std::sync::LazyLock::new(|| TUnion::from_atomic(TAtomic::Mixed(TMixed::new())));

/// `new CastString_($expr)` printed: the expression in parentheses unless it binds tighter than a cast. Rector's
/// printer leaves them out around an assignment (`(string) $k = $m`, which does not parse) and around a ternary
/// whose condition is no cast (`(string) $k ? $a : $b`, which casts the condition only); Pun keeps them.
fn cast(ctx: &Context<'_>, expr: &Expression<'_>) -> String {
    let inner = unparenthesize(expr);
    match prec(inner) {
        Prec::None => format!("(string) {}", text(ctx, inner)),
        _ => format!("(string) ({})", text(ctx, inner)),
    }
}

/// `shouldSkipType`.
fn should_skip_type(t: &Ty<'_>) -> bool {
    t.mixed().is_none() && !t.is_null() && !t.is_valid_union()
}

/// A `mixed` with only `null` taken out (`$value !== null`, `isset($value)`), not a truthy or a falsy one.
fn is_null_subtracted(m: &TMixed) -> bool {
    m.is_non_null() && !m.is_truthy() && !m.is_falsy() && !m.is_empty()
}

fn is_function_like(node: &Node<'_, '_>) -> bool {
    matches!(
        node,
        Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
    )
}

fn parameters<'ast, 'arena>(
    node: &Node<'ast, 'arena>,
) -> Option<&'ast FunctionLikeParameterList<'arena>> {
    match node {
        Node::Function(f) => Some(&f.parameter_list),
        Node::Method(m) => Some(&m.parameter_list),
        Node::Closure(c) => Some(&c.parameter_list),
        Node::ArrowFunction(a) => Some(&a.parameter_list),
        _ => None,
    }
}
