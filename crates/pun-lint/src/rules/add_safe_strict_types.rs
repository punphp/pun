//! `lint/statements/add-safe-strict-types`, ported from Rector 2.8.4's `SafeDeclareStrictTypesRector`
//! (rules/TypeDeclaration/Rector/StmtsAwareInterface) and its `StrictTypeSafetyChecker` (rules/TypeDeclaration/
//! NodeAnalyzer).
//!
//! A file without `declare(strict_types=...)` and without inline HTML gets `declare(strict_types=1);` when strict mode
//! cannot change what it does: every call, `new` and attribute reaches a function or method whose parameters accept
//! each argument's native type in strict mode, every function's returns are accepted by its return type, and every
//! `=` to a property is accepted by the property's native type. PHPStan's `accepts($type, strictTypes: true)` is
//! pun-phpstan's, with a `mixed` value as a `StrictMixedType`; the native type of a value is Mago's
//! ([`Facts::native_type`]).
//!
//! Where Rector reads reflection, Pun reads:
//! - a built-in function's or method's parameters: PHPStan's own signatures (`pun_phpstan::symbols`);
//! - a user one's: Mago's codebase, the native type with the PHPDoc type over it (`TypehintHelper::decideType`);
//! - a method: the one class of its receiver's type (`getObjectClassNames()`, so a nullable or a union receiver has
//!   none), looked up with the methods PHPDocs declare (`@method`), as `hasMethod()` with a scope does.
//!
//! A return type `self` or `parent` is Rector's `SelfStaticType` or `ParentStaticType`, which accepts no value a scope
//! gives (checked on the oracle), so a function declaring one and returning a value is never safe.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::metadata::parameter::FunctionLikeParameterMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, Argument, ArgumentList, AssignmentOperator, ClassLikeMemberSelector, Expression,
    FunctionLikeReturnTypeHint, MethodBody, Node, NodeKind, PartialArgument, PartialArgumentList,
    Variable,
};
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{
    Accessory, NeverKind, StrictMixed, Type, accepts_with, decide_type, intersect, mixed,
};
use pun_problem::{Fix, Safety};

use super::lint_b_helpers::contains_html;
use super::lint_b_names::resolve_function;
use super::phpunit::require_strict_types_in_tests::{declare_strict_types, has_strict_types};
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::{TypeScope, expr_type, from_mago};
use super::types::compare::{Object, RType};
use super::types::facts::{Facts, facts, is_builtin_class, is_builtin_function};
use super::types::names::resolve_class_name;
use super::types::{any_scoped, class_scope, returns_scoped};
use crate::{Context, Report, Rule};

pub struct AddSafeStrictTypes;

