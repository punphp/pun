//! RuleLevelHelper: how strictly a level compares types (`checkNullables`, `checkUnionTypes`,
//! `checkExplicitMixed`, `checkImplicitMixed`), and `findTypeToCheck()` of an expression.

use mago_syntax::cst::*;
use pun_phpstan::symbols;
use pun_phpstan::types::{StrictMixed, Type, accepts_reasons, ancestor_args, accepts_with, intersect, to_benevolent, union};

use super::super::Checker;

/// The level's switches (conf/config.level*.neon).
#[derive(Clone, Copy)]
pub struct Levels {
    pub check_nullables: bool,
    pub check_union_types: bool,
    pub check_explicit_mixed: bool,
    pub check_implicit_mixed: bool,
}

impl Levels {
    pub fn of(level: u8) -> Levels {
        Levels {
            check_union_types: level >= 7,
            check_nullables: level >= 8,
            check_explicit_mixed: level >= 9,
            check_implicit_mixed: level >= 10,
        }
    }

    pub fn strict_mixed(self) -> StrictMixed {
        StrictMixed {
            explicit: self.check_explicit_mixed,
            implicit: self.check_implicit_mixed,
        }
    }

    /// `transformAcceptedType()`: without `checkNullables`, `null` is dropped from what is accepted (not from a
    /// callable's parameters, nor from a benevolent union).
    fn transform_accepted(self, accepting: &Type, accepted: &Type) -> Type {
        fn go(t: &Type, accepting: &Type) -> Type {
            match t {
                Type::Callable(_) | Type::Closure(_) => t.clone(),
                Type::Object(name, args, exact) if !args.is_empty() => generic(name, args, *exact, accepting),
                Type::Null | Type::Benevolent(_) => t.traverse(&|i| go(i, accepting)),
                _ if !matches!(accepting, Type::Null) => t.clone().remove_null().traverse(&|i| go(i, accepting)),
                _ => t.traverse(&|i| go(i, accepting)),
            }
        }
        // `traverseWithoutMapping()`: a generic object's argument loses its null only where the accepting type's
        // does not take one
        fn generic(name: &str, args: &[Type], exact: bool, accepting: &Type) -> Type {
            let contains_null = |t: &Type| matches!(t, Type::Null) || matches!(t, Type::Union(ts) if ts.contains(&Type::Null));
            let theirs = match accepting {
                Type::Object(a, theirs, _) if a.eq_ignore_ascii_case(name) => Some(theirs.clone()),
                Type::Object(a, theirs, _) => symbols::lookup(a).and_then(|c| ancestor_args(&c, name, theirs)),
                _ => None,
            };
            match theirs.filter(|theirs| theirs.len() == args.len()) {
                Some(theirs) => Type::Object(
                    name.to_string(),
                    args.iter()
                        .zip(&theirs)
                        .map(|(t, o)| match t {
                            Type::Union(_) if contains_null(t) && !contains_null(o) => go(&t.clone().remove_null(), accepting),
                            t => t.clone(),
                        })
                        .collect(),
                    exact,
                ),
                None => Type::Object(name.to_string(), args.to_vec(), exact),
            }
        }
        if self.check_nullables { accepted.clone() } else { go(accepted, accepting) }
    }

    /// `RuleLevelHelper::accepts()->result`: below level 7 a union is accepted when any member is.
    pub fn accepts(self, accepting: &Type, accepted: &Type, strict_types: bool) -> bool {
        let accepted = self.transform_accepted(accepting, accepted);
        let accepting = self.strict_mixed().transform(accepting);
        let r = accepts_with(&accepting, &accepted, strict_types, self.strict_mixed());
        if self.check_union_types { r.yes() } else { !r.no() }
    }

    /// `RuleLevelHelper::accepts()->reasons` as an error's tip (`acceptsReasonsTip()`: one reason as is, more as a
    /// bullet list).
    pub fn accepts_tip(self, accepting: &Type, accepted: &Type, strict_types: bool) -> Option<String> {
        let accepted = self.transform_accepted(accepting, accepted);
        let accepting = self.strict_mixed().transform(accepting);
        match accepts_reasons(&accepting, &accepted, strict_types, self.strict_mixed()).as_slice() {
            [] => None,
            [one] => Some(one.clone()),
            all => Some(all.iter().map(|r| format!("• {r}")).collect::<Vec<_>>().join("\n")),
        }
    }
}

/// `FoundTypeResult`: `None` is an ErrorType (nothing to check), then with the unknown classes the type names
/// (`getUnknownClassErrors()`, for the rules that report them).
pub struct Found {
    pub ty: Option<Type>,
    pub unknown: Vec<String>,
}

impl<'a, 'arena> Checker<'a, 'arena> {
    pub(in super::super) fn levels(&self) -> Levels {
        Levels::of(self.level)
    }

