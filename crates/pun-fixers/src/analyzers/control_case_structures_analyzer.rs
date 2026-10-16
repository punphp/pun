//! Port of `PhpCsFixer\Tokenizer\Analyzer\ControlCaseStructuresAnalyzer` for `[T_SWITCH]`, the only type Pun's
//! callers ask for, plus the analysis value objects it builds (`Analysis\{Switch,Case,Default}Analysis`).
//! `match` and `enum` structures are still tracked (their braces and `case`/`default` must not leak into an
//! enclosing switch) but never reported.
use pun_tokens::{Kind, Kind::*, Tokens};

const SUPPORTED_TYPES_WITH_CASE_OR_DEFAULT: &[Kind] = &[T_SWITCH, T_MATCH, T_ENUM];

/// `CaseAnalysis`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseAnalysis {
    index: usize,
    colon_index: usize,
}

impl CaseAnalysis {
    pub fn get_colon_index(&self) -> usize {
        self.colon_index
    }
}

/// `DefaultAnalysis`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefaultAnalysis {
    index: usize,
    colon_index: usize,
}

impl DefaultAnalysis {
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn get_colon_index(&self) -> usize {
        self.colon_index
    }
}

/// `SwitchAnalysis`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlCaseStructuresAnalysis {
    open_index: usize,
    cases: Vec<CaseAnalysis>,
    default: Option<DefaultAnalysis>,
}

impl ControlCaseStructuresAnalysis {
    pub fn get_open_index(&self) -> usize {
        self.open_index
    }
    pub fn get_cases(&self) -> &[CaseAnalysis] {
        &self.cases
    }
    pub fn get_default_analysis(&self) -> Option<DefaultAnalysis> {
        self.default
    }
}

struct Frame {
    kind: Kind,
    brace_count: i64,
    cases: Vec<(usize, usize)>,
    default: Option<(usize, usize)>,
    alternative_syntax: bool,
    open: usize,
}

pub struct ControlCaseStructuresAnalyzer;

impl ControlCaseStructuresAnalyzer {
    /// `findControlStructures($tokens, [T_SWITCH])`: analyses in the order upstream's generator yields them
    /// (innermost structure first). Collected eagerly; callers that only clear or retag tokens see the
    /// same results as with the lazy generator.
    pub fn find_switch_structures(tokens: &Tokens) -> Vec<ControlCaseStructuresAnalysis> {
        let mut out = Vec::new();
        if !tokens.is_token_kind_found(T_SWITCH) {
            return out; // quick skip
        }

        let mut stack: Vec<Frame> = Vec::new();
        let mut is_type_of_interest = false;

        let mut index = 0;
        while index < tokens.count() {
            let token = &tokens[index];
            if token.is_given_kind(SUPPORTED_TYPES_WITH_CASE_OR_DEFAULT) {
                let kind = token.id.unwrap();
                let mut frame = Frame { kind, brace_count: 0, cases: Vec::new(), default: None, alternative_syntax: false, open: 0 };
                is_type_of_interest = kind == T_SWITCH;
                // upstream reassigns the foreach variable `$index` here, which does not affect the iteration
                if kind == T_SWITCH {
                    let i = tokens.get_next_meaningful_token(index).unwrap();
                    let i = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i);
                    frame.open = tokens.get_next_meaningful_token(i).unwrap();
                    frame.alternative_syntax = tokens[frame.open].equals(":", true);
                }
                stack.push(frame);
                index += 1;
                continue;
            }

            let Some(top) = stack.last_mut() else {
                index += 1;
                continue;
            };

            if token.equals("{", true) {
                top.brace_count += 1;
                index += 1;
                continue;
            }

            if token.equals("}", true) {
                top.brace_count -= 1;
                if top.brace_count == 0 && !top.alternative_syntax {
                    let frame = stack.pop().unwrap();
                    if is_type_of_interest {
                        out.push(Self::build_switch_analysis(frame));
                    }
                    is_type_of_interest = stack.last().is_some_and(|f| f.kind == T_SWITCH);
                }
                index += 1;
                continue;
            }

            let mut current = index;
            if token.is_kind(T_ENDSWITCH) {
                assert!(top.alternative_syntax, "Analysis syntax failure, unexpected \"T_ENDSWITCH\".");
                assert!(top.kind == T_SWITCH, "Analysis type failure, unexpected \"T_ENDSWITCH\".");
                assert!(top.brace_count == 0, "Analysis count failure, unexpected \"T_ENDSWITCH\".");
                // upstream reassigns `$index` (used below for `$tokens[$index]` lookups only via `$token`)
                current = tokens.get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true).unwrap();
                let frame = stack.pop().unwrap();
                out.push(Self::build_switch_analysis(frame));
                is_type_of_interest = stack.last().is_some_and(|f| f.kind == T_SWITCH);
            }

            if !is_type_of_interest {
                index += 1;
                continue; // don't bother to analyse stuff that caller is not interested in
            }

            // `$token` is still the token at the original foreach index.
            let top = stack.last_mut().expect("case outside of a control structure");
            if token.is_kind(T_CASE) {
                let open = Self::find_case_open(tokens, current);
                top.cases.push((current, open));
            } else if token.is_kind(T_DEFAULT) {
                assert!(top.default.is_none(), "Analysis multiple \"default\" found.");
                let open = tokens.get_next_token_of_kind(current, &[":".into(), ";".into()], true).unwrap();
                top.default = Some((current, open));
            }
            index += 1;
        }
        out
    }

    fn build_switch_analysis(frame: Frame) -> ControlCaseStructuresAnalysis {
        let default = frame.default.map(|(index, colon_index)| DefaultAnalysis { index, colon_index });
        let mut cases: Vec<CaseAnalysis> =
            frame.cases.iter().map(|&(index, colon_index)| CaseAnalysis { index, colon_index }).collect();
        cases.sort();
        ControlCaseStructuresAnalysis { open_index: frame.open, cases, default }
    }

    /// `findCaseOpen` for a switch: the `:`/`;` after the case value, skipping blocks and ternaries.
    fn find_case_open(tokens: &Tokens, mut index: usize) -> usize {
        let mut ternaries_count = 0;
        loop {
            if tokens[index].equals_any(&["(".into(), "{".into()], true) {
                // skip constructs
                let (t, _) = Tokens::detect_block_type(&tokens[index]).unwrap();
                index = tokens.find_block_end(t, index) + 1;
                continue;
            }
            if tokens[index].equals("?", true) {
                ternaries_count += 1;
                index += 1;
                continue;
            }
            if tokens[index].equals_any(&[":".into(), ";".into()], true) {
                if ternaries_count == 0 {
                    return index;
                }
                ternaries_count -= 1;
            }
            index += 1;
        }
    }
}
