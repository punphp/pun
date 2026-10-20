//! Types against PHPStan 2.3.1 (scripts/export-phpstan-symbols.php): every signature-map type string through
//! `TypeStringResolver` + `describe()`, every type in the reflected signatures through `toPhpDocNode()` and back,
//! and every built-in function's and method's signature on PHP 8.4 and 8.5.

mod common;

use std::collections::HashMap;

use common::fixture;
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::symbols::{self, Function, PhpVersion};
use pun_phpstan::types::{Level, is_super_type_of};

/// `ConstantResolver` builds PHP_SAPI's union with `new UnionType()`, unnormalized; read back from its string it
/// normalizes to `non-falsy-string` (the `PHP_SAPI` identifier itself resolves unnormalized, see `constants`).
const UNNORMALIZED: &[&str] = &[
    "('apache' | 'apache2handler' | 'cgi' | 'cli' | 'cli-server' | 'embed' | 'fpm-fcgi' | 'litespeed' | 'phpdbg' | non-falsy-string)",
];

/// PHPStan 2.3 signature types Pun does not model yet, as `Class::type`: a template parameter whose bound is a
/// benevolent union stays the template (Pun resolves it to the bound), and `static` of an iterator whose
/// `@extends` names its template arguments keeps them as arguments of the parent.
const SIGNATURE_GAPS: &[&str] = &[
    "ArrayIterator::(__template<TKey, 'class ArrayIterator', __benevolent<(int | string)>> | null)",
    "ArrayObject::(__template<TKey, 'class ArrayObject', __benevolent<(int | string)>> | null)",
    "ParentIterator::static",
    "RecursiveCachingIterator::static",
    "RecursiveFilterIterator::static",
    "RecursiveRegexIterator::static",
];

fn skipped(skip: &[&str], ctx: Option<&str>, input: &str) -> bool {
    skip.contains(&input) || ctx.is_some_and(|c| skip.contains(&format!("{c}::{input}").as_str()))
}

fn describe(input: &str, ctx: Option<&str>) -> [String; 3] {
    let ty = resolve_str(input, ctx);
    [Level::Precise, Level::Value, Level::TypeOnly].map(|l| ty.describe(l))
}

/// (context class, type string, its three descriptions, the type string in Pun's syntax when it differs: a
/// probe's function templates as `__template<...>`; a probe's iterable key and value types when it is iterable)
type Row = (
    Option<String>,
    String,
    [String; 3],
    Option<String>,
    Option<[String; 2]>,
);

fn rows(name: &str) -> Vec<Row> {
    let f = fixture(name);
    f.as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let s = |i: usize| row[i].as_str().unwrap().to_string();
            (
                row[0].as_str().map(str::to_string),
                s(1),
                [s(2), s(3), s(4)],
                row.get(5).and_then(|v| v.as_str()).map(str::to_string),
                row.get(6)
                    .and_then(|v| v.as_array())
                    .map(|_| [0, 1].map(|i| row[6][i].as_str().unwrap().to_string())),
            )
        })
        .collect()
}

