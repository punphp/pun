//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTypesOrderFixer`.
use crate::analyzers::docblock::type_expression::{REGEX_TYPES, php_spaceship};
use crate::analyzers::docblock::{DocBlock, TAGS_WITH_TYPES, TypeExpression};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{strcasecmp, strtolower};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::cmp::Ordering;
use std::path::Path;
use std::sync::LazyLock;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NullAdjustment {
    AlwaysFirst,
    AlwaysLast,
    None,
}

struct PhpdocTypesOrderFixer {
    alpha: bool,
    null_adjustment: NullAdjustment,
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocTypesOrderFixer {
        alpha: true,
        // `Future::getV4OrV3('always_last', 'always_first')`: Pint runs without future mode.
        null_adjustment: NullAdjustment::AlwaysFirst,
        case_sensitive: false,
    })
}

static METHOD_CALLABLE: LazyLock<String> =
    LazyLock::new(|| format!(r"/\*\h*@method\h+{REGEX_TYPES}\h+\K(?&callable)/"));

impl Fixer for PhpdocTypesOrderFixer {
    fn name(&self) -> &'static str {
        "phpdoc_types_order"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &["sort_algorithm", "null_adjustment", "case_sensitive"],
        )?;
        self.alpha = c.string("sort_algorithm", "alpha", &["alpha", "none"])? == "alpha";
        self.null_adjustment = c.choice(
            "null_adjustment",
            "always_first",
            &[
                ("always_first", NullAdjustment::AlwaysFirst),
                ("always_last", NullAdjustment::AlwaysLast),
                ("none", NullAdjustment::None),
            ],
        )?;
        self.case_sensitive = c.bool("case_sensitive", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let annotations = doc.get_annotations_of_type(TAGS_WITH_TYPES);
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                // fix main types
                if let Some(type_expression) = annotation.get_type_expression() {
                    annotation.set_types(&self.sort_types(&type_expression));
                }
                // fix @method parameters types
                let line = doc.get_line(annotation.get_start()).unwrap();
                let content = Preg::replace_callback(
                    &*METHOD_CALLABLE,
                    |m| self.sort_types(&TypeExpression::new(m.str(0))).join(&b'|'),
                    line.borrow().get_content(),
                );
                line.borrow_mut().set_content(content);
            }
            tokens.set(index, Token::new(T_DOC_COMMENT, doc.get_content()));
        }
    }
}

impl PhpdocTypesOrderFixer {
    fn sort_types(&self, type_expression: &TypeExpression) -> Vec<Vec<u8>> {
        let normalize_type = |t: &[u8]| Preg::replace(r"/^\(*\??\\?/", "", t);
        type_expression
            .sort_types(&|a, b| {
                let a = normalize_type(a.to_string());
                let b = normalize_type(b.to_string());
                let lower_case_a = strtolower(&a);
                let lower_case_b = strtolower(&b);
                if self.null_adjustment != NullAdjustment::None {
                    let last = self.null_adjustment == NullAdjustment::AlwaysLast;
                    if lower_case_a == b"null" && lower_case_b != b"null" {
                        return if last {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                    }
                    if lower_case_a != b"null" && lower_case_b == b"null" {
                        return if last {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        };
                    }
                }
                if self.alpha {
                    return if self.case_sensitive {
                        php_spaceship(&a, &b)
                    } else {
                        strcasecmp(&a, &b).cmp(&0)
                    };
                }
                Ordering::Equal
            })
            .get_types()
    }
}
