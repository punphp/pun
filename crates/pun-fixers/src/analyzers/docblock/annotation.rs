//! `PhpCsFixer\DocBlock\Annotation`.

use super::line::LineRef;
use super::preg;
use super::tag::Tag;
use super::type_expression::{Context, REGEX_IDENTIFIER, REGEX_TYPES, TypeExpression};
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

pub const TAGS_WITH_TYPES: &[&str] = &[
    "extends", "implements", "method", "param", "param-out", "phpstan-import-type", "phpstan-type", "phpstan-var", "property",
    "property-read", "property-write", "psalm-import-type", "psalm-type", "psalm-var", "return", "throws", "type", "var",
];

#[derive(Clone, Debug)]
pub struct Annotation {
    lines: Vec<LineRef>,
    start: usize,
    end: usize,
    tag: OnceCell<Tag>,
    types_content: RefCell<Option<Vec<u8>>>,
    types: RefCell<Option<Vec<Vec<u8>>>>,
    ctx: Rc<Context>,
}

impl Annotation {
    /// `lines` are `(index in the docblock, line)` pairs, like upstream's preserved-key slice.
    pub fn new(lines: Vec<(usize, LineRef)>, ctx: Rc<Context>) -> Self {
        let start = lines.first().unwrap().0;
        let end = lines.last().unwrap().0;
        Annotation {
            lines: lines.into_iter().map(|(_, l)| l).collect(),
            start,
            end,
            tag: OnceCell::new(),
            types_content: RefCell::new(None),
            types: RefCell::new(None),
            ctx,
        }
    }

    pub fn get_tags_with_types() -> &'static [&'static str] {
        TAGS_WITH_TYPES
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
        self.get_types_content().map(|c| TypeExpression::new(&c, self.ctx.clone()))
    }

    pub fn get_variable_name(&self) -> Option<Vec<u8>> {
        let ty = preg::quote(&self.get_types_content().unwrap_or_default(), Some(b'/'));
        let regex = [
            &b"/@"[..],
            &self.get_tag().get_name(),
            br"\s+(",
            &ty,
            br"\s*)?(&\s*)?(\.{3}\s*)?(?<variable>\$",
            REGEX_IDENTIFIER.as_bytes(),
            b")(?:.*|$)/",
        ]
        .concat();
        let content = self.get_content();
        preg::captures(&regex, &content).map(|m| m.name_str("variable").to_vec())
    }

    pub fn get_types(&self) -> Vec<Vec<u8>> {
        if self.types.borrow().is_none() {
            let t = self.get_type_expression().map_or_else(Vec::new, |e| e.get_types());
            *self.types.borrow_mut() = Some(t);
        }
        self.types.borrow().clone().unwrap()
    }

    pub fn set_types(&self, types: &[Vec<u8>]) {
        let orig_types_content = self.get_types_content().unwrap_or_default();
        // The glue only joins two or more types; skip re-parsing the expression for the usual single one.
        let new_types_content = match types {
            [one] => one.clone(),
            _ => types.join(&[self.get_type_expression().and_then(|e| e.get_types_glue()).unwrap_or(b'|')][..]),
        };
        if orig_types_content == new_types_content {
            return;
        }
        let split = |s: &[u8]| preg::split(r"/([^\n\r]+\R*)/", s, preg::SPLIT_NO_EMPTY | preg::SPLIT_DELIM_CAPTURE);
        let original_types_lines = split(&orig_types_content);
        let new_types_lines = split(&new_types_content);
        for (index, line) in new_types_lines.iter().enumerate() {
            let pattern = [&b"/"[..], &preg::quote(&original_types_lines[index], Some(b'/')), b"/"].concat();
            let content = self.lines[index].borrow().get_content().to_vec();
            // the replacement is a literal in upstream only by accident of content; keep preg_replace semantics
            let new = preg::replace(&pattern, line, &content, 1);
            self.lines[index].borrow_mut().set_content(new);
        }
        self.clear_cache();
    }

    pub fn get_normalized_types(&self) -> Vec<Vec<u8>> {
        let Some(e) = self.get_type_expression() else { return vec![] };
        e.map_types(&mut |v| v.lowercased()).sort_types(&mut |a, b| a.to_string().cmp(b.to_string())).get_types()
    }

    pub fn remove(&self) {
        for line in &self.lines {
            let mut line = line.borrow_mut();
            if line.is_the_start() && line.is_the_end() {
                line.remove();
            } else if line.is_the_start() {
                let content = preg::replace(r"#(\s*/\*\*).*#", b"$1", line.get_content(), -1);
                line.set_content(content);
            } else if line.is_the_end() {
                let content = preg::replace(r"#(\s*)\S.*(\*/.*)#", b"$1$2", line.get_content(), -1);
                line.set_content(content);
            } else {
                line.remove();
            }
        }
        self.clear_cache();
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.lines.iter().flat_map(|l| l.borrow().get_content().to_vec()).collect()
    }

    pub fn support_types(&self) -> bool {
        TAGS_WITH_TYPES.iter().any(|t| t.as_bytes() == self.get_tag().get_name())
    }

    /// Panics like upstream's RuntimeException when the tag does not support types.
    pub fn get_types_content(&self) -> Option<Vec<u8>> {
        if self.types_content.borrow().is_none() {
            let name = self.get_tag().get_name();
            if !self.support_types() {
                panic!("This tag does not support types.");
            }
            let pattern = [&br"{^(?:\h*\*|/\*\*)[\h*]*@"[..], &name, br"\h+", REGEX_TYPES.as_bytes(), br"(?:(?:[*\h\v]|\&?[\.\$\s]).*)?\r?$}is"].concat();
            let content = self.get_content();
            if let Some(m) = preg::captures(&pattern, &content) {
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
