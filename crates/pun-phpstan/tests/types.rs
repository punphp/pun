//! Types against PHPStan 2.2.7 (scripts/export-phpstan-symbols.php): every signature-map type string through
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
    for (ctx, input, want, pun, iterable) in &rows {
        if skip.contains(&input.as_str()) {
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
        rows.len() - failures.len() - skip.len(),
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
    check("reflected-types.json", UNNORMALIZED);
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

/// Nette's `Strings::truncate` cuts before whitespace or ASCII punctuation, DEL not included: PHPStan 2.2.7's
/// `(new ConstantStringType("aaaa bbbbbbbbbbbbbb\x7fcccc"))->describe(VerbosityLevel::value())`.
#[test]
fn long_constant_string_truncates_before_punctuation() {
    let ty = resolve_str("'aaaa bbbbbbbbbbbbbb\u{7f}cccc'", None);
    assert_eq!(ty.describe(Level::Value), "'aaaa…'");
}
