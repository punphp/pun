//! The string types a regex group can have, united as PHPStan's `TypeCombinator::union()` unites them.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::string::{TString, TStringCasing};
use mago_codex::ttype::union::TUnion;

/// `string` with its accessories: `non-empty-string`, `non-falsy-string`, `numeric-string`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Flags {
    pub(super) non_empty: bool,
    pub(super) non_falsy: bool,
    pub(super) numeric: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Atom {
    Const(String),
    Str(Flags),
    Null,
}

/// A union of [`Atom`]s, none a subtype of another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RType(pub(super) Vec<Atom>);

impl From<Atom> for RType {
    fn from(atom: Atom) -> Self {
        RType(vec![atom])
    }
}

/// `$a->isSuperTypeOf($b)->yes()`.
pub(super) fn is_super_type(a: &Atom, b: &Atom) -> bool {
    match (a, b) {
        (Atom::Null, Atom::Null) => true,
        (Atom::Const(a), Atom::Const(b)) => a == b,
        (Atom::Str(f), Atom::Const(s)) => {
            (!f.non_empty || !s.is_empty())
                && (!f.non_falsy || !(s.is_empty() || s == "0"))
                && (!f.numeric || TString::known_literal(mago_word::word(s)).is_numeric)
        }
        (Atom::Str(f), Atom::Str(g)) => {
            (!f.non_empty || g.non_empty)
                && (!f.non_falsy || g.non_falsy)
                && (!f.numeric || g.numeric)
        }
        _ => false,
    }
}

/// `TypeCombinator::compareTypesInUnion()`'s `''` and `'0'` cases: the merged type replacing `b`.
fn merge(a: &Atom, b: &Atom) -> Option<Atom> {
    let (Atom::Const(s), Atom::Str(f)) = (a, b) else {
        return None;
    };
    // '' | non-empty-string and '' | non-falsy-string are string
    if s.is_empty() && !f.numeric {
        return Some(Atom::Str(Flags::default()));
    }
    // '0' | non-falsy-string-X is non-empty-string-X
    (s == "0" && f.non_falsy).then_some(Atom::Str(Flags {
        non_falsy: false,
        ..*f
    }))
}

impl RType {
    /// `TypeCombinator::union()`.
    pub(super) fn union(types: impl IntoIterator<Item = RType>) -> RType {
        let mut out = Vec::new();
        for atom in types.into_iter().flat_map(|t| t.0) {
            add(&mut out, atom);
        }
        RType(out)
    }

    /// `TypeCombinator::removeNull()`.
    pub(super) fn remove_null(&self) -> RType {
        RType(
            self.0
                .iter()
                .filter(|a| **a != Atom::Null)
                .cloned()
                .collect(),
        )
    }

    pub(super) fn to_union(&self) -> TUnion {
        TUnion::from_vec(
            self.0
                .iter()
                .map(|atom| match atom {
                    Atom::Const(s) => {
                        TAtomic::Scalar(TScalar::String(TString::known_literal(mago_word::word(s))))
                    }
                    Atom::Str(f) => TAtomic::Scalar(TScalar::String(TString::general_with_props(
                        f.numeric,
                        f.non_falsy,
                        // numeric-string implies non-empty; Mago prints an explicit flag as `non-empty-string&...`
                        f.non_empty && !f.numeric,
                        false,
                        TStringCasing::Unspecified,
                    ))),
                    Atom::Null => TAtomic::Null,
                })
                .collect(),
        )
    }
}

fn add(out: &mut Vec<Atom>, atom: Atom) {
    let mut i = 0;
    while i < out.len() {
        if is_super_type(&out[i], &atom) {
            return;
        }
        if is_super_type(&atom, &out[i]) {
            out.remove(i);
            continue;
        }
        if let Some(merged) = merge(&atom, &out[i]).or_else(|| merge(&out[i], &atom)) {
            out.remove(i);
            return add(out, merged);
        }
        i += 1;
    }
    out.push(atom);
}
