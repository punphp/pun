//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDedicateAssertFixer`.
use crate::analyzers::abstract_php_unit_fixer::{
    AssertCall, PreviousAssertCalls, apply_fix, is_candidate, target_fulfills,
};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{strpos, strtolower};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

/// A `FIX_MAP` entry that is an array: the positive and negative assertions (`None` for `false`), the argument
/// count and whether the two arguments swap.
struct FixDetails {
    positive: &'static str,
    negative: Option<&'static str>,
    argument_count: usize,
    swap_arguments: bool,
}

const fn details(positive: &'static str, negative: Option<&'static str>) -> FixDetails {
    FixDetails {
        positive,
        negative,
        argument_count: 1,
        swap_arguments: false,
    }
}

/// The array entries of `FIX_MAP`; its `true` entries (`is_array`, `is_bool`, ...) are absent, as `isset` and
/// `is_array` treat them alike with a function `FIX_MAP` lacks.
fn fix_map(function: &[u8]) -> Option<FixDetails> {
    Some(match function {
        b"array_key_exists" => FixDetails {
            argument_count: 2,
            ..details("assertArrayHasKey", Some("assertArrayNotHasKey"))
        },
        b"empty" => details("assertEmpty", Some("assertNotEmpty")),
        b"file_exists" => details("assertFileExists", Some("assertFileNotExists")),
        b"is_dir" => details("assertDirectoryExists", Some("assertDirectoryNotExists")),
        b"is_infinite" => details("assertInfinite", Some("assertFinite")),
        b"is_nan" => details("assertNan", None),
        b"is_null" => details("assertNull", Some("assertNotNull")),
        b"is_readable" => details("assertIsReadable", Some("assertNotIsReadable")),
        b"is_writable" => details("assertIsWritable", Some("assertNotIsWritable")),
        b"str_contains" => FixDetails {
            argument_count: 2,
            swap_arguments: true,
            ..details(
                "assertStringContainsString",
                Some("assertStringNotContainsString"),
            )
        },
        b"str_ends_with" => FixDetails {
            argument_count: 2,
            swap_arguments: true,
            ..details("assertStringEndsWith", Some("assertStringEndsNotWith"))
        },
        b"str_starts_with" => FixDetails {
            argument_count: 2,
            swap_arguments: true,
            ..details("assertStringStartsWith", Some("assertStringStartsNotWith"))
        },
        _ => return None,
    })
}

struct PhpUnitDedicateAssertFixer {
    functions: Vec<&'static str>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitDedicateAssertFixer {
        functions: functions("newest"),
    })
}

/// `configurePostNormalisation`.
fn functions(target: &str) -> Vec<&'static str> {
    let mut functions = vec![
        "array_key_exists",
        "file_exists",
        "is_null",
        "str_ends_with",
        "str_starts_with",
    ];
    if target_fulfills(target, "3.5") {
        functions.extend([
            "empty",
            "is_array",
            "is_bool",
            "is_boolean",
            "is_callable",
            "is_double",
            "is_float",
            "is_int",
            "is_integer",
            "is_long",
            "is_numeric",
            "is_object",
            "is_real",
            "is_scalar",
            "is_string",
        ]);
    }
    if target_fulfills(target, "5.0") {
        functions.extend(["is_infinite", "is_nan"]);
    }
    if target_fulfills(target, "5.6") {
        functions.extend(["is_dir", "is_readable", "is_writable"]);
    }
    if target_fulfills(target, "7.5") {
        functions.push("str_contains");
    }
    functions
}

impl Fixer for PhpUnitDedicateAssertFixer {
    fn name(&self) -> &'static str {
        "php_unit_dedicate_assert"
    }
    fn priority(&self) -> i32 {
        -9
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target"])?;
        let target = c.string("target", "newest", &["3.0", "3.5", "5.0", "5.6", "newest"])?;
        self.functions = functions(&target);
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            let mut calls = PreviousAssertCalls::new(start_index, end_index);
            while let Some(assert_call) = calls.next(tokens) {
                // test and fix for assertTrue/False to dedicated asserts
                match assert_call.lowered_name.as_slice() {
                    b"asserttrue" | b"assertfalse" => {
                        self.fix_assert_true_false(tokens, &assert_call)
                    }
                    b"assertsame" | b"assertnotsame" | b"assertequals" | b"assertnotequals" => {
                        fix_assert_same_equals(tokens, &assert_call)
                    }
                    _ => {}
                }
            }
        });
    }
}

