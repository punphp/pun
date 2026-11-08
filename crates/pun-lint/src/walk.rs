//! The tree walk behind [`Linter::check`](crate::Linter::check), on Mago's typed [`Walker`].
//!
//! It visits the nodes [`Node::visit_children`] visits, in the same order, with the same ancestors: every rule sees
//! the tree it would see from a walk over `visit_children`. Mago's walker has a function per node type, which the
//! compiler inlines into its parent's, where a walk over `visit_children` matches on [`Node`] and calls a closure for
//! every node; over laravel-framework's parsed files the walk takes a third less time.
//!
//! The walker and `visit_children` disagree on a few node types; the `walk_*` overrides at the end follow
//! `visit_children` there. `tests/walk.rs` checks that the two walks give the same nodes on files with every node type.

use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use crate::{Context, Found, Linter};

/// What the walk carries from node to node: the file, the ancestors of the current node, outermost first, and the
/// reports. The file is here rather than on [`Walk`] because the walker must be `Sync` and the file's tables are not.
pub(crate) struct State<'ast, 'arena, 'c> {
    pub ctx: &'c Context<'c>,
    pub ancestors: Vec<Node<'ast, 'arena>>,
    pub found: Found,
}

/// The rules, for the length of one walk.
pub(crate) struct Walk<'a> {
    pub linter: &'a Linter,
}

impl Walk<'_> {
    /// Call the rules that target `kind` on `node`, then make `node` the innermost ancestor.
    #[inline(always)]
    #[expect(
        clippy::inline_always,
        reason = "the per-node dispatch of every lint run"
    )]
    fn enter<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        kind: NodeKind,
        state: &mut State<'ast, 'arena, '_>,
    ) {
        for &(id, i) in &self.linter.by_kind[kind as usize] {
            self.linter.rules[i].visit(node, &state.ancestors, state.ctx, &mut state.found.reports);
            state.found.rules.resize(state.found.reports.len(), id);
        }
        state.ancestors.push(node);
    }
}

/// `walk_in_*` enters the node and `walk_out_*` leaves it, for each node type `visit_children` yields. The two
/// adaptation types of `TraitUseAdaptation` are not listed: `visit_children` goes straight to their children.
macro_rules! hooks {
    ($($variant:ident($ty:ty) $walk_in:ident $walk_out:ident;)*) => {
        $(
            #[inline(always)]
            fn $walk_in(&self, node: &'ast $ty, state: &mut State<'ast, 'arena, 'c>) {
                self.enter(Node::$variant(node), NodeKind::$variant, state);
            }

            #[inline(always)]
            fn $walk_out(&self, _: &'ast $ty, state: &mut State<'ast, 'arena, 'c>) {
                state.ancestors.pop();
            }
        )*
    };
}

