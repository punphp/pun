//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDataProviderMethodOrderFixer`, on `OrderedClassElementsFixer`'s
//! `getElements` and `sortTokens`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::data_provider_analyzer::get_data_providers;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::ordered_class_elements::{Element, get_elements, sort_tokens};
use indexmap::IndexMap;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

struct PhpUnitDataProviderMethodOrderFixer {
    before: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitDataProviderMethodOrderFixer { before: false })
}

impl Fixer for PhpUnitDataProviderMethodOrderFixer {
    fn name(&self) -> &'static str {
        "php_unit_data_provider_method_order"
    }
    fn priority(&self) -> i32 {
        64
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["placement"])?;
        self.before = c.string("placement", "after", &["after", "before"])? == "before";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, _| {
            self.apply_class(tokens, start_index)
        });
    }
}

impl PhpUnitDataProviderMethodOrderFixer {
    fn apply_class(&self, tokens: &mut Tokens, start_index: usize) {
        let elements = get_elements(tokens, start_index);
        let Some(last) = elements.last() else {
            return;
        };
        let end_index = last.end;
        let pairs = orig_usage_data_provider_order_pairs(tokens, start_index, end_index);
        let mut sorted = elements.clone();
        let mut placed: Vec<Vec<u8>> = Vec::new();
        if self.before {
            for (usage, provider) in &pairs {
                if !placed.contains(provider) {
                    placed.push(provider.clone());
                    sorted = move_method_element(sorted, usage, provider, false);
                }
            }
        } else {
            let mut same: Option<(Vec<u8>, Vec<u8>)> = None;
            for (usage, provider) in &pairs {
                if placed.contains(provider) {
                    continue;
                }
                placed.push(provider.clone());
                let before = sorted.clone();
                let keep = match &same {
                    Some((same_usage, same_provider)) if same_usage == usage => {
                        same_provider.clone()
                    }
                    _ => usage.clone(),
                };
                sorted = move_method_element(sorted, &keep, provider, true);
                same = Some((usage.clone(), provider.clone()));
                if before != sorted {
                    placed.retain(|p| p != provider);
                }
            }
        }
        if sorted != elements {
            sort_tokens(tokens, start_index, end_index, &sorted);
        }
    }
}

/// `moveMethodElement`.
fn move_method_element(
    mut elements: Vec<Element>,
    keep: &[u8],
    to_move: &[u8],
    after: bool,
) -> Vec<Element> {
    let mut i_keep = None;
    let mut i_to_move = None;
    for (i, element) in elements.iter().enumerate() {
        if element.type_ == "method" {
            if element.name == keep {
                i_keep = Some(i);
            } else if element.name == to_move {
                i_to_move = Some(i);
            }
        }
    }
    // Upstream asserts both were found; with assertions off (a test that is its own provider leaves `$iToMove`
    // `false`), PHP reads `false` as `0` in arithmetic and keys, and compares it with an integer as booleans.
    let keep_at = i_keep.map_or(0, |k| k as isize);
    if i_to_move.is_some_and(|m| m as isize == keep_at + if after { 1 } else { -1 }) {
        return elements;
    }
    let element = elements.remove(i_to_move.unwrap_or(0));
    let before = i_keep.is_some_and(|k| i_to_move.map_or(k != 0, |m| m < k));
    let c = keep_at + isize::from(after) - isize::from(before);
    // `array_slice($elements, 0, $c)` and `array_slice($elements, $c)`: a negative `$c` counts from the end
    let len = elements.len() as isize;
    let at = (if c < 0 { len + c } else { c }).clamp(0, len);
    elements.insert(at as usize, element);
    elements
}

/// `getDataProvidersWithUsagePairs` and `getOrigUsageDataProviderOrderPairs`: (test name, provider name) pairs in
/// the order of the tests, then of the usages' offsets.
fn orig_usage_data_provider_order_pairs(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    // ((test name index, test name, usage offset), provider name)
    let mut pairs: Vec<((usize, Vec<u8>, usize), Vec<u8>)> = Vec::new();
    for provider in get_data_providers(tokens, start_index, end_index) {
        let mut usages: IndexMap<usize, (usize, Vec<u8>, usize)> = IndexMap::new();
        for &(usage, offset) in &provider.usage_indices {
            let Some(found) = tokens.find_sequence(
                &[T_FUNCTION.into(), T_STRING.into()],
                usage,
                Some(end_index),
                |_| true,
            ) else {
                continue;
            };
            let name_index = *found.last().unwrap();
            usages.insert(
                name_index,
                (
                    name_index,
                    tokens[name_index].get_content().to_vec(),
                    offset,
                ),
            );
        }
        for usage in usages.into_values() {
            pairs.push((usage, provider.name.clone()));
        }
    }
    pairs.sort_by(|a, b| (a.0.0, a.0.2).cmp(&(b.0.0, b.0.2)));
    pairs
        .into_iter()
        .map(|((_, name, _), provider)| (name, provider))
        .collect()
}
