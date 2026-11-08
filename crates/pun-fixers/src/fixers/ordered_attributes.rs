//! Port of `PhpCsFixer\Fixer\AttributeNotation\OrderedAttributesFixer`.
use crate::analyzers::analysis::NamespaceUseType;
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::docblock::type_expression::php_spaceship;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::fully_qualified_name_analyzer::FullyQualifiedNameAnalyzer;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::ltrim_chars;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct OrderedAttributesFixer {
    custom: bool,
    /// `array_flip` of the configured FQNs: the position of each.
    order: Vec<Vec<u8>>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(OrderedAttributesFixer {
        custom: false,
        order: vec![],
    })
}

/// One sortable item: an attribute group (`#[...]`) or an attribute inside one.
#[derive(Clone)]
struct Element {
    name: Vec<u8>,
    start: usize,
    end: usize,
}

impl Fixer for OrderedAttributesFixer {
    fn name(&self) -> &'static str {
        "ordered_attributes"
    }
    fn priority(&self) -> i32 {
        // Must run after FullyQualifiedStrictTypesFixer.
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ATTRIBUTE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["sort_algorithm", "order"])?;
        let custom = o.choice(
            "sort_algorithm",
            "alpha",
            &[("alpha", false), ("custom", true)],
        )?;
        let order = o.strings("order", &[], &[])?;
        // The `order` normalizer: `$value !== array_unique($value)`, then `array_flip` of the names without `\`.
        if (1..order.len()).any(|i| order[..i].contains(&order[i])) {
            bail!("[ordered_attributes] The list includes attributes that are not unique.");
        }
        if custom && order.is_empty() {
            bail!(
                "[ordered_attributes] The custom order strategy requires providing `order` option with a list of attributes's FQNs."
            );
        }
        self.custom = custom;
        self.order = (order.iter())
            .map(|a| ltrim_chars(a.as_bytes(), b"\\").to_vec())
            .collect();
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while let Some(found) = tokens.get_next_token_of_kind(index, &[T_ATTRIBUTE.into()], true) {
            index = found;
            let analyses = AttributeAnalyzer::collect(tokens, index);
            let mut elements = vec![];
            for analysis in &analyses {
                let attributes: Vec<Element> = (analysis.attributes.iter())
                    .map(|a| Element {
                        name: a.name.clone(),
                        start: a.start,
                        end: a.end,
                    })
                    .collect();
                elements.push(Element {
                    name: self.sort_attributes(tokens, analysis.start_index, attributes),
                    start: analysis.start_index,
                    end: analysis.end_index,
                });
            }
            let end_index = elements.last().unwrap().end;
            if elements.len() != 1 {
                let sorted = self.sort_elements(&elements);
                if !same_order(&elements, &sorted) {
                    sort_tokens(tokens, index, end_index, &sorted, None);
                }
            }
            index = end_index;
        }
    }
}

impl OrderedAttributesFixer {
    fn sort_attributes(
        &self,
        tokens: &mut Tokens,
        index: usize,
        mut attributes: Vec<Element>,
    ) -> Vec<u8> {
        if attributes.len() == 1 {
            return self.get_attribute_name(tokens, &attributes[0].name, attributes[0].start);
        }
        for attribute in &mut attributes {
            attribute.name = self.get_attribute_name(tokens, &attribute.name, attribute.start);
        }
        let sorted = self.sort_elements(&attributes);
        if same_order(&attributes, &sorted) {
            return attributes.swap_remove(0).name;
        }
        let end = attributes.last().unwrap().end;
        sort_tokens(tokens, index + 1, end, &sorted, Some(Token::char(",")));
        sorted[0].name.clone()
    }

    fn get_attribute_name(&self, tokens: &Tokens, name: &[u8], index: usize) -> Vec<u8> {
        if self.custom {
            // Upstream keeps one analyzer per run; its namespace and import caches see the same declarations.
            return FullyQualifiedNameAnalyzer::new(tokens).get_fully_qualified_name(
                name,
                index,
                NamespaceUseType::Class,
            );
        }
        ltrim_chars(name, b"\\").to_vec()
    }

    fn sort_elements(&self, elements: &[Element]) -> Vec<Element> {
        let mut sorted = elements.to_vec();
        if self.custom {
            let position = |name: &[u8]| {
                (self.order.iter())
                    .position(|o| o == name)
                    .map_or(i64::MAX, |p| p as i64)
            };
            sorted.sort_by_key(|e| position(&e.name));
        } else {
            sorted.sort_by(|a, b| php_spaceship(&a.name, &b.name));
        }
        sorted
    }
}

/// `$elements === $sortedElements`: the sort kept every element in place.
fn same_order(a: &[Element], b: &[Element]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.start == y.start)
}

fn sort_tokens(
    tokens: &mut Tokens,
    start: usize,
    end: usize,
    elements: &[Element],
    delimiter: Option<Token>,
) {
    let mut replace = vec![];
    for (pos, element) in elements.iter().enumerate() {
        replace.extend(
            tokens.as_slice()[element.start..=element.end]
                .iter()
                .cloned(),
        );
        if let Some(d) = &delimiter
            && pos != elements.len() - 1
        {
            replace.push(d.clone());
        }
    }
    tokens.override_range(start, end, replace);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// OrderedAttributesFixerTest::provideInvalidConfigurationCases: the messages the real fixer's configure() throws.
    #[test]
    fn invalid_configuration() {
        let raw = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/ordered_attributes.json"
        ))
        .unwrap();
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let test = (doc["tests"].as_array().unwrap().iter())
            .find(|t| t["method"] == "testInvalidConfiguration")
            .unwrap();
        for case in test["cases"].as_array().unwrap() {
            let (config, message) = (&case["args"][0], case["args"][1].as_str().unwrap());
            let error = fixer().configure(config).unwrap_err().to_string();
            assert_eq!(error, format!("[ordered_attributes] {message}"), "{config}");
        }
    }
}