impl Rule for AddSafeStrictTypes {
    fn id(&self) -> &'static str {
        "lint/statements/add-safe-strict-types"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersion::PHP_70
        70000
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        if has_strict_types(&program.statements) || contains_html(node, ancestors) {
            return;
        }
        let facts = facts(ctx);
        if !with_classes(facts.documented(), || {
            is_file_strict_type_safe(&facts, node)
        }) {
            return;
        }
        let Some((at, edit)) = declare_strict_types(ctx, &program.statements) else {
            return;
        };
        out.push(Report {
            span: at,
            message: "This file never relies on scalar type coercion: declare strict types."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}

/// `isFileStrictTypeSafe`: every node of the file passes its check.
fn is_file_strict_type_safe(facts: &Facts, program: Node<'_, '_>) -> bool {
    // depth first, with the ancestors of each node
    let mut stack = vec![(program, 0usize)];
    let mut path: Vec<Node<'_, '_>> = Vec::new();
    while let Some((node, depth)) = stack.pop() {
        path.truncate(depth);
        if !is_node_safe(facts, node, &path) {
            return false;
        }
        path.push(node);
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev().map(|c| (c, depth + 1)));
    }
    true
}

/// The check of `StrictTypeSafetyChecker` that `node` gets, if any.
fn is_node_safe(facts: &Facts, node: Node<'_, '_>, ancestors: &[Node<'_, '_>]) -> bool {
    let at = At { facts, ancestors };
    match node {
        // `isCallLikeSafe`
        Node::FunctionCall(c) => {
            let Expression::Identifier(id) = c.function else {
                return false;
            };
            at.function_parameters(id)
                .is_some_and(|p| at.are_args_safe(&args(&c.argument_list), &p))
        }
        Node::MethodCall(c) => {
            let ClassLikeMemberSelector::Identifier(name) = &c.method else {
                return false;
            };
            (at.single_class(c.object))
                .and_then(|class| at.method_parameters(&class, name.value))
                .is_some_and(|p| at.are_args_safe(&args(&c.argument_list), &p))
        }
        // `resolveFunctionLikeReflectionFromCall` has no reflection for a nullsafe call
        Node::NullSafeMethodCall(_) => false,
        Node::StaticMethodCall(c) => {
            let ClassLikeMemberSelector::Identifier(name) = &c.method else {
                return false;
            };
            (at.class_of_reference(c.class))
                .and_then(|class| at.method_parameters(&class, name.value))
                .is_some_and(|p| at.are_args_safe(&args(&c.argument_list), &p))
        }
        Node::Instantiation(new) => {
            let arguments = new.argument_list.as_ref().map_or_else(Vec::new, args);
            (at.class_of_reference(new.class))
                .and_then(|class| at.method_parameters(&class, b"__construct"))
                .is_some_and(|p| at.are_args_safe(&arguments, &p))
        }
        Node::AnonymousClass(class) => {
            let name = facts.anonymous_class_name(mago_span::HasSpan::span(class));
            let arguments = class
                .argument_list
                .as_ref()
                .map_or(Some(Vec::new()), partial_args);
            match (arguments, at.method_parameters(&name, b"__construct")) {
                (Some(arguments), Some(p)) => at.are_args_safe(&arguments, &p),
                _ => false,
            }
        }
        // `isAttributeSafe`
        Node::Attribute(attribute) => {
            let class = resolve_class_name(ancestors, &attribute.name);
            let arguments = attribute
                .argument_list
                .as_ref()
                .map_or(Some(Vec::new()), partial_args);
            match (arguments, at.method_parameters(&class, b"__construct")) {
                (Some(arguments), Some(p)) => at.are_args_safe(&arguments, &p),
                _ => false,
            }
        }
        // `areFunctionReturnsTypeSafe`
        Node::Function(f) => {
            at.are_returns_safe(f.return_type_hint.as_ref(), Returns::Block(&f.body))
        }
        Node::Method(m) => match &m.body {
            MethodBody::Concrete(body) => {
                at.are_returns_safe(m.return_type_hint.as_ref(), Returns::Block(body))
            }
            MethodBody::Abstract(_) => true,
        },
        Node::Closure(c) => {
            at.are_returns_safe(c.return_type_hint.as_ref(), Returns::Block(&c.body))
        }
        Node::ArrowFunction(a) => at.are_returns_safe(
            a.return_type_hint.as_ref(),
            Returns::Expression(a.expression),
        ),
        // `isPropertyAssignSafe`
        Node::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_)) => {
            at.is_property_assign_safe(a.lhs, a.rhs)
        }
        _ => true,
    }
}

/// An argument as `areArgsSafe` reads it.
struct Arg<'a> {
    value: &'a Expression<'a>,
    name: Option<&'a [u8]>,
    unpack: bool,
}

fn args<'a>(list: &'a ArgumentList<'a>) -> Vec<Arg<'a>> {
    (list.arguments.iter())
        .map(|a| match a {
            Argument::Positional(p) => Arg {
                value: p.value,
                name: None,
                unpack: p.ellipsis.is_some(),
            },
            Argument::Named(n) => Arg {
                value: n.value,
                name: Some(n.name.value),
                unpack: false,
            },
        })
        .collect()
}

/// The arguments of an attribute or an anonymous class; `None` with a placeholder (no valid code has one there).
fn partial_args<'a>(list: &'a PartialArgumentList<'a>) -> Option<Vec<Arg<'a>>> {
    (list.arguments.iter())
        .map(|a| {
            Some(Arg {
                value: a.value()?,
                name: match a {
                    PartialArgument::Named(n) => Some(n.name.value),
                    _ => None,
                },
                unpack: matches!(a, PartialArgument::Positional(p) if p.ellipsis.is_some()),
            })
        })
        .collect()
}

/// A parameter as `ParameterReflection` gives it: its name (without `$`), `getType()` and `isVariadic()`.
struct Param {
    name: Vec<u8>,
    r#type: Type,
    variadic: bool,
}

/// What a function returns: the `return`s of its body, or an arrow function's expression.
enum Returns<'a> {
    Block(&'a mago_syntax::cst::Block<'a>),
    Expression(&'a Expression<'a>),
}

/// Where a node is: the file's facts and the node's ancestors.
struct At<'a, 'ast, 'arena> {
    facts: &'a Facts,
    ancestors: &'a [Node<'ast, 'arena>],
}

