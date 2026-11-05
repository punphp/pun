//! PHPStan 2.3.1's `RegexGroupParser` with its value objects (`RegexAlternation`, `RegexCapturingGroup`,
//! `RegexNonCapturingGroup`, `RegexAstWalkResult`, `RegexGroupWalkResult`) and `RegexExpressionHelper`'s pattern
//! helpers.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};

use pun_phpstan::types::Tri;

use super::hoa::{self, Node};
use super::types::{Atom, Flags, RType};

/// `NOT_SUPPORTED_MODIFIERS`.
const NOT_SUPPORTED_MODIFIERS: &[char] = &['J'];

/// `LITERALS_LIMIT`.
const LITERALS_LIMIT: usize = 100;

/// `RegexAlternation`; its id is its index in [`Walk::alternations`].
#[derive(Clone, Debug)]
pub(super) struct Alternation {
    pub(super) count: usize,
    /// `getGroupCombinations()`: capture group ids by branch index, in insertion order of the branch index.
    pub(super) combinations: Vec<(usize, Vec<i64>)>,
}

/// `RegexCapturingGroup`.
#[derive(Clone, Debug)]
pub(super) struct Capturing {
    pub(super) id: i64,
    pub(super) name: Option<String>,
    pub(super) alternation: Option<usize>,
    in_optional_quantification: bool,
    parent: Option<Rc<Parent>>,
    ty: RType,
    force_non_optional: bool,
    force_type: Option<RType>,
    force_alternation_branch_matched: bool,
}

/// `RegexNonCapturingGroup`.
#[derive(Clone, Debug)]
pub(super) struct NonCapturing {
    alternation: Option<usize>,
    in_optional_quantification: bool,
    parent: Option<Rc<Parent>>,
    reset_group_counter: bool,
}

#[derive(Clone, Debug)]
pub(super) enum Parent {
    Capturing(Capturing),
    NonCapturing(NonCapturing),
}

impl Parent {
    fn alternation_id(&self) -> Option<usize> {
        match self {
            Parent::Capturing(g) => g.alternation,
            Parent::NonCapturing(g) => g.alternation,
        }
    }

    fn is_optional(&self) -> bool {
        match self {
            Parent::Capturing(g) => g.is_optional(),
            Parent::NonCapturing(g) => g.is_optional(),
        }
    }

    fn in_optional_quantification(&self) -> bool {
        match self {
            Parent::Capturing(g) => g.in_optional_quantification,
            Parent::NonCapturing(g) => g.in_optional_quantification,
        }
    }

    pub(super) fn parent(&self) -> Option<&Rc<Parent>> {
        match self {
            Parent::Capturing(g) => g.parent.as_ref(),
            Parent::NonCapturing(g) => g.parent.as_ref(),
        }
    }

    fn resets_group_counter(&self) -> bool {
        match self {
            Parent::Capturing(g) => g.resets_group_counter(),
            Parent::NonCapturing(g) => g.reset_group_counter,
        }
    }
}

impl NonCapturing {
    fn is_optional(&self) -> bool {
        self.alternation.is_some()
            || self.in_optional_quantification
            || self.parent.as_ref().is_some_and(|p| p.is_optional())
    }

    fn is_top_level(&self) -> bool {
        match &self.parent {
            None => true,
            Some(p) => matches!(**p, Parent::NonCapturing(ref g) if g.is_top_level()),
        }
    }
}

impl Capturing {
    pub(super) fn force_non_optional(&self) -> Self {
        Self {
            force_non_optional: true,
            ..self.clone()
        }
    }

    pub(super) fn force_alternation_branch_matched(&self) -> Self {
        Self {
            force_alternation_branch_matched: true,
            ..self.clone()
        }
    }

    /// `forceType()`: it sets the group's type, not its forced type, as upstream does.
    pub(super) fn force_type(&self, ty: RType) -> Self {
        Self { ty, ..self.clone() }
    }

