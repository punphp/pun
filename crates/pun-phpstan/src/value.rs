//! PHP values as nette/neon and Nette DI produce them: ordered arrays with int|string keys (PHP's key coercion and
//! next-index rules), entities/statements, and the scalar conversions PHP applies on the way.

use indexmap::IndexMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Int(i64),
    Str(String),
}

impl Key {
    /// PHP array-key coercion of a string: canonical decimal integers become int keys.
    pub fn from_string(s: String) -> Key {
        match canonical_int(&s) {
            Some(i) => Key::Int(i),
            None => Key::Str(s),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Key::Int(i) => i.to_string(),
            Key::Str(s) => s.clone(),
        }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        Key::from_string(s.to_string())
    }
}

/// `"123"`, `"-5"`, not `"0123"`, `"-0"`, `"+1"` or out of i64 range (zend_handle_numeric_str).
fn canonical_int(s: &str) -> Option<i64> {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
    {
        return None;
    }
    if s == "-0" {
        return None;
    }
    s.parse().ok()
}

/// A PHP array: insertion-ordered, int|string keys, `[]` appends at the next free int index.
#[derive(Clone, Debug, Default)]
pub struct Array {
    map: IndexMap<Key, Value>,
    next: Option<i64>,
}

impl PartialEq for Array {
    /// Ordered comparison (PHP `===` on arrays also compares order).
    fn eq(&self, other: &Array) -> bool {
        self.map.len() == other.map.len()
            && self.map.iter().zip(other.map.iter()).all(|(a, b)| a == b)
    }
}

impl Array {
    pub fn new() -> Array {
        Array::default()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&Value> {
        self.map.get(&Key::from(key))
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.map.get_mut(key)
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.map.contains_key(key)
    }

    pub fn set(&mut self, key: Key, value: Value) {
        if let Key::Int(i) = key
            && self.next.is_none_or(|n| i >= n)
        {
            self.next = Some(i.saturating_add(1));
        }
        self.map.insert(key, value);
    }

    pub fn push(&mut self, value: Value) {
        let i = self.next.unwrap_or(0);
        self.set(Key::Int(i), value);
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.map.shift_remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.map.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.map.values()
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.map.keys()
    }

    /// `array_is_list()`.
    pub fn is_list(&self) -> bool {
        self.map
            .keys()
            .enumerate()
            .all(|(i, k)| *k == Key::Int(i as i64))
    }

    pub fn list(values: impl IntoIterator<Item = Value>) -> Array {
        let mut a = Array::new();
        for v in values {
            a.push(v);
        }
        a
    }
}

impl IntoIterator for Array {
    type Item = (Key, Value);
    type IntoIter = indexmap::map::IntoIter<Key, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

/// A NEON entity `value(attributes)`, which Nette DI turns into a `Statement` with the same shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub value: Value,
    pub attributes: Array,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Array(Array),
    Entity(Box<Entity>),
    /// A NEON date/time literal (`new \DateTimeImmutable($raw)`), kept as written.
    DateTime(String),
    /// PHPStan's `OptionalPath` (an `excludePaths` entry written as `path (?)`).
    OptionalPath(String),
}

impl Value {
    pub fn str(s: impl Into<String>) -> Value {
        Value::Str(s.into())
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// PHP `gettype()`.
    pub fn gettype(&self) -> &'static str {
        match self {
            Value::Null => "NULL",
            Value::Bool(_) => "boolean",
            Value::Int(_) => "integer",
            Value::Float(_) => "double",
            Value::Str(_) => "string",
            Value::Array(_) => "array",
            _ => "object",
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Str(_)
        )
    }

