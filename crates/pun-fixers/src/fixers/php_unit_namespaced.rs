//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitNamespacedFixer`.
use crate::analyzers::abstract_php_unit_fixer::target_fulfills;
use crate::analyzers::classy_analyzer::ClassyAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;

const CLASS_MAP: &[(&str, &str)] = &[
    (
        "PHPUnit_Extensions_PhptTestCase",
        "PHPUnit\\Runner\\PhptTestCase",
    ),
    (
        "PHPUnit_Framework_Constraint",
        "PHPUnit\\Framework\\Constraint\\Constraint",
    ),
    (
        "PHPUnit_Framework_Constraint_StringMatches",
        "PHPUnit\\Framework\\Constraint\\StringMatchesFormatDescription",
    ),
    (
        "PHPUnit_Framework_Constraint_JsonMatches_ErrorMessageProvider",
        "PHPUnit\\Framework\\Constraint\\JsonMatchesErrorMessageProvider",
    ),
    (
        "PHPUnit_Framework_Constraint_PCREMatch",
        "PHPUnit\\Framework\\Constraint\\RegularExpression",
    ),
    (
        "PHPUnit_Framework_Constraint_ExceptionMessageRegExp",
        "PHPUnit\\Framework\\Constraint\\ExceptionMessageRegularExpression",
    ),
    (
        "PHPUnit_Framework_Constraint_And",
        "PHPUnit\\Framework\\Constraint\\LogicalAnd",
    ),
    (
        "PHPUnit_Framework_Constraint_Or",
        "PHPUnit\\Framework\\Constraint\\LogicalOr",
    ),
    (
        "PHPUnit_Framework_Constraint_Not",
        "PHPUnit\\Framework\\Constraint\\LogicalNot",
    ),
    (
        "PHPUnit_Framework_Constraint_Xor",
        "PHPUnit\\Framework\\Constraint\\LogicalXor",
    ),
    (
        "PHPUnit_Framework_Error",
        "PHPUnit\\Framework\\Error\\Error",
    ),
    (
        "PHPUnit_Framework_TestSuite_DataProvider",
        "PHPUnit\\Framework\\DataProviderTestSuite",
    ),
    (
        "PHPUnit_Framework_MockObject_Invocation_Static",
        "PHPUnit\\Framework\\MockObject\\Invocation\\StaticInvocation",
    ),
    (
        "PHPUnit_Framework_MockObject_Invocation_Object",
        "PHPUnit\\Framework\\MockObject\\Invocation\\ObjectInvocation",
    ),
    (
        "PHPUnit_Framework_MockObject_Stub_Return",
        "PHPUnit\\Framework\\MockObject\\Stub\\ReturnStub",
    ),
    (
        "PHPUnit_Runner_Filter_Group_Exclude",
        "PHPUnit\\Runner\\Filter\\ExcludeGroupFilterIterator",
    ),
    (
        "PHPUnit_Runner_Filter_Group_Include",
        "PHPUnit\\Runner\\Filter\\IncludeGroupFilterIterator",
    ),
    (
        "PHPUnit_Runner_Filter_Test",
        "PHPUnit\\Runner\\Filter\\NameFilterIterator",
    ),
    ("PHPUnit_Util_PHP", "PHPUnit\\Util\\PHP\\AbstractPhpProcess"),
    (
        "PHPUnit_Util_PHP_Default",
        "PHPUnit\\Util\\PHP\\DefaultPhpProcess",
    ),
    (
        "PHPUnit_Util_PHP_Windows",
        "PHPUnit\\Util\\PHP\\WindowsPhpProcess",
    ),
    ("PHPUnit_Util_Regex", "PHPUnit\\Util\\RegularExpression"),
    (
        "PHPUnit_Util_TestDox_ResultPrinter_XML",
        "PHPUnit\\Util\\TestDox\\XmlResultPrinter",
    ),
    (
        "PHPUnit_Util_TestDox_ResultPrinter_HTML",
        "PHPUnit\\Util\\TestDox\\HtmlResultPrinter",
    ),
    (
        "PHPUnit_Util_TestDox_ResultPrinter_Text",
        "PHPUnit\\Util\\TestDox\\TextResultPrinter",
    ),
    (
        "PHPUnit_Util_TestSuiteIterator",
        "PHPUnit\\Framework\\TestSuiteIterator",
    ),
    ("PHPUnit_Util_XML", "PHPUnit\\Util\\Xml"),
];