    pub(super) fn with_parent(&self, parent: Capturing) -> Self {
        Self {
            parent: Some(Rc::new(Parent::Capturing(parent))),
            ..self.clone()
        }
    }

    pub(super) fn parent(&self) -> Option<&Rc<Parent>> {
        self.parent.as_ref()
    }

    pub(super) fn resets_group_counter(&self) -> bool {
        self.parent
            .as_ref()
            .is_some_and(|p| matches!(**p, Parent::NonCapturing(ref g) if g.reset_group_counter))
    }

    pub(super) fn is_optional(&self) -> bool {
        if self.force_non_optional {
            return false;
        }
        if self.force_alternation_branch_matched {
            return self.is_optional_within_alternation_branch();
        }
        self.alternation.is_some()
            || self.in_optional_quantification
            || self.parent.as_ref().is_some_and(|p| p.is_optional())
    }

    fn is_optional_within_alternation_branch(&self) -> bool {
        if self.in_optional_quantification {
            return true;
        }
        let mut parent = self.parent.as_ref();
        while let Some(p) = parent.filter(|p| p.alternation_id() == self.alternation) {
            if p.in_optional_quantification() {
                return true;
            }
            parent = p.parent();
        }
        false
    }

    pub(super) fn in_optional_quantification(&self) -> bool {
        self.in_optional_quantification
    }

    /// Asked only of a group in the top-level alternation, so upstream's `!$this->inAlternation()` case does not occur.
    pub(super) fn in_optional_alternation(&self) -> bool {
        let mut parent = self.parent.as_ref();
        while let Some(p) = parent.filter(|p| p.alternation_id() == self.alternation) {
            if !matches!(**p, Parent::NonCapturing(_)) {
                return false;
            }
            parent = p.parent();
        }
        parent.is_some_and(|p| p.is_optional())
    }

    pub(super) fn is_top_level(&self) -> bool {
        match &self.parent {
            None => true,
            Some(p) => matches!(**p, Parent::NonCapturing(ref g) if g.is_top_level()),
        }
    }

    pub(super) fn ty(&self) -> &RType {
        self.force_type.as_ref().unwrap_or(&self.ty)
    }
}

/// `RegexAstWalkResult`, with the alternations its groups point into.
#[derive(Clone, Debug)]
pub(super) struct AstWalk {
    capture_group_id: i64,
    pub(super) groups: Vec<Capturing>,
    pub(super) mark_verbs: Vec<String>,
    pub(super) subject_base_type: Atom,
    pub(super) alternations: Vec<Alternation>,
}

/// `RegexGroupWalkResult`.
#[derive(Clone, Debug)]
struct GroupWalk {
    in_optional_quantification: bool,
    only_literals: Option<Vec<String>>,
    non_empty: Tri,
    non_falsy: Tri,
    numeric: Tri,
}

impl GroupWalk {
    fn empty() -> Self {
        Self {
            in_optional_quantification: false,
            only_literals: Some(Vec::new()),
            non_empty: Tri::Maybe,
            non_falsy: Tri::Maybe,
            numeric: Tri::Maybe,
        }
    }

    fn might_contain_empty_string_literal(&self) -> bool {
        self.only_literals
            .as_ref()
            .is_some_and(|l| l.iter().any(String::is_empty))
    }
}

/// `RegexExpressionHelper::getPatternDelimiter()`.
fn delimiter(pattern: &str) -> Option<u8> {
    ltrim(pattern).as_bytes().first().copied()
}

/// PHP's `ltrim()` with its default characters.
fn ltrim(s: &str) -> &str {
    s.trim_start_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
}

/// `RegexExpressionHelper::getEndDelimiterPos()`.
fn end_delimiter_pos(pattern: &str) -> Option<usize> {
    let end = match delimiter(pattern)? {
        b'{' => b'}',
        b'(' => b')',
        b'[' => b']',
        b'<' => b'>',
        d => d,
    };
    pattern.bytes().rposition(|b| b == end)
}