impl At<'_, '_, '_> {
    /// `areArgsSafe`.
    fn are_args_safe(&self, args: &[Arg<'_>], parameters: &[Param]) -> bool {
        args.iter().enumerate().all(|(position, arg)| {
            if arg.unpack {
                return false;
            }
            let parameter = match arg.name {
                Some(name) => parameters.iter().find(|p| p.name == name),
                None => parameters
                    .get(position)
                    .or_else(|| parameters.last().filter(|p| p.variadic)),
            };
            let Some(parameter) = parameter else {
                return false;
            };
            is_type_safe_for_strict_mode(
                &parameter.r#type,
                &expr_type(self.facts, self.ancestors, arg.value, true),
            )
        })
    }

    /// `areFunctionReturnsTypeSafe`.
    fn are_returns_safe(
        &self,
        hint: Option<&FunctionLikeReturnTypeHint<'_>>,
        returns: Returns<'_>,
    ) -> bool {
        let Some(hint) = hint else {
            return true;
        };
        let declared = declared_type(
            &TypeScope::new(self.ancestors, self.facts.documented()).map_hint(&hint.hint),
        );
        if matches!(declared, Type::Mixed { .. } | Type::Never(_)) || declared.is_void().yes() {
            return true;
        }
        let values: Vec<&Expression<'_>> = match returns {
            Returns::Block(body) => returns_scoped(body)
                .into_iter()
                .filter_map(|r| r.value)
                .collect(),
            Returns::Expression(e) => {
                let yields = matches!(e, Expression::Yield(_))
                    || any_scoped(Node::Expression(e), &mut |n| matches!(n, Node::Yield(_)));
                if yields { vec![] } else { vec![e] }
            }
        };
        values.into_iter().all(|value| {
            is_type_safe_for_strict_mode(
                &declared,
                &expr_type(self.facts, self.ancestors, value, true),
            )
        })
    }

    /// `isPropertyAssignSafe`.
    fn is_property_assign_safe(&self, var: &Expression<'_>, value: &Expression<'_>) -> bool {
        let (class, name) = match var {
            Expression::Access(Access::Property(p)) => {
                let ClassLikeMemberSelector::Identifier(id) = &p.property else {
                    return false;
                };
                (self.single_class(p.object), [b"$", id.value].concat())
            }
            Expression::Access(Access::StaticProperty(p)) => {
                let Variable::Direct(v) = &p.property else {
                    return false;
                };
                (self.class_of_reference(p.class), v.name.to_vec())
            }
            _ => return true,
        };
        // `resolvePropertyReflectionFromPropertyFetch`: a property the class declares natively
        let Some(property) = class.and_then(|c| {
            self.facts
                .codebase
                .get_declaring_property(c.as_bytes(), &name)
        }) else {
            return false;
        };
        let declared = property
            .type_declaration_metadata
            .as_ref()
            .map_or_else(mixed, |t| from_mago(&t.type_union));
        is_type_safe_for_strict_mode(
            &declared,
            &expr_type(self.facts, self.ancestors, value, true),
        )
    }

    /// `ClassNameFromObjectTypeResolver::resolve(getType($expr))`: the one class of the expression's type.
    fn single_class(&self, expr: &Expression<'_>) -> Option<String> {
        let span = mago_span::HasSpan::span(expr);
        let span = pun_problem::Span::new(span.start.offset as usize, span.end.offset as usize);
        // inside a trait PHPStan does not know `$this` (see `Facts::native_type`)
        self.facts.native_type(span)?;
        let t = self.facts.full_type(span)?;
        match t.types.as_ref() {
            [TAtomic::Object(TObject::Named(n))] if n.intersection_types.is_none() => {
                Some(n.name.to_string())
            }
            [TAtomic::Object(TObject::Enum(e))] => Some(e.name.to_string()),
            _ => None,
        }
    }

    /// The class a `new` or a static call names: a name, `self`, `static`, `parent`, or the one class of an
    /// expression's type.
    fn class_of_reference(&self, class: &Expression<'_>) -> Option<String> {
        let scope = || class_scope(self.ancestors);
        match class {
            Expression::Identifier(id) => Some(resolve_class_name(self.ancestors, id)),
            Expression::Self_(_) | Expression::Static(_) => scope()?.name,
            Expression::Parent(_) => scope()?.parent_class,
            other => self.single_class(other),
        }
    }

    /// `resolveFunctionReflectionFromFuncCall` and the parameters of the variant the call selects.
    fn function_parameters(&self, id: &mago_syntax::cst::Identifier<'_>) -> Option<Vec<Param>> {
        let (resolved, explicit) = resolve_function(self.ancestors, id);
        let written = id.value().strip_prefix(b"\\").unwrap_or(id.value());
        let codebase = &self.facts.codebase;
        let name =
            if explicit || codebase.get_function(&resolved).is_some() || written.contains(&b'\\') {
                resolved
            } else {
                written.to_vec()
            };
        if is_builtin_function(&name) {
            return builtin_parameters(pun_phpstan::symbols::function(&String::from_utf8_lossy(
                &name,
            ))?);
        }
        let native = codebase.get_function(&name)?;
        Some(user_parameters(
            native,
            self.facts.documented().get_function(&name),
        ))
    }

    /// `resolveMethodReflection($class, $method, $scope)` and its parameters.
    fn method_parameters(&self, class: &str, method: &[u8]) -> Option<Vec<Param>> {
        let documented = self.facts.documented();
        documented.get_class_like(class.as_bytes())?;
        let declaring = documented.get_declaring_method_class(class.as_bytes(), method)?;
        if is_builtin_class(declaring.as_bytes()) {
            let builtin = pun_phpstan::symbols::class(&declaring.to_string())?;
            return builtin_parameters(
                builtin
                    .methods
                    .get(&String::from_utf8_lossy(method).to_ascii_lowercase())?,
            );
        }
        let doc = documented.get_declaring_method(class.as_bytes(), method)?;
        let native = self
            .facts
            .codebase
            .get_declaring_method(class.as_bytes(), method);
        // a method only a PHPDoc declares (`@method`) has no native signature
        Some(user_parameters(native.unwrap_or(doc), Some(doc)))
    }
}