struct PhpUnitNamespacedFixer {
    original_class_reg_ex: &'static str,
    use_class_map: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitNamespacedFixer {
        original_class_reg_ex: r"/^PHPUnit_\w+$/i",
        use_class_map: true,
    })
}

impl Fixer for PhpUnitNamespacedFixer {
    fn name(&self) -> &'static str {
        "php_unit_namespaced"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target"])?;
        let target = c.string("target", "newest", &["4.8", "5.7", "6.0", "newest"])?;
        self.use_class_map = target_fulfills(&target, "6.0");
        self.original_class_reg_ex = if self.use_class_map {
            r"/^PHPUnit_\w+$/i"
        } else if target_fulfills(&target, "5.7") {
            r"/^PHPUnit_Framework_(TestCase|Assert|BaseTestListener|TestListener)+$/i"
        } else {
            r"/^PHPUnit_Framework_TestCase$/i"
        };
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut imported_original_classes: HashSet<Vec<u8>> = HashSet::new();
        let mut curr_index = 0;
        while let Some(found) = tokens.get_next_token_of_kind(curr_index, &[T_STRING.into()], true)
        {
            curr_index = found;
            let prev_index = tokens.get_prev_meaningful_token(curr_index).unwrap();
            if tokens[prev_index].is_given_kind(&[T_CONST, T_DOUBLE_COLON]) {
                continue;
            }
            let original_class = tokens[curr_index].get_content().to_vec();
            let allowed = ClassyAnalyzer.is_classy_invocation(tokens, curr_index)
                || is_import(tokens, curr_index);
            if !allowed || !Preg::r#match(self.original_class_reg_ex, &original_class) {
                curr_index += 1;
                continue;
            }
            let mut substitute = self.generate_replacement(&original_class);
            tokens.clear_at(curr_index);
            if imported_original_classes.contains(&original_class) {
                substitute = vec![substitute.pop().unwrap()];
            }
            tokens.insert_at(curr_index, substitute);
            let mut prev_index = tokens.get_prev_meaningful_token(curr_index).unwrap();
            if tokens[prev_index].is_kind(T_USE) {
                imported_original_classes.insert(original_class);
            } else if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
                prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
                if tokens[prev_index].is_kind(T_USE) {
                    imported_original_classes.insert(original_class);
                }
            }
        }
    }
}

impl PhpUnitNamespacedFixer {
    /// `generateReplacement`.
    fn generate_replacement(&self, original_class_name: &[u8]) -> Vec<Token> {
        let lower = strtolower(original_class_name);
        let mapped = CLASS_MAP
            .iter()
            .find(|(from, _)| self.use_class_map && strtolower(from.as_bytes()) == lower);
        let (delimiter, string): (u8, &[u8]) = match mapped {
            Some((_, to)) => (b'\\', to.as_bytes()),
            None => (b'_', original_class_name),
        };
        let mut tokens = Vec::new();
        for (i, part) in string.split(|&b| b == delimiter).enumerate() {
            if i > 0 {
                tokens.push(Token::new(T_NS_SEPARATOR, b"\\".to_vec()));
            }
            tokens.push(Token::new(T_STRING, part.to_vec()));
        }
        tokens
    }
}

/// `isImport`.
fn is_import(tokens: &Tokens, curr_index: usize) -> bool {
    let mut prev_index = tokens.get_prev_meaningful_token(curr_index).unwrap();
    if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
        prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
    }
    tokens[prev_index].is_kind(T_USE)
}
