//! php-parser 5.9's semantic/name failures, before reflection scanning or rule callbacks. RichParser and
//! SimpleParser collect parser-stage errors first, then stop at the first NameResolver error. Grammar
//! translation is limited to the compilation-unit entry case below, not arbitrary recovered CSTs.

use std::collections::HashSet;

use mago_database::file::File;
use mago_php_version::PHPVersion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::error::ParseError;
use mago_syntax::token::TokenKind;
use mago_syntax::walker::MutWalker;
use pun_phpstan::ignore::Error;

/// FileAnalyser's ParserErrorsException diagnostics. An invalid scan-only file instead exports no symbols.
pub(crate) fn errors(file: &File, program: &Program, version: PHPVersion) -> Vec<Error> {
    if program.has_errors() {
        // PHPParser tokenizes the whole file before entering its grammar. Lexical failures need that separate
        // collection contract, even when they follow an unrecoverable compilation-unit error. PHP also rejects
        // vertical tabs and form feeds that Mago's whitespace table accepts; those are not shared trivia.
        if program
            .errors
            .iter()
            .any(|error| matches!(error, ParseError::SyntaxError(_)))
            || program.trivia.iter().any(|trivia| {
                matches!(trivia.kind, TriviaKind::WhiteSpace)
                    && trivia
                        .value
                        .iter()
                        .any(|byte| matches!(byte, b'\x0b' | b'\x0c'))
            })
        {
            return Vec::new();
        }
        let ParseError::UnexpectedToken(_, TokenKind::RightBrace, at) = &program.errors[0] else {
            return Vec::new();
        };
        let Some(Statement::OpeningTag(OpeningTag::Full(tag))) = program.statements.first() else {
            return Vec::new();
        };
        // Only the shared full-tag spelling belongs to this slice. Do not reinterpret Mago's short tags or
        // its joined <?php} / <?php/**/ openers as PHPParser grammar failures.
        if !matches!(
            program.source_text[tag.span.end.offset as usize],
            b' ' | b'\t' | b'\r' | b'\n'
        ) {
            return Vec::new();
        }
        // Contiguous lexer trivia proves there were no earlier statements or expression tokens. The error
        // expression itself may have been discarded at EOF, so its presence in the recovered CST is not proof.
        let mut end = tag.span.end;
        for trivia in program
            .trivia
            .iter()
            .take_while(|trivia| trivia.span.start < at.start)
        {
            if trivia.span.start != end {
                return Vec::new();
            }
            end = trivia.span.end;
        }
        return if end == at.start {
            // php.y's empty top_statement_list reduces to start; Php8 state 532 then expects only EOF.
            // ParserAbstract::doParse cannot recover here, so suffix semantics and Mago cascades do not run.
            vec![error(
                file,
                *at,
                "Syntax error, unexpected '}', expecting EOF".into(),
            )]
        } else {
            Vec::new()
        };
    }
    let mut reserved = Reserved {
        file,
        version,
        continuation: None,
        created_arrays: Vec::new(),
        errors: Vec::new(),
    };
    reserved.walk_program(program, &mut ());
    if !reserved.errors.is_empty() {
        return reserved.errors;
    }
    let mut names = Names {
        file,
        errors: Vec::new(),
        aliases: Default::default(),
    };
    names.walk_program(program, &mut ());
    names.errors
}

fn error(file: &File, at: Span, message: String) -> Error {
    // PHP's tokenizer counts CRLF, LF and bare CR even in a file with mixed line endings. Mago's line table
    // chooses one convention from the first newline, so it cannot locate an InlineHTML after a bare CR.
    let line = 1 + file.contents[..at.start.offset as usize]
        .iter()
        .enumerate()
        .filter(|(index, byte)| {
            **byte == b'\n' || (**byte == b'\r' && file.contents.get(index + 1) != Some(&b'\n'))
        })
        .count() as i64;
    Error {
        identifier: Some("phpstan.parse".into()),
        can_be_ignored: false,
        ..Error::new(
            format!("{message} on line {line}"),
            String::from_utf8_lossy(&file.name),
            Some(line),
        )
    }
}

/// Name::toString() omits the fully-qualified or namespace-relative marker, retaining original case.
fn name(value: &[u8]) -> &[u8] {
    if let Some(value) = value.strip_prefix(b"\\") {
        value
    } else if value.len() >= 10 && value[..10].eq_ignore_ascii_case(b"namespace\\") {
        &value[10..]
    } else {
        value
    }
}

fn special(value: &[u8]) -> bool {
    value.eq_ignore_ascii_case(b"self")
        || value.eq_ignore_ascii_case(b"parent")
        || value.eq_ignore_ascii_case(b"static")
}

/// ParserAbstract's collecting semantic checks. Modifiers are checked as tokens arrive;
/// declaration checks run on reduction (after their bodies). Every parser-stage error wins over NameResolver.
struct Reserved<'a> {
    file: &'a File,
    version: PHPVersion,
    /// Only direct expression-statement/assignment and return continuations are modeled for reverse-emulated
    /// anonymous headers. Other expression contexts and grammar recovery inside anonymous bodies are not a
    /// general older-version parser. Header errors after attributes or other modifiers are context-independent.
    continuation: Option<(Span, &'static str)>,
    /// ParserAbstract's createdArrays registry: array reduction order, then empty item token locations.
    created_arrays: Vec<(Span, Vec<Span>)>,
    errors: Vec<Error>,
}

