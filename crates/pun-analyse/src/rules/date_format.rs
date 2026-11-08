//! PHPStan 2.3.5's DateFormatFunctionReturnTypeExtension (`date_format()`) and DateFormatMethodReturnTypeExtension
//! (`DateTimeInterface::format()`) as Mago return type providers, over DateFunctionReturnTypeHelper. The helper runs
//! PHP's `date()` on a format it has no case for, in the default timezone the oracle's PHP runs with (UTC).

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::string::{TString, TStringCasing};
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{combine_union_types, get_string};

static META: ProviderMeta = ProviderMeta::new(
    "pun-date-format",
    "pun",
    "PHPStan's date format return types",
);

pub(super) struct DateFormat;

impl Provider for DateFormat {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for DateFormat {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"date_format")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        if invocation.argument_count() < 2 {
            return Some(get_string());
        }
        let format = context.get_expression_type(invocation.get_argument(1, &[b"format"])?)?;
        Some(type_from_format_type(context.codebase(), format))
    }
}

impl MethodReturnTypeProvider for DateFormat {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 1] = [MethodTarget::any_class(b"format")];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        _: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        // the class the method is called on (Mago names the declaring one, which can be a trait)
        let class = invocation
            .inner()
            .target
            .get_method_context()
            .map_or(class, |t| t.class_like_metadata.name.as_bytes());
        if !class.eq_ignore_ascii_case(b"DateTimeInterface")
            && !context.is_instance_of(class, b"DateTimeInterface")
        {
            return None;
        }
        if invocation.argument_count() == 0 {
            return Some(get_string());
        }
        let format = context.get_expression_type(invocation.get_argument(0, &[b"format"])?)?;
        Some(type_from_format_type(context.codebase(), format))
    }
}

/// `getConstantStrings()`: every member a known literal string, else none.
fn constant_strings(t: &TUnion) -> Vec<&[u8]> {
    t.types
        .iter()
        .map(|a| match a {
            TAtomic::Scalar(TScalar::String(s)) => s.get_known_literal_value(),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default()
}

fn string(t: TString) -> TUnion {
    TUnion::from_atomic(TAtomic::Scalar(TScalar::String(t)))
}

fn literal(s: &[u8]) -> TUnion {
    string(TString::known_literal(mago_word::word(s)))
}

fn non_empty_string() -> TUnion {
    string(TString::non_empty())
}

/// DateFunctionReturnTypeHelper::getTypeFromFormatType() with `$useMicrosec` (both extensions pass `true`).
fn type_from_format_type(codebase: &CodebaseMetadata, format: &TUnion) -> TUnion {
    let union =
        |a: &TUnion, b: &TUnion| combine_union_types(a, b, codebase, CombinerOptions::default());
    let types: Vec<TUnion> = constant_strings(format)
        .into_iter()
        .map(from_format)
        .collect();
    let non_empty_format = format
        .types
        .iter()
        .all(|a| matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.is_non_empty()));
    match types.split_first() {
        None if non_empty_format => non_empty_string(),
        None => get_string(),
        Some((first, rest)) => rest.iter().fold(first.clone(), |t, next| union(&t, next)),
    }
}

/// `buildReturnTypeFromFormat()` with `$useMicrosec`.
fn from_format(format: &[u8]) -> TUnion {
    let range = |min: u32, max: u32, pad: bool| {
        TUnion::from_vec(
            (min..=max)
                .map(|i| {
                    let s = if pad {
                        format!("{i:02}")
                    } else {
                        i.to_string()
                    };
                    TAtomic::Scalar(TScalar::String(TString::known_literal(mago_word::word(
                        s.as_bytes(),
                    ))))
                })
                .collect(),
        )
    };
    match format {
        b"d" => return range(1, 31, true),
        b"j" => return range(1, 31, false),
        b"N" => return range(1, 7, false),
        b"w" => return range(0, 6, false),
        b"m" => return range(1, 12, true),
        b"n" | b"g" => return range(1, 12, false),
        b"t" => return range(28, 31, false),
        b"L" | b"I" => return range(0, 1, false),
        b"G" => return range(0, 23, false),
        b"h" => return range(1, 12, true),
        b"H" => return range(0, 23, true),
        b"u" | b"v" => {
            return string(TString::general_with_props(
                true,
                true,
                true,
                false,
                TStringCasing::Unspecified,
            ));
        }
        // values which do not depend on the locale (PHPStan 2.3.3)
        b"a" => return strings(&["am", "pm"]),
        b"A" => return strings(&["AM", "PM"]),
        b"S" => return strings(&["st", "nd", "rd", "th"]),
        b"D" => return strings(&["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        b"l" => {
            return strings(&[
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ]);
        }
        b"M" => {
            return strings(&[
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]);
        }
        b"F" => {
            return strings(&[
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]);
        }
        _ => {}
    }
    let date = date(format);
    if date == format {
        return literal(&date);
    }
    if std::str::from_utf8(&date).is_ok_and(is_numeric) {
        return string(TString::numeric());
    }
    string(TString::truthy())
}

/// `buildConstantStringsType()`
fn strings(values: &[&str]) -> TUnion {
    TUnion::from_vec(
        values
            .iter()
            .map(|v| {
                TAtomic::Scalar(TScalar::String(TString::known_literal(mago_word::word(
                    v.as_bytes(),
                ))))
            })
            .collect(),
    )
}

/// PHP 8's `is_numeric()` of a string.
fn is_numeric(s: &str) -> bool {
    pun_phpstan::value::numeric(s).is_some()
}

/// PHP's `date($format)` in UTC (ext/date's `date_format()`), of the Unix epoch. The helper only asks whether the
/// result is numeric or the format itself, and every format character's output has the same shape (digits, a sign,
/// letters) at any time of years 1000 to 9999, so one fixed timestamp answers as the current time does.
fn date(format: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut bytes = format.iter();
    while let Some(&c) = bytes.next() {
        let part: &[u8] = match c {
            b'd' | b'm' | b'W' => b"01",
            b'D' => b"Thu",
            b'j' | b'n' => b"1",
            b'l' => b"Thursday",
            b'N' | b'w' => b"4",
            b'S' => b"st",
            b'z' | b'L' | b'G' | b'I' | b'Z' | b'U' => b"0",
            b'F' => b"January",
            b'M' => b"Jan",
            b't' => b"31",
            b'o' | b'x' | b'Y' => b"1970",
            b'X' => b"+1970",
            b'y' => b"70",
            b'a' => b"am",
            b'A' => b"AM",
            b'B' => b"041",
            b'g' | b'h' => b"12",
            b'H' | b'i' | b's' => b"00",
            b'u' => b"000000",
            b'v' => b"000",
            b'e' | b'T' => b"UTC",
            b'O' => b"+0000",
            b'P' => b"+00:00",
            b'p' => b"Z",
            b'c' => b"1970-01-01T00:00:00+00:00",
            b'r' => b"Thu, 01 Jan 1970 00:00:00 +0000",
            // a backslash copies the next byte, and any other byte is copied as is
            b'\\' => {
                out.extend(bytes.next());
                continue;
            }
            c => {
                out.push(c);
                continue;
            }
        };
        out.extend_from_slice(part);
    }
    out
}
