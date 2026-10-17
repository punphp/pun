//! Composer version constraints, just enough to find the lowest version a constraint admits.

pub type V = (u64, u64, u64);

#[derive(Clone, Copy)]
struct Range {
    lo: V,
    hi: Option<(V, bool)>, // (version, inclusive)
}

/// The lowest version the composer constraint admits, or `None` when it admits none (`>=8.4 <8.3`).
/// `|` and `||` are OR, spaces and commas are AND, as in composer/semver's `VersionParser::parseConstraints`.
pub fn lowest(constraint: &str) -> Result<Option<V>, String> {
    let mut low: Option<V> = None;
    for alt in constraint
        .split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let r = conjunction(alt).map_err(|e| format!("invalid constraint `{constraint}`: {e}"))?;
        let nonempty = match r.hi {
            None => true,
            Some((hi, inc)) => r.lo < hi || (inc && r.lo == hi),
        };
        if nonempty {
            low = Some(low.map_or(r.lo, |l| l.min(r.lo)));
        }
    }
    Ok(low)
}

fn conjunction(s: &str) -> Result<Range, String> {
    // Hyphen range: "8.1 - 8.3".
    if let Some((a, b)) = s.split_once(" - ") {
        let (a, b) = (parse(a.trim())?, parse(b.trim())?);
        let hi = match b {
            (x, Some(y), Some(z)) => ((x, y, z), true),
            (x, Some(y), None) => ((x, y + 1, 0), false),
            (x, None, _) => ((x + 1, 0, 0), false),
        };
        return Ok(Range {
            lo: pad(a),
            hi: Some(hi),
        });
    }
    // Split on spaces/commas, re-attaching bare operators ("> = 8.1" style is not valid composer).
    let mut parts: Vec<String> = Vec::new();
    let mut pending = String::new();
    for tok in s.split([' ', ',']).filter(|t| !t.is_empty()) {
        if tok.chars().all(|c| "<>=!".contains(c)) {
            pending.push_str(tok);
        } else {
            parts.push(std::mem::take(&mut pending) + tok);
        }
    }
    if !pending.is_empty() {
        return Err(format!("missing version after `{pending}`"));
    }
    let mut r = Range {
        lo: (0, 0, 0),
        hi: None,
    };
    for p in parts {
        let c = single(&p)?;
        r.lo = r.lo.max(c.lo);
        r.hi = match (r.hi, c.hi) {
            (None, h) | (h, None) => h,
            (Some(a), Some(b)) => Some(if b.0 < a.0 || (b.0 == a.0 && !b.1) {
                b
            } else {
                a
            }),
        };
    }
    Ok(r)
}

fn single(p: &str) -> Result<Range, String> {
    let any = Range {
        lo: (0, 0, 0),
        hi: None,
    };
    let at = |v: V, hi: Option<(V, bool)>| Range { lo: v, hi };
    let (op, ver) = match p.find(|c: char| !"<>=!^~".contains(c)) {
        Some(i) => p.split_at(i),
        None => return Err(format!("missing version in `{p}`")),
    };
    if ver == "*" || ver == "x" {
        return Ok(any);
    }
    let v = parse(ver)?;
    let (x, y, z) = v;
    let full = pad(v);
    Ok(match op {
        "^" => at(
            full,
            Some((
                match (x, y, z) {
                    (0, Some(0), Some(z)) => (0, 0, z + 1),
                    (0, Some(y), _) if y > 0 || z.is_none() => (0, y + 1, 0),
                    (0, None, _) => (1, 0, 0),
                    _ => (x + 1, 0, 0),
                },
                false,
            )),
        ),
        "~" => at(
            full,
            Some((
                if let (Some(y), Some(_)) = (y, z) {
                    (x, y + 1, 0)
                } else {
                    (x + 1, 0, 0)
                },
                false,
            )),
        ),
        ">=" => at(full, None),
        ">" => Range {
            lo: bump_tiny(full),
            hi: None,
        },
        "<" => Range {
            lo: (0, 0, 0),
            hi: Some((full, false)),
        },
        "<=" => Range {
            lo: (0, 0, 0),
            hi: Some((full, true)),
        },
        "!=" | "<>" => any,
        "" | "=" | "==" if is_wild(ver) => at(full, Some((wild_upper(v), false))),
        "" | "=" | "==" => at(full, Some((full, true))),
        _ => return Err(format!("unknown operator `{op}`")),
    })
}

