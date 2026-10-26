//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitTestCaseStaticMethodCallsFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate, target_fulfills};
use crate::analyzers::fixer_configuration_resolver::{PhpKey, resolve};
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::utils::natural_language_join;
use anyhow::bail;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// `METHODS`, in upstream's order (the order of the invalid-key message).
const METHODS: &[&str] = &[
    "anything",
    "arrayHasKey",
    "assertArrayHasKey",
    "assertArrayIsEqualToArrayIgnoringListOfKeys",
    "assertArrayIsEqualToArrayOnlyConsideringListOfKeys",
    "assertArrayIsIdenticalToArrayIgnoringListOfKeys",
    "assertArrayIsIdenticalToArrayOnlyConsideringListOfKeys",
    "assertArrayNotHasKey",
    "assertArraySubset",
    "assertArraysAreIdentical",
    "assertArraysAreIdenticalIgnoringOrder",
    "assertArraysHaveIdenticalValues",
    "assertArraysHaveIdenticalValuesIgnoringOrder",
    "assertArraysAreEqual",
    "assertArraysAreEqualIgnoringOrder",
    "assertArraysHaveEqualValues",
    "assertArraysHaveEqualValuesIgnoringOrder",
    "assertAttributeContains",
    "assertAttributeContainsOnly",
    "assertAttributeCount",
    "assertAttributeEmpty",
    "assertAttributeEquals",
    "assertAttributeGreaterThan",
    "assertAttributeGreaterThanOrEqual",
    "assertAttributeInstanceOf",
    "assertAttributeInternalType",
    "assertAttributeLessThan",
    "assertAttributeLessThanOrEqual",
    "assertAttributeNotContains",
    "assertAttributeNotContainsOnly",
    "assertAttributeNotCount",
    "assertAttributeNotEmpty",
    "assertAttributeNotEquals",
    "assertAttributeNotInstanceOf",
    "assertAttributeNotInternalType",
    "assertAttributeNotSame",
    "assertAttributeSame",
    "assertClassHasAttribute",
    "assertClassHasStaticAttribute",
    "assertClassNotHasAttribute",
    "assertClassNotHasStaticAttribute",
    "assertContains",
    "assertContainsEquals",
    "assertContainsNotOnlyArray",
    "assertContainsNotOnlyBool",
    "assertContainsNotOnlyCallable",
    "assertContainsNotOnlyClosedResource",
    "assertContainsNotOnlyFloat",
    "assertContainsNotOnlyInstancesOf",
    "assertContainsNotOnlyInt",
    "assertContainsNotOnlyIterable",
    "assertContainsNotOnlyNull",
    "assertContainsNotOnlyNumeric",
    "assertContainsNotOnlyObject",
    "assertContainsNotOnlyResource",
    "assertContainsNotOnlyScalar",
    "assertContainsNotOnlyString",
    "assertContainsOnly",
    "assertContainsOnlyArray",
    "assertContainsOnlyBool",
    "assertContainsOnlyCallable",
    "assertContainsOnlyClosedResource",
    "assertContainsOnlyFloat",
    "assertContainsOnlyInstancesOf",
    "assertContainsOnlyInt",
    "assertContainsOnlyIterable",
    "assertContainsOnlyNull",
    "assertContainsOnlyNumeric",
    "assertContainsOnlyObject",
    "assertContainsOnlyResource",
    "assertContainsOnlyScalar",
    "assertContainsOnlyString",
    "assertCount",
    "assertDirectoryDoesNotExist",
    "assertDirectoryExists",
    "assertDirectoryIsNotReadable",
    "assertDirectoryIsNotWritable",
    "assertDirectoryIsReadable",
    "assertDirectoryIsWritable",
    "assertDirectoryNotExists",
    "assertDirectoryNotIsReadable",
    "assertDirectoryNotIsWritable",
    "assertDoesNotMatchRegularExpression",
    "assertEmpty",
    "assertEquals",
    "assertEqualsCanonicalizing",
    "assertEqualsIgnoringCase",
    "assertEqualsWithDelta",
    "assertEqualXMLStructure",
    "assertFalse",
    "assertFileDoesNotExist",
    "assertFileEquals",
    "assertFileEqualsCanonicalizing",
    "assertFileEqualsFileIgnoringWhitespace",
    "assertFileEqualsIgnoringCase",
    "assertFileExists",
    "assertFileIsNotReadable",
    "assertFileIsNotWritable",
    "assertFileIsReadable",
    "assertFileIsWritable",
    "assertFileMatchesFormat",
    "assertFileMatchesFormatFile",
    "assertFileNotEquals",
    "assertFileNotEqualsCanonicalizing",
    "assertFileNotEqualsFileIgnoringWhitespace",
    "assertFileNotEqualsIgnoringCase",
    "assertFileNotExists",
    "assertFileNotIsReadable",
    "assertFileNotIsWritable",
    "assertFinite",
    "assertGreaterThan",
    "assertGreaterThanOrEqual",
    "assertInfinite",
    "assertInstanceOf",
    "assertInternalType",
    "assertIsArray",
    "assertIsBool",
    "assertIsCallable",
    "assertIsClosedResource",
    "assertIsFloat",
    "assertIsInt",
    "assertIsIterable",
    "assertIsList",
    "assertIsNotArray",
    "assertIsNotBool",
    "assertIsNotCallable",
    "assertIsNotClosedResource",
    "assertIsNotFloat",
    "assertIsNotInt",
    "assertIsNotIterable",
    "assertIsNotNumeric",
    "assertIsNotObject",
    "assertIsNotReadable",
    "assertIsNotResource",
    "assertIsNotScalar",
    "assertIsNotString",
    "assertIsNotWritable",
    "assertIsNumeric",
    "assertIsObject",
    "assertIsReadable",
    "assertIsResource",
    "assertIsScalar",
    "assertIsString",
    "assertIsWritable",
    "assertJson",
    "assertJsonFileEqualsJsonFile",
    "assertJsonFileNotEqualsJsonFile",
    "assertJsonStringEqualsJsonFile",
    "assertJsonStringEqualsJsonString",
    "assertJsonStringNotEqualsJsonFile",
    "assertJsonStringNotEqualsJsonString",
    "assertLessThan",
    "assertLessThanOrEqual",
    "assertMatchesRegularExpression",
    "assertNan",
    "assertNotContains",
    "assertNotContainsEquals",
    "assertNotContainsOnly",
    "assertNotCount",
    "assertNotEmpty",
    "assertNotEquals",
    "assertNotEqualsCanonicalizing",
    "assertNotEqualsIgnoringCase",
    "assertNotEqualsWithDelta",
    "assertNotFalse",
    "assertNotInstanceOf",
    "assertNotInternalType",
    "assertNotIsReadable",
    "assertNotIsWritable",
    "assertNotNull",
    "assertNotRegExp",
    "assertNotSame",
    "assertNotSameSize",
    "assertNotTrue",
    "assertNull",
    "assertObjectEquals",
    "assertObjectHasAttribute",
    "assertObjectHasProperty",
    "assertObjectNotEquals",
    "assertObjectNotHasAttribute",
    "assertObjectNotHasProperty",
    "assertRegExp",
    "assertSame",
    "assertSameSize",
    "assertStringContainsString",
    "assertStringContainsStringIgnoringCase",
    "assertStringContainsStringIgnoringLineEndings",
    "assertStringEndsNotWith",
    "assertStringEndsWith",
    "assertStringEqualsFile",
    "assertStringEqualsFileCanonicalizing",
    "assertStringEqualsFileIgnoringCase",
    "assertStringEqualsFileIgnoringWhitespace",
    "assertStringEqualsStringIgnoringLineEndings",
    "assertStringEqualsStringIgnoringWhitespace",
    "assertStringMatchesFormat",
    "assertStringMatchesFormatFile",
    "assertStringNotContainsString",
    "assertStringNotContainsStringIgnoringCase",
    "assertStringNotEqualsFile",
    "assertStringNotEqualsFileCanonicalizing",
    "assertStringNotEqualsFileIgnoringCase",
    "assertStringNotEqualsFileIgnoringWhitespace",
    "assertStringNotEqualsStringIgnoringWhitespace",
    "assertStringNotMatchesFormat",
    "assertStringNotMatchesFormatFile",
    "assertStringStartsNotWith",
    "assertStringStartsWith",
    "assertThat",
    "assertTrue",
    "assertXmlFileEqualsXmlFile",
    "assertXmlFileEqualsXmlFileConsideringComments",
    "assertXmlFileNotEqualsXmlFile",
    "assertXmlFileNotEqualsXmlFileConsideringComments",
    "assertXmlStringEqualsXmlFile",
    "assertXmlStringEqualsXmlFileConsideringComments",
    "assertXmlStringEqualsXmlString",
    "assertXmlStringEqualsXmlStringConsideringComments",
    "assertXmlStringNotEqualsXmlFile",
    "assertXmlStringNotEqualsXmlFileConsideringComments",
    "assertXmlStringNotEqualsXmlString",
    "assertXmlStringNotEqualsXmlStringConsideringComments",
    "attribute",
    "attributeEqualTo",
    "callback",
    "classHasAttribute",
    "classHasStaticAttribute",
    "contains",
    "containsEqual",
    "containsIdentical",
    "containsOnly",
    "containsOnlyArray",
    "containsOnlyBool",
    "containsOnlyCallable",
    "containsOnlyClosedResource",
    "containsOnlyFloat",
    "containsOnlyInstancesOf",
    "containsOnlyInt",
    "containsOnlyIterable",
    "containsOnlyNull",
    "containsOnlyNumeric",
    "containsOnlyObject",
    "containsOnlyResource",
    "containsOnlyScalar",
    "containsOnlyString",
    "countOf",
    "directoryExists",
    "equalTo",
    "equalToCanonicalizing",
    "equalToIgnoringCase",
    "equalToWithDelta",
    "fail",
    "fileExists",
    "getCount",
    "getObjectAttribute",
    "getStaticAttribute",
    "greaterThan",
    "greaterThanOrEqual",
    "identicalTo",
    "isArray",
    "isBool",
    "isCallable",
    "isClosedResource",
    "isEmpty",
    "isFalse",
    "isFinite",
    "isFloat",
    "isInfinite",
    "isInstanceOf",
    "isInt",
    "isIterable",
    "isJson",
    "isList",
    "isNan",
    "isNull",
    "isNumeric",
    "isObject",
    "isReadable",
    "isResource",
    "isScalar",
    "isString",
    "isTrue",
    "isType",
    "isWritable",
    "lessThan",
    "lessThanOrEqual",
    "logicalAnd",
    "logicalNot",
    "logicalOr",
    "logicalXor",
    "markTestIncomplete",
    "markTestSkipped",
    "matches",
    "matchesRegularExpression",
    "objectEquals",
    "objectHasAttribute",
    "readAttribute",
    "resetCount",
    "stringContains",
    "stringEndsWith",
    "stringEqualsStringIgnoringLineEndings",
    "stringEqualsStringIgnoringWhitespace",
    "stringStartsWith",
    "any",
    "at",
    "atLeast",
    "atLeastOnce",
    "atMost",
    "createStub",
    "createConfiguredStub",
    "createStubForIntersectionOfInterfaces",
    "exactly",
    "getStubBuilder",
    "never",
    "once",
    "onConsecutiveCalls",
    "returnArgument",
    "returnCallback",
    "returnSelf",
    "returnValue",
    "returnValueMap",
    "setUpBeforeClass",
    "tearDownAfterClass",
    "throwException",
];