    /// RuleLevelHelper::isThis(): php-parser discards parentheses before testing expression eligibility.
    pub(in super::super) fn is_this_expression(e: &Expression) -> bool {
        matches!(super::flow::unparen(e), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
    }

    pub(in super::super) fn checks_expression(&self, e: &Expression) -> bool {
        !self.check_this_only || Self::is_this_expression(e)
    }

    /// `findTypeToCheck($scope, $var, ...)`: the expression's type as the level checks it, `criteria` saying which
    /// members of a union are fine without `checkUnionTypes`.
    pub(in super::super) fn find_type_to_check(&self, e: &Expression, criteria: &dyn Fn(&Type) -> bool) -> Found {
        let l = self.levels();
        let mut unknown = Vec::new();
        let ty = self.checks_expression(e).then(|| self.phpstan_type(e)).flatten()
            .and_then(|t| self.find_in(l, t, criteria, Some(&mut unknown)));
        Found { ty, unknown }
    }

    /// [`Self::find_type_to_check()`] of an expression whose type is `ty` (`None`: no type known).
    pub(in super::super) fn find_type_to_check_of(&self, ty: Option<Type>, criteria: &dyn Fn(&Type) -> bool) -> Option<Type> {
        ty.and_then(|t| self.find_in(self.levels(), t, criteria, Some(&mut Vec::new())))
    }

    /// `findTypeToCheckImplementation()`: `None` for an ErrorType. The checked copy is StrictMixedType,
    /// never a replacement for the expression's stored/inferred type.
    pub(super) fn find_in(&self, l: Levels, mut t: Type, criteria: &dyn Fn(&Type) -> bool, top: Option<&mut Vec<String>>) -> Option<Type> {
        if !l.check_nullables && !t.is_null().yes() && !criteria(&Type::Null) {
            t = t.remove_null();
        }
        // ErrorType and SubtractedMixed are MixedType too. A mixed template is always explicitly mixed.
        if let Some(explicit) = t.mixed_explicitness() {
            let strict = if explicit { l.check_explicit_mixed } else { l.check_implicit_mixed };
            return strict.then(|| t.to_strict_mixed());
        }
        if matches!(t, Type::Never(_)) {
            return None;
        }
        if let Some(unknown) = top {
            let missing = t.object_class_names().into_iter().filter(|n| match self.codebase.get_class_like(n.as_bytes()) {
                Some(k) => k.kind == mago_codex::symbol::SymbolKind::Trait,
                None => pun_phpstan::symbols::class(n).is_none(),
            });
            // a class class_exists() guards is not reported, but the type is still not checked
            let (guarded, missing): (Vec<String>, Vec<String>) = missing.partition(|n| self.in_class_exists(n));
            unknown.extend(missing);
            if !unknown.is_empty() || !guarded.is_empty() {
                return None;
            }
        }
        if !l.check_union_types && t.is_object().yes() && t.object_class_names().is_empty() {
            return None;
        }
        // TemplateUnionType/TemplateIntersectionType inherit their bound's compound dispatch.
        let template_compound = match &t {
            Type::Template(tt) if matches!(tt.bound, Type::Union(_) | Type::Benevolent(_) | Type::Intersection(_)) => Some(tt.bound.clone()),
            _ => None,
        };
        if let Type::Union(types) | Type::Benevolent(types) = template_compound.as_ref().unwrap_or(&t) {
            let t = template_compound.as_ref().unwrap_or(&t);
            // checkBenevolentUnionTypes is off: a benevolent union is always filtered
            let filter = !l.check_union_types || matches!(t, Type::Benevolent(_));
            let kept: Vec<Type> = types
                .iter()
                .filter(|i| !filter || criteria(i))
                // Filter the original members first, then retain recursive ErrorTypes during recombination.
                .map(|i| self.find_in(l, i.clone(), criteria, None).unwrap_or(Type::Error))
                .collect();
            if !kept.is_empty() {
                let u = union(kept);
                return (!matches!(u, Type::Error)).then(|| if matches!(t, Type::Benevolent(_)) { to_benevolent(u) } else { u });
            }
        }
        if let Type::Intersection(parts) = template_compound.as_ref().unwrap_or(&t)
            && parts.iter().any(|t| matches!(t, Type::Template(_)) && t.mixed_explicitness().is_some())
        {
            // Only direct TemplateMixedType members are converted, even below level 9. Do not descend into
            // unrelated array/generic arguments, defaults or arbitrary intersection members.
            return Some(intersect(parts.iter().map(|t| {
                if matches!(t, Type::Template(_)) { t.to_strict_mixed() } else { t.clone() }
            }).collect()));
        }
        Some(t)
    }
}
