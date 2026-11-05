//! The method reflection consumed by CallMethodsRule and its named-argument collector. Leaf lookup stays in
//! receivers; composition happens before selecting an acceptor in the call's per-name scope.

use mago_codex::visibility::Visibility;
use pun_phpstan::types::{Level, Tri, Type};

use super::{Checker, declared, members, params};

pub(super) struct Method<'a> {
    pub declaring: members::Method<'a>,
    pub display: String,
    pub name: String,
    pub variants: params::Variants,
    /// Resolved class context for structural parameter descriptions, not a call-template inference result.
    variant_types: Vec<Vec<Type>>,
    pub accepts: bool,
    pub private: bool,
    pub public: bool,
    pub final_class: bool,
    /// Combined parameter types, template maps and return intersections are not yet a semantic signature.
    /// Both consumers can bind names/counts, but must not type-check against an arbitrary surviving leaf.
    pub compound: bool,
}

impl<'a> Method<'a> {
    pub fn leaf(c: &Checker, receiver: &Type, declaring: members::Method<'a>) -> Self {
        let (accepts, final_class) = match &declaring {
            members::Method::User {
                declaring, name, ..
            } => (
                declared::reflected_accepts_named(c, declaring, &name.to_string()),
                declaring.flags.is_final(),
            ),
            members::Method::Builtin { declaring, .. } => (true, declaring.is_final),
        };
        let final_class = final_class
            || matches!(receiver, Type::Object(n, _, true)
            if n.eq_ignore_ascii_case(&declaring.declaring_name()));
        let (display, names, args) = context(c, receiver, &declaring);
        let mut variants = declaring.variants();
        let scope = format!("class {}", declaring.declaring_name());
        // `static` in a PHPDoc type resolves against the class reflection (a nested one too)
        let variant_types = declared::with_classes(c, || variants
            .all
            .iter_mut()
            .map(|v| {
                let types: Vec<_> = (0..v.names.len())
                    .map(|i| {
                        let ty = super::types::declared_type(c, v, i)
                            .unwrap_or_else(pun_phpstan::types::mixed);
                        called_on(&super::types::with_args(&ty, &scope, &names, &args), receiver)
                    })
                    .collect();
                v.types = types.iter().map(|t| t.describe(Level::TypeOnly)).collect();
                types
            })
            .collect());
        Self {
            display,
            name: declaring.name(),
            variants,
            variant_types,
            accepts,
            private: declaring.visibility().is_private(),
            public: declaring.visibility().is_public(),
            final_class,
            declaring,
            compound: false,
        }
    }

    /// UnionTypeMethodReflection: first declaring class, every original variant, restrictive visibility/policy.
    pub fn union(c: &Checker, mut methods: Vec<Self>, name: &[u8]) -> Option<Self> {
        let mut combined = params::combine(
            &methods
                .iter()
                .flat_map(|m| &m.variants.all)
                .collect::<Vec<_>>(),
        );
        let types: Vec<_> = (0..combined.names.len())
            .map(|i| {
                let tail = combined.variadics.get(i);
                declared::with_classes(c, || {
                    pun_phpstan::types::union(
                        methods
                            .iter()
                            .flat_map(|m| &m.variant_types)
                            .flat_map(|types| {
                                types
                                    .iter()
                                    .skip(i)
                                    .take(if tail { usize::MAX } else { 1 })
                                    .cloned()
                            })
                            .collect(),
                    )
                })
            })
            .collect();
        combined.types = types.iter().map(|t| t.describe(Level::TypeOnly)).collect();
        let accepts = methods.iter().all(|m| m.accepts);
        let private = methods.iter().any(|m| m.private);
        let public = methods.iter().all(|m| m.public);
        let mut first = methods.drain(..).next()?;
        first.variants = params::Variants {
            all: vec![combined],
            named: None,
        };
        first.variant_types = vec![types];
        first.name = String::from_utf8_lossy(name).into_owned();
        first.accepts = accepts;
        first.private = private;
        first.public = public;
        first.compound = true;
        Some(first)
    }