// ">8.3" admits 8.3.1, so the lowest admitted version is just above 8.3.0.
fn bump_tiny((x, y, z): V) -> V {
    (x, y, z + 1)
}

fn is_wild(ver: &str) -> bool {
    ver.ends_with(".*") || ver.ends_with(".x") || ver.ends_with(".X")
}

fn wild_upper(v: (u64, Option<u64>, Option<u64>)) -> V {
    match v {
        (x, None, _) => (x + 1, 0, 0),
        (x, Some(y), None) => (x, y + 1, 0),
        (x, Some(y), Some(z)) => (x, y, z + 1),
    }
}

fn pad((x, y, z): (u64, Option<u64>, Option<u64>)) -> V {
    (x, y.unwrap_or(0), z.unwrap_or(0))
}

/// "8", "8.4", "v8.4.1", "8.4.*", "8.4.0-dev", "8.4@stable" -> components; wildcards become None.
fn parse(s: &str) -> Result<(u64, Option<u64>, Option<u64>), String> {
    let s = s.trim().trim_start_matches(['v', 'V']);
    let s = s.split(['@', '-', '+']).next().unwrap_or("");
    let mut it = s.split('.').map(|c| match c {
        "*" | "x" | "X" => Ok(None),
        _ => c
            .parse::<u64>()
            .map(Some)
            .map_err(|_| format!("bad version `{s}`")),
    });
    let x = it.next().ok_or("empty version")??.ok_or("bad major")?;
    let y = it.next().transpose()?.flatten();
    let z = if y.is_some() {
        it.next().transpose()?.flatten()
    } else {
        None
    };
    Ok((x, y, z))
}

#[cfg(test)]
mod tests {
    use super::{V, lowest};

    fn allows_below(c: &str, b: V) -> Result<bool, String> {
        Ok(lowest(c)?.is_some_and(|l| l < b))
    }

    /// Expected values from composer/semver (Pint's vendor): `parseConstraints($c)->matches(parseConstraints('<8.4.0.0-dev'))`.
    #[test]
    fn php_84_gate_matches_composer() {
        let b = (8, 4, 0);
        let cases = [
            ("^8.4", false),
            ("~8.4.0", false),
            (">=8.4", false),
            (">= 8.4", false),
            ("8.4.*", false),
            ("^8.4.0", false),
            (">=8.4 <9.0", false),
            ("<9 >=8.4", false),
            ("8.4", false),
            ("==8.4", false),
            ("v8.4", false),
            ("8.4@stable", false),
            ("8.4.0-dev", false),
            (">8.3.99 <8.3.99|^8.4", false),
            (">=8.4 <8.3", false),
            ("8.4 - 8.5", false),
            ("8.4 - 8.3", false),
            ("^8.2", true),
            (">=8.1", true),
            ("8.3.*|^8.4", true),
            ("8.3.* || ^8.4", true),
            ("~8.3", true),
            ("~8.3.1", true),
            ("~8", true),
            ("^7.4|^8.0", true),
            ("*", true),
            ("x", true),
            (">=8.2,<8.5", true),
            (">8.3", true),
            ("<9", true),
            ("8.1 - 8.4", true),
            ("8.1 - 8.3.2", true),
            ("8.1 - 8", true),
            ("^0.0.3", true),
            ("^0.2", true),
            ("^0", true),
            ("^0.0", true),
            ("<8.4", true),
            ("<=8.4", true),
            ("!=8.4", true),
            ("<>8.0", true),
            ("=8.3", true),
            ("8.*", true),
            ("8.x", true),
            ("8.3.x", true),
            ("8.3.1.*", true),
            ("^8.3 <8.3.5", true),
            ("<8.5 <=8.5", true),
            ("<8.5, <=8.5", true),
            ("<8.5 <8.5", true),
            ("<=8.3 <8.3", true),
        ];
        for (c, below) in cases {
            assert_eq!(allows_below(c, b), Ok(below), "{c}");
        }
        // composer: "Could not parse version constraint ..."
        for bad in [">", "^", ">=8.4 <", "abc", "%8.4", "=>8.4", "!8.4"] {
            assert!(allows_below(bad, b).is_err(), "{bad} should not parse");
        }
    }
}