    /// PHP `(string)` cast of a scalar or null (`None` for arrays and objects).
    pub fn to_php_string(&self) -> Option<String> {
        Some(match self {
            Value::Null | Value::Bool(false) => String::new(),
            Value::Bool(true) => "1".into(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => float_to_string(*f),
            Value::Str(s) => s.clone(),
            _ => return None,
        })
    }

    /// PHP `var_export($v, true)` for scalars and null.
    pub fn var_export(&self) -> String {
        match self {
            Value::Null => "NULL".into(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => float_var_export(*f),
            Value::Str(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
            other => format!("{other:?}"),
        }
    }
}

/// zend_gcvt: `digits`/`decpt` from a correctly rounded decimal expansion, scientific when the exponent is below -4 or
/// at least `precision`.
fn gcvt(f: f64, digits: String, exp: i32, precision: i32, e: char) -> String {
    let sign = if f.is_sign_negative() { "-" } else { "" };
    let digits = digits.trim_end_matches('0');
    let digits = if digits.is_empty() { "0" } else { digits };
    let decpt = if f == 0.0 { 1 } else { exp + 1 };
    if decpt < -3 || decpt > precision {
        let (first, rest) = digits.split_at(1);
        let rest = if rest.is_empty() { "0" } else { rest };
        let x = decpt - 1;
        format!(
            "{sign}{first}.{rest}{e}{}{}",
            if x < 0 { '-' } else { '+' },
            x.abs()
        )
    } else if decpt <= 0 {
        format!("{sign}0.{}{digits}", "0".repeat(-decpt as usize))
    } else {
        let d = decpt as usize;
        if digits.len() <= d {
            format!("{sign}{digits}{}", "0".repeat(d - digits.len()))
        } else {
            format!("{sign}{}.{}", &digits[..d], &digits[d..])
        }
    }
}

fn split_exp(s: &str) -> (String, i32) {
    let s = s.trim_start_matches('-');
    let (m, e) = s.split_once('e').expect("exponent");
    (m.replace('.', ""), e.parse().expect("exponent"))
}

fn special(f: f64) -> Option<String> {
    if f.is_nan() {
        Some("NAN".into())
    } else if f.is_infinite() {
        Some(if f > 0.0 { "INF" } else { "-INF" }.into())
    } else {
        None
    }
}

/// `(string) $float` (precision=14).
pub fn float_to_string(f: f64) -> String {
    if let Some(s) = special(f) {
        return s;
    }
    let (d, e) = split_exp(&format!("{f:.13e}"));
    gcvt(f, d, e, 14, 'E')
}

/// Shortest round-trip digits (serialize_precision=-1), as `json_encode` prints them.
pub fn float_json(f: f64) -> String {
    let (d, e) = split_exp(&format!("{f:e}"));
    gcvt(f, d, e, 17, 'e')
}

/// `var_export($float, true)`.
pub fn float_var_export(f: f64) -> String {
    if let Some(s) = special(f) {
        return s;
    }
    let (d, e) = split_exp(&format!("{f:e}"));
    let s = gcvt(f, d, e, 17, 'E');
    if s.contains(['.', 'E']) { s } else { s + ".0" }
}

/// PHP `is_numeric()` + `$value * 1`: leading/trailing whitespace, sign, digits, fraction, exponent; integers that
/// fit stay int, everything else is float.
pub fn numeric(s: &str) -> Option<Value> {
    let ws = |c: char| matches!(c, ' ' | '\t' | '\n' | '\r' | '\x0B' | '\x0C');
    let t = s.trim_start_matches(ws).trim_end_matches(ws);
    let b = t.as_bytes();
    let mut i = usize::from(matches!(b.first(), Some(b'+' | b'-')));
    let int_start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let int_digits = i - int_start;
    let mut frac_digits = 0;
    let mut float = false;
    if i < b.len() && b[i] == b'.' {
        float = true;
        i += 1;
        let f = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        frac_digits = i - f;
    }
    if int_digits + frac_digits == 0 {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        let d = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j > d {
            float = true;
            i = j;
        }
    }
    if i != b.len() {
        return None;
    }
    if !float && let Ok(n) = t.parse::<i64>() {
        return Some(Value::Int(n));
    }
    Some(Value::Float(t.parse().expect("validated float syntax")))
}

/// `hexdec()`/`octdec()`/`bindec()` on digits of that base: int while it fits, float after.
pub fn base_dec(digits: &str, base: u32) -> Value {
    let mut i: i64 = 0;
    let mut f: f64 = 0.0;
    let mut overflow = false;
    for c in digits.chars() {
        let d = c.to_digit(base).expect("digit") as i64;
        if !overflow {
            match i.checked_mul(base as i64).and_then(|x| x.checked_add(d)) {
                Some(x) => i = x,
                None => {
                    overflow = true;
                    f = i as f64 * base as f64 + d as f64;
                }
            }
        } else {
            f = f * base as f64 + d as f64;
        }
    }
    if overflow {
        Value::Float(f)
    } else {
        Value::Int(i)
    }
}

/// `(new \DateTimeImmutable($raw))->format($format)` for the NEON date/time literal forms (`Y-m-d`, optional
/// `[Tt ]H:i:s[.frac]`, optional `Z`/`±H[H][[:]MM]`), with formats built from `Y m d H i s u O P`.
/// ponytail: a literal without a zone is read as UTC (PHP uses php.ini's date.timezone).
pub fn datetime_format(raw: &str, format: &str) -> String {
    let num = |s: &str| s.parse::<i64>().unwrap_or(0);
    let (date, rest) = raw.split_at(
        raw.find(['T', 't', ' '])
            .unwrap_or(raw.len()),
    );
    let mut d = date.split('-').map(num);
    let (y, m, day) = (
        d.next().unwrap_or(0),
        d.next().unwrap_or(1),
        d.next().unwrap_or(1),
    );
    let rest = rest.trim_start_matches(['T', 't', ' ']);
    let zone_at = rest.find(['Z', '+', '-']).unwrap_or(rest.len());
    let (time, zone) = rest.split_at(zone_at);
    let time = time.trim_end();
    let (hms, frac) = time.split_once('.').unwrap_or((time, ""));
    let mut t = hms.split(':').map(num);
    let (h, i, s) = (
        t.next().unwrap_or(0),
        t.next().unwrap_or(0),
        t.next().unwrap_or(0),
    );
    let micro = num(&format!("{:0<6}", &frac[..frac.len().min(6)]));
    let offset = match zone {
        "" | "Z" => 0,
        z => {
            let sign = if z.starts_with('-') { -1 } else { 1 };
            let digits: String = z[1..].chars().filter(|c| c.is_ascii_digit()).collect();
            let (hh, mm) = if digits.len() > 2 {
                digits.split_at(digits.len() - 2)
            } else {
                (digits.as_str(), "0")
            };
            sign * (num(hh) * 60 + num(mm))
        }
    };
    // days_from_civil / civil_from_days (H. Hinnant) normalise overflowing days like PHP does.
    let days = {
        let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * m + 2) / 5;
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468 + day - 1
    };
    let secs = days * 86400 + h * 3600 + i * 60 + s;
    let (days, sod) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let dd = doy - (153 * mp + 2) / 5 + 1;
    let mm = if mp < 10 { mp + 3 } else { mp - 9 };
    let yy = yoe + era * 400 + i64::from(mm <= 2);
    let sign = if offset < 0 { '-' } else { '+' };
    let (oh, om) = (offset.abs() / 60, offset.abs() % 60);
    let mut out = String::new();
    for c in format.chars() {
        match c {
            'Y' => out.push_str(&format!("{yy:04}")),
            'm' => out.push_str(&format!("{mm:02}")),
            'd' => out.push_str(&format!("{dd:02}")),
            'H' => out.push_str(&format!("{:02}", sod / 3600)),
            'i' => out.push_str(&format!("{:02}", sod / 60 % 60)),
            's' => out.push_str(&format!("{:02}", sod % 60)),
            'u' => out.push_str(&format!("{micro:06}")),
            'O' => out.push_str(&format!("{sign}{oh:02}{om:02}")),
            'P' => out.push_str(&format!("{sign}{oh:02}:{om:02}")),
            c => out.push(c),
        }
    }
    out
}
