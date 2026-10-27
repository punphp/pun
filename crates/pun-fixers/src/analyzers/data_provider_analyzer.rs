//! Port of `PhpCsFixer\Tokenizer\Analyzer\DataProviderAnalyzer` and its `DataProviderAnalysis`.
use crate::analyzers::analysis::{Attribute, NamespaceUseType};
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::docblock::type_expression::REGEX_IDENTIFIER;
use crate::analyzers::fully_qualified_name_analyzer::FullyQualifiedNameAnalyzer;
use crate::analyzers::preg::Preg;
use indexmap::IndexMap;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use pun_tokens::php_compat::strtolower;

/// `DataProviderAnalysis`.
pub(crate) struct DataProviderAnalysis {
    pub name: Vec<u8>,
    pub name_index: usize,
    /// (token index, offset in it) of each usage: an attribute's string, or a `@dataProvider` tag in a doc comment.
    pub usage_indices: Vec<(usize, usize)>,
}

/// `DataProviderAnalyzer::getDataProviders`, ordered by the provider's name index.
pub(crate) fn get_data_providers(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Vec<DataProviderAnalysis> {
    let fqn = FullyQualifiedNameAnalyzer::new(tokens);
    let methods = get_methods(tokens, start_index, end_index);
    let mut data_providers: IndexMap<Vec<u8>, Vec<(usize, usize)>> = IndexMap::new();
    let class = format!(r"(?:\\?+{REGEX_IDENTIFIER}(\\{REGEX_IDENTIFIER})*+)");
    let pattern = format!(r"/@dataProvider\h+(({class}::)?{REGEX_IDENTIFIER})/");
    for &method_index in methods.values() {
        let (attribute_index, doc_comment_index) = attribute_and_doc_comment(tokens, method_index);
        if let Some(attribute_index) = attribute_index {
            for analysis in AttributeAnalyzer::collect(tokens, attribute_index) {
                for attribute in &analysis.attributes {
                    let Some(name_index) = data_provider_name_index(tokens, &fqn, attribute) else {
                        continue;
                    };
                    let content = tokens[name_index].get_content();
                    let name = content[1..content.len() - 1].to_vec();
                    data_providers
                        .entry(name)
                        .or_default()
                        .push((name_index, 0));
                }
            }
        }
        if let Some(doc) = doc_comment_index {
            for m in Preg::match_all(&pattern, tokens[doc].get_content()) {
                data_providers
                    .entry(m.str(1).to_vec())
                    .or_default()
                    .push((doc, m.start()));
            }
        }
    }
    let mut analyses: Vec<DataProviderAnalysis> = Vec::new();
    for (name, usages) in data_providers {
        let Some(&index) = methods.get(&strtolower(&name)) else {
            continue;
        };
        // a later name for the same method replaces the earlier one in place
        if let Some(existing) = analyses.iter_mut().find(|a| a.name_index == index) {
            existing.usage_indices = usages;
            continue;
        }
        analyses.push(DataProviderAnalysis {
            name: tokens[index].get_content().to_vec(),
            name_index: index,
            usage_indices: usages,
        });
    }
    analyses.sort_by_key(|a| a.name_index);
    analyses
}

/// `getMethods`: lower-cased method name => the name's index.
fn get_methods(tokens: &Tokens, start_index: usize, end_index: usize) -> IndexMap<Vec<u8>, usize> {
    let mut functions = IndexMap::new();
    for index in start_index..end_index {
        if !tokens[index].is_kind(T_FUNCTION) {
            continue;
        }
        // the class's `}` comes after it
        let name_index = tokens.get_next_non_whitespace(index, None).unwrap();
        if !tokens[name_index].is_kind(T_STRING) {
            continue;
        }
        functions.insert(strtolower(tokens[name_index].get_content()), name_index);
    }
    functions
}

/// `getAttributeIndexAndDocCommentIndices`: the first attribute and doc comment found walking back from the method.
fn attribute_and_doc_comment(tokens: &Tokens, mut index: usize) -> (Option<usize>, Option<usize>) {
    let mut attribute_index = None;
    let mut doc_comment_index = None;
    while !tokens[index].equals_any(
        &[";".into(), "{".into(), "}".into(), T_OPEN_TAG.into()],
        true,
    ) {
        index -= 1;
        if tokens[index].is_kind(T_ATTRIBUTE) {
            attribute_index = Some(index);
        } else if tokens[index].is_kind(T_DOC_COMMENT) {
            doc_comment_index = Some(index);
        }
    }
    (attribute_index, doc_comment_index)
}

/// `getDataProviderNameIndex`.
fn data_provider_name_index(
    tokens: &Tokens,
    fqn: &FullyQualifiedNameAnalyzer<'_>,
    attribute: &Attribute,
) -> Option<usize> {
    let name = fqn.get_fully_qualified_name(
        &attribute.name,
        tokens.get_next_meaningful_token(attribute.start)?,
        NamespaceUseType::Class,
    );
    if name != b"PHPUnit\\Framework\\Attributes\\DataProvider" {
        return None;
    }
    // the attribute's `#[` comes before it
    let close = tokens
        .get_prev_token_of_kind(attribute.end + 1, &[")".into(), T_ATTRIBUTE.into()], true)
        .unwrap();
    if tokens[close].is_kind(T_ATTRIBUTE) {
        return None;
    }
    let name_index = tokens.get_prev_meaningful_token(close)?;
    if !tokens[name_index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
        return None;
    }
    let open = tokens.get_prev_meaningful_token(name_index)?;
    tokens[open].equals("(", true).then_some(name_index)
}