/// The parameters of PHPStan's signature of a built-in function or method, of its first variant.
fn builtin_parameters(function: &pun_phpstan::symbols::Function) -> Option<Vec<Param>> {
    let variant = function.variants.first()?;
    Some(
        (variant.params.iter())
            .map(|p| Param {
                name: p.name.trim_start_matches('$').as_bytes().to_vec(),
                r#type: resolve_str(&p.ty, None),
                variadic: p.variadic,
            })
            .collect(),
    )
}

/// The parameters of a user function: `decideType()` of the native type and the PHPDoc type.
fn user_parameters(
    native: &FunctionLikeMetadata,
    documented: Option<&FunctionLikeMetadata>,
) -> Vec<Param> {
    (native.parameters.iter())
        .map(|p| {
            let doc = documented
                .and_then(|d| d.parameters.iter().find(|q| q.name == p.name))
                .and_then(FunctionLikeParameterMetadata::get_type_metadata)
                .filter(|t| t.from_docblock)
                .map(|t| doc_type(&t.type_union));
            let declared = p
                .type_declaration_metadata
                .as_ref()
                .map_or_else(mixed, |t| from_mago(&t.type_union));
            Param {
                name: p.name.0.as_bytes()[1..].to_vec(),
                r#type: decide_type(declared, doc),
                variadic: p.flags.is_variadic(),
            }
        })
        .collect()
}

/// A PHPDoc type of Mago's as PHPStan's, with the accessory types of a string Mago keeps as flags.
fn doc_type(t: &TUnion) -> Type {
    let members: Vec<Type> = (t.types.iter())
        .map(|a| match a {
            TAtomic::Scalar(TScalar::String(s)) if s.literal.is_none() => {
                let accessory = if s.is_numeric {
                    Some(Accessory::NumericString)
                } else if s.is_truthy {
                    Some(Accessory::NonFalsyString)
                } else if s.is_non_empty {
                    Some(Accessory::NonEmptyString)
                } else {
                    None
                };
                accessory.map_or(Type::String, |accessory| {
                    intersect(vec![Type::String, Type::Accessory(accessory)])
                })
            }
            a => from_mago(&TUnion::from_atomic(a.clone())),
        })
        .collect();
    match members.len() {
        1 => members.into_iter().next().expect("one member"),
        _ => pun_phpstan::types::union(members),
    }
}

/// The declared return type as PHPStan's, Rector's `SelfStaticType` and `ParentStaticType` accepting nothing.
fn declared_type(t: &RType) -> Type {
    match t {
        RType::Object(Object::SelfStatic(_) | Object::ParentStatic(_)) => {
            Type::Never(NeverKind::NonAccepting)
        }
        RType::Union(types) => Type::Union(types.iter().map(declared_type).collect()),
        other => other.to_super(),
    }
}

/// `isTypeSafeForStrictMode`: `accepts($value, strictTypes: true)`, a `mixed` value as a `StrictMixedType`.
fn is_type_safe_for_strict_mode(declared: &Type, value: &Type) -> bool {
    accepts_with(
        declared,
        value,
        true,
        StrictMixed {
            explicit: true,
            implicit: true,
        },
    )
    .yes()
}
