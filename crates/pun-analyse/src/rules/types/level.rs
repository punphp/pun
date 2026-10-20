//! RuleLevelHelper: how strictly a level compares types (`checkNullables`, `checkUnionTypes`,
//! `checkExplicitMixed`, `checkImplicitMixed`), and `findTypeToCheck()` of an expression.

use mago_syntax::cst::*;
use pun_phpstan::types::{StrictMixed, Type, accepts_with, to_benevolent, union};

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
                Type::Null | Type::Benevolent(_) => t.traverse(&|i| go(i, accepting)),
                _ if !matches!(accepting, Type::Null) => t.clone().remove_null().traverse(&|i| go(i, accepting)),
                _ => t.traverse(&|i| go(i, accepting)),
            }
        }
        if self.check_nullables { accepted.clone() } else { go(accepted, accepting) }
    }

    /// `RuleLevelHelper::accepts()->result`: below level 7 a union is accepted when any member is.
    pub fn accepts(self, accepting: &Type, accepted: &Type, strict_types: bool) -> bool {
        let accepted = self.transform_accepted(accepting, accepted);
        let r = accepts_with(accepting, &accepted, strict_types, self.strict_mixed());
        if self.check_union_types { r.yes() } else { !r.no() }
    }
}

/// `FoundTypeResult`: `None` is an ErrorType (nothing to check).
// ponytail: the unknown classes it names (class.notFound at levels 3+) come with the first rule that reports them
pub struct Found {
    pub ty: Option<Type>,
}

impl<'a, 'arena> Checker<'a, 'arena> {
    pub(in super::super) fn levels(&self) -> Levels {
        Levels::of(self.level)
    }

    /// `findTypeToCheck($scope, $var, ...)`: the expression's type as the level checks it, `criteria` saying which
    /// members of a union are fine without `checkUnionTypes`.
    pub(in super::super) fn find_type_to_check(&self, e: &Expression, criteria: &dyn Fn(&Type) -> bool) -> Found {
        let l = self.levels();
        // (checkThisOnly, levels 0 and 1, is never asked: the rules that call this start at level 2)
        Found {
            ty: self.phpstan_type(e).and_then(|t| self.find_in(l, t, criteria, true)),
        }
    }

    /// [`Self::find_type_to_check()`] of an expression whose type is `ty` (`None`: no type known).
    pub(in super::super) fn find_type_to_check_of(&self, ty: Option<Type>, criteria: &dyn Fn(&Type) -> bool) -> Option<Type> {
        ty.and_then(|t| self.find_in(self.levels(), t, criteria, true))
    }

    /// `findTypeToCheckImplementation()`: `None` for an ErrorType. A strict `mixed` stays `mixed`, which
    /// [`Levels::strict_mixed`] tells apart.
    fn find_in(&self, l: Levels, mut t: Type, criteria: &dyn Fn(&Type) -> bool, top: bool) -> Option<Type> {
        if !l.check_nullables && !t.is_null().yes() && !criteria(&Type::Null) {
            t = t.remove_null();
        }
        let mixed = match &t {
            Type::Mixed { explicit } => Some(*explicit),
            Type::Template(tt) => match tt.bound {
                Type::Mixed { explicit } => Some(explicit),
                _ => None,
            },
            _ => None,
        };
        if let Some(explicit) = mixed {
            let strict = if explicit { l.check_explicit_mixed } else { l.check_implicit_mixed };
            // a template of `mixed` stays the template (`T of mixed` in messages)
            return strict.then_some(t);
        }
        if matches!(t, Type::Never(_) | Type::Error) {
            return None;
        }
        if top
            && self.object_names(&t).iter().any(|n| match self.codebase.get_class_like(n.as_bytes()) {
                Some(k) => k.kind == mago_codex::symbol::SymbolKind::Trait,
                None => pun_phpstan::symbols::class(n).is_none(),
            })
        {
            // ponytail: the class.notFound errors it builds come with the first rule that reports them (level 3+)
            return None;
        }
        if !l.check_union_types && matches!(t, Type::ObjectWithoutClass | Type::ObjectShape(_)) {
            return None;
        }
        // a TemplateUnionType is a UnionType (TemplateBenevolentUnionType a benevolent one): filtered by its members
        let template_union = match &t {
            Type::Template(tt) if matches!(tt.bound, Type::Union(_) | Type::Benevolent(_)) => Some(tt.bound.clone()),
            _ => None,
        };
        if let Type::Union(types) | Type::Benevolent(types) = template_union.as_ref().unwrap_or(&t) {
            let t = template_union.as_ref().unwrap_or(&t);
            // checkBenevolentUnionTypes is off: a benevolent union is always filtered
            let filter = !l.check_union_types || matches!(t, Type::Benevolent(_));
            let kept: Vec<Type> = types
                .iter()
                .filter(|i| !filter || criteria(i))
                .filter_map(|i| self.find_in(l, i.clone(), criteria, false))
                .collect();
            if !kept.is_empty() {
                let u = union(kept);
                return Some(if matches!(t, Type::Benevolent(_)) { to_benevolent(u) } else { u });
            }
        }
        Some(t)
    }

    /// `getObjectClassNames()`.
    fn object_names(&self, t: &Type) -> Vec<String> {
        match t {
            Type::Object(n, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => vec![n.clone()],
            Type::Union(ts) | Type::Benevolent(ts) => ts.iter().flat_map(|t| self.object_names(t)).collect(),
            Type::Intersection(ts) => ts.iter().flat_map(|t| self.object_names(t)).collect(),
            _ => Vec::new(),
        }
    }
}