impl PhpUnitDedicateAssertFixer {
    /// `fixAssertTrueFalse`.
    fn fix_assert_true_false(&self, tokens: &mut Tokens, assert_call: &AssertCall) {
        let mut test_default_namespace_token_index = None;
        let mut test_index = tokens
            .get_next_meaningful_token(assert_call.open_brace_index)
            .unwrap();
        if !tokens[test_index].is_given_kind(&[T_EMPTY, T_STRING]) {
            if fix_assert_true_false_instanceof(tokens, assert_call, test_index) {
                return;
            }
            if !tokens[test_index].is_kind(T_NS_SEPARATOR) {
                return;
            }
            test_default_namespace_token_index = Some(test_index);
            test_index = tokens.get_next_meaningful_token(test_index).unwrap();
        }
        let test_open_index = tokens.get_next_meaningful_token(test_index).unwrap();
        if !tokens[test_open_index].equals("(", true) {
            return;
        }
        let test_close_index =
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, test_open_index);
        let assert_call_close_index = tokens.get_next_meaningful_token(test_close_index).unwrap();
        if !tokens[assert_call_close_index].equals_any(&[")".into(), ",".into()], true) {
            return;
        }
        let content = strtolower(tokens[test_index].get_content());
        if !self.functions.iter().any(|f| f.as_bytes() == content) {
            return;
        }
        let arguments = ArgumentsAnalyzer.get_arguments(tokens, test_open_index, test_close_index);
        let is_positive = assert_call.lowered_name == b"asserttrue";
        if let Some(fix_details) = fix_map(&content) {
            if fix_details.argument_count != arguments.len() {
                return;
            }
            let Some(name) = (if is_positive {
                Some(fix_details.positive)
            } else {
                fix_details.negative
            }) else {
                return;
            };
            tokens.set(
                assert_call.index,
                Token::new(T_STRING, name.as_bytes().to_vec()),
            );
            remove_function_call(
                tokens,
                test_default_namespace_token_index,
                test_index,
                test_open_index,
                test_close_index,
            );
            if fix_details.swap_arguments {
                swap_arguments(tokens, &arguments);
            }
            return;
        }
        if arguments.len() != 1 {
            return;
        }
        let type_ = &content[3..];
        let name: &[u8] = if is_positive {
            b"assertInternalType"
        } else {
            b"assertNotInternalType"
        };
        tokens.set(assert_call.index, Token::new(T_STRING, name.to_vec()));
        tokens.set(
            test_index,
            Token::new(T_CONSTANT_ENCAPSED_STRING, [b"'", type_, b"'"].concat()),
        );
        tokens.set(test_open_index, Token::char(b","));
        tokens.clear_token_and_merge_surrounding_whitespace(test_close_index);
        let comma_index = tokens.get_prev_meaningful_token(test_close_index).unwrap();
        if tokens[comma_index].equals(",", true) {
            tokens.remove_trailing_whitespace(comma_index, None);
            tokens.clear_at(comma_index);
        }
        if !tokens[test_open_index + 1].is_whitespace(None) {
            tokens.insert_at(
                test_open_index + 1,
                vec![Token::new(T_WHITESPACE, b" ".to_vec())],
            );
        }
        if let Some(index) = test_default_namespace_token_index {
            tokens.clear_token_and_merge_surrounding_whitespace(index);
        }
    }
}

/// `fixAssertTrueFalseInstanceof`.
fn fix_assert_true_false_instanceof(
    tokens: &mut Tokens,
    assert_call: &AssertCall,
    test_index: usize,
) -> bool {
    let is_positive_assertion = assert_call.lowered_name == b"asserttrue";
    let (variable_index, is_positive_condition) = if tokens[test_index].equals("!", true) {
        (tokens.get_next_meaningful_token(test_index).unwrap(), false)
    } else {
        (test_index, true)
    };
    if !tokens[variable_index].is_kind(T_VARIABLE) {
        return false;
    }
    let instance_of_index = tokens.get_next_meaningful_token(variable_index).unwrap();
    if !tokens[instance_of_index].is_kind(T_INSTANCEOF) {
        return false;
    }
    let mut class_end_index = tokens.get_next_meaningful_token(instance_of_index).unwrap();
    let mut class_part_tokens = Vec::new();
    loop {
        class_part_tokens.push(tokens[class_end_index].clone());
        class_end_index = tokens.get_next_meaningful_token(class_end_index).unwrap();
        if !tokens[class_end_index].is_given_kind(&[T_STRING, T_NS_SEPARATOR, T_VARIABLE]) {
            break;
        }
    }
    if tokens[class_end_index].equals_any(&[",".into(), ")".into()], true) {
        // do the fixing
        let is_instance_of_var = class_part_tokens[0].is_kind(T_VARIABLE);
        let mut new_tokens = class_part_tokens;
        if !is_instance_of_var {
            new_tokens.push(Token::new(T_DOUBLE_COLON, b"::".to_vec()));
            new_tokens.push(Token::new(CT_CLASS_CONSTANT, b"class".to_vec()));
        }
        new_tokens.push(Token::char(b","));
        new_tokens.push(Token::new(T_WHITESPACE, b" ".to_vec()));
        new_tokens.push(tokens[variable_index].clone());
        for i in (test_index..class_end_index).rev() {
            if !tokens[i].is_comment() {
                tokens.clear_token_and_merge_surrounding_whitespace(i);
            }
        }
        let name: &[u8] = if is_positive_assertion == is_positive_condition {
            b"assertInstanceOf"
        } else {
            b"assertNotInstanceOf"
        };
        // Upstream keys each new token by its own index from `testIndex` on, so each goes before a different old token.
        let slices: BTreeMap<usize, Vec<Token>> = new_tokens
            .into_iter()
            .enumerate()
            .map(|(k, token)| (test_index + k, vec![token]))
            .collect();
        tokens.insert_slices(slices);
        tokens.set(assert_call.index, Token::new(T_STRING, name.to_vec()));
    }
    true
}

