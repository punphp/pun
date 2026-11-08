//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocOrderByValueFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{strtolower, trim};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

const ALLOWED_VALUES: &[&str] = &[
    "author",
    "covers",
    "coversNothing",
    "dataProvider",
    "depends",
    "group",
    "internal",
    "method",
    "mixin",
    "property",
    "property-read",
    "property-write",
    "requires",
    "throws",
    "uses",
];

struct PhpdocOrderByValueFixer {
    /// `annotation => strtolower(annotation)`, in configured order (later duplicates collapse onto the first).
    annotations: Vec<(String, String)>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocOrderByValueFixer {
        annotations: vec![],
    })
}

/// The option's normalizer: `$normalized[$annotation] = strtolower($annotation)`.
fn normalize(value: &[String]) -> Vec<(String, String)> {
    let mut normalized: Vec<(String, String)> = vec![];
    for annotation in value {
        if !normalized.iter().any(|(a, _)| a == annotation) {
            normalized.push((annotation.clone(), annotation.to_ascii_lowercase()));
        }
    }
    normalized
}

impl Fixer for PhpdocOrderByValueFixer {
    fn name(&self) -> &'static str {
        "phpdoc_order_by_value"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpUnitFqcnAnnotationFixer,
        // PhpdocIndentFixer, PhpdocScalarFixer, PhpdocToCommentFixer, PhpdocTypesFixer.
        -10
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["annotations"])?;
        self.annotations = normalize(&c.strings("annotations", &["covers"], ALLOWED_VALUES)?);
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_CLASS, T_DOC_COMMENT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.annotations.is_empty() {
            return;
        }
        let mut index = tokens.count() - 1;
        while index > 0 {
            for (r#type, type_lower_case) in &self.annotations {
                if tokens[index].is_kind(T_DOC_COMMENT) {
                    fix_annotation(tokens, index, r#type, type_lower_case);
                }
            }
            index -= 1;
        }
    }
}

fn fix_annotation(tokens: &mut Tokens, index: usize, r#type: &str, type_lower_case: &str) {
    let find_pattern = format!("/@{type}\\s.+@{type}\\s/s");
    if !Preg::r#match(&find_pattern, tokens[index].get_content()) {
        return;
    }
    let doc_block = DocBlock::new(tokens[index].get_content());
    let annotations = doc_block.get_annotations_of_type(&[r#type]);

    let (replace_pattern, replacement) =
        if ["property", "property-read", "property-write"].contains(&r#type) {
            (
                format!(r"/(?s)\*\s*@{type}\s+(?P<optionalTypes>.+\s+)?\$(?P<comparableContent>\S+).*/"),
                r"\2",
            )
        } else if r#type == "method" {
            (
                r"/(?s)\*\s*@method\s+(?P<optionalReturnTypes>.+\s+)?(?P<comparableContent>.+)\(.*/"
                    .to_string(),
                r"\2",
            )
        } else {
            (
                format!(r"/\*\s*@{type_lower_case}\s+(?P<comparableContent>.+)/"),
                r"\1",
            )
        };

    // `$annotationMap[$comparableContent] = $rawContent`: a repeated key keeps its first place and its last value.
    let mut annotation_map: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    for annotation in &annotations {
        let raw_content = annotation.get_content();
        let comparable_content = Preg::replace(
            &replace_pattern,
            replacement,
            strtolower(trim(&raw_content)),
        );
        match annotation_map
            .iter_mut()
            .find(|(k, _)| *k == comparable_content)
        {
            Some(entry) => entry.1 = raw_content,
            None => annotation_map.push((comparable_content, raw_content)),
        }
    }

    // ksort(SORT_STRING) on unique keys.
    let mut ordered_annotation_map = annotation_map.clone();
    ordered_annotation_map.sort_by(|a, b| a.0.cmp(&b.0));
    if ordered_annotation_map == annotation_map {
        return;
    }

    let mut lines: Vec<Vec<u8>> = doc_block
        .get_lines()
        .iter()
        .map(|l| l.borrow().get_content().to_vec())
        .collect();
    for annotation in annotations.iter().rev() {
        // array_pop() of an emptied map is null, which array_splice() inserts as nothing.
        let replacement: Vec<Vec<u8>> = ordered_annotation_map
            .pop()
            .map(|(_, raw)| raw)
            .into_iter()
            .collect();
        lines.splice(annotation.get_start()..=annotation.get_end(), replacement);
    }
    tokens.set(index, Token::new(T_DOC_COMMENT, lines.concat()));
}