    /// IntersectionTypeMethodReflection::getMethodWithMostParameters(): variadic wins, then width, first tie.
    pub fn intersection(mut methods: Vec<Self>, name: &[u8]) -> Option<Self> {
        if methods.len() == 1 {
            return methods.pop();
        }
        let mut selected = 0;
        let mut widest = (false, 0);
        for (i, method) in methods.iter().enumerate() {
            for variant in &method.variants.all {
                let width = (variant.variadic, variant.names.len());
                if width > widest {
                    widest = width;
                    selected = i;
                }
            }
        }
        let accepts = methods.iter().any(|m| m.accepts);
        let private = methods.iter().all(|m| m.private);
        let public = methods.iter().any(|m| m.public);
        if methods.is_empty() {
            return None;
        }
        let mut method = methods.swap_remove(selected);
        method.variants.named = None;
        method.name = String::from_utf8_lossy(name).into_owned();
        method.accepts = accepts;
        method.private = private;
        method.public = public;
        method.compound = true;
        Some(method)
    }

    pub fn builtin(&self) -> bool {
        matches!(self.declaring, members::Method::Builtin { .. })
    }

    pub fn visibility(&self) -> Visibility {
        if self.public {
            Visibility::Public
        } else if self.private {
            Visibility::Private
        } else {
            Visibility::Protected
        }
    }

    /// FunctionCallParametersCheck receives these separately from the combined acceptor in PHPStan.
    pub fn select(
        &self,
        c: &Checker,
        counts: &params::Counts,
        scope: Option<&super::infer::NamedCallScope>,
    ) -> params::Acceptor {
        let variants = params::Variants {
            all: self.variants.all.clone(),
            named: self.variants.named.clone(),
        };
        let mut acceptor = params::select_in(c, variants, counts, scope);
        acceptor.builtin = self.builtin();
        acceptor.accepts_named_arguments = Tri::from(self.accepts);
        acceptor
    }
}

/// `CalledOnTypeUnresolvedMethodPrototypeReflection`: a method called on an object resolves its `static` and
/// `$this` types to that object; called on `static`/`$this` they stay late.
pub(super) fn called_on(t: &Type, receiver: &Type) -> Type {
    match t {
        Type::Static(..) | Type::This(_) if matches!(receiver, Type::Object(..)) => receiver.clone(),
        _ => t.traverse(&|i| called_on(i, receiver)),
    }
}

/// ObjectType resolves the declaring ancestor's class template map before composing reflections. This display
/// keeps supplied generic arguments (including through mixins), or the declaring class's defaults/bounds.
fn context(
    c: &Checker,
    receiver: &Type,
    method: &members::Method,
) -> (String, Vec<String>, Vec<Type>) {
    let display = method.declaring_display(c);
    let declaring = method.declaring_name();
    declared::with_classes(c, || {
        let args = match receiver {
            Type::Object(n, args, _) | Type::Static(n, args)
                if n.eq_ignore_ascii_case(&declaring) =>
            {
                args.clone()
            }
            Type::Object(n, args, _) | Type::Static(n, args) => pun_phpstan::symbols::lookup(n)
                .and_then(|class| pun_phpstan::types::ancestor_args(&class, &declaring, args))
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        // The real declaring reflection already owns its templates; do not invent a fallback class lookup.
        let templates: Vec<_> = match method {
            members::Method::User { declaring, .. } => declared::class_templates(c, declaring)
                .into_iter()
                .map(|t| (t.name, t.default.unwrap_or(t.bound)))
                .collect(),
            members::Method::Builtin { declaring, .. } => declaring
                .templates
                .iter()
                .map(|t| {
                    (
                        t.name.clone(),
                        pun_phpstan::resolve::resolve_str(
                            t.default.as_ref().unwrap_or(&t.bound),
                            Some(&declaring.name),
                        ),
                    )
                })
                .collect(),
        };
        let args: Vec<_> = templates
            .iter()
            .enumerate()
            .map(|(i, (_, fallback))| args.get(i).cloned().unwrap_or_else(|| fallback.clone()))
            .collect();
        let names = templates.into_iter().map(|(name, _)| name).collect();
        let display = if args.is_empty() {
            display
        } else {
            format!(
                "{display}<{}>",
                args.iter()
                    .map(|t| t.describe(Level::TypeOnly))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };
        (display, names, args)
    })
}
