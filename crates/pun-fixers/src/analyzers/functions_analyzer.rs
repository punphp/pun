//! Port of `PhpCsFixer\Tokenizer\Analyzer\FunctionsAnalyzer`.
use super::analysis::{ArgumentAnalysis, NamespaceUseAnalysis, TypeAnalysis};
use super::arguments_analyzer::ArgumentsAnalyzer;
use super::attribute_analyzer::AttributeAnalyzer;
use super::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_tokens::NamespacesAnalyzer;
use pun_tokens::{Kind, Kind::*, Tokens, php_compat, token::CLASSY_KINDS};
use std::cell::RefCell;

const POSSIBLE_KINDS: &[Kind] = &[
    T_DOUBLE_COLON,
    T_FUNCTION,
    CT_NAMESPACE_OPERATOR,
    T_NEW,
    CT_RETURN_REF,
    T_STRING,
    T_OBJECT_OPERATOR,
    T_NULLSAFE_OBJECT_OPERATOR,
    T_ATTRIBUTE,
];

#[derive(Default)]
struct FunctionsAnalysis {
    built: bool,
    imports: Vec<NamespaceUseAnalysis>,
    declarations: Vec<usize>,
}

/// One instance per token collection, like upstream's per-fixer-run usage. The functions/imports
/// analysis is cached and rebuilt when the collection is changed (upstream: `isChanged()` or a
/// different code hash).
#[derive(Default)]
pub(crate) struct FunctionsAnalyzer {
    functions_analysis: RefCell<FunctionsAnalysis>,
}

impl FunctionsAnalyzer {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn is_global_function_call(&self, tokens: &Tokens, index: usize) -> bool {
        self.is_global_function_call_or_usage(tokens, index, true)
    }

    pub(crate) fn is_global_function_usage(&self, tokens: &Tokens, index: usize) -> bool {
        self.is_global_function_call_or_usage(tokens, index, false)
    }

