//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDataProviderNameFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::data_provider_analyzer::get_data_providers;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use indexmap::IndexMap;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpUnitDataProviderNameFixer {
    prefix: String,
    suffix: String,
}

impl Default for PhpUnitDataProviderNameFixer {
    fn default() -> Self {
        Self {
            prefix: "provide".to_string(),
            suffix: "Cases".to_string(),
        }
    }
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitDataProviderNameFixer::default())
}

impl Fixer for PhpUnitDataProviderNameFixer {
    fn name(&self) -> &'static str {
        "php_unit_data_provider_name"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["prefix", "suffix"])?;
        self.prefix = c.string("prefix", "provide", &[])?;
        self.suffix = c.string("suffix", "Cases", &[])?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            self.apply_class(tokens, start_index, end_index)
        });
    }
}

impl PhpUnitDataProviderNameFixer {
    fn apply_class(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let providers = get_data_providers(tokens, start_index, end_index);
        let next_function = |tokens: &Tokens, i: usize| {
            tokens
                .get_next_token_of_kind(i, &[T_FUNCTION.into()], true)
                .unwrap()
        };
        let mut methods_providers: IndexMap<usize, IndexMap<Vec<u8>, usize>> = IndexMap::new();
        let mut providers_methods: IndexMap<Vec<u8>, IndexMap<usize, usize>> = IndexMap::new();
        for provider in &providers {
            for &(usage, _) in &provider.usage_indices {
                let method = next_function(tokens, usage);
                methods_providers
                    .entry(method)
                    .or_default()
                    .insert(provider.name.clone(), usage);
                providers_methods
                    .entry(provider.name.clone())
                    .or_default()
                    .insert(method, usage);
            }
        }
        for provider in &providers {
            if providers_methods[&provider.name].len() > 1 {
                continue;
            }
            let method = next_function(tokens, provider.usage_indices[0].0);
            if methods_providers[&method].len() > 1 {
                continue;
            }
            let new_name = self.name_for_usage(tokens, method);
            if tokens
                .find_sequence(
                    &[T_FUNCTION.into(), (T_STRING, new_name.as_str()).into()],
                    start_index,
                    Some(end_index),
                    |_| true,
                )
                .is_some()
            {
                continue;
            }
            for &(usage, _) in &provider.usage_indices {
                tokens.set(
                    provider.name_index,
                    Token::new(T_STRING, new_name.as_bytes().to_vec()),
                );
                let content = tokens[usage].get_content().to_vec();
                let new_content = if tokens[usage].is_kind(T_DOC_COMMENT) {
                    Preg::replace(
                        format!(
                            r"/(@dataProvider\s+){}/",
                            String::from_utf8_lossy(&provider.name)
                        ),
                        format!("$1{new_name}"),
                        &content,
                    )
                } else {
                    let quote = content[0] as char;
                    format!("{quote}{new_name}{quote}").into_bytes()
                };
                let id = tokens[usage].get_id();
                tokens.set(usage, Token::with_id(id, new_content));
            }
        }
    }

    /// `getDataProviderNameForUsageIndex`, from the `function` token of a method `DataProviderAnalyzer` found: its
    /// name comes right after it, so upstream's loop (over attributes and other tokens) ends on its first step.
    fn name_for_usage(&self, tokens: &Tokens, index: usize) -> String {
        let index = tokens.get_next_meaningful_token(index).unwrap();
        let name = Preg::replace(r"/^test_*/i", "", tokens[index].get_content());
        let mut name = String::from_utf8_lossy(&name).into_owned();
        if self.prefix.is_empty() {
            name = lcfirst(&name);
        } else if !self.prefix.ends_with('_') {
            name = ucfirst(&name);
        }
        format!("{}{name}{}", self.prefix, self.suffix)
    }
}

fn ucfirst(s: &str) -> String {
    let mut b = s.as_bytes().to_vec();
    if let Some(c) = b.first_mut() {
        c.make_ascii_uppercase();
    }
    String::from_utf8(b).unwrap_or_default()
}

fn lcfirst(s: &str) -> String {
    let mut b = s.as_bytes().to_vec();
    if let Some(c) = b.first_mut() {
        c.make_ascii_lowercase();
    }
    String::from_utf8(b).unwrap_or_default()
}