#[derive(Clone, Copy, PartialEq)]
enum CallType {
    This,
    Self_,
    Static,
}

impl CallType {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "this" => Some(CallType::This),
            "self" => Some(CallType::Self_),
            "static" => Some(CallType::Static),
            _ => None,
        }
    }

    /// `$conversionMap[$callType]`.
    fn conversion(self) -> (Token, Token) {
        match self {
            CallType::This => (
                Token::new(T_OBJECT_OPERATOR, b"->".to_vec()),
                Token::new(T_VARIABLE, b"$this".to_vec()),
            ),
            CallType::Self_ => (
                Token::new(T_DOUBLE_COLON, b"::".to_vec()),
                Token::new(T_STRING, b"self".to_vec()),
            ),
            CallType::Static => (
                Token::new(T_DOUBLE_COLON, b"::".to_vec()),
                Token::new(T_STATIC, b"static".to_vec()),
            ),
        }
    }
}

struct PhpUnitTestCaseStaticMethodCallsFixer {
    call_type: CallType,
    methods: HashMap<String, CallType>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitTestCaseStaticMethodCallsFixer {
        call_type: CallType::Static,
        methods: HashMap::new(),
    })
}

impl Fixer for PhpUnitTestCaseStaticMethodCallsFixer {
    fn name(&self) -> &'static str {
        "php_unit_test_case_static_method_calls"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let name = self.name();
        let c = resolve(name, options, &["call_type", "methods", "target"])?;
        // `Future::getV4OrV3(CALL_TYPE_THIS, CALL_TYPE_STATIC)` and `(VERSION_NEWEST, VERSION_10_0)`: v3 defaults.
        let call_type = c.string("call_type", "static", &["this", "self", "static"])?;
        c.check_types("methods", &["string[]"])?; // setAllowedTypes(['array<string, string>'])
        let mut methods = HashMap::new();
        for (method, value) in c.php_array("methods").unwrap_or_default() {
            let known = matches!(method, PhpKey::Str(m) if METHODS.contains(&m));
            if !known {
                bail!(
                    "[{name}] Invalid configuration: Unexpected \"methods\" key, expected any of {}, got \"{}\".",
                    natural_language_join(METHODS),
                    method.typed()
                );
            }
            let method = method.typed()["string#".len()..].to_string();
            let value = value.as_str().unwrap();
            let Some(value_type) = CallType::parse(value) else {
                bail!(
                    "[{name}] Invalid configuration: Unexpected value for method \"{method}\", expected any of {}, got \"string#{value}\".",
                    natural_language_join(["this", "self", "static"])
                );
            };
            methods.insert(method, value_type);
        }
        let target = c.string("target", "10.0", &["10.0", "11.0", "newest"])?;
        // `configurePostNormalisation`.
        let mut dynamic_methods: Vec<&str> = Vec::new();
        if target_fulfills(&target, "11.0") {
            dynamic_methods.extend([
                "any",
                "atLeast",
                "atLeastOnce",
                "atMost",
                "exactly",
                "never",
                "once",
                "throwException",
            ]);
        }
        if target == "11.0" {
            dynamic_methods.extend([
                "onConsecutiveCalls",
                "returnArgument",
                "returnCallback",
                "returnSelf",
                "returnValue",
                "returnValueMap",
            ]);
        }
        for method in dynamic_methods {
            if methods.contains_key(method) {
                bail!(
                    "[{name}] Configuration cannot contain method \"{method}\" and target \"{target}\", it is dynamic in that PHPUnit version."
                );
            }
            methods.insert(method.to_string(), CallType::This);
        }
        self.call_type = CallType::parse(&call_type).unwrap();
        self.methods = methods;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            self.apply_php_unit_class_fix(tokens, start_index, end_index)
        });
    }
}