/// `RegexExpressionHelper::getPatternModifiers()`.
pub(super) fn pattern_modifiers(pattern: &str) -> Option<&str> {
    pattern.get(end_delimiter_pos(pattern)? + 1..)
}

/// `RegexExpressionHelper::removeDelimitersAndModifiers()`.
/// The pattern compiled, so its end delimiter follows the start one.
fn remove_delimiters_and_modifiers(pattern: &str) -> Option<&str> {
    let pattern = ltrim(pattern);
    pattern.get(1..end_delimiter_pos(pattern)?)
}

static PARSED: LazyLock<Mutex<HashMap<String, Option<Arc<Node>>>>> = LazyLock::new(Mutex::default);

/// `RegexGroupParser::parseGroups()`; `capture_only_named_supported` is `PhpVersion::supportsPregCaptureOnlyNamedGroups()`.
pub(super) fn parse_groups(regex: &str, capture_only_named_supported: bool) -> Option<AstWalk> {
    let modifiers = pattern_modifiers(regex).unwrap_or("").to_string();
    let ast = PARSED.lock().unwrap().get(regex).cloned();
    let ast = match ast {
        Some(ast) => ast?,
        None => {
            let ast = parse_ast(regex, &modifiers).map(Arc::new);
            PARSED
                .lock()
                .unwrap()
                .insert(regex.to_string(), ast.clone());
            ast?
        }
    };
    let mut ast = (*ast).clone();
    update_alternation_ast(&mut ast);
    update_capturing_ast(&mut ast);

    let capture_only_named = capture_only_named_supported && modifiers.contains('n');
    let mut walk = AstWalk {
        // use different start-index for groups to make it easier to distinguish groupids from other ids
        capture_group_id: 100,
        groups: Vec::new(),
        mark_verbs: Vec::new(),
        subject_base_type: Atom::Str(Flags::default()),
        alternations: Vec::new(),
    };
    let walker = Walker {
        modifiers: &modifiers,
    };
    walker.walk_regex_ast(
        &ast,
        None,
        0,
        false,
        None,
        capture_only_named,
        false,
        &mut walk,
    );

    let subject = walker.walk_group_ast(&ast, false, GroupWalk::empty());
    if !subject.might_contain_empty_string_literal() && !contains_escape_k(&ast) {
        // we could handle numeric-string, in case we know the regex is delimited by ^ and $
        if subject.non_falsy.yes() {
            walk.subject_base_type = Atom::Str(Flags {
                non_empty: true,
                non_falsy: true,
                numeric: false,
            });
        } else if subject.non_empty.yes() {
            walk.subject_base_type = Atom::Str(Flags {
                non_empty: true,
                non_falsy: false,
                numeric: false,
            });
        }
    }
    Some(walk)
}

/// The cached part of `parseGroups()`: the pattern compiles, no unsupported modifier, and hoa parses it.
fn parse_ast(regex: &str, modifiers: &str) -> Option<Node> {
    // pattern is invalid, so let the RegularExpressionPatternRule report it
    pun_phpstan::php::preg(regex).ok()?;
    if modifiers.contains(NOT_SUPPORTED_MODIFIERS) {
        return None;
    }
    let stripped;
    let regex = if modifiers.contains('x') {
        // in freespacing mode the # character starts a comment and runs until the end of the line
        // but \# is an escaped literal hash, and (?#...) is an inline comment - neither starts a line comment
        static COMMENT: LazyLock<pcre2::bytes::Regex> = LazyLock::new(|| {
            pcre2::bytes::Regex::new(r"(?<!\\)(?<!\(\?)#.*").expect("upstream comment pattern")
        });
        let mut out = Vec::new();
        let mut last = 0;
        for m in COMMENT.find_iter(regex.as_bytes()).filter_map(Result::ok) {
            out.extend_from_slice(&regex.as_bytes()[last..m.start()]);
            last = m.end();
        }
        out.extend_from_slice(&regex.as_bytes()[last..]);
        // `#` and `\n` are ASCII, so cutting at them keeps the pattern UTF-8
        stripped = String::from_utf8(out).ok()?;
        stripped.as_str()
    } else {
        regex
    };
    hoa::parse(remove_delimiters_and_modifiers(regex)?)
}