impl<'ast, 'arena, 'c> Walker<'ast, 'arena, State<'ast, 'arena, 'c>> for Walk<'_> {
    hooks! {
        Program(Program<'arena>) walk_in_program walk_out_program;
        Statement(Statement<'arena>) walk_in_statement walk_out_statement;
        OpeningTag(OpeningTag<'arena>) walk_in_opening_tag walk_out_opening_tag;
        FullOpeningTag(FullOpeningTag<'arena>) walk_in_full_opening_tag walk_out_full_opening_tag;
        ShortOpeningTag(ShortOpeningTag) walk_in_short_opening_tag walk_out_short_opening_tag;
        ClosingTag(ClosingTag) walk_in_closing_tag walk_out_closing_tag;
        Inline(Inline<'arena>) walk_in_inline walk_out_inline;
        Namespace(Namespace<'arena>) walk_in_namespace walk_out_namespace;
        NamespaceBody(NamespaceBody<'arena>) walk_in_namespace_body walk_out_namespace_body;
        NamespaceImplicitBody(NamespaceImplicitBody<'arena>) walk_in_namespace_implicit_body walk_out_namespace_implicit_body;
        Terminator(Terminator<'arena>) walk_in_terminator walk_out_terminator;
        Use(Use<'arena>) walk_in_use walk_out_use;
        UseItems(UseItems<'arena>) walk_in_use_items walk_out_use_items;
        UseItemSequence(UseItemSequence<'arena>) walk_in_use_item_sequence walk_out_use_item_sequence;
        UseItem(UseItem<'arena>) walk_in_use_item walk_out_use_item;
        UseItemAlias(UseItemAlias<'arena>) walk_in_use_item_alias walk_out_use_item_alias;
        TypedUseItemSequence(TypedUseItemSequence<'arena>) walk_in_typed_use_item_sequence walk_out_typed_use_item_sequence;
        UseType(UseType<'arena>) walk_in_use_type walk_out_use_type;
        TypedUseItemList(TypedUseItemList<'arena>) walk_in_typed_use_item_list walk_out_typed_use_item_list;
        MixedUseItemList(MixedUseItemList<'arena>) walk_in_mixed_use_item_list walk_out_mixed_use_item_list;
        MaybeTypedUseItem(MaybeTypedUseItem<'arena>) walk_in_maybe_typed_use_item walk_out_maybe_typed_use_item;
        AttributeList(AttributeList<'arena>) walk_in_attribute_list walk_out_attribute_list;
        Attribute(Attribute<'arena>) walk_in_attribute walk_out_attribute;
        ArgumentList(ArgumentList<'arena>) walk_in_argument_list walk_out_argument_list;
        PartialArgumentList(PartialArgumentList<'arena>) walk_in_partial_argument_list walk_out_partial_argument_list;
        Argument(Argument<'arena>) walk_in_argument walk_out_argument;
        PartialArgument(PartialArgument<'arena>) walk_in_partial_argument walk_out_partial_argument;
        PositionalArgument(PositionalArgument<'arena>) walk_in_positional_argument walk_out_positional_argument;
        NamedArgument(NamedArgument<'arena>) walk_in_named_argument walk_out_named_argument;
        PlaceholderArgument(PlaceholderArgument) walk_in_placeholder_argument walk_out_placeholder_argument;
        NamedPlaceholderArgument(NamedPlaceholderArgument<'arena>) walk_in_named_placeholder_argument walk_out_named_placeholder_argument;
        VariadicPlaceholderArgument(VariadicPlaceholderArgument) walk_in_variadic_placeholder_argument walk_out_variadic_placeholder_argument;
        Modifier(Modifier<'arena>) walk_in_modifier walk_out_modifier;
        Extends(Extends<'arena>) walk_in_extends walk_out_extends;
        Implements(Implements<'arena>) walk_in_implements walk_out_implements;
        Class(Class<'arena>) walk_in_class walk_out_class;
        Interface(Interface<'arena>) walk_in_interface walk_out_interface;
        Trait(Trait<'arena>) walk_in_trait walk_out_trait;
        Enum(Enum<'arena>) walk_in_enum walk_out_enum;
        EnumBackingTypeHint(EnumBackingTypeHint<'arena>) walk_in_enum_backing_type_hint walk_out_enum_backing_type_hint;
        ClassLikeMember(ClassLikeMember<'arena>) walk_in_class_like_member walk_out_class_like_member;
        TraitUse(TraitUse<'arena>) walk_in_trait_use walk_out_trait_use;
        TraitUseSpecification(TraitUseSpecification<'arena>) walk_in_trait_use_specification walk_out_trait_use_specification;
        TraitUseAbstractSpecification(TraitUseAbstractSpecification<'arena>) walk_in_trait_use_abstract_specification walk_out_trait_use_abstract_specification;
        TraitUseConcreteSpecification(TraitUseConcreteSpecification<'arena>) walk_in_trait_use_concrete_specification walk_out_trait_use_concrete_specification;
        TraitUseAdaptation(TraitUseAdaptation<'arena>) walk_in_trait_use_adaptation walk_out_trait_use_adaptation;
        TraitUseAbsoluteMethodReference(TraitUseAbsoluteMethodReference<'arena>) walk_in_trait_use_absolute_method_reference walk_out_trait_use_absolute_method_reference;
        TraitUseMethodReference(TraitUseMethodReference<'arena>) walk_in_trait_use_method_reference walk_out_trait_use_method_reference;
        ClassLikeConstant(ClassLikeConstant<'arena>) walk_in_class_like_constant walk_out_class_like_constant;
        ClassLikeConstantItem(ClassLikeConstantItem<'arena>) walk_in_class_like_constant_item walk_out_class_like_constant_item;
        Property(Property<'arena>) walk_in_property walk_out_property;
        PlainProperty(PlainProperty<'arena>) walk_in_plain_property walk_out_plain_property;
        PropertyItem(PropertyItem<'arena>) walk_in_property_item walk_out_property_item;
        PropertyAbstractItem(PropertyAbstractItem<'arena>) walk_in_property_abstract_item walk_out_property_abstract_item;
        PropertyConcreteItem(PropertyConcreteItem<'arena>) walk_in_property_concrete_item walk_out_property_concrete_item;
        HookedProperty(HookedProperty<'arena>) walk_in_hooked_property walk_out_hooked_property;
        PropertyHookList(PropertyHookList<'arena>) walk_in_property_hook_list walk_out_property_hook_list;
        PropertyHook(PropertyHook<'arena>) walk_in_property_hook walk_out_property_hook;
        PropertyHookBody(PropertyHookBody<'arena>) walk_in_property_hook_body walk_out_property_hook_body;
        PropertyHookAbstractBody(PropertyHookAbstractBody) walk_in_property_hook_abstract_body walk_out_property_hook_abstract_body;
        PropertyHookConcreteBody(PropertyHookConcreteBody<'arena>) walk_in_property_hook_concrete_body walk_out_property_hook_concrete_body;
        PropertyHookConcreteExpressionBody(PropertyHookConcreteExpressionBody<'arena>) walk_in_property_hook_concrete_expression_body walk_out_property_hook_concrete_expression_body;
        FunctionLikeParameterList(FunctionLikeParameterList<'arena>) walk_in_function_like_parameter_list walk_out_function_like_parameter_list;
        FunctionLikeParameter(FunctionLikeParameter<'arena>) walk_in_function_like_parameter walk_out_function_like_parameter;
        FunctionLikeParameterDefaultValue(FunctionLikeParameterDefaultValue<'arena>) walk_in_function_like_parameter_default_value walk_out_function_like_parameter_default_value;
        EnumCase(EnumCase<'arena>) walk_in_enum_case walk_out_enum_case;
        EnumCaseItem(EnumCaseItem<'arena>) walk_in_enum_case_item walk_out_enum_case_item;
        EnumCaseUnitItem(EnumCaseUnitItem<'arena>) walk_in_enum_case_unit_item walk_out_enum_case_unit_item;
        EnumCaseBackedItem(EnumCaseBackedItem<'arena>) walk_in_enum_case_backed_item walk_out_enum_case_backed_item;
        Method(Method<'arena>) walk_in_method walk_out_method;
        MethodBody(MethodBody<'arena>) walk_in_method_body walk_out_method_body;
        MethodAbstractBody(MethodAbstractBody<'arena>) walk_in_method_abstract_body walk_out_method_abstract_body;
        FunctionLikeReturnTypeHint(FunctionLikeReturnTypeHint<'arena>) walk_in_function_like_return_type_hint walk_out_function_like_return_type_hint;
        Block(Block<'arena>) walk_in_block walk_out_block;
        Constant(Constant<'arena>) walk_in_constant walk_out_constant;
        ConstantItem(ConstantItem<'arena>) walk_in_constant_item walk_out_constant_item;
        Function(Function<'arena>) walk_in_function walk_out_function;
        Declare(Declare<'arena>) walk_in_declare walk_out_declare;
        DeclareItem(DeclareItem<'arena>) walk_in_declare_item walk_out_declare_item;
        DeclareBody(DeclareBody<'arena>) walk_in_declare_body walk_out_declare_body;
        DeclareColonDelimitedBody(DeclareColonDelimitedBody<'arena>) walk_in_declare_colon_delimited_body walk_out_declare_colon_delimited_body;
        Goto(Goto<'arena>) walk_in_goto walk_out_goto;
        Label(Label<'arena>) walk_in_label walk_out_label;
        Try(Try<'arena>) walk_in_try walk_out_try;
        TryCatchClause(TryCatchClause<'arena>) walk_in_try_catch_clause walk_out_try_catch_clause;
        TryFinallyClause(TryFinallyClause<'arena>) walk_in_try_finally_clause walk_out_try_finally_clause;
        Foreach(Foreach<'arena>) walk_in_foreach walk_out_foreach;
        ForeachTarget(ForeachTarget<'arena>) walk_in_foreach_target walk_out_foreach_target;
        ForeachValueTarget(ForeachValueTarget<'arena>) walk_in_foreach_value_target walk_out_foreach_value_target;
        ForeachKeyValueTarget(ForeachKeyValueTarget<'arena>) walk_in_foreach_key_value_target walk_out_foreach_key_value_target;
        ForeachBody(ForeachBody<'arena>) walk_in_foreach_body walk_out_foreach_body;
        ForeachColonDelimitedBody(ForeachColonDelimitedBody<'arena>) walk_in_foreach_colon_delimited_body walk_out_foreach_colon_delimited_body;
        For(For<'arena>) walk_in_for walk_out_for;
        ForBody(ForBody<'arena>) walk_in_for_body walk_out_for_body;
        ForColonDelimitedBody(ForColonDelimitedBody<'arena>) walk_in_for_colon_delimited_body walk_out_for_colon_delimited_body;
        While(While<'arena>) walk_in_while walk_out_while;
        WhileBody(WhileBody<'arena>) walk_in_while_body walk_out_while_body;
        WhileColonDelimitedBody(WhileColonDelimitedBody<'arena>) walk_in_while_colon_delimited_body walk_out_while_colon_delimited_body;
        DoWhile(DoWhile<'arena>) walk_in_do_while walk_out_do_while;
        Continue(Continue<'arena>) walk_in_continue walk_out_continue;
        Break(Break<'arena>) walk_in_break walk_out_break;
        Switch(Switch<'arena>) walk_in_switch walk_out_switch;
        SwitchBody(SwitchBody<'arena>) walk_in_switch_body walk_out_switch_body;
        SwitchBraceDelimitedBody(SwitchBraceDelimitedBody<'arena>) walk_in_switch_brace_delimited_body walk_out_switch_brace_delimited_body;
        SwitchColonDelimitedBody(SwitchColonDelimitedBody<'arena>) walk_in_switch_colon_delimited_body walk_out_switch_colon_delimited_body;
        SwitchCase(SwitchCase<'arena>) walk_in_switch_case walk_out_switch_case;
        SwitchExpressionCase(SwitchExpressionCase<'arena>) walk_in_switch_expression_case walk_out_switch_expression_case;
        SwitchDefaultCase(SwitchDefaultCase<'arena>) walk_in_switch_default_case walk_out_switch_default_case;
        SwitchCaseSeparator(SwitchCaseSeparator) walk_in_switch_case_separator walk_out_switch_case_separator;
        If(If<'arena>) walk_in_if walk_out_if;
        IfBody(IfBody<'arena>) walk_in_if_body walk_out_if_body;
        IfStatementBody(IfStatementBody<'arena>) walk_in_if_statement_body walk_out_if_statement_body;
        IfStatementBodyElseIfClause(IfStatementBodyElseIfClause<'arena>) walk_in_if_statement_body_else_if_clause walk_out_if_statement_body_else_if_clause;
        IfStatementBodyElseClause(IfStatementBodyElseClause<'arena>) walk_in_if_statement_body_else_clause walk_out_if_statement_body_else_clause;
        IfColonDelimitedBody(IfColonDelimitedBody<'arena>) walk_in_if_colon_delimited_body walk_out_if_colon_delimited_body;
        IfColonDelimitedBodyElseIfClause(IfColonDelimitedBodyElseIfClause<'arena>) walk_in_if_colon_delimited_body_else_if_clause walk_out_if_colon_delimited_body_else_if_clause;
        IfColonDelimitedBodyElseClause(IfColonDelimitedBodyElseClause<'arena>) walk_in_if_colon_delimited_body_else_clause walk_out_if_colon_delimited_body_else_clause;
        Return(Return<'arena>) walk_in_return walk_out_return;
        ExpressionStatement(ExpressionStatement<'arena>) walk_in_statement_expression walk_out_statement_expression;
        EchoTag(EchoTag<'arena>) walk_in_echo_tag walk_out_echo_tag;
        Echo(Echo<'arena>) walk_in_echo walk_out_echo;
        Global(Global<'arena>) walk_in_global walk_out_global;
        Static(Static<'arena>) walk_in_static walk_out_static;
        StaticItem(StaticItem<'arena>) walk_in_static_item walk_out_static_item;
        StaticAbstractItem(StaticAbstractItem<'arena>) walk_in_static_abstract_item walk_out_static_abstract_item;
        StaticConcreteItem(StaticConcreteItem<'arena>) walk_in_static_concrete_item walk_out_static_concrete_item;
        HaltCompiler(HaltCompiler<'arena>) walk_in_halt_compiler walk_out_halt_compiler;
        Unset(Unset<'arena>) walk_in_unset walk_out_unset;
        Expression(Expression<'arena>) walk_in_expression walk_out_expression;
        Binary(Binary<'arena>) walk_in_binary walk_out_binary;
        BinaryOperator(BinaryOperator<'arena>) walk_in_binary_operator walk_out_binary_operator;
        UnaryPrefix(UnaryPrefix<'arena>) walk_in_unary_prefix walk_out_unary_prefix;
        UnaryPrefixOperator(UnaryPrefixOperator<'arena>) walk_in_unary_prefix_operator walk_out_unary_prefix_operator;
        UnaryPostfix(UnaryPostfix<'arena>) walk_in_unary_postfix walk_out_unary_postfix;
        UnaryPostfixOperator(UnaryPostfixOperator) walk_in_unary_postfix_operator walk_out_unary_postfix_operator;
        Parenthesized(Parenthesized<'arena>) walk_in_parenthesized walk_out_parenthesized;
        Literal(Literal<'arena>) walk_in_literal_expression walk_out_literal_expression;
        LiteralString(LiteralString<'arena>) walk_in_literal_string walk_out_literal_string;
        LiteralInteger(LiteralInteger<'arena>) walk_in_literal_integer walk_out_literal_integer;
        LiteralFloat(LiteralFloat<'arena>) walk_in_literal_float walk_out_literal_float;
        Keyword(Keyword<'arena>) walk_in_true_keyword walk_out_true_keyword;
        Keyword(Keyword<'arena>) walk_in_false_keyword walk_out_false_keyword;
        Keyword(Keyword<'arena>) walk_in_null_keyword walk_out_null_keyword;
        CompositeString(CompositeString<'arena>) walk_in_composite_string walk_out_composite_string;
        ShellExecuteString(ShellExecuteString<'arena>) walk_in_shell_execute_string walk_out_shell_execute_string;
        InterpolatedString(InterpolatedString<'arena>) walk_in_interpolated_string walk_out_interpolated_string;
        DocumentString(DocumentString<'arena>) walk_in_document_string walk_out_document_string;
        StringPart(StringPart<'arena>) walk_in_string_part walk_out_string_part;
        LiteralStringPart(LiteralStringPart<'arena>) walk_in_literal_string_part walk_out_literal_string_part;
        BracedExpressionStringPart(BracedExpressionStringPart<'arena>) walk_in_braced_expression_string_part walk_out_braced_expression_string_part;
        Assignment(Assignment<'arena>) walk_in_assignment walk_out_assignment;
        AssignmentOperator(AssignmentOperator) walk_in_assignment_operator walk_out_assignment_operator;
        Conditional(Conditional<'arena>) walk_in_conditional walk_out_conditional;
        Array(Array<'arena>) walk_in_array walk_out_array;
        ArrayElement(ArrayElement<'arena>) walk_in_array_element walk_out_array_element;
        KeyValueArrayElement(KeyValueArrayElement<'arena>) walk_in_key_value_array_element walk_out_key_value_array_element;
        ValueArrayElement(ValueArrayElement<'arena>) walk_in_value_array_element walk_out_value_array_element;
        VariadicArrayElement(VariadicArrayElement<'arena>) walk_in_variadic_array_element walk_out_variadic_array_element;
        MissingArrayElement(MissingArrayElement) walk_in_missing_array_element walk_out_missing_array_element;
        LegacyArray(LegacyArray<'arena>) walk_in_legacy_array walk_out_legacy_array;
        List(List<'arena>) walk_in_list walk_out_list;
        ArrayAccess(ArrayAccess<'arena>) walk_in_array_access walk_out_array_access;
        ArrayAppend(ArrayAppend<'arena>) walk_in_array_append walk_out_array_append;
        AnonymousClass(AnonymousClass<'arena>) walk_in_anonymous_class walk_out_anonymous_class;
        Closure(Closure<'arena>) walk_in_closure walk_out_closure;
        ClosureUseClause(ClosureUseClause<'arena>) walk_in_closure_use_clause walk_out_closure_use_clause;
        ClosureUseClauseVariable(ClosureUseClauseVariable<'arena>) walk_in_closure_use_clause_variable walk_out_closure_use_clause_variable;
        ArrowFunction(ArrowFunction<'arena>) walk_in_arrow_function walk_out_arrow_function;
        Variable(Variable<'arena>) walk_in_variable walk_out_variable;
        DirectVariable(DirectVariable<'arena>) walk_in_direct_variable walk_out_direct_variable;
        IndirectVariable(IndirectVariable<'arena>) walk_in_indirect_variable walk_out_indirect_variable;
        NestedVariable(NestedVariable<'arena>) walk_in_nested_variable walk_out_nested_variable;
        Identifier(Identifier<'arena>) walk_in_identifier walk_out_identifier;
        LocalIdentifier(LocalIdentifier<'arena>) walk_in_local_identifier walk_out_local_identifier;
        QualifiedIdentifier(QualifiedIdentifier<'arena>) walk_in_qualified_identifier walk_out_qualified_identifier;
        FullyQualifiedIdentifier(FullyQualifiedIdentifier<'arena>) walk_in_fully_qualified_identifier walk_out_fully_qualified_identifier;
        Match(Match<'arena>) walk_in_match walk_out_match;
        MatchArm(MatchArm<'arena>) walk_in_match_arm walk_out_match_arm;
        MatchExpressionArm(MatchExpressionArm<'arena>) walk_in_match_expression_arm walk_out_match_expression_arm;
        MatchDefaultArm(MatchDefaultArm<'arena>) walk_in_match_default_arm walk_out_match_default_arm;
        Yield(Yield<'arena>) walk_in_yield walk_out_yield;
        YieldValue(YieldValue<'arena>) walk_in_yield_value walk_out_yield_value;
        YieldPair(YieldPair<'arena>) walk_in_yield_pair walk_out_yield_pair;
        YieldFrom(YieldFrom<'arena>) walk_in_yield_from walk_out_yield_from;
        Construct(Construct<'arena>) walk_in_construct walk_out_construct;
        IssetConstruct(IssetConstruct<'arena>) walk_in_isset_construct walk_out_isset_construct;
        EmptyConstruct(EmptyConstruct<'arena>) walk_in_empty_construct walk_out_empty_construct;
        EvalConstruct(EvalConstruct<'arena>) walk_in_eval_construct walk_out_eval_construct;
        IncludeConstruct(IncludeConstruct<'arena>) walk_in_include_construct walk_out_include_construct;
        IncludeOnceConstruct(IncludeOnceConstruct<'arena>) walk_in_include_once_construct walk_out_include_once_construct;
        RequireConstruct(RequireConstruct<'arena>) walk_in_require_construct walk_out_require_construct;
        RequireOnceConstruct(RequireOnceConstruct<'arena>) walk_in_require_once_construct walk_out_require_once_construct;
        PrintConstruct(PrintConstruct<'arena>) walk_in_print_construct walk_out_print_construct;
        ExitConstruct(ExitConstruct<'arena>) walk_in_exit_construct walk_out_exit_construct;
        DieConstruct(DieConstruct<'arena>) walk_in_die_construct walk_out_die_construct;
        Throw(Throw<'arena>) walk_in_throw walk_out_throw;
        Clone(Clone<'arena>) walk_in_clone walk_out_clone;
        Call(Call<'arena>) walk_in_call walk_out_call;
        FunctionCall(FunctionCall<'arena>) walk_in_function_call walk_out_function_call;
        MethodCall(MethodCall<'arena>) walk_in_method_call walk_out_method_call;
        NullSafeMethodCall(NullSafeMethodCall<'arena>) walk_in_null_safe_method_call walk_out_null_safe_method_call;
        StaticMethodCall(StaticMethodCall<'arena>) walk_in_static_method_call walk_out_static_method_call;
        PartialApplication(PartialApplication<'arena>) walk_in_partial_application walk_out_partial_application;
        FunctionPartialApplication(FunctionPartialApplication<'arena>) walk_in_function_partial_application walk_out_function_partial_application;
        MethodPartialApplication(MethodPartialApplication<'arena>) walk_in_method_partial_application walk_out_method_partial_application;
        StaticMethodPartialApplication(StaticMethodPartialApplication<'arena>) walk_in_static_method_partial_application walk_out_static_method_partial_application;
        ClassLikeMemberSelector(ClassLikeMemberSelector<'arena>) walk_in_class_like_member_selector walk_out_class_like_member_selector;
        ClassLikeMemberExpressionSelector(ClassLikeMemberExpressionSelector<'arena>) walk_in_class_like_member_expression_selector walk_out_class_like_member_expression_selector;
        ConstantAccess(ConstantAccess<'arena>) walk_in_constant_access walk_out_constant_access;
        Access(Access<'arena>) walk_in_access walk_out_access;
        PropertyAccess(PropertyAccess<'arena>) walk_in_property_access walk_out_property_access;
        NullSafePropertyAccess(NullSafePropertyAccess<'arena>) walk_in_null_safe_property_access walk_out_null_safe_property_access;
        StaticPropertyAccess(StaticPropertyAccess<'arena>) walk_in_static_property_access walk_out_static_property_access;
        ClassConstantAccess(ClassConstantAccess<'arena>) walk_in_class_constant_access walk_out_class_constant_access;
        ClassLikeConstantSelector(ClassLikeConstantSelector<'arena>) walk_in_class_like_constant_selector walk_out_class_like_constant_selector;
        Keyword(Keyword<'arena>) walk_in_parent_keyword walk_out_parent_keyword;
        Keyword(Keyword<'arena>) walk_in_static_keyword walk_out_static_keyword;
        Keyword(Keyword<'arena>) walk_in_self_keyword walk_out_self_keyword;
        Instantiation(Instantiation<'arena>) walk_in_instantiation walk_out_instantiation;
        MagicConstant(MagicConstant<'arena>) walk_in_magic_constant walk_out_magic_constant;
        Pipe(Pipe<'arena>) walk_in_pipe walk_out_pipe;
        Hint(Hint<'arena>) walk_in_hint walk_out_hint;
        ParenthesizedHint(ParenthesizedHint<'arena>) walk_in_parenthesized_hint walk_out_parenthesized_hint;
        NullableHint(NullableHint<'arena>) walk_in_nullable_hint walk_out_nullable_hint;
        UnionHint(UnionHint<'arena>) walk_in_union_hint walk_out_union_hint;
        IntersectionHint(IntersectionHint<'arena>) walk_in_intersection_hint walk_out_intersection_hint;
        Keyword(Keyword<'arena>) walk_in_keyword walk_out_keyword;
    }

    // Where `visit_children` differs from Mago's walker.

    /// `new` comes before the attributes and modifiers.
    fn walk_anonymous_class(
        &self,
        class: &'ast AnonymousClass<'arena>,
        state: &mut State<'ast, 'arena, 'c>,
    ) {
        self.walk_in_anonymous_class(class, state);
        self.walk_keyword(&class.new, state);
        for attribute_list in &class.attribute_lists {
            self.walk_attribute_list(attribute_list, state);
        }
        for modifier in &class.modifiers {
            self.walk_modifier(modifier, state);
        }
        self.walk_keyword(&class.class, state);
        if let Some(argument_list) = &class.argument_list {
            self.walk_partial_argument_list(argument_list, state);
        }
        if let Some(extends) = &class.extends {
            self.walk_extends(extends, state);
        }
        if let Some(implements) = &class.implements {
            self.walk_implements(implements, state);
        }
        for member in &class.members {
            self.walk_class_like_member(member, state);
        }
        self.walk_out_anonymous_class(class, state);
    }

    /// No `static` keyword.
    fn walk_closure(&self, closure: &'ast Closure<'arena>, state: &mut State<'ast, 'arena, 'c>) {
        self.walk_in_closure(closure, state);
        for attribute_list in &closure.attribute_lists {
            self.walk_attribute_list(attribute_list, state);
        }
        self.walk_keyword(&closure.function, state);
        self.walk_function_like_parameter_list(&closure.parameter_list, state);
        if let Some(use_clause) = &closure.use_clause {
            self.walk_closure_use_clause(use_clause, state);
        }
        if let Some(return_type_hint) = &closure.return_type_hint {
            self.walk_function_like_return_type_hint(return_type_hint, state);
        }
        self.walk_block(&closure.body, state);
        self.walk_out_closure(closure, state);
    }

    /// The `use` keyword too.
    fn walk_closure_use_clause(
        &self,
        clause: &'ast ClosureUseClause<'arena>,
        state: &mut State<'ast, 'arena, 'c>,
    ) {
        self.walk_in_closure_use_clause(clause, state);
        self.walk_keyword(&clause.r#use, state);
        for variable in &clause.variables {
            self.walk_closure_use_clause_variable(variable, state);
        }
        self.walk_out_closure_use_clause(clause, state);
    }

    /// The `enddeclare` keyword too.
    fn walk_declare_colon_delimited_body(
        &self,
        body: &'ast DeclareColonDelimitedBody<'arena>,
        state: &mut State<'ast, 'arena, 'c>,
    ) {
        self.walk_in_declare_colon_delimited_body(body, state);
        for statement in &body.statements {
            self.walk_statement(statement, state);
        }
        self.walk_keyword(&body.end_declare, state);
        self.walk_terminator(&body.terminator, state);
        self.walk_out_declare_colon_delimited_body(body, state);
    }

    /// No terminator.
    fn walk_global(&self, global: &'ast Global<'arena>, state: &mut State<'ast, 'arena, 'c>) {
        self.walk_in_global(global, state);
        self.walk_keyword(&global.global, state);
        for variable in &global.variables {
            self.walk_variable(variable, state);
        }
        self.walk_out_global(global, state);
    }

    /// No terminator.
    fn walk_goto(&self, goto: &'ast Goto<'arena>, state: &mut State<'ast, 'arena, 'c>) {
        self.walk_in_goto(goto, state);
        self.walk_keyword(&goto.goto, state);
        self.walk_local_identifier(&goto.label, state);
        self.walk_out_goto(goto, state);
    }

    /// `var` before the modifiers, and no terminator.
    fn walk_plain_property(
        &self,
        property: &'ast PlainProperty<'arena>,
        state: &mut State<'ast, 'arena, 'c>,
    ) {
        self.walk_in_plain_property(property, state);
        for attribute_list in &property.attribute_lists {
            self.walk_attribute_list(attribute_list, state);
        }
        if let Some(var) = &property.var {
            self.walk_keyword(var, state);
        }
        for modifier in &property.modifiers {
            self.walk_modifier(modifier, state);
        }
        if let Some(hint) = &property.hint {
            self.walk_hint(hint, state);
        }
        for item in &property.items {
            self.walk_property_item(item, state);
        }
        self.walk_out_plain_property(property, state);
    }

    /// `var` before the modifiers.
    fn walk_hooked_property(
        &self,
        property: &'ast HookedProperty<'arena>,
        state: &mut State<'ast, 'arena, 'c>,
    ) {
        self.walk_in_hooked_property(property, state);
        for attribute_list in &property.attribute_lists {
            self.walk_attribute_list(attribute_list, state);
        }
        if let Some(var) = &property.var {
            self.walk_keyword(var, state);
        }
        for modifier in &property.modifiers {
            self.walk_modifier(modifier, state);
        }
        if let Some(hint) = &property.hint {
            self.walk_hint(hint, state);
        }
        self.walk_property_item(&property.item, state);
        self.walk_property_hook_list(&property.hook_list, state);
        self.walk_out_hooked_property(property, state);
    }

    /// No `array` keyword.
    fn walk_legacy_array(&self, array: &'ast LegacyArray<'arena>, state: &mut State<'ast, 'arena, 'c>) {
        self.walk_in_legacy_array(array, state);
        for element in &array.elements {
            self.walk_array_element(element, state);
        }
        self.walk_out_legacy_array(array, state);
    }

    /// No `list` keyword.
    fn walk_list(&self, list: &'ast List<'arena>, state: &mut State<'ast, 'arena, 'c>) {
        self.walk_in_list(list, state);
        for element in &list.elements {
            self.walk_array_element(element, state);
        }
        self.walk_out_list(list, state);
    }
}
