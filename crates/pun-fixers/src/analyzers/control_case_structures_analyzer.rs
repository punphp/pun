//! Port of `PhpCsFixer\Tokenizer\Analyzer\ControlCaseStructuresAnalyzer`, plus the analysis value
//! objects it builds (`Analysis\{Switch,Match,Enum,Case,Default}Analysis`).
use pun_tokens::{Kind, Kind::*, Tokens};

const SUPPORTED_TYPES_WITH_CASE_OR_DEFAULT: &[Kind] = &[T_SWITCH, T_MATCH, T_ENUM];

/// `CaseAnalysis`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseAnalysis {
    index: usize,
    colon_index: usize,
}

impl CaseAnalysis {
    pub fn get_index(&self) -> usize {
        self.index
    }
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

/// `AbstractControlCaseStructuresAnalysis` and its subclasses: `kind` is `T_SWITCH` (SwitchAnalysis:
/// cases + default), `T_MATCH` (MatchAnalysis: default only) or `T_ENUM` (EnumAnalysis: cases only).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlCaseStructuresAnalysis {
    kind: Kind,
    index: usize,
    open_index: usize,
    close_index: usize,
    cases: Vec<CaseAnalysis>,
    default: Option<DefaultAnalysis>,
}

impl ControlCaseStructuresAnalysis {
    pub fn get_kind(&self) -> Kind {
        self.kind
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn get_open_index(&self) -> usize {
        self.open_index
    }
    pub fn get_close_index(&self) -> usize {
        self.close_index
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
    index: usize,
    brace_count: i64,
    cases: Vec<(usize, usize)>,
    default: Option<(usize, usize)>,
    alternative_syntax: bool,
    open: Option<usize>,
}

pub struct ControlCaseStructuresAnalyzer;

impl ControlCaseStructuresAnalyzer {
    /// `findControlStructures($tokens, $types)`: analyses in the order upstream's generator yields them
    /// (innermost structure first). Collected eagerly; callers that only clear or retag tokens see the
    /// same results as with the lazy generator.
    pub fn find_control_structures(tokens: &Tokens, types: &[Kind]) -> Vec<ControlCaseStructuresAnalysis> {
        let mut out = Vec::new();
        if types.is_empty() {
            return out; // quick skip
        }
        for t in types {
            assert!(SUPPORTED_TYPES_WITH_CASE_OR_DEFAULT.contains(t), "Unexpected type \"{}\".", t.name());
        }
        if !tokens.is_any_token_kinds_found(types) {
            return out; // quick skip
        }

        let mut stack: Vec<Frame> = Vec::new();
        let mut is_type_of_interest = false;

        let mut index = 0;
        while index < tokens.count() {
            let token = &tokens[index];
            if token.is_given_kind(SUPPORTED_TYPES_WITH_CASE_OR_DEFAULT) {
                let kind = token.id.unwrap();
                let mut frame = Frame {
                    kind,
                    index,
                    brace_count: 0,
                    cases: Vec::new(),
                    default: None,
                    alternative_syntax: false,
                    open: None,
                };
                is_type_of_interest = types.contains(&kind);
                // upstream reassigns the foreach variable `$index` here, which does not affect the iteration
                if kind == T_SWITCH {
                    let i = tokens.get_next_meaningful_token(index).unwrap();
                    let i = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i);
                    let open = tokens.get_next_meaningful_token(i).unwrap();
                    frame.open = Some(open);
                    frame.alternative_syntax = tokens[open].equals(":", true);
                } else if kind == T_MATCH {
                    let i = tokens.get_next_meaningful_token(index).unwrap();
                    let i = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i);
                    frame.open = tokens.get_next_meaningful_token(i);
                } else {
                    frame.open = tokens.get_next_token_of_kind(index, &["{".into()], true);
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
                if top.brace_count == 0 {
                    if top.alternative_syntax {
                        index += 1;
                        continue;
                    }
                    let frame = stack.pop().unwrap();
                    if is_type_of_interest {
                        out.push(Self::build_control_case_structure_analysis(frame, index));
                    }
                    if let Some(f) = stack.last() {
                        is_type_of_interest = types.contains(&f.kind);
                    }
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
                if is_type_of_interest {
                    out.push(Self::build_control_case_structure_analysis(frame, current));
                }
                if let Some(f) = stack.last() {
                    is_type_of_interest = types.contains(&f.kind);
                }
            }

            if !is_type_of_interest {
                index += 1;
                continue; // don't bother to analyse stuff that caller is not interested in
            }

            // `$token` is still the token at the original foreach index.
            if token.is_kind(T_CASE) {
                let top = stack.last_mut().expect("case outside of a control structure");
                let open = Self::find_case_open(tokens, top.kind, current);
                top.cases.push((current, open));
            } else if token.is_kind(T_DEFAULT) {
                let top = stack.last_mut().expect("default outside of a control structure");
                assert!(top.default.is_none(), "Analysis multiple \"default\" found.");
                let open = Self::find_default_open(tokens, top.kind, current);
                top.default = Some((current, open));
            }
            index += 1;
        }
        out
    }

    fn build_control_case_structure_analysis(frame: Frame, end: usize) -> ControlCaseStructuresAnalysis {
        let default = frame.default.map(|(index, colon_index)| DefaultAnalysis { index, colon_index });
        let mut cases: Vec<CaseAnalysis> =
            frame.cases.iter().map(|&(index, colon_index)| CaseAnalysis { index, colon_index }).collect();
        cases.sort();
        let open_index = frame.open.expect("open index");
        match frame.kind {
            T_SWITCH => ControlCaseStructuresAnalysis { kind: T_SWITCH, index: frame.index, open_index, close_index: end, cases, default },
            T_ENUM => ControlCaseStructuresAnalysis { kind: T_ENUM, index: frame.index, open_index, close_index: end, cases, default: None },
            _ => ControlCaseStructuresAnalysis { kind: T_MATCH, index: frame.index, open_index, close_index: end, cases: Vec::new(), default },
        }
    }

    fn find_case_open(tokens: &Tokens, kind: Kind, mut index: usize) -> usize {
        if kind == T_SWITCH {
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
                        break;
                    }
                    ternaries_count -= 1;
                }
                index += 1;
            }
            return index;
        }
        if kind == T_ENUM {
            return tokens.get_next_token_of_kind(index, &["=".into(), ";".into()], true).unwrap();
        }
        panic!("Unexpected case for type \"{}\".", kind.name());
    }

    fn find_default_open(tokens: &Tokens, kind: Kind, index: usize) -> usize {
        if kind == T_SWITCH {
            return tokens.get_next_token_of_kind(index, &[":".into(), ";".into()], true).unwrap();
        }
        if kind == T_MATCH {
            return tokens.get_next_token_of_kind(index, &[T_DOUBLE_ARROW.into()], true).unwrap();
        }
        panic!("Unexpected default for type \"{}\".", kind.name());
    }
}