fn empty_token() -> Node {
    Node::Token {
        token: "literal",
        value: String::new(),
    }
}

/// `updateAlternationAstRemoveVerticalBarsAndAddEmptyToken()`. Upstream re-adds an empty branch under the bar's
/// unset key, which PHP appends at the end of the array, so empty branches come after the others.
fn update_alternation_ast(ast: &mut Node) {
    let Node::Rule { id, children } = ast else {
        return;
    };
    for child in children.iter_mut() {
        update_alternation_ast(child);
    }
    if *id != "#alternation" {
        return;
    }
    let is_bar = |n: &Node| n.token() == Some("alternation");
    let empties = (0..children.len())
        .filter(|&i| is_bar(&children[i]) && (i == 0 || children.get(i + 1).is_none_or(is_bar)))
        .count();
    children.retain(|c| !is_bar(c));
    children.extend((0..empties).map(|_| empty_token()));
}

/// `updateCapturingAstAddEmptyToken()`.
fn update_capturing_ast(ast: &mut Node) {
    let Node::Rule { id, children } = ast else {
        return;
    };
    for child in children.iter_mut() {
        update_capturing_ast(child);
    }
    if *id == "#capturing" && children.is_empty() {
        children.push(Node::Rule {
            id: "#alternation",
            children: vec![empty_token()],
        });
    }
}

/// `containsEscapeK()`.
fn contains_escape_k(ast: &Node) -> bool {
    ast.token() == Some("match_point_reset") || ast.children().iter().any(contains_escape_k)
}

/// `getQuantificationRange()`.
fn quantification_range(node: &Node) -> (Option<i64>, Option<i64>) {
    let last = node.children().last();
    let token = last
        .and_then(Node::token)
        .unwrap_or("")
        .replace("_possessive", "")
        .replace("_lazy", "");
    let value = last
        .and_then(Node::value)
        .unwrap_or("")
        .trim_end_matches(['+', '?']);
    // sscanf('%d'): the digits between the braces (the token patterns allow nothing else)
    let numbers: Vec<i64> = value
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|n| n.parse().unwrap_or(i64::MAX))
        .collect();
    match token.as_str() {
        "n_to_m" => (Some(numbers[0]), numbers.get(1).copied()),
        "n_or_more" => (Some(numbers[0]), None),
        "exactly_n" => (Some(numbers[0]), Some(numbers[0])),
        "zero_or_one" => (Some(0), Some(1)),
        "zero_or_more" => (Some(0), None),
        // one_or_more: the grammar has no other quantifier token
        _ => (Some(1), None),
    }
}

struct Walker<'a> {
    modifiers: &'a str,
}