    /// Argument name => analysis, in argument order (later duplicates overwrite in place).
    pub(crate) fn get_function_arguments(
        &self,
        tokens: &Tokens,
        function_index: usize,
    ) -> Vec<(Vec<u8>, ArgumentAnalysis)> {
        let arguments_start = tokens
            .get_next_token_of_kind(function_index, &["(".into()], true)
            .unwrap();
        let arguments_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, arguments_start);
        let argument_analyzer = ArgumentsAnalyzer;
        let mut arguments: Vec<(Vec<u8>, ArgumentAnalysis)> = Vec::new();
        for (start, end) in argument_analyzer.get_arguments(tokens, arguments_start, arguments_end)
        {
            let info = argument_analyzer.get_argument_info(tokens, start, end);
            let key = info.name.clone().unwrap_or_default();
            match arguments.iter_mut().find(|(k, _)| *k == key) {
                Some(slot) => slot.1 = info,
                None => arguments.push((key, info)),
            }
        }
        arguments
    }

    pub(crate) fn get_function_return_type(
        &self,
        tokens: &Tokens,
        method_index: usize,
    ) -> Option<TypeAnalysis> {
        let arguments_start = tokens
            .get_next_token_of_kind(method_index, &["(".into()], true)
            .unwrap();
        let arguments_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, arguments_start);
        let type_colon_index = tokens.get_next_meaningful_token(arguments_end)?;
        if !tokens[type_colon_index].is_kind(CT_TYPE_COLON) {
            return None;
        }
        let mut type_ = Vec::new();
        let type_start_index = tokens.get_next_meaningful_token(type_colon_index).unwrap();
        let mut type_end_index = type_start_index;
        let function_body_start = tokens
            .get_next_token_of_kind(
                type_colon_index,
                &["{".into(), ";".into(), T_DOUBLE_ARROW.into()],
                true,
            )
            .unwrap();
        for i in type_start_index..function_body_start {
            if tokens[i].is_whitespace(None) || tokens[i].is_comment() {
                continue;
            }
            type_.extend_from_slice(&tokens[i].content);
            type_end_index = i;
        }
        Some(TypeAnalysis::new(
            &type_,
            Some(type_start_index),
            Some(type_end_index),
        ))
    }

    fn is_global_function_call_or_usage(
        &self,
        tokens: &Tokens,
        index: usize,
        call_only: bool,
    ) -> bool {
        if !tokens[index].is_kind(T_STRING) {
            return false;
        }
        let open_parenthesis_index = match tokens.get_next_meaningful_token(index) {
            Some(i) if tokens[i].equals("(", true) => i,
            _ => return false,
        };
        let mut previous_is_namespace_separator = false;
        let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
            previous_is_namespace_separator = true;
            prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
        }
        if tokens[prev_index].is_given_kind(POSSIBLE_KINDS) {
            return false;
        }
        if call_only
            && tokens[tokens
                .get_next_meaningful_token(open_parenthesis_index)
                .unwrap()]
            .is_kind(CT_FIRST_CLASS_CALLABLE)
        {
            return false;
        }
        if previous_is_namespace_separator {
            return true;
        }

        let function_name = php_compat::strtolower(&tokens[index].content);

        if function_name == b"set" {
            if !tokens[prev_index].equals_any(
                &[CT_PROPERTY_HOOK_BRACE_OPEN.into(), ";".into(), "}".into()],
                true,
            ) {
                return true;
            }
            let close_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_index);
            let after = tokens
                .get_next_meaningful_token(close_parenthesis_index)
                .unwrap();
            if tokens[after].equals_any(&["{".into(), T_DOUBLE_ARROW.into()], true) {
                return false;
            }
        }

        if tokens.is_changed() || !self.functions_analysis.borrow().built {
            self.build_functions_analysis(tokens);
        }

        let mut scope_start_index = 0;
        let mut scope_end_index = tokens.count() - 1;
        let mut in_global_namespace = false;
        for declaration in NamespacesAnalyzer.declarations(tokens).iter() {
            scope_start_index = declaration.scope_start_index;
            scope_end_index = declaration.scope_end_index;
            if index >= scope_start_index && index <= scope_end_index {
                in_global_namespace = declaration.is_global_namespace();
                break;
            }
        }

        let analysis = self.functions_analysis.borrow();

        // check if the call is to a function declared in the same namespace as the call is done,
        // if the call is already in the global namespace than declared functions are in the same
        // global namespace and don't need checking
        if !in_global_namespace {
            for &function_name_index in &analysis.declarations {
                if function_name_index < scope_start_index || function_name_index > scope_end_index
                {
                    continue;
                }
                if php_compat::strtolower(&tokens[function_name_index].content) == function_name {
                    return false;
                }
            }
        }

        for function_use in &analysis.imports {
            if function_use.start_index < scope_start_index
                || function_use.end_index > scope_end_index
            {
                continue;
            }
            if function_name != php_compat::strtolower(&function_use.short_name) {
                continue;
            }
            // global import like `use function \str_repeat;`
            return function_use.short_name
                == php_compat::ltrim_chars(&function_use.full_name, b"\\");
        }

        if AttributeAnalyzer::is_attribute(tokens, index) {
            return false;
        }
        true
    }

    fn build_functions_analysis(&self, tokens: &Tokens) {
        let mut analysis = FunctionsAnalysis {
            built: true,
            ..Default::default()
        };

        if tokens.is_token_kind_found(T_FUNCTION) {
            let end = tokens.count();
            let mut i = 0;
            while i < end {
                if tokens[i].is_given_kind(CLASSY_KINDS) {
                    i = tokens
                        .get_next_token_of_kind(i, &["(".into(), "{".into()], true)
                        .unwrap();
                    if tokens[i].equals("(", true) {
                        // anonymous class
                        i = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i);
                        i = tokens
                            .get_next_token_of_kind(i, &["{".into()], true)
                            .unwrap();
                    }
                    i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, i) + 1;
                    continue;
                }
                if !tokens[i].is_kind(T_FUNCTION) {
                    i += 1;
                    continue;
                }
                i = tokens.get_next_meaningful_token(i).unwrap();
                if tokens[i].is_kind(CT_RETURN_REF) {
                    i = tokens.get_next_meaningful_token(i).unwrap();
                }
                if !tokens[i].is_kind(T_STRING) {
                    i += 1;
                    continue;
                }
                analysis.declarations.push(i);
                i += 1;
            }
        }

        if tokens.is_token_kind_found(CT_FUNCTION_IMPORT) {
            for declaration in NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, false) {
                if declaration.is_function() {
                    analysis.imports.push(declaration);
                }
            }
        }
        *self.functions_analysis.borrow_mut() = analysis;
    }
}
