//! The class a rebound closure's body is checked in (StaticCallHandler's `Closure::bind()` scope factory,
//! MethodCallHandler's `->call()`, MutatingScope::enterClosureBind()/enterClosureCall()): the bound `$this`'s class.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_syntax::cst::*;
use pun_phpstan::types::Type;

use super::types::flow::unparen;
use super::{Checker, declared, members, params};

/// A PHPDoc binding assigns `$this` as well as a visibility scope. Explicit `Closure::bind()` keeps the existing
/// scope-only approximation below: its third argument does not establish that `$this` exists.
pub(super) struct Binding {
    pub class: Option<String>,
    pub defines_this: bool,
}

pub(super) fn has_inline_closure(args: &ArgumentList) -> bool {
    args.arguments.iter().any(|a| {
        matches!(
            unparen(a.value()),
            Expression::Closure(_) | Expression::ArrowFunction(_)
        )
    })
}

impl<'arena> Checker<'_, 'arena> {
    /// ArgumentsHandler::processArgs()/resolveClosureThisType(): named function calls, with NameResolver's
    /// namespaced-to-global fallback (but never falling back through a `use function` import).
    pub(super) fn bind_function_arguments(&mut self, call: &FunctionCall<'arena>) {
        if !has_inline_closure(&call.argument_list) {
            return;
        }
        let Expression::Identifier(id) = call.function else {
            return;
        };
        let name = super::function_checks::function_name(self.names, self.codebase, id);
        if let Some(f) = self
            .codebase
            .get_function(name.as_bytes())
            .filter(|f| !f.flags.is_built_in())
        {
            self.bind_arguments(&call.argument_list, f, None, None);
        }
    }

    /// CalledOnTypeUnresolvedMethodPrototypeReflection::transformStaticType(): `static`/`$this` tags use the
    /// receiver, not the method's declaring parent or trait; `self` tags remain their resolved declared class.
    /// Other receiver expressions use their single inferred object class, ignoring nullability, independently of
    /// the level-2 call checks. Multiple classes are not collapsed to an arbitrary binding scope.
    pub(super) fn bind_method_arguments(
        &mut self,
        receiver: &Expression<'arena>,
        name: &ClassLikeMemberSelector<'arena>,
        args: &ArgumentList<'arena>,
        is_static: bool,
    ) {
        if !has_inline_closure(args) {
            return;
        }
        let ClassLikeMemberSelector::Identifier(name) = name else {
            return;
        };
        let original_receiver = receiver;
        let receiver = unparen(receiver);
        let class = match receiver {
            Expression::Identifier(id) if is_static => self.class_like(&self.resolved(id)).found(),
            Expression::Self_(_) | Expression::Static(_) if is_static => self.class_scope(),
            Expression::Parent(_) if is_static => self
                .class_scope()
                .and_then(|k| k.direct_parent_class)
                .and_then(|p| self.codebase.get_class_like(p.as_bytes())),
            _ => self
                .bound_class_of(receiver, false)
                .and_then(|n| self.class_like(&n).found())
                .or_else(|| {
                    let names = self.phpstan_type(original_receiver)?.remove_null().object_class_names();
                    let [name] = names.as_slice() else { return None };
                    self.class_like(name).found()
                }),
        };
        // MixinMethodsClassReflectionExtension: a method a `@mixin` class has, whose `static` is that class
        let class = class.and_then(|class| match members::method(self, class, name.value) {
            Some(method) => Some((class, method)),
            None => {
                let display = self.reflected_method(&Type::Object(self.display(class), vec![], false), name.value).ok()??.display;
                let mixin = self.class_like(display.split('<').next().unwrap_or(&display)).found()?;
                Some((mixin, members::method(self, mixin, name.value)?))
            }
        });
        if let Some((class, members::Method::User { f, declaring, .. })) = class {
            // Mago's populated `used_traits` includes inherited traits. Their `self` is the class that first
            // imports the method, whereas `static` below is still the receiver at this call site.
            let declaring = class
                .appearing_method_ids
                .get(&mago_word::word(name.value.to_ascii_lowercase()))
                .and_then(|id| self.codebase.get_class_like(id.get_class_name().as_bytes()))
                .unwrap_or(declaring);
            // `parent::` looks up the parent method, but forwards the calling class for late static binding.
            let class = if is_static && matches!(receiver, Expression::Parent(_)) {
                self.class_scope().unwrap_or(class)
            } else {
                class
            };
            let called = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
            self.bind_arguments(args, f, Some(declaring), Some(&called));
        }
    }

    /// ArgumentsHandler's parameter matching, then its non-static inline Closure/ArrowFunction branches. An
    /// unpack makes later positional indexes unknown, so it ends this direct-argument binding slice.
    fn bind_arguments(
        &mut self,
        args: &ArgumentList<'arena>,
        f: &FunctionLikeMetadata,
        declaring: Option<&ClassLikeMetadata>,
        called: Option<&str>,
    ) {
        let tags = declared::with_classes(self, || {
            let mut tags = declared::codex_closure_this(self, f, declaring);
            // A concrete trait method is reflected as declared by its using class (members::method()), but the
            // codex only stores the method under the trait. Resolve its own tags at the trait's source location,
            // with the using class as `self`/`parent`, retaining inherited tags for other parameters.
            if let Some(class) = declaring
                && !(f.span.file_id == class.span.file_id
                    && class.span.start.offset <= f.span.start.offset
                    && f.span.end.offset <= class.span.end.offset)
                && let Some((_, entry, _)) = declared::doc_at(self, f.span)
            {
                let doc = &entry.doc;
                let scope = declared::scope_at(self, f.span, Some(class));
                let scope = declared::with_method_templates(
                    &scope,
                    class,
                    &String::from_utf8_lossy(f.original_name.as_bytes()),
                    &doc.templates,
                );
                tags.extend(super::doc::Doc::named_types(&doc.closure_this, &scope));
            }
            tags
        });
        if tags.is_empty() {
            return;
        }
        let acceptor = params::Acceptor::of(f);
        for (i, arg) in args.arguments.iter().enumerate() {
            let parameter = match arg {
                Argument::Named(n) => acceptor
                    .names
                    .iter()
                    .position(|p| p.as_bytes() == n.name.value),
                Argument::Positional(p) if p.ellipsis.is_some() => break,
                Argument::Positional(_) => (i < acceptor.names.len()).then_some(i),
            }
            .or_else(|| acceptor.variadics.iter().rposition(|v| *v));
            let Some(tag) = parameter.and_then(|i| tags.get(&acceptor.names[i])) else {
                continue;
            };
            let class = match tag {
                Type::Object(n, _, _) => Some(n.as_str()),
                Type::Static(_, _) | Type::This(_) => called,
                // Templates, conditionals, unions and intersections need full closure `$this` type propagation,
                // not a guessed single visibility class.
                _ => None,
            };
            let Some(class) = class else { continue };
            let value = unparen(arg.value());
            if matches!(value, Expression::Closure(f) if f.r#static.is_none())
                || matches!(value, Expression::ArrowFunction(f) if f.r#static.is_none())
            {
                self.rebind(value, Some(class.to_string()), true);
            }
        }
    }

    /// The class of `Closure::bind($closure, $newThis, $newScope)`'s `$this`: `$newScope`'s class when it is given,
    /// else `$newThis`'s.
    // ponytail: classes read from the written expressions (`X::class`, `'X'`, `new X`, `$this`), not their types; the
    // bound scope classes are not kept apart from `$this`'s class
    pub(super) fn bind_class(&self, args: &ArgumentList<'arena>) -> Option<String> {
        let arg = |i: usize| args.arguments.get(i).map(|a| a.value());
        match arg(2) {
            Some(scope) => self.bound_class_of(scope, true),
            None => self.bound_class_of(arg(1)?, false),
        }
    }

    /// The class of `->call($newThis)`'s `$this`.
    pub(super) fn call_class(&self, args: &ArgumentList<'arena>) -> Option<String> {
        self.bound_class_of(args.arguments.first()?.value(), false)
    }

    /// The class an object expression is an instance of, or (`class_string`) the class a class string names.
    fn bound_class_of(&self, e: &Expression<'arena>, class_string: bool) -> Option<String> {
        let current = || {
            self.class_scope()
                .map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
        };
        match e {
            Expression::Parenthesized(p) => self.bound_class_of(p.expression, class_string),
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => current(),
            Expression::Instantiation(new) => match new.class {
                Expression::Identifier(id) => Some(self.resolved(id)),
                Expression::Self_(_) | Expression::Static(_) => current(),
                _ => None,
            },
            Expression::Access(Access::ClassConstant(k))
                if class_string
                    && matches!(&k.constant, ClassLikeConstantSelector::Identifier(i) if i.value.eq_ignore_ascii_case(b"class")) =>
            {
                match k.class {
                    Expression::Identifier(id) => Some(self.resolved(id)),
                    Expression::Self_(_) | Expression::Static(_) => current(),
                    object => self.bound_class_of(object, false),
                }
            }
            Expression::Literal(Literal::String(s)) if class_string => {
                let name = String::from_utf8_lossy(s.value?).replace("\\\\", "\\");
                if name.eq_ignore_ascii_case("static") {
                    current()
                } else {
                    Some(name.trim_start_matches('\\').to_string())
                }
            }
            _ => None,
        }
    }
}