impl Walker<'_> {
    /// `walkRegexAst()`.
    #[allow(clippy::too_many_arguments)]
    fn walk_regex_ast(
        &self,
        ast: &Node,
        mut alternation: Option<usize>,
        mut combination_index: usize,
        in_optional_quantification: bool,
        mut parent_group: Option<Rc<Parent>>,
        capture_only_named: bool,
        mut repeated_more_than_once: bool,
        walk: &mut AstWalk,
    ) {
        let mut group = None;
        match ast.id() {
            id @ ("#capturing" | "#namedcapturing") => {
                walk.capture_group_id += 1;
                let name = (id == "#namedcapturing")
                    .then(|| ast.children()[0].value().unwrap_or("").to_string());
                let maybe_constant =
                    self.allow_constant_types(repeated_more_than_once, parent_group.as_deref());
                let g = Capturing {
                    id: walk.capture_group_id,
                    name,
                    alternation,
                    in_optional_quantification,
                    parent: parent_group.clone(),
                    ty: self.create_group_type(ast, maybe_constant),
                    force_non_optional: false,
                    force_type: None,
                    force_alternation_branch_matched: false,
                };
                parent_group = Some(Rc::new(Parent::Capturing(g.clone())));
                group = Some(g);
            }
            id @ ("#noncapturing" | "#noncapturingreset") => {
                parent_group = Some(Rc::new(Parent::NonCapturing(NonCapturing {
                    alternation,
                    in_optional_quantification,
                    parent: parent_group,
                    reset_group_counter: id == "#noncapturingreset",
                })));
            }
            _ => {}
        }

        let mut in_optional_quantification = false;
        if ast.id() == "#quantification" {
            let (min, max) = quantification_range(ast);
            if min == Some(0) {
                in_optional_quantification = true;
            }
            if max.is_none_or(|max| max > 1) {
                repeated_more_than_once = true;
            }
        }

        if ast.id() == "#alternation" {
            walk.alternations.push(Alternation {
                count: ast.children().len(),
                combinations: Vec::new(),
            });
            alternation = Some(walk.alternations.len() - 1);
        }

        if ast.id() == "#mark" {
            walk.mark_verbs
                .push(ast.children()[0].value().unwrap_or("").to_string());
            return;
        }

        if let Some(group) = group.filter(|g| !capture_only_named || g.name.is_some()) {
            if let Some(alternation) = alternation {
                let combinations = &mut walk.alternations[alternation].combinations;
                match combinations
                    .iter_mut()
                    .find(|(i, _)| *i == combination_index)
                {
                    Some((_, ids)) => ids.push(group.id),
                    None => combinations.push((combination_index, vec![group.id])),
                }
            }
            walk.groups.push(group);
        }

        for child in ast.children() {
            self.walk_regex_ast(
                child,
                alternation,
                combination_index,
                in_optional_quantification,
                parent_group.clone(),
                capture_only_named,
                repeated_more_than_once,
                walk,
            );
            if ast.id() == "#alternation" {
                combination_index += 1;
            }
        }
    }

    /// `allowConstantTypes()`.
    fn allow_constant_types(
        &self,
        repeated_more_than_once: bool,
        parent_group: Option<&Parent>,
    ) -> bool {
        // if caseless, we don't use constant types because it likely yields too many combinations
        !self.modifiers.contains('i')
            && !repeated_more_than_once
            && !parent_group.is_some_and(Parent::resets_group_counter)
    }

    /// `createGroupType()`.
    fn create_group_type(&self, group: &Node, maybe_constant: bool) -> RType {
        if let Some(root) = root_alternation(group) {
            return RType::union(
                root.children()
                    .iter()
                    .map(|a| self.create_group_type(a, maybe_constant)),
            );
        }
        let walk = self.walk_group_ast(group, false, GroupWalk::empty());
        if maybe_constant
            && let Some(literals) = walk.only_literals.as_ref().filter(|l| !l.is_empty())
        {
            return RType::union(literals.iter().map(|l| RType::from(Atom::Const(l.clone()))));
        }
        if walk.numeric.yes() {
            if walk.non_falsy.yes() {
                return Atom::Str(Flags {
                    non_empty: true,
                    non_falsy: true,
                    numeric: true,
                })
                .into();
            }
            let numeric = RType::from(Atom::Str(Flags {
                non_empty: true,
                non_falsy: false,
                numeric: true,
            }));
            if !walk.non_empty.yes() {
                return RType::union([Atom::Const(String::new()).into(), numeric]);
            }
            return numeric;
        }
        if walk.non_falsy.yes() {
            return Atom::Str(Flags {
                non_empty: true,
                non_falsy: true,
                numeric: false,
            })
            .into();
        }
        if walk.non_empty.yes() {
            return Atom::Str(Flags {
                non_empty: true,
                non_falsy: false,
                numeric: false,
            })
            .into();
        }
        Atom::Str(Flags::default()).into()
    }

    /// `walkGroupAst()`.
    fn walk_group_ast(&self, ast: &Node, mut in_class: bool, mut walk: GroupWalk) -> GroupWalk {
        let children = ast.children();
        let mut quantified_literals = None;
        let id = ast.id();

        if id == "#concatenation" && !children.is_empty() && !walk.in_optional_quantification {
            let mut meaningful_tokens = 0;
            for child in children {
                let mut non_falsy = false;
                if self.is_maybe_empty_node(child, &mut non_falsy) {
                    continue;
                }
                meaningful_tokens += 1;
                if !non_falsy {
                    continue;
                }
                // a single token non-falsy on its own
                walk.non_falsy = Tri::Yes;
                break;
            }
            if meaningful_tokens > 0 {
                walk.non_empty = Tri::Yes;
                // two non-empty tokens concatenated results in a non-falsy string
                if meaningful_tokens > 1 {
                    walk.non_falsy = Tri::Yes;
                }
            }
        } else if id == "#quantification" {
            let (min, max) = quantification_range(ast);
            if min == Some(0) {
                walk.in_optional_quantification = true;
            }
            if !walk.in_optional_quantification {
                if min >= Some(1) {
                    walk.non_empty = Tri::Yes;
                }
                if min >= Some(2) {
                    walk.non_falsy = Tri::Yes;
                }
            }
            // "a?" yields 'a'|'', "a{1,2}" yields 'a'|'aa', etc. so a bounded quantification
            // over constant literals can be combined with the surrounding literals
            quantified_literals = self.quantified_literals(ast, min, max, in_class, &walk);
            walk.only_literals = None;
        } else if id == "#class" && walk.only_literals.is_some() {
            in_class = true;
            let mut new_literals = Vec::new();
            for child in children {
                let mut old_literals = walk.only_literals.clone();
                self.literal_value(child, &mut old_literals, true, true);
                new_literals.extend(old_literals.unwrap_or_default());
            }
            walk.only_literals = Some(new_literals);
        } else if id == "token" {
            let mut only_literals = walk.only_literals.take();
            let literal = self.literal_value(ast, &mut only_literals, !in_class, false);
            walk.only_literals = only_literals;
            if let Some(literal) = literal {
                if !is_digits(&literal) {
                    walk.numeric = Tri::No;
                } else if walk.numeric.maybe() {
                    walk.numeric = Tri::Yes;
                }
                if !walk.in_optional_quantification && !literal.is_empty() {
                    walk.non_empty = Tri::Yes;
                }
            }
        } else if !matches!(id, "#capturing" | "#namedcapturing" | "#alternation") {
            walk.only_literals = None;
        }

        // the alternation has a child per branch: updateAlternationAst() adds an empty token to an empty one
        if id == "#alternation" {
            // literals accumulated before the alternation form a common prefix that has to be
            // combined with every branch, e.g. "a(b|c)" yields "ab"|"ac", not "b"|"c"
            let prefix = walk.only_literals.clone();
            let mut new_literals = Some(Vec::new());
            let (mut non_empty, mut non_falsy, mut numeric) = (Tri::Yes, Tri::Yes, Tri::Yes);
            for child in children {
                let child_walk = self.walk_group_ast(
                    child,
                    in_class,
                    GroupWalk {
                        only_literals: Some(Vec::new()),
                        non_empty: Tri::Maybe,
                        non_falsy: Tri::Maybe,
                        numeric: Tri::Maybe,
                        ..walk.clone()
                    },
                );
                non_empty = non_empty.and(child_walk.non_empty);
                non_falsy = non_falsy.and(child_walk.non_falsy);
                numeric = numeric.and(child_walk.numeric);
                let Some(out) = new_literals.as_mut() else {
                    continue;
                };
                let (Some(prefix), Some(child_literals)) =
                    (&prefix, child_walk.only_literals.filter(|l| !l.is_empty()))
                else {
                    new_literals = None;
                    continue;
                };
                for child_literal in child_literals {
                    if prefix.is_empty() {
                        out.push(child_literal);
                    } else {
                        out.extend(prefix.iter().map(|p| format!("{p}{child_literal}")));
                    }
                }
            }
            return GroupWalk {
                only_literals: new_literals,
                non_empty: walk.non_empty.or(non_empty),
                non_falsy: walk.non_falsy.or(non_falsy),
                numeric: walk.numeric.and(numeric),
                ..walk
            };
        }

        // [^0-9] should not parse as numeric-string, and [^list-everything-but-numbers] is technically
        // doable but really silly compared to just \d so we can safely assume the string is not numeric
        // for negative classes
        if id == "#negativeclass" {
            walk.numeric = Tri::No;
        }

        for child in children {
            walk = self.walk_group_ast(child, in_class, walk);
        }

        if id == "#quantification" {
            // the bottom walk above nulls the literals via the quantifier token,
            // so restore the literals enumerated up-front for bounded quantifications
            walk.only_literals = quantified_literals;
        }
        walk
    }

    /// `getQuantifiedLiterals()`.
    fn quantified_literals(
        &self,
        ast: &Node,
        min: Option<i64>,
        max: Option<i64>,
        in_class: bool,
        walk: &GroupWalk,
    ) -> Option<Vec<String>> {
        let (prefix, min, max) = (walk.only_literals.as_ref()?, min?, max?);
        // walk the quantified atom standalone (everything but the trailing quantifier token);
        // the atom itself is not optional, so reset the flag to let concatenations accumulate literals
        let atom_children = &ast.children()[..ast.children().len() - 1];
        let mut atom = GroupWalk {
            only_literals: Some(Vec::new()),
            in_optional_quantification: false,
            ..walk.clone()
        };
        for child in atom_children {
            atom = self.walk_group_ast(child, in_class, atom);
        }
        let repeated = repeat_literals(&atom.only_literals?, min, max)?;
        let mut new_literals = Vec::new();
        for repeated in repeated {
            if prefix.is_empty() {
                new_literals.push(repeated);
            } else {
                new_literals.extend(prefix.iter().map(|p| format!("{p}{repeated}")));
            }
            if new_literals.len() > LITERALS_LIMIT {
                return None;
            }
        }
        Some(new_literals)
    }

    /// `isMaybeEmptyNode()`.
    fn is_maybe_empty_node(&self, node: &Node, is_non_falsy: &mut bool) -> bool {
        // every quantifier token has a minimum, so a quantification is maybe-empty exactly when it is 0
        if node.id() == "#quantification" {
            return quantification_range(node).0 == Some(0);
        }
        if let Some(literal) = self.literal_value(node, &mut None, false, false) {
            if !literal.is_empty() && literal != "0" {
                *is_non_falsy = true;
            }
            return literal.is_empty();
        }
        // an alternation is maybe-empty if any of its branches is maybe-empty,
        // and non-falsy only if every branch is non-falsy
        if node.id() == "#alternation" {
            let mut maybe_empty = false;
            let mut all_non_falsy = true;
            for child in node.children() {
                let mut child_non_falsy = false;
                if self.is_maybe_empty_node(child, &mut child_non_falsy) {
                    maybe_empty = true;
                }
                all_non_falsy = all_non_falsy && child_non_falsy;
            }
            if all_non_falsy {
                *is_non_falsy = true;
            }
            return maybe_empty;
        }
        node.children()
            .iter()
            .all(|child| self.is_maybe_empty_node(child, is_non_falsy))
    }

    /// `getLiteralValue()`.
    fn literal_value(
        &self,
        node: &Node,
        only_literals: &mut Option<Vec<String>>,
        append: bool,
        in_character_class: bool,
    ) -> Option<String> {
        let (Some(token), Some(value)) = (node.token(), node.value()) else {
            return None;
        };
        // token is the token name from grammar without the namespace so literal and class:literal are both called literal here
        if matches!(token, "literal" | "range" | "class_" | "_class") {
            if self.modifiers.contains('x')
                && value
                    .trim_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
                    .is_empty()
            {
                return None;
            }
            let mut value = value.to_string();
            let mut is_escaped = false;
            if value.len() > 1 && value.starts_with('\\') {
                // `substr($value, 1) ?: ''`: an escaped "0" is falsy, so it becomes ''
                value = Some(value[1..].to_string())
                    .filter(|v| v != "0")
                    .unwrap_or_default();
                is_escaped = true;
            }
            if append && let Some(literals) = only_literals.as_mut() {
                if value == "." && !(is_escaped || in_character_class) {
                    *only_literals = None;
                } else if literals.is_empty() {
                    literals.push(value.clone());
                } else {
                    literals
                        .iter_mut()
                        .for_each(|literal| literal.push_str(&value));
                }
            }
            return Some(value);
        }
        if token != "capturing_name" {
            *only_literals = None;
        }
        // character escape sequences, just return a fixed string
        if matches!(token, "character" | "dynamic_character" | "character_type") {
            if token == "character_type" && value == r"\d" {
                return Some("0".into());
            }
            return Some(value.to_string());
        }
        // [:digit:] and the like, more support coming later
        if token == "posix_class" {
            let literal = match value {
                "[:digit:]" => "0",
                "[:alpha:]" | "[:alnum:]" | "[:upper:]" | "[:lower:]" | "[:word:]"
                | "[:ascii:]" | "[:print:]" | "[:xdigit:]" | "[:graph:]" => "a",
                "[:blank:]" => " \t",
                "[:cntrl:]" => "\x00\x1F",
                "[:space:]" => " \t\r\n\x0B\x0C",
                "[:punct:]" => r##"!"#$%&'()*+,\-./:;<=>?@[\]^_`{|}~"##,
                _ => return None,
            };
            return Some(literal.into());
        }
        matches!(token, "anchor" | "match_point_reset").then(String::new)
    }
}