/// php.y's top_statement_list before handleNamespaces moves semicolon bodies. Keep the original CST nodes,
/// but project PHP's dropped opening tags, close-tag semicolons/newlines and comment-bearing Nops. Mago also
/// splits a leading shebang from adjacent HTML; PHP keeps them in one InlineHTML node.
fn namespace_statements<'ast, 'arena>(
    program: &'ast Program<'arena>,
) -> Vec<(&'ast Statement<'arena>, Span)> {
    let mut result: Vec<(&Statement<'arena>, Span)> = Vec::new();
    let mut previous_end = 0;
    let mut after_close = false;
    let statements = program.statements.iter().flat_map(|statement| {
        let body = match statement {
            Statement::Namespace(Namespace {
                body: NamespaceBody::Implicit(body),
                ..
            }) => body.statements.as_slice(),
            _ => &[],
        };
        std::iter::once(statement).chain(body.iter())
    });
    for statement in statements {
        if matches!(statement, Statement::OpeningTag(_)) {
            after_close = false;
            continue;
        }
        let mut at = statement.span();
        // An implicit namespace is still just its header at this reduction; its body follows in this list.
        let (end, closes) = match statement {
            Statement::Namespace(Namespace {
                body: NamespaceBody::Implicit(body),
                ..
            }) => (
                body.terminator.span().end.offset,
                body.terminator.is_closing_tag(),
            ),
            _ => (at.end.offset, statement.terminates_scripting()),
        };
        let before = previous_end;
        previous_end = end;
        let strip_newline = after_close;
        after_close = closes;
        match statement {
            Statement::Inline(_) => {
                if strip_newline {
                    let skip = match &program.source_text
                        [at.start.offset as usize..at.end.offset as usize]
                    {
                        [b'\r', b'\n', ..] => 2,
                        [b'\r' | b'\n', ..] => 1,
                        _ => 0,
                    };
                    at.start = at.start.forward(skip);
                }
                if at.start == at.end {
                    continue;
                }
                if let Some((Statement::Inline(_), previous)) = result.last_mut()
                    && previous.end == at.start
                {
                    previous.end = at.end;
                    continue;
                }
            }
            Statement::Noop(_) | Statement::ClosingTag(_) => {
                // maybeCreateNop/getCommentBeforeToken retain only comments among the dropped tokens since
                // the previous significant token. Even a discarded semicolon stops that backwards search.
                if !program.trivia.comments().any(|comment| {
                    comment.span.start.offset >= before
                        && comment.span.end.offset <= at.start.offset
                }) {
                    continue;
                }
            }
            Statement::HaltCompiler(_) => {
                // handleHaltCompiler consumes the remaining InlineHTML as payload, not as another statement.
                result.push((statement, at));
                break;
            }
            Statement::Constant(node) => {
                if let Some(attributes) = node.attribute_lists.first() {
                    at.start = attributes.span().start;
                }
            }
            _ => {}
        }
        result.push((statement, at));
    }
    result
}

impl Reserved<'_> {
    /// ParserAbstract::getNamespacingStyle/getNamespaceErrorAttributes. A mixed style returns semicolon so
    /// handleNamespaces does not also report outside code; only the first style mismatch is collected.
    fn namespacing_style(
        &mut self,
        statements: &[(&Statement<'_>, Span)],
        source: &[u8],
    ) -> Option<bool> {
        static HASHBANG: std::sync::LazyLock<pcre2::bytes::Regex> =
            std::sync::LazyLock::new(|| {
                pcre2::bytes::Regex::new(r"\A#!.*\r?\n\z").expect("upstream hashbang pattern")
            });
        let mut style = None;
        let mut forbidden = false;
        for (index, (statement, at)) in statements.iter().enumerate() {
            match statement {
                Statement::Namespace(node) => {
                    let braced = matches!(node.body, NamespaceBody::BraceDelimited(_));
                    if let Some(previous) = style {
                        if previous != braced {
                            self.errors.push(error(
                                self.file,
                                node.namespace.span,
                                "Cannot mix bracketed namespace declarations with unbracketed namespace declarations".into(),
                            ));
                            return Some(false);
                        }
                    } else {
                        style = Some(braced);
                        if forbidden {
                            self.errors.push(error(
                                self.file,
                                node.namespace.span,
                                "Namespace declaration statement has to be the very first statement in the script".into(),
                            ));
                        }
                    }
                }
                Statement::Declare(_)
                | Statement::HaltCompiler(_)
                | Statement::Noop(_)
                | Statement::ClosingTag(_) => {}
                Statement::Inline(_)
                    if index == 0
                        && HASHBANG
                            .is_match(&source[at.start.offset as usize..at.end.offset as usize])
                            .unwrap_or(false) => {}
                _ => forbidden = true,
            }
        }
        style
    }

    fn handle_namespaces(&mut self, program: &Program<'_>) {
        let statements = namespace_statements(program);
        if self.namespacing_style(&statements, program.source_text) != Some(true) {
            return;
        }
        // handleNamespaces' braced pass permits Nops and HaltCompiler, but not declare(), after the first
        // namespace. Mago already holds semicolon bodies, so its normalization needs no CST mutation here.
        let mut after_namespace = false;
        for (statement, at) in statements {
            match statement {
                Statement::Namespace(_) => after_namespace = true,
                Statement::Noop(_) | Statement::ClosingTag(_) | Statement::HaltCompiler(_) => {}
                _ if after_namespace => {
                    self.errors.push(error(
                        self.file,
                        at,
                        "No code may exist outside of namespace {}".into(),
                    ));
                    break;
                }
                _ => {}
            }
        }
    }

    /// ParserAbstract::doParse/getErrorMessage: an unexpected token starts three-shift error recovery. Semantic
    /// checks still collect during recovery. The supplied follow set belongs to the owning php.y production;
    /// it is never inferred from Mago's error text.
    fn header_error(&mut self, recovery: &mut u8, at: Span, token: &str, follow: Option<&str>) {
        if *recovery == 0
            && let Some(follow) = follow
        {
            self.errors.push(error(
                self.file,
                at,
                format!("Syntax error, unexpected {token}{follow}"),
            ));
        }
        *recovery = 3;
    }

    /// Modifiers::verifyClassModifier/checkClassModifier: duplicate flags precede abstract/final conflicts,
    /// and class_modifiers ORs every token even after a collected error. This is not the member contract.
    fn class_modifiers(
        &mut self,
        modifiers: &Sequence<Modifier>,
        class: Span,
        attributes: Option<&str>,
        continuation: Option<&str>,
    ) {
        let mut flags = 0_u16;
        let mut expression = false;
        let mut recovery = 0;
        for modifier in modifiers {
            let (flag, name) = match modifier {
                Modifier::Abstract(_) => (16, "abstract"),
                Modifier::Final(_) => (32, "final"),
                Modifier::Readonly(_) => (64, "readonly"),
                // Mago accepts generic modifiers here. Other tokens fail in PHPParser's grammar instead.
                _ => return,
            };
            // ReadonlyTokenEmulator::getPhpVersion/KeywordEmulator::reverseEmulate turns T_READONLY into
            // T_STRING below 8.1. A leading bare name becomes an expression (or `new`'s class-name reference),
            // not a class modifier. After a class modifier/attributes it is already an unexpected token.
            if flag == 64 && self.version < PHPVersion::PHP81 {
                let follow = if expression {
                    continuation
                } else if flags != 0 {
                    Some(", expecting T_ABSTRACT or T_FINAL or T_READONLY or T_CLASS")
                } else {
                    attributes
                };
                if expression || flags != 0 || attributes.is_some() {
                    self.header_error(&mut recovery, modifier.span(), "T_STRING", follow);
                }
                flags = 0;
                expression = true;
                recovery = recovery.saturating_sub(1);
                continue;
            }
            if expression {
                let token = if flag == 16 { "T_ABSTRACT" } else { "T_FINAL" };
                self.header_error(&mut recovery, modifier.span(), token, continuation);
                expression = false;
            }
            recovery = recovery.saturating_sub(1);
            let message = if flags & flag != 0 {
                Some(format!("Multiple {name} modifiers are not allowed"))
            } else if flags & 48 != 0 && flag & 48 != 0 {
                Some("Cannot use the final modifier on an abstract class".into())
            } else {
                None
            };
            if let Some(message) = message {
                self.errors.push(error(self.file, modifier.span(), message));
            }
            flags |= flag;
        }
        if expression {
            self.header_error(&mut recovery, class, "T_CLASS", continuation);
        }
    }

    /// Modifiers::verifyModifier, called by checkModifier and checkPropertyHookModifiers. Preserve its priority:
    /// access conflicts, duplicate flags, then abstract/final. php.y ORs each flag even after a collected error.
    fn modifiers(&mut self, modifiers: &Sequence<Modifier>, property_hook: bool) {
        let mut flags = 0_u16;
        for modifier in modifiers {
            let (flag, name) = match modifier {
                Modifier::Public(_) => (1, "public"),
                Modifier::Protected(_) => (2, "protected"),
                Modifier::Private(_) => (4, "private"),
                Modifier::Static(_) => (8, "static"),
                Modifier::Abstract(_) => (16, "abstract"),
                Modifier::Final(_) => (32, "final"),
                Modifier::Readonly(_) => (64, "readonly"),
                Modifier::PublicSet(_) => (128, "public(set)"),
                Modifier::ProtectedSet(_) => (256, "protected(set)"),
                Modifier::PrivateSet(_) => (512, "private(set)"),
            };
            let message =
                if (flags & 7 != 0 && flag & 7 != 0) || (flags & 896 != 0 && flag & 896 != 0) {
                    Some("Multiple access type modifiers are not allowed".into())
                } else if flags & flag != 0 {
                    Some(format!("Multiple {name} modifiers are not allowed"))
                } else if flags & 48 != 0 && flag & 48 != 0 {
                    Some("Cannot use the final modifier on an abstract class member".into())
                } else {
                    None
                };
            if let Some(message) = message {
                self.errors.push(error(self.file, modifier.span(), message));
            }
            // checkPropertyHookModifiers also rejects every non-final token, independently of verifyModifier.
            if property_hook && !modifier.is_final() {
                self.errors.push(error(
                    self.file,
                    modifier.span(),
                    format!("Cannot use the {name} modifier on a property hook"),
                ));
            }
            flags |= flag;
        }
    }

    fn class_name(&mut self, value: &[u8], at: Span, role: &str) {
        let value = name(value);
        if special(value) {
            self.errors.push(error(
                self.file,
                at,
                format!(
                    "Cannot use '{}' as {role} name as it is reserved",
                    String::from_utf8_lossy(value)
                ),
            ));
        }
    }

    // identifier_not_reserved excludes T_STATIC at declaration and explicit-alias sites. Unlike self/parent,
    // static fails in the grammar, not checkClassName/checkUseUse. No Mago error text supplies this message.
    fn declaration(&mut self, id: &LocalIdentifier) {
        if id.value.eq_ignore_ascii_case(b"static") {
            self.errors.push(error(
                self.file,
                id.span,
                "Syntax error, unexpected T_STATIC, expecting T_STRING".into(),
            ));
        } else {
            self.class_name(id.value, id.span, "class");
        }
    }

    fn no_comma(&mut self, values: &TokenSeparatedSequence<&Expression>) {
        // php.y's no_comma attaches attributes to the comma itself, not the list or its last value.
        if let Some(comma) = values.get_trailing_token() {
            self.errors.push(error(
                self.file,
                comma.span_for(self.file.id),
                "A trailing comma is not allowed here".into(),
            ));
        }
    }

    fn array(&mut self, at: Span, elements: &TokenSeparatedSequence<ArrayElement>) {
        self.created_arrays.push((
            at,
            elements
                .iter()
                .filter_map(|element| match element {
                    ArrayElement::Missing(missing) => Some(missing.comma),
                    _ => None,
                })
                .collect(),
        ));
    }

    fn fixup_array_destructuring(
        &mut self,
        at: Span,
        elements: &TokenSeparatedSequence<ArrayElement>,
    ) {
        self.created_arrays.retain(|(array, _)| *array != at);
        // fixupArrayDestructuring recurses only into direct item values, not keys, accesses or List_.
        // PHPParser erases parentheses before this check and treats short and legacy Array_ alike.
        for mut value in elements.iter().filter_map(ArrayElement::get_value) {
            while let Expression::Parenthesized(inner) = value {
                value = inner.expression;
            }
            match value {
                Expression::Array(array) => {
                    self.fixup_array_destructuring(array.span(), &array.elements);
                }
                Expression::LegacyArray(array) => {
                    self.fixup_array_destructuring(array.span(), &array.elements);
                }
                _ => {}
            }
        }
    }

    fn parents(&mut self, extends: Option<&Extends>, implements: Option<&Implements>) {
        if let Some(extends) = extends {
            for id in &extends.types {
                self.class_name(id.value(), id.span(), "class");
            }
        }
        if let Some(implements) = implements {
            for id in &implements.types {
                self.class_name(id.value(), id.span(), "interface");
            }
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Reserved<'_> {
    fn walk_out_program(&mut self, node: &'ast Program<'arena>, _: &mut ()) {
        // php.y's start production reduces only after every top-level body semantic check has collected.
        self.handle_namespaces(node);
        // ParserAbstract::parse checks surviving arrays only after doParse (including handleNamespaces).
        for (_, items) in &self.created_arrays {
            for at in items {
                self.errors.push(error(
                    self.file,
                    *at,
                    "Cannot use empty array elements in arrays".into(),
                ));
            }
        }
    }
    fn walk_out_array(&mut self, node: &'ast Array<'arena>, _: &mut ()) {
        self.array(node.span(), &node.elements);
    }
    fn walk_out_legacy_array(&mut self, node: &'ast LegacyArray<'arena>, _: &mut ()) {
        self.array(node.span(), &node.elements);
    }
    fn walk_out_foreach_target(&mut self, node: &'ast ForeachTarget<'arena>, _: &mut ()) {
        let value = match node {
            ForeachTarget::Value(target) => target.value,
            ForeachTarget::KeyValue(target) => target.value,
        };
        if let Expression::Array(array) = value {
            self.fixup_array_destructuring(array.span(), &array.elements);
        }
    }
    fn walk_for(&mut self, node: &'ast For<'arena>, context: &mut ()) {
        self.walk_keyword(&node.r#for, context);
        // Each for_expr reduces its own expr_list_forbid_comma before the next clause and the body.
        for values in [&node.initializations, &node.conditions, &node.increments] {
            for value in values {
                self.walk_expression(value, context);
            }
            self.no_comma(values);
        }
        self.walk_for_body(&node.body, context);
    }
    fn walk_out_echo(&mut self, node: &'ast Echo<'arena>, _: &mut ()) {
        self.no_comma(&node.values);
    }
    fn walk_out_echo_tag(&mut self, node: &'ast EchoTag<'arena>, _: &mut ()) {
        // ParserAbstract::createTokenMap maps T_OPEN_TAG_WITH_ECHO to the same T_ECHO production.
        self.no_comma(&node.values);
    }
    fn walk_return(&mut self, node: &'ast Return<'arena>, context: &mut ()) {
        let saved = self.continuation;
        self.continuation = node.value.map(|value| (value.span(), ", expecting ';'"));
        mago_syntax::walker::walk_return_mut(self, node, context);
        self.continuation = saved;
    }
    fn walk_statement_expression(
        &mut self,
        node: &'ast ExpressionStatement<'arena>,
        context: &mut (),
    ) {
        let saved = self.continuation;
        let mut value = node.expression;
        while let Expression::Assignment(assignment) = value {
            value = assignment.rhs;
        }
        self.continuation = Some((value.span(), ""));
        mago_syntax::walker::walk_statement_expression_mut(self, node, context);
        self.continuation = saved;
    }
    fn walk_out_namespace(&mut self, node: &'ast Namespace<'arena>, _: &mut ()) {
        // ParserAbstract::checkNamespace runs on reduction, after all body errors, and inspects only direct
        // children. Semicolon namespaces have null stmts at this stage, not Mago's later implicit body.
        if let NamespaceBody::BraceDelimited(body) = &node.body {
            for statement in &body.statements {
                if let Statement::Namespace(child) = statement {
                    self.errors.push(error(
                        self.file,
                        child.namespace.span,
                        "Namespace declarations cannot be nested".into(),
                    ));
                }
            }
        }
    }
    fn walk_out_constant(&mut self, node: &'ast Constant<'arena>, _: &mut ()) {
        // ParserAbstract::checkConstantAttributes runs after every value expression has reduced. PHPParser's
        // declaration attributes start at the first attribute group, unlike Mago's const-keyword span.
        if let Some(attributes) = node.attribute_lists.first()
            && node.items.len() > 1
        {
            self.errors.push(error(
                self.file,
                attributes.span(),
                "Cannot use attributes on multiple constants at once".into(),
            ));
        }
    }
    fn walk_in_class(&mut self, node: &'ast Class<'arena>, _: &mut ()) {
        self.class_modifiers(
            &node.modifiers,
            node.class.span,
            (!node.attribute_lists.is_empty())
                .then_some(", expecting T_TRAIT or T_INTERFACE or T_ENUM"),
            Some(""),
        );
    }
    fn walk_in_anonymous_class(&mut self, node: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.class_modifiers(
            &node.modifiers,
            node.class.span,
            (!node.attribute_lists.is_empty())
                .then_some(", expecting T_ABSTRACT or T_FINAL or T_READONLY or T_CLASS"),
            self.continuation
                .filter(|(at, _)| *at == node.span())
                .map(|(_, follow)| follow),
        );
    }
    fn walk_in_plain_property(&mut self, node: &'ast PlainProperty<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, false);
    }
    fn walk_in_hooked_property(&mut self, node: &'ast HookedProperty<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, false);
    }
    fn walk_in_method(&mut self, node: &'ast Method<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, false);
    }
    fn walk_in_class_like_constant(&mut self, node: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, false);
    }
    fn walk_in_function_like_parameter(
        &mut self,
        node: &'ast FunctionLikeParameter<'arena>,
        _: &mut (),
    ) {
        self.modifiers(&node.modifiers, false);
    }
    fn walk_out_function_like_parameter(
        &mut self,
        node: &'ast FunctionLikeParameter<'arena>,
        _: &mut (),
    ) {
        // ParserAbstract::checkParam runs after the default expression and hooks have reduced. Both checks
        // collect independently, with the variadic default before the direct (not nullable/union) void type.
        if node.ellipsis.is_some()
            && let Some(default) = &node.default_value
        {
            let mut value = default.value;
            // php.y's '(' expr ')' production returns the inner node, retaining its own start attributes.
            while let Expression::Parenthesized(inner) = value {
                value = inner.expression;
            }
            self.errors.push(error(
                self.file,
                value.span(),
                "Variadic parameter cannot have a default value".into(),
            ));
        }
        if let Some(Hint::Void(hint)) = &node.hint {
            self.errors.push(error(
                self.file,
                hint.span(),
                "void cannot be used as a parameter type".into(),
            ));
        }
    }
    fn walk_out_assignment(&mut self, node: &'ast Assignment<'arena>, _: &mut ()) {
        // Only array_short_syntax '=' expr invokes fixupArrayDestructuring at the root. Registering
        // then removing its short array is equivalent to PHPParser never registering that root.
        if node.operator.is_assign()
            && let Expression::Array(array) = node.lhs
        {
            self.fixup_array_destructuring(array.span(), &array.elements);
        }
        // php.y's variable '=' ampersand new_expr production checks after both sides reduce. Do not
        // unwrap parentheses: a parenthesized LHS or new, and a dereferenced new, use other productions.
        if self.version >= PHPVersion::PHP70
            && node.operator.is_assign()
            && matches!(
                node.lhs,
                Expression::Variable(_)
                    | Expression::ArrayAccess(_)
                    | Expression::ArrayAppend(_)
                    | Expression::Call(_)
                    | Expression::PartialApplication(_)
                    | Expression::Access(
                        Access::Property(_)
                            | Access::NullSafeProperty(_)
                            | Access::StaticProperty(_)
                    )
            )
            && let Expression::UnaryPrefix(reference) = node.rhs
            && reference.operator.is_reference()
            && matches!(
                reference.operand,
                Expression::Instantiation(_) | Expression::AnonymousClass(_)
            )
        {
            self.errors.push(error(
                self.file,
                node.span(),
                "Cannot assign new by reference".into(),
            ));
        }
    }
    fn walk_out_pipe(&mut self, node: &'ast Pipe<'arena>, _: &mut ()) {
        // checkPipeOperatorParentheses runs after the RHS reduces. Mago retains parentheses explicitly,
        // unlike PHPParser's parenthesizedArrowFunctions registry. Earlier targets do not tokenize T_PIPE.
        if self.version >= PHPVersion::PHP85
            && let Expression::ArrowFunction(arrow) = node.callable
        {
            self.errors.push(error(
                self.file,
                arrow.span(),
                "Arrow functions on the right hand side of |> must be parenthesized".into(),
            ));
        }
    }
    fn walk_out_try(&mut self, node: &'ast Try<'arena>, _: &mut ()) {
        // ParserAbstract::checkTryCatch runs after the body, catches and finally have reduced.
        if node.catch_clauses.is_empty() && node.finally_clause.is_none() {
            self.errors.push(error(
                self.file,
                node.r#try.span,
                "Cannot use try without catch or finally".into(),
            ));
        }
    }
    fn walk_in_property_hook(&mut self, node: &'ast PropertyHook<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, true);
    }
    fn walk_out_property_hook(&mut self, node: &'ast PropertyHook<'arena>, _: &mut ()) {
        // ParserAbstract::checkPropertyHook follows the parameter list and body, including their errors.
        let name = node.name.value;
        if !name.eq_ignore_ascii_case(b"get") && !name.eq_ignore_ascii_case(b"set") {
            self.errors.push(error(
                self.file,
                node.name.span,
                format!(
                    "Unknown hook \"{}\", expected \"get\" or \"set\"",
                    String::from_utf8_lossy(name)
                ),
            ));
        }
        if name.eq_ignore_ascii_case(b"get")
            && let Some(parameters) = &node.parameter_list
        {
            self.errors.push(error(
                self.file,
                parameters.left_parenthesis,
                "get hook must not have a parameter list".into(),
            ));
        }
    }
    fn walk_out_property_hook_list(&mut self, node: &'ast PropertyHookList<'arena>, _: &mut ()) {
        // checkEmptyPropertyHookList runs for property and parameter hook lists (including promotion),
        // before the enclosing checkParam. An absent list is not an empty list.
        if node.hooks.is_empty() {
            self.errors.push(error(
                self.file,
                node.left_brace,
                "Property hook list cannot be empty".into(),
            ));
        }
    }
    fn walk_out_method(&mut self, node: &'ast Method<'arena>, _: &mut ()) {
        // ParserAbstract::checkClassMethod runs after the body, with method_modifiers' first token as #2.
        let Some(first) = node.modifiers.first() else {
            return;
        };
        if node.modifiers.contains_static() {
            let kind = match node.name.value.to_ascii_lowercase().as_slice() {
                b"__construct" => Some("Constructor"),
                b"__destruct" => Some("Destructor"),
                b"__clone" => Some("Clone method"),
                _ => None,
            };
            if let Some(kind) = kind {
                self.errors.push(error(
                    self.file,
                    first.span(),
                    format!(
                        "{kind} {}() cannot be static",
                        String::from_utf8_lossy(node.name.value)
                    ),
                ));
            }
        }
        if node.modifiers.contains_readonly() {
            self.errors.push(error(
                self.file,
                first.span(),
                format!(
                    "Method {}() cannot be readonly",
                    String::from_utf8_lossy(node.name.value)
                ),
            ));
        }
    }
    fn walk_out_class_like_constant(&mut self, node: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        // ParserAbstract::checkClassConst runs on reduction and reports each forbidden flag once at #2.
        let Some(first) = node.modifiers.first() else {
            return;
        };
        for (present, name) in [
            (node.modifiers.contains_static(), "static"),
            (node.modifiers.contains_abstract(), "abstract"),
            (node.modifiers.contains_readonly(), "readonly"),
        ] {
            if present {
                self.errors.push(error(
                    self.file,
                    first.span(),
                    format!("Cannot use '{name}' as constant modifier"),
                ));
            }
        }
    }
    fn walk_out_class(&mut self, node: &'ast Class<'arena>, _: &mut ()) {
        self.declaration(&node.name);
        self.parents(node.extends.as_ref(), node.implements.as_ref());
    }
    fn walk_out_anonymous_class(&mut self, node: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.parents(node.extends.as_ref(), node.implements.as_ref());
    }
    fn walk_out_interface(&mut self, node: &'ast Interface<'arena>, _: &mut ()) {
        self.declaration(&node.name);
        if let Some(extends) = &node.extends {
            for id in &extends.types {
                self.class_name(id.value(), id.span(), "interface");
            }
        }
    }
    fn walk_out_enum(&mut self, node: &'ast Enum<'arena>, _: &mut ()) {
        self.declaration(&node.name);
        self.parents(None, node.implements.as_ref());
    }
    fn walk_in_use_item(&mut self, node: &'ast UseItem<'arena>, _: &mut ()) {
        if let Some(alias) = &node.alias {
            let value = alias.identifier.value;
            if value.eq_ignore_ascii_case(b"static") {
                self.errors.push(error(
                    self.file,
                    alias.identifier.span,
                    "Syntax error, unexpected T_STATIC, expecting T_STRING".into(),
                ));
            } else if special(value) {
                self.errors.push(error(
                    self.file,
                    alias.identifier.span,
                    format!(
                        "Cannot use {} as {} because '{}' is a special class name",
                        String::from_utf8_lossy(
                            node.name
                                .value()
                                .strip_prefix(b"\\")
                                .unwrap_or(node.name.value())
                        ),
                        String::from_utf8_lossy(value),
                        String::from_utf8_lossy(value)
                    ),
                ));
            }
        }
    }
}

/// NameContext's three independent import tables and NameResolver's throwing error handler.
struct Names<'a> {
    file: &'a File,
    errors: Vec<Error>,
    aliases: [HashSet<Vec<u8>>; 3],
}

impl Names<'_> {
    fn class_name(&mut self, id: &Identifier) {
        if self.errors.is_empty() && !id.is_local() && special(name(id.value())) {
            self.errors.push(error(
                self.file,
                id.span(),
                format!(
                    "'\\{}' is an invalid class name",
                    String::from_utf8_lossy(name(id.value()))
                ),
            ));
        }
    }

    fn receiver(&mut self, expr: &Expression) {
        if let Expression::Identifier(id) = expr {
            self.class_name(id);
        }
    }

    fn attributes(&mut self, groups: &Sequence<AttributeList>) {
        for group in groups {
            for attr in &group.attributes {
                self.class_name(&attr.name);
            }
        }
    }

    fn signature(
        &mut self,
        params: &FunctionLikeParameterList,
        result: Option<&FunctionLikeReturnTypeHint>,
    ) {
        for param in &params.parameters {
            if let Some(hint) = &param.hint {
                self.walk_hint(hint, &mut ());
            }
            self.attributes(&param.attribute_lists);
        }
        if let Some(result) = result {
            self.walk_hint(&result.hint, &mut ());
        }
    }

    fn alias(&mut self, item: &UseItem, kind: usize, prefix: &[u8]) {
        let alias = item
            .alias
            .as_ref()
            .map_or_else(|| item.name.last_segment(), |a| a.identifier.value);
        let key = if kind == 2 {
            alias.to_vec()
        } else {
            alias.to_ascii_lowercase()
        };
        if self.errors.is_empty() && !self.aliases[kind].insert(key) {
            let target = item
                .name
                .value()
                .strip_prefix(b"\\")
                .unwrap_or(item.name.value());
            let target = if prefix.is_empty() {
                target.to_vec()
            } else {
                [prefix, b"\\", target].concat()
            };
            let role = ["", "function ", "const "][kind];
            self.errors.push(error(
                self.file,
                item.name.span(),
                format!(
                    "Cannot use {role}{} as {} because the name is already in use",
                    String::from_utf8_lossy(&target),
                    String::from_utf8_lossy(alias)
                ),
            ));
        }
    }
}

fn use_kind(kind: Option<&UseType>) -> usize {
    match kind {
        None => 0,
        Some(UseType::Function(_)) => 1,
        Some(UseType::Const(_)) => 2,
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Names<'_> {
    fn walk_in_namespace(&mut self, _: &'ast Namespace<'arena>, _: &mut ()) {
        for aliases in &mut self.aliases {
            aliases.clear();
        }
    }
    fn walk_in_use(&mut self, node: &'ast Use<'arena>, _: &mut ()) {
        match &node.items {
            UseItems::Sequence(items) => {
                for item in &items.items {
                    self.alias(item, 0, b"");
                }
            }
            UseItems::TypedSequence(items) => {
                for item in &items.items {
                    self.alias(item, use_kind(Some(&items.r#type)), b"");
                }
            }
            UseItems::TypedList(items) => {
                for item in &items.items {
                    self.alias(
                        item,
                        use_kind(Some(&items.r#type)),
                        items
                            .namespace
                            .value()
                            .strip_prefix(b"\\")
                            .unwrap_or(items.namespace.value()),
                    );
                }
            }
            UseItems::MixedList(items) => {
                for item in &items.items {
                    self.alias(
                        &item.item,
                        use_kind(item.r#type.as_ref()),
                        items
                            .namespace
                            .value()
                            .strip_prefix(b"\\")
                            .unwrap_or(items.namespace.value()),
                    );
                }
            }
        }
    }
    fn walk_in_class(&mut self, node: &'ast Class<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_interface(&mut self, node: &'ast Interface<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_trait(&mut self, node: &'ast Trait<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_constant(&mut self, node: &'ast Constant<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_enum_case(&mut self, node: &'ast EnumCase<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
    }
    fn walk_enum(&mut self, node: &'ast Enum<'arena>, _: &mut ()) {
        // Enum_::scalarType is not resolved as a class name; BackedEnumTypeRule checks it instead.
        self.attributes(&node.attribute_lists);
        for group in &node.attribute_lists {
            self.walk_attribute_list(group, &mut ());
        }
        for member in &node.members {
            self.walk_class_like_member(member, &mut ());
        }
    }
    fn walk_in_function(&mut self, node: &'ast Function<'arena>, _: &mut ()) {
        self.signature(&node.parameter_list, node.return_type_hint.as_ref());
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_method(&mut self, node: &'ast Method<'arena>, _: &mut ()) {
        self.signature(&node.parameter_list, node.return_type_hint.as_ref());
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_closure(&mut self, node: &'ast Closure<'arena>, _: &mut ()) {
        self.signature(&node.parameter_list, node.return_type_hint.as_ref());
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_arrow_function(&mut self, node: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.signature(&node.parameter_list, node.return_type_hint.as_ref());
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_property_hook(&mut self, node: &'ast PropertyHook<'arena>, _: &mut ()) {
        if let Some(params) = &node.parameter_list {
            self.signature(params, None);
        }
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_plain_property(&mut self, node: &'ast PlainProperty<'arena>, _: &mut ()) {
        if let Some(hint) = &node.hint {
            self.walk_hint(hint, &mut ());
        }
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_hooked_property(&mut self, node: &'ast HookedProperty<'arena>, _: &mut ()) {
        if let Some(hint) = &node.hint {
            self.walk_hint(hint, &mut ());
        }
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_class_like_constant(&mut self, node: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        if let Some(hint) = &node.hint {
            self.walk_hint(hint, &mut ());
        }
        self.attributes(&node.attribute_lists);
    }
    fn walk_in_attribute_list(&mut self, node: &'ast AttributeList<'arena>, _: &mut ()) {
        self.attributes(&Sequence::from_slice(std::slice::from_ref(node)));
    }
    fn walk_in_hint(&mut self, node: &'ast Hint<'arena>, _: &mut ()) {
        if let Hint::Identifier(id) = node {
            self.class_name(id);
        }
    }
    fn walk_in_trait_use(&mut self, node: &'ast TraitUse<'arena>, _: &mut ()) {
        for id in &node.trait_names {
            self.class_name(id);
        }
    }
    fn walk_in_trait_use_absolute_method_reference(
        &mut self,
        node: &'ast TraitUseAbsoluteMethodReference<'arena>,
        _: &mut (),
    ) {
        self.class_name(&node.trait_name);
    }
    fn walk_in_trait_use_precedence_adaptation(
        &mut self,
        node: &'ast TraitUsePrecedenceAdaptation<'arena>,
        _: &mut (),
    ) {
        self.class_name(&node.method_reference.trait_name);
        for id in &node.trait_names {
            self.class_name(id);
        }
    }
    fn walk_in_instantiation(&mut self, node: &'ast Instantiation<'arena>, _: &mut ()) {
        self.receiver(node.class);
    }
    fn walk_in_static_method_call(&mut self, node: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.receiver(node.class);
    }
    fn walk_in_static_method_partial_application(
        &mut self,
        node: &'ast StaticMethodPartialApplication<'arena>,
        _: &mut (),
    ) {
        self.receiver(node.class);
    }
    fn walk_in_static_property_access(
        &mut self,
        node: &'ast StaticPropertyAccess<'arena>,
        _: &mut (),
    ) {
        self.receiver(node.class);
    }
    fn walk_in_class_constant_access(
        &mut self,
        node: &'ast ClassConstantAccess<'arena>,
        _: &mut (),
    ) {
        self.receiver(node.class);
    }
    fn walk_in_binary(&mut self, node: &'ast Binary<'arena>, _: &mut ()) {
        if matches!(node.operator, BinaryOperator::Instanceof(_)) {
            self.receiver(node.rhs);
        }
    }
    // New_ traverses its Class_ (attributes and body) before constructor arguments. Mago stores those
    // arguments inside AnonymousClass, so move just their visit to the end rather than choosing by offset.
    fn walk_anonymous_class(&mut self, node: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.attributes(&node.attribute_lists);
        for group in &node.attribute_lists {
            self.walk_attribute_list(group, &mut ());
        }
        for member in &node.members {
            self.walk_class_like_member(member, &mut ());
        }
        if let Some(args) = &node.argument_list {
            self.walk_partial_argument_list(args, &mut ());
        }
    }
}
