//! `PhpCsFixer\DocBlock\Annotation`.

use super::line::LineRef;
use super::tag::Tag;
use super::type_expression::{REGEX_IDENTIFIER, REGEX_TYPES, TypeExpression};
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg, preg_quote};
use std::cell::{OnceCell, RefCell};

pub const TAGS_WITH_TYPES: &[&str] = &[
    "extends",
    "implements",
    "method",
    "param",
    "param-out",
    "phpstan-import-type",
    "phpstan-type",
    "phpstan-var",
    "property",
    "property-read",
    "property-write",
    "psalm-import-type",
    "psalm-type",
    "psalm-var",
    "return",
    "throws",
    "type",
    "var",
];

#[derive(Clone, Debug)]
pub struct Annotation {
    lines: Vec<LineRef>,
    start: usize,
    end: usize,
    tag: OnceCell<Tag>,
    types_content: RefCell<Option<Vec<u8>>>,
    types: RefCell<Option<Vec<Vec<u8>>>>,
}

impl Annotation {
    /// `lines` are `(index in the docblock, line)` pairs, like upstream's preserved-key slice.
    pub fn new(lines: Vec<(usize, LineRef)>) -> Self {
        let start = lines.first().unwrap().0;
        let end = lines.last().unwrap().0;
        Annotation {
            lines: lines.into_iter().map(|(_, l)| l).collect(),
            start,
            end,
            tag: OnceCell::new(),
            types_content: RefCell::new(None),
            types: RefCell::new(None),
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_tag(&self) -> &Tag {
        self.tag.get_or_init(|| Tag::new(self.lines[0].clone()))
    }

    pub fn get_type_expression(&self) -> Option<TypeExpression> {
        self.get_types_content().map(|c| TypeExpression::new(&c))
    }

    pub fn get_variable_name(&self) -> Option<Vec<u8>> {
        let types = self.get_types_content().unwrap_or_default();
        let content = self.get_content();
        if let Some(v) = variable_name_ascii(&content, self.get_tag().get_name(), &types) {
            return Some(v);
        }
        let ty = preg_quote(&types, Some(b'/'));
        let regex = [
            &b"/@"[..],
            self.get_tag().get_name(),
            br"\s+(",
            &ty,
            br"\s*)?(&\s*)?(\.{3}\s*)?(?<variable>\$",
            REGEX_IDENTIFIER.as_bytes(),
            b")(?:.*|$)/",
        ]
        .concat();
        // Built from the types, so kept out of the pattern cache (it would grow with every distinct type).
        Preg::match_groups_uncached(&regex, &content).map(|m| m.name_str("variable").to_vec())
    }

    pub fn get_types(&self) -> Vec<Vec<u8>> {
        if self.types.borrow().is_none() {
            let t = self
                .get_type_expression()
                .map_or_else(Vec::new, |e| e.get_types());
            *self.types.borrow_mut() = Some(t);
        }
        self.types.borrow().clone().unwrap()
    }

    pub fn set_types(&self, types: &[Vec<u8>]) {
        let orig_types_content = self.get_types_content().unwrap_or_default();
        // The glue only joins two or more types; skip re-parsing the expression for the usual single one.
        let new_types_content = match types {
            [one] => one.clone(),
            _ => types.join(
                &[self
                    .get_type_expression()
                    .and_then(|e| e.get_types_glue())
                    .unwrap_or(b'|')][..],
            ),
        };
        if orig_types_content == new_types_content {
            return;
        }
        let split = |s: &[u8]| {
            Preg::split(
                r"/([^\n\r]+\R*)/",
                s,
                -1,
                PREG_SPLIT_NO_EMPTY | PREG_SPLIT_DELIM_CAPTURE,
            )
            .into_iter()
            .map(|p| p.text)
            .collect::<Vec<_>>()
        };
        let original_types_lines = split(&orig_types_content);
        let new_types_lines = split(&new_types_content);
        for (index, line) in new_types_lines.iter().enumerate() {
            let pattern = [
                &b"/"[..],
                &preg_quote(&original_types_lines[index], Some(b'/')),
                b"/",
            ]
            .concat();
            let content = self.lines[index].borrow().get_content().to_vec();
            // the replacement is a literal in upstream only by accident of content; keep preg_replace semantics
            let new = Preg::replace_limit_uncached(&pattern, line, &content, 1).0;
            self.lines[index].borrow_mut().set_content(new);
        }
        self.clear_cache();
    }

    pub fn remove(&self) {
        for line in &self.lines {
            let mut line = line.borrow_mut();
            if line.is_the_start() && line.is_the_end() {
                line.remove();
            } else if line.is_the_start() {
                let content =
                    Preg::replace_limit(r"#(\s*/\*\*).*#", b"$1", line.get_content(), -1).0;
                line.set_content(content);
            } else if line.is_the_end() {
                let content =
                    Preg::replace_limit(r"#(\s*)\S.*(\*/.*)#", b"$1$2", line.get_content(), -1).0;
                line.set_content(content);
            } else {
                line.remove();
            }
        }
        self.clear_cache();
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.lines
            .iter()
            .flat_map(|l| l.borrow().get_content().to_vec())
            .collect()
    }

    pub fn support_types(&self) -> bool {
        let name = self.get_tag().get_name();
        TAGS_WITH_TYPES.iter().any(|t| t.as_bytes() == name)
    }

    /// Panics like upstream's RuntimeException when the tag does not support types.
    pub fn get_types_content(&self) -> Option<Vec<u8>> {
        if self.types_content.borrow().is_none() {
            let name = self.get_tag().get_name();
            if !self.support_types() {
                panic!("This tag does not support types.");
            }
            let pattern = [
                &br"{^(?:\h*\*|/\*\*)[\h*]*@"[..],
                name,
                br"\h+",
                REGEX_TYPES.as_bytes(),
                br"(?:(?:[*\h\v]|\&?[\.\$\s]).*)?\r?$}is",
            ]
            .concat();
            let content = self.get_content();
            if let Some(m) = Preg::match_groups(&pattern, &content) {
                *self.types_content.borrow_mut() = Some(m.name_str("types").to_vec());
            }
        }
        self.types_content.borrow().clone()
    }

    fn clear_cache(&self) {
        *self.types.borrow_mut() = None;
        *self.types_content.borrow_mut() = None;
    }
}

/// The first path `getVariableName()`'s pattern tries at the first `@tag`, for ASCII content (where `\s`, the identifier
/// class and the quoted types are all plain bytes): `@tag\s+` then `types\s*`, `&\s*`, `...\s*` greedily, then `$name`.
/// That path is the regex's leftmost-first match whenever it succeeds; otherwise (`None`) the regex decides.
fn variable_name_ascii(content: &[u8], tag: &[u8], types: &[u8]) -> Option<Vec<u8>> {
    if !content.is_ascii() {
        return None;
    }
    let ws = |s: &[u8]| {
        s.iter()
            .take_while(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c))
            .count()
    };
    let at = content
        .windows(tag.len() + 1)
        .position(|w| w[0] == b'@' && &w[1..] == tag)?;
    let mut rest = &content[at + 1 + tag.len()..];
    let n = ws(rest);
    if n == 0 {
        return None;
    }
    rest = &rest[n..];
    if let Some(r) = rest.strip_prefix(types) {
        rest = &r[ws(r)..];
    }
    if let Some(r) = rest.strip_prefix(b"&") {
        rest = &r[ws(r)..];
    }
    if let Some(r) = rest.strip_prefix(b"...") {
        rest = &r[ws(r)..];
    }
    let name = rest.strip_prefix(b"$")?;
    let len = name
        .iter()
        .take_while(|b| b.is_ascii_alphanumeric() || **b == b'_')
        .count();
    (len > 0 && !name[0].is_ascii_digit()).then(|| rest[..1 + len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::variable_name_ascii;

    #[test]
    fn variable_name_fast_path_matches_php() {
        // Expected values: PHP 8.5 preg_match() with getVariableName()'s pattern on the same input.
        let v = |c: &str, tag: &str, ty: &str| {
            variable_name_ascii(c.as_bytes(), tag.as_bytes(), ty.as_bytes())
                .map(|v| String::from_utf8(v).unwrap())
        };
        assert_eq!(
            v("     * @param string $foo bar\n", "param", "string"),
            Some("$foo".into())
        );
        assert_eq!(
            v("     * @param $this $a1_b\n", "param", "$this"),
            Some("$a1_b".into())
        );
        assert_eq!(
            v(" * @param int &...  $x\n", "param", "int"),
            Some("$x".into())
        );
        assert_eq!(v(" * @param\t\n  $x\n", "param", ""), Some("$x".into()));
        // Not the first path (or no match at all): left to the regex.
        assert_eq!(v(" * @param int $1x\n", "param", "int"), None);
        assert_eq!(v(" * @param int x\n", "param", "int"), None);
        assert_eq!(v(" * @param$x\n", "param", ""), None);
        assert_eq!(v(" * @return int\n", "param", "int"), None);
        assert_eq!(v(" * @param int $\u{e9}\n", "param", "int"), None);
    }
}