/// `getRootAlternation()`.
fn root_alternation(group: &Node) -> Option<&Node> {
    let children = group.children();
    match group.id() {
        "#capturing" if children.len() == 1 && children[0].id() == "#alternation" => {
            Some(&children[0])
        }
        // 1st token within a named capturing group is a token holding the group-name
        "#namedcapturing" if children.len() == 2 && children[1].id() == "#alternation" => {
            Some(&children[1])
        }
        _ => None,
    }
}

/// `Strings::match($literal, '/^\d+$/') !== null`: `$` also matches before a final newline.
fn is_digits(literal: &str) -> bool {
    let digits = literal.strip_suffix('\n').unwrap_or(literal);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

/// `repeatLiterals()`.
fn repeat_literals(literals: &[String], min: i64, max: i64) -> Option<Vec<String>> {
    let mut collected: Vec<String> = Vec::new();
    let mut seen: BTreeMap<String, ()> = BTreeMap::new();
    let mut collect = |value: String, collected: &mut Vec<String>| {
        if seen.insert(value.clone(), ()).is_none() {
            collected.push(value);
        }
    };
    if min == 0 {
        collect(String::new(), &mut collected);
    }
    let mut current = vec![String::new()];
    for k in 1..=max {
        let mut next = Vec::new();
        for prefix in &current {
            next.extend(literals.iter().map(|l| format!("{prefix}{l}")));
            if next.len() > LITERALS_LIMIT {
                return None;
            }
        }
        current = next;
        if k < min {
            continue;
        }
        for value in &current {
            collect(value.clone(), &mut collected);
        }
        if collected.len() > LITERALS_LIMIT {
            return None;
        }
    }
    Some(collected)
}
