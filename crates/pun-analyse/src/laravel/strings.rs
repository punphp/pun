//! The parts of Laravel's `Illuminate\Support\Str` that model and column names go through.

/// `class_basename()`: the class name without its namespace.
pub(super) fn class_basename(class: &str) -> &str {
    class.rsplit('\\').next().unwrap_or(class)
}

/// `Str::snake($value)` on a class name (no whitespace, so `ucwords` and the whitespace removal change nothing): `_`
/// before every capital that follows a character, lowercase.
pub(super) fn snake(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        if c.is_ascii_uppercase() && !out.is_empty() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// `Str::studly($value)`: words split on `-`, `_` and spaces, each capitalised, joined.
pub(super) fn studly(value: &str) -> String {
    value
        .split(['-', '_', ' '])
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or_else(String::new, |f| {
                f.to_ascii_uppercase().to_string() + c.as_str()
            })
        })
        .collect()
}

/// `Str::camel($value)`: `studly` with the first letter lowercase.
pub(super) fn camel(value: &str) -> String {
    let s = studly(value);
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| {
        f.to_ascii_lowercase().to_string() + c.as_str()
    })
}

/// `Str::pluralStudly($value)`: the last studly word made plural.
pub(super) fn plural_studly(value: &str) -> String {
    let split = value
        .char_indices()
        .filter(|&(i, c)| i > 0 && c.is_ascii_uppercase())
        .map(|(i, _)| i)
        .next_back()
        .unwrap_or(0);
    format!("{}{}", &value[..split], plural(&value[split..]))
}

/// `Str::plural($value)` for regular English nouns.
// ponytail: Doctrine's English inflector also has irregular and uncountable words (person, child, data, ...) and
// more suffix rules; these are its most common regular ones
fn plural(word: &str) -> String {
    let lower = word.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    if ["x", "ch", "ss", "sh"].iter().any(|s| lower.ends_with(s)) {
        format!("{word}es")
    } else if lower.ends_with('s') {
        word.to_string()
    } else if bytes.len() > 1 && lower.ends_with('y') && !b"aeiou".contains(&bytes[bytes.len() - 2])
    {
        format!("{}ies", &word[..word.len() - 1])
    } else {
        format!("{word}s")
    }
}