/// `fixAssertSameEquals`.
fn fix_assert_same_equals(tokens: &mut Tokens, assert_call: &AssertCall) {
    // @ $this->/self::assertEquals/Same([$nextIndex])
    let expected_index = tokens
        .get_next_meaningful_token(assert_call.open_brace_index)
        .unwrap();
    // do not fix
    // let $a = [1,2]; $b = "2";
    // "$this->assertEquals("2", count($a)); $this->assertEquals($b, count($a)); $this->assertEquals(2.1, count($a));"
    if tokens[expected_index].is_kind(T_VARIABLE) {
        let next = tokens.get_next_meaningful_token(expected_index).unwrap();
        if !tokens[next].equals(",", true) {
            return;
        }
    } else if !tokens[expected_index].is_kind(T_LNUMBER) {
        return;
    }
    // @ $this->/self::assertEquals/Same([$nextIndex,$commaIndex])
    let comma_index = tokens.get_next_meaningful_token(expected_index).unwrap();
    if !tokens[comma_index].equals(",", true) {
        return;
    }
    // @ $this->/self::assertEquals/Same([$nextIndex,$commaIndex,$countCallIndex])
    let mut count_call_index = tokens.get_next_meaningful_token(comma_index).unwrap();
    let default_namespace_token_index = if tokens[count_call_index].is_kind(T_NS_SEPARATOR) {
        let index = count_call_index;
        count_call_index = tokens.get_next_meaningful_token(count_call_index).unwrap();
        Some(index)
    } else {
        None
    };
    if !tokens[count_call_index].is_kind(T_STRING) {
        return;
    }
    let lower_content = strtolower(tokens[count_call_index].get_content());
    if lower_content != b"count" && lower_content != b"sizeof" {
        return; // not a call to "count" or "sizeOf"
    }
    let count_call_open_brace_index = tokens.get_next_meaningful_token(count_call_index).unwrap();
    if !tokens[count_call_open_brace_index].equals("(", true) {
        return;
    }
    let count_call_close_brace_index =
        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, count_call_open_brace_index);
    let after = tokens
        .get_next_meaningful_token(count_call_close_brace_index)
        .unwrap();
    if !tokens[after].equals_any(&[")".into(), ",".into()], true) {
        return;
    }
    remove_function_call(
        tokens,
        default_namespace_token_index,
        count_call_index,
        count_call_open_brace_index,
        count_call_close_brace_index,
    );
    let name: &[u8] = if strpos(&assert_call.lowered_name, b"not", 6).is_none() {
        b"assertCount"
    } else {
        b"assertNotCount"
    };
    tokens.set(assert_call.index, Token::new(T_STRING, name.to_vec()));
}

/// `removeFunctionCall`.
fn remove_function_call(
    tokens: &mut Tokens,
    call_ns_index: Option<usize>,
    call_index: usize,
    open_index: usize,
    close_index: usize,
) {
    tokens.clear_token_and_merge_surrounding_whitespace(call_index);
    if let Some(index) = call_ns_index {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    }
    tokens.clear_token_and_merge_surrounding_whitespace(open_index);
    let comma_index = tokens.get_prev_meaningful_token(close_index).unwrap();
    if tokens[comma_index].equals(",", true) {
        tokens.remove_trailing_whitespace(comma_index, None);
        tokens.clear_at(comma_index);
    }
    tokens.clear_token_and_merge_surrounding_whitespace(close_index);
}

/// `swapArguments`, with `cloneAndClearTokens`.
fn swap_arguments(tokens: &mut Tokens, arguments: &BTreeMap<usize, usize>) {
    let mut arguments = arguments.iter();
    let (&first_argument_index, &first_argument_end_index) = arguments.next().unwrap();
    let (&second_argument_index, &second_argument_end_index) = arguments.next().unwrap();
    let mut first_clone =
        clone_and_clear_tokens(tokens, first_argument_index, first_argument_end_index);
    let mut second_clone =
        clone_and_clear_tokens(tokens, second_argument_index, second_argument_end_index);
    if !first_clone[0].is_whitespace(None) {
        first_clone.insert(0, Token::new(T_WHITESPACE, b" ".to_vec()));
    }
    tokens.insert_at(second_argument_index, first_clone);
    if second_clone[0].is_whitespace(None) {
        second_clone.remove(0);
    }
    tokens.insert_at(first_argument_index, second_clone);
}

/// `cloneAndClearTokens`.
fn clone_and_clear_tokens(tokens: &mut Tokens, start: usize, end: usize) -> Vec<Token> {
    let mut clone = Vec::new();
    for i in start..=end {
        if tokens[i].get_content().is_empty() {
            continue;
        }
        clone.push(tokens[i].clone());
        tokens.clear_at(i);
    }
    clone
}
