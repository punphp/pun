//! php-parser 5.9's name, modifier and parameter failures, before reflection scanning or rule callbacks. RichParser and
//! SimpleParser collect parser-stage errors first, then stop at the first NameResolver error. This is not a
//! translation of Mago's grammar errors: a recovered CST remains outside this validation contract.

use std::collections::HashSet;

use mago_database::file::File;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::ignore::Error;

/// FileAnalyser's ParserErrorsException diagnostics. An invalid scan-only file instead exports no symbols.
pub(crate) fn errors(file: &File, program: &Program) -> Vec<Error> {
    if program.has_errors() {
        return Vec::new();
    }
    let mut reserved = Reserved {
        file,
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
    let line = i64::from(file.line_number(at.start.offset)) + 1;
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

/// ParserAbstract's reserved-name, member-modifier and parameter checks. Modifiers are checked as tokens arrive;
/// declaration checks run on reduction (after their bodies). Every parser-stage error wins over NameResolver.
struct Reserved<'a> {
    file: &'a File,
    errors: Vec<Error>,
}

impl Reserved<'_> {
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
    fn walk_in_property_hook(&mut self, node: &'ast PropertyHook<'arena>, _: &mut ()) {
        self.modifiers(&node.modifiers, true);
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