impl PhpUnitTestCaseStaticMethodCallsFixer {
    fn apply_php_unit_class_fix(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let mut index = start_index;
        while index < end_index {
            // skip anonymous classes
            if tokens[index].is_kind(T_CLASS) {
                index = find_end_of_next_block(tokens, index) + 1;
                continue;
            }
            let mut call_type = self.call_type;
            if tokens[index].is_kind(T_FUNCTION) {
                let analyzer = TokensAnalyzer::new(tokens);
                // skip lambda
                if analyzer.is_lambda(index) {
                    index = find_end_of_next_block(tokens, index) + 1;
                    continue;
                }
                // do not change `self` to `this` in static methods
                if call_type == CallType::This && analyzer.get_method_attributes(index).static_ {
                    index = find_end_of_next_block(tokens, index) + 1;
                    continue;
                }
            }
            let content = tokens[index].get_content();
            if !tokens[index].is_kind(T_STRING) || !METHODS.iter().any(|m| m.as_bytes() == content)
            {
                index += 1;
                continue;
            }
            let next_index = tokens.get_next_meaningful_token(index).unwrap();
            if !tokens[next_index].equals("(", true) {
                index = next_index + 1;
                continue;
            }
            let after = tokens.get_next_meaningful_token(next_index).unwrap();
            if tokens[after].is_kind(CT_FIRST_CLASS_CALLABLE) {
                index += 1;
                continue;
            }
            let method_name = String::from_utf8_lossy(content).into_owned();
            if let Some(&method_call_type) = self.methods.get(&method_name) {
                call_type = method_call_type;
            }
            let operator_index = tokens.get_prev_meaningful_token(index).unwrap();
            let reference_index = tokens.get_prev_meaningful_token(operator_index).unwrap();
            let (operator, reference) = call_type.conversion();
            // `needsConversion`
            if FunctionsAnalyzer::new().is_the_same_class_call(tokens, index)
                && !tokens[reference_index].equals(&reference, false)
            {
                tokens.set(operator_index, operator);
                tokens.set(reference_index, reference);
            }
            index += 1;
        }
    }
}

/// `findEndOfNextBlock`.
fn find_end_of_next_block(tokens: &Tokens, index: usize) -> usize {
    let next_index = tokens
        .get_next_token_of_kind(index, &[";".into(), "{".into()], true)
        .unwrap();
    if tokens[next_index].equals("{", true) {
        tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, next_index)
    } else {
        next_index
    }
}