fn check(name: &str, skip: &[&str]) {
    let rows = rows(name);
    let mut failures = Vec::new();
    let mut skipped_rows = 0;
    for (ctx, input, want, pun, iterable) in &rows {
        if skipped(skip, ctx.as_deref(), input) {
            skipped_rows += 1;
            continue;
        }
        let ty = resolve_str(pun.as_deref().unwrap_or(input), ctx.as_deref());
        let got = [Level::Precise, Level::Value, Level::TypeOnly].map(|l| ty.describe(l));
        if got != *want {
            failures.push(format!("{ctx:?} {input}\n  want {want:?}\n  got  {got:?}"));
        } else if let Some(want) = iterable {
            let got = [ty.iterable_key_type(), ty.iterable_value_type()]
                .map(|t| t.describe(Level::Precise));
            if got != *want {
                failures.push(format!(
                    "{ctx:?} {input} (iterable key, value)\n  want {want:?}\n  got  {got:?}"
                ));
            }
        }
    }
    eprintln!(
        "{name}: {}/{} match",
        rows.len() - failures.len() - skipped_rows,
        rows.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn signature_map_type_strings() {
    check("type-strings.json", &[]);
}

#[test]
fn reflected_types() {
    check("reflected-types.json", &[UNNORMALIZED, SIGNATURE_GAPS].concat());
}

/// The type strings of one function's variants, each with what PHPStan's reflection describes it as.
fn signature_strings(f: &Function) -> Vec<&str> {
    let mut strings = Vec::new();
    for v in f.variants.iter().chain(f.named.iter().flatten()) {
        for p in &v.params {
            strings.push(p.ty.as_str());
            strings.extend(p.default.as_deref());
        }
        strings.push(v.ret.as_str());
    }
    strings
}

#[test]
fn built_in_signatures() {
    let reflected: HashMap<(Option<String>, String), [String; 3]> = rows("reflected-types.json")
        .into_iter()
        .map(|(ctx, s, d, ..)| ((ctx, s), d))
        .collect();
    for version in [PhpVersion::Php84, PhpVersion::Php85] {
        let symbols = symbols::symbols(version);
        let mut callables: Vec<(Option<&str>, &Function)> =
            symbols.functions.values().map(|f| (None, f)).collect();
        for class in symbols.classes.values() {
            callables.extend(
                class
                    .methods
                    .values()
                    .map(|m| (Some(class.name.as_str()), m)),
            );
        }
        let mut failures = Vec::new();
        for (ctx, f) in &callables {
            for s in signature_strings(f) {
                if skipped(SIGNATURE_GAPS, *ctx, s) {
                    continue;
                }
                let want = &reflected[&(ctx.map(str::to_string), s.to_string())];
                let got = describe(s, *ctx);
                if got != *want {
                    failures.push(format!(
                        "{ctx:?} {}: {s}\n  want {want:?}\n  got  {got:?}",
                        f.name
                    ));
                }
            }
        }
        eprintln!(
            "{version:?}: {} functions, {} methods; {} signature types differ",
            symbols.functions.len(),
            callables.len() - symbols.functions.len(),
            failures.len()
        );
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}

#[test]
fn constants() {
    let reflected: HashMap<String, [String; 3]> = rows("reflected-types.json")
        .into_iter()
        .filter(|(ctx, ..)| ctx.is_none())
        .map(|(_, s, d, ..)| (s, d))
        .collect();
    let symbols = symbols::symbols(PhpVersion::Php84);
    let mut failures = Vec::new();
    for (name, ty) in &symbols.constants {
        // A constant named like a class (or a keyword) is not a constant to a PHPDoc type.
        if !name
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
            || symbols::class(name).is_some()
        {
            continue;
        }
        let got = describe(name, None);
        if got != reflected[ty] {
            failures.push(format!(
                "{name}: {ty}\n  want {:?}\n  got  {got:?}",
                reflected[ty]
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Probes Pun does not model yet (open for M2.3): a constant-key array type against `list`, subtracted mixed,
/// object-shape and conditional intersections, callable arrays checked against the class's methods, generic
/// arguments on a non-generic class.
const PROBE_GAPS: &[&str] = &[
    "array{'Countable', 'count'}&callable",
    "array<1|2, int>&list",
    "non-empty-mixed",
    "object{a: int}&object{b: int}",
    "(T is int ? string : bool)&int",
    // its iterable value is `static`, which symbols.json has as the bare class
    "SimpleXMLElement<int, string, bool>",
    // call-site variance (`GenericObjectType`'s variances) is parsed but not kept
    "ArrayIterator<int, covariant string>",
    "ArrayIterator<contravariant int, string>",
    // PHPStan 2.3's TypeCombinator: a possibly-empty shape intersected with a list keeps its 0..n key prefix
    // (`ConstantArrayType::makeList()`), where Pun still gives `*NEVER*`
    "non-empty-list&array{a?: int}",
    "array<'a', int>&list",
    "list<int>&array{a?: int}",
    "non-empty-list<int>&array{a?: int}",
    "array{a?: int}&list<int>",
    "array{a?: int}&non-empty-list<int>",
    // PHPStan 2.3's union keeps a template argument next to its bound's members (`int|string|TKey of
    // (int|string)`), where Pun lets the template absorb them
    "array|static",
    "array|$this",
    "array{a: int}|static",
    "array{a: int}|$this",
    "list<int>|static",
    "list<int>|$this",
    "non-empty-array<int>|static",
    "non-empty-array<int>|$this",
    "static|array",
    "static|array{a: int}",
    "static|list<int>",
    "static|non-empty-array<int>",
    "$this|array",
    "$this|array{a: int}",
    "$this|list<int>",
    "$this|non-empty-array<int>",
];

/// `isSuperTypeOf()` pairs Pun does not answer like PHPStan 2.3 yet: a possibly-empty shape is maybe a list
/// (2.3's `ConstantArrayType::isList()` over optional keys), and a generic class's invariant template argument
/// compared with a wider one (2.3's template-argument variance check).
const SUPER_GAPS: &[(&str, &str)] = &[
    ("list<int>", "array{a?: int}"),
    ("non-empty-list<int>", "array{a?: int}"),
    ("ArrayIterator<int, int>", "ArrayIterator<array-key, int>"),
];

#[test]
fn probes() {
    check("type-probes.json", PROBE_GAPS);
}

/// `$a->isSuperTypeOf($b)` for every ordered pair of the probe atoms.
#[test]
fn super_types() {
    let f = fixture("type-super.json");
    let rows = f.as_array().unwrap();
    let mut failures = Vec::new();
    for row in rows {
        let [ctx, a, b, want] = [0, 1, 2, 3].map(|i| row[i].as_str());
        if SUPER_GAPS.contains(&(a.unwrap(), b.unwrap())) {
            continue;
        }
        let (a, b) = (resolve_str(a.unwrap(), ctx), resolve_str(b.unwrap(), ctx));
        let got = format!("{:?}", is_super_type_of(&a, &b));
        if got != want.unwrap() {
            failures.push(format!(
                "{ctx:?} {} isSuperTypeOf {}: want {}, got {got}",
                row[1],
                row[2],
                want.unwrap()
            ));
        }
    }
    eprintln!(
        "type-super.json: {}/{} match",
        rows.len() - failures.len(),
        rows.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Nette's `Strings::truncate` cuts before whitespace or ASCII punctuation, DEL not included: PHPStan 2.3.1's
/// `(new ConstantStringType("aaaa bbbbbbbbbbbbbb\x7fcccc"))->describe(VerbosityLevel::value())`.
#[test]
fn long_constant_string_truncates_before_punctuation() {
    let ty = resolve_str("'aaaa bbbbbbbbbbbbbb\u{7f}cccc'", None);
    assert_eq!(ty.describe(Level::Value), "'aaaa…'");
}

const VERBOSITY_GAPS: &[&str] = &["Traversable<int>"];

/// ponytail: a callable array's offsets (IntersectionType::hasOffsetValueType()) are not ported; checkOurKeys() sees a
/// plain array's
const ACCEPT_GAPS: &[(&str, &str)] = &[("array{ArrayIterator<int, string>, ArrayIterator<int, int>}", "callable&array")];

/// TemplateStrictMixedType::isAcceptedBy() (`isSubTypeOf()`): the same template of mixed accepts it, another maybe.
#[test]
fn template_strict_mixed() {
    use pun_phpstan::types::{StrictMixed, Tri, accepts_with};
    let t = resolve_str("__template<T, 'function probe()', mixed>", None);
    let u = resolve_str("__template<U, 'function probe()', mixed>", None);
    let sm = StrictMixed { explicit: true, implicit: true };
    assert_eq!(accepts_with(&t, &t, true, sm), Tri::Yes);
    assert_eq!(accepts_with(&u, &t, true, sm), Tri::Maybe);
}

/// `$a->accepts($b, $strictTypes)` (strict and not) and `VerbosityLevel::getRecommendedLevelByType($a, $b)` for every
/// ordered pair of the accept atoms; `strict-mixed` is RuleLevelHelper's StrictMixedType, an explicit `mixed` (or a
/// template of mixed) under `StrictMixed`.
#[test]
fn accepts() {
    use pun_phpstan::types::{StrictMixed, accepts_with, recommended_level};
    let f = fixture("type-accepts.json");
    let rows = f.as_array().unwrap();
    let mut failures = Vec::new();
    for row in rows {
        let [ctx, a, b, strict, loose, level] = [0, 1, 2, 3, 4, 5].map(|i| row[i].as_str());
        let at = resolve_str(a.unwrap(), ctx);
        let got_level = match b {
            None => recommended_level(&at, None),
            Some("strict-mixed") => recommended_level(&at, Some(&resolve_str("mixed", None))),
            Some(b) => recommended_level(&at, Some(&resolve_str(b, ctx))),
        };
        let mut got = vec![format!("{got_level:?}")];
        let mut want = vec![level.unwrap().to_string()];
        if let Some(b) = b {
            let (bts, sm) = if b == "strict-mixed" {
                let all = StrictMixed { explicit: true, implicit: true };
                (vec![resolve_str("mixed", None), resolve_str("__template<T, 'function strictProbe()', mixed>", None)], all)
            } else {
                (vec![resolve_str(b, ctx)], StrictMixed::default())
            };
            for bt in &bts {
                got.push(format!("{:?}", accepts_with(&at, bt, true, sm)));
                got.push(format!("{:?}", accepts_with(&at, bt, false, sm)));
                want.extend([strict, loose].map(|w| w.unwrap().to_string()));
            }
        }
        // ponytail: the symbols carry no template variance, so a covariant built-in template (Traversable's) counts as
        // invariant in getRecommendedLevelByType()
        if VERBOSITY_GAPS.contains(&a.unwrap()) {
            got.remove(0);
            want.remove(0);
        }
        if got != want && !ACCEPT_GAPS.contains(&(a.unwrap(), b.unwrap_or(""))) {
            failures.push(format!("{ctx:?} {} accepts {}: want {want:?}, got {got:?}", row[1], row[2]));
        }
    }
    eprintln!("type-accepts.json: {}/{} match", rows.len() - failures.len(), rows.len());
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Whether `toString()`, `toNumber()`, `toInteger()`, `toFloat()` and `toBitwiseNotType()` are an ErrorType
/// (`toBoolean()` never is), and `isCloneable()`.
#[test]
fn casts() {
    use pun_phpstan::types::{Cast, StrictMixed, cast_is_error};
    let f = fixture("type-casts.json");
    let mut failures = Vec::new();
    for row in f.as_array().unwrap() {
        let (ctx, a) = (row[0].as_str(), row[1].as_str().unwrap());
        let (t, sm) = if a == "strict-mixed" {
            (resolve_str("mixed", None), StrictMixed { explicit: true, implicit: false })
        } else {
            (resolve_str(a, ctx), StrictMixed::default())
        };
        let casts = [Cast::String, Cast::Number, Cast::Integer, Cast::Float, Cast::BitwiseNot];
        let got: Vec<bool> = casts.iter().map(|&c| cast_is_error(&t, c, sm)).collect();
        let want: Vec<bool> = (2..7).map(|i| row[i].as_bool().unwrap()).collect();
        if got != want || cast_is_error(&t, Cast::Boolean, sm) {
            failures.push(format!("{ctx:?} {a}: want {want:?}, got {got:?}"));
        }
        let cloneable = format!("{:?}", t.is_cloneable());
        if row[7].as_str().is_some_and(|want| want != cloneable) {
            failures.push(format!("{ctx:?} {a}: isCloneable() want {}, got {cloneable}", row[7]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A StaticType of a class the symbols do not know is its plain object (`["Nope", "static", "static", "Yes", ...]` in
/// type-accepts.json); the resolver makes an ErrorType of such a `static`, the analyser a StaticType of a project
/// class outside `declared::with_classes()`.
#[test]
fn static_of_unknown_class() {
    use pun_phpstan::types::{Type, accepts};
    let s = Type::Static("Nope".into(), vec![]);
    assert_eq!(format!("{:?}", accepts(&s, &s, true)), "Yes");
}

/// `inferTemplateTypes()` of each pattern from each received type, and the pattern resolved over the inferred map
/// (type-infer.json).
#[test]
fn infer_template_types() {
    use pun_phpstan::types::{Template, Type, infer_template_types, resolve_template_types};
    let f = fixture("type-infer.json");
    let rows = f.as_array().unwrap();
    let mut failures = Vec::new();
    for row in rows {
        let (a, b) = (row[0].as_str().unwrap(), row[1].as_str().unwrap());
        let (at, bt) = (resolve_str(a, None), resolve_str(b, None));
        let map = infer_template_types(&at, &bt);
        let mut got: Vec<(String, String)> = map.types().into_iter().map(|(n, _, t)| (n.to_string(), t.describe(Level::Precise))).collect();
        got.sort();
        let want: Vec<(String, String)> =
            row[2].as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string())).collect();
        let templates = std::cell::RefCell::new(Vec::<Template>::new());
        fn collect(t: &Type, out: &std::cell::RefCell<Vec<Template>>) {
            if let Type::Template(tt) = t {
                out.borrow_mut().push((**tt).clone());
            }
            t.traverse(&|i| {
                collect(i, out);
                i.clone()
            });
        }
        collect(&at, &templates);
        let templates = templates.into_inner();
        let resolved = resolve_template_types(&at, &map.with_errors(&templates.iter().collect::<Vec<_>>())).describe(Level::Precise);
        if got != want || resolved != row[3].as_str().unwrap() {
            failures.push(format!("{a} from {b}: want {want:?} {}, got {got:?} {resolved}", row[3]));
        }
    }
    eprintln!("type-infer.json: {}/{} match", rows.len() - failures.len(), rows.len());
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The reasons `accepts()` gives (type-reasons.json): every ordered pair of the reason atoms.
#[test]
fn accepts_reasons() {
    use pun_phpstan::types::{StrictMixed, accepts_reasons};
    let f = fixture("type-reasons.json");
    let rows = f.as_array().unwrap();
    let mut failures = Vec::new();
    for row in rows {
        let (a, b) = (row[0].as_str().unwrap(), row[1].as_str().unwrap());
        let got = accepts_reasons(&resolve_str(a, None), &resolve_str(b, None), true, StrictMixed::default());
        let want: Vec<String> = row[2].as_array().unwrap().iter().map(|r| r.as_str().unwrap().to_string()).collect();
        if got != want {
            failures.push(format!("{a} accepts {b}: want {want:?}, got {got:?}"));
        }
    }
    eprintln!("type-reasons.json: {}/{} match", rows.len() - failures.len(), rows.len());
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
