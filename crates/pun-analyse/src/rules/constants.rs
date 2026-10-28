//! Class-constant declaration rules: ClassAsClassConstantRule, ConstantsInTraitsRule, FinalConstantRule,
//! NativeTypedClassConstantRule, ConflictingTraitConstantsRule and OverridingConstantRule (its native type and
//! PHPDoc type checks need checkPhpDocMethodSignatures, on from level 3), plus ReadOnlyClassRule, which PHPStan
//! reports under `classConstant.nativeTypeNotSupported`.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::class_like_constant::ClassLikeConstantMetadata;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Level, Type, decide_type, is_super_type_of};

use super::{Checker, declared};

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// The visibility a declaration's modifiers give (public without one).
fn visibility(modifiers: &[Modifier]) -> Visibility {
    modifiers
        .iter()
        .find_map(|m| match m {
            Modifier::Private(_) => Some(Visibility::Private),
            Modifier::Protected(_) => Some(Visibility::Protected),
            Modifier::Public(_) => Some(Visibility::Public),
            _ => None,
        })
        .unwrap_or(Visibility::Public)
}

fn is_final(modifiers: &[Modifier]) -> bool {
    modifiers.iter().any(|m| matches!(m, Modifier::Final(_)))
}

/// The rules registered for `Node\Stmt\ClassConst`.
pub fn class_constant<'arena>(c: &mut Checker<'_, 'arena>, k: &ClassLikeConstant<'arena>) {
    class_as_class_constant(c, k);
    let modifiers = k.modifiers.as_slice();
    // ConstantsInTraitsRule (`$scope->isInTrait()`), PhpVersion::supportsConstantsInTraits()
    if !c.surely_at_least(80200) && c.in_target {
        c.report_non_ignorable(
            k,
            "classConstant.inTrait",
            "Constant is declared inside a trait but is only supported on PHP 8.2 and later."
                .to_string(),
            None,
        );
    }
    // FinalConstantRule, PhpVersion::supportsFinalConstants()
    if is_final(modifiers) && !c.surely_at_least(80100) {
        c.report_non_ignorable(
            k,
            "classConstant.finalNotSupported",
            "Final class constants are supported only on PHP 8.1 and later.".to_string(),
            None,
        );
    }
    // NativeTypedClassConstantRule, PhpVersion::supportsNativeTypesInClassConstants()
    if k.hint.is_some() && !c.surely_at_least(80300) {
        c.report_non_ignorable(
            k,
            "classConstant.nativeTypeNotSupported",
            "Class constants with native types are supported only on PHP 8.3 and later."
                .to_string(),
            None,
        );
    }
    // class constants are walked inside their class-like (trait bodies only in a class's context)
    let class = c
        .class_scope()
        .expect("a class constant is walked inside its class-like");
    conflicting_trait_constants(c, class, k);
    overriding_constant(c, class, k);
}

/// ClassAsClassConstantRule.
fn class_as_class_constant(c: &mut Checker, k: &ClassLikeConstant) {
    for item in k.items.iter() {
        if !item.name.value.eq_ignore_ascii_case(b"class") {
            continue;
        }
        c.report_non_ignorable(
            item,
            "classConstant.class",
            "A class constant must not be called 'class'; it is reserved for class name fetching."
                .to_string(),
            None,
        );
    }
}

/// ReadOnlyClassRule (`InClassNode`): a `readonly` class declared at `at`.
pub fn read_only_class(c: &mut Checker, at: Span, modifiers: &[Modifier], anonymous: bool) {
    // Below PHP 8.1 php-parser lexes `readonly` as a name: PHPStan reports a parse error and never reaches the rule.
    if c.php_version < 80100 || !modifiers.iter().any(|m| matches!(m, Modifier::Readonly(_))) {
        return;
    }
    let message = if anonymous {
        // PhpVersion::supportsReadOnlyAnonymousClasses()
        if c.surely_at_least(80300) {
            return;
        }
        "Anonymous readonly classes are supported only on PHP 8.3 and later."
    } else {
        // PhpVersion::supportsReadOnlyClasses()
        if c.surely_at_least(80200) {
            return;
        }
        "Readonly classes are supported only on PHP 8.2 and later."
    };
    c.report_non_ignorable(
        at,
        "classConstant.nativeTypeNotSupported",
        message.to_string(),
        None,
    );
}

impl<'a> Checker<'a, '_> {
    fn class_named(&self, name: &mago_word::Word) -> Option<&'a ClassLikeMetadata> {
        self.codebase.get_class_like(name.as_bytes())
    }

    /// A native type written at `span` of the file `file`, as PHPStan describes it (`self` resolved in `class`).
    // assistant's call: the hint's source text through pun-phpstan's type resolver (class names stay as written)
    fn native_type(
        &self,
        file: mago_database::file::FileId,
        span: Span,
        class: &str,
    ) -> Option<Type> {
        let contents = self.file_contents(file)?;
        let written = contents.get(span.to_range_usize())?;
        Some(resolve_str(&text(written), Some(class)))
    }
}

/// ConflictingTraitConstantsRule.
fn conflicting_trait_constants<'arena>(
    c: &mut Checker<'_, 'arena>,
    class: &ClassLikeMetadata,
    k: &ClassLikeConstant<'arena>,
) {
    let display = c.display(class);
    let class_name = text(class.name.as_bytes());
    // `getTraits(true)`: the class's traits, theirs, and those of its parents (the codex's `used_traits`)
    let recursive: Vec<&ClassLikeMetadata> = class
        .used_traits
        .iter()
        .filter_map(|t| c.class_named(t))
        .collect();
    for item in k.items.iter() {
        for t in &recursive {
            if let Some(constant) = t.constants.get(&mago_word::word(item.name.value)) {
                single_trait_constant(c, &display, &class_name, t, constant, k, item);
            }
        }
    }
    let immediate: Vec<&ClassLikeMetadata> = class
        .direct_trait_names()
        .filter_map(|t| c.class_named(&t))
        .collect();
    for item in k.items.iter() {
        let name = mago_word::word(item.name.value);
        for t in &immediate {
            let Some(theirs) = t.constants.get(&name) else {
                continue;
            };
            // Both expressions belong to the consuming context, including a trait expression hidden
            // by an own declaration. Reflected Consumer::VALUE cannot stand in for that source.
            if let (Some(ours), Some(theirs)) = (
                source_value(c, item.span(), class),
                source_value(c, theirs.span, class),
            ) && !initializer_values_equal(&ours, &theirs)
            {
                let name = text(item.name.value);
                c.report_non_ignorable(
                    k,
                    "classConstant.value",
                    format!(
                        "Constant {display}::{name} with value {} overriding constant {}::{name} with different value {} should have the same value.",
                        ours.describe(Level::Value),
                        c.display(t),
                        theirs.describe(Level::Value),
                    ),
                    None,
                );
            }
        }
    }
}

/// UnionType::equals is unordered, including inside ordered ConstantArrayType values. Reflection's
/// member order is observable during getConstant(), but is not a trait initializer inequality.
fn initializer_values_equal(left: &Type, right: &Type) -> bool {
    match (left, right) {
        (Type::Union(left), Type::Union(right)) => {
            let mut unmatched: Vec<_> = right.iter().collect();
            left.len() == right.len()
                && left.iter().all(|value| {
                    if let Some(index) = unmatched
                        .iter()
                        .position(|other| initializer_values_equal(value, other))
                    {
                        unmatched.remove(index);
                        true
                    } else {
                        false
                    }
                })
        }
        (Type::ConstArray(left), Type::ConstArray(right)) => {
            left.keys == right.keys
                && left.optional == right.optional
                && left.unsealed == right.unsealed
                && left.values.len() == right.values.len()
                && left
                    .values
                    .iter()
                    .zip(&right.values)
                    .all(|(left, right)| initializer_values_equal(left, right))
        }
        _ => left == right,
    }
}

fn source_value(c: &Checker, at: Span, class: &ClassLikeMetadata) -> Option<Type> {
    let contents = c.file_contents(at.file_id)?;
    let mut file = mago_database::file::File::ephemeral(
        b"trait-initializer.php".as_slice().into(),
        contents.into_owned().into(),
    );
    file.id = at.file_id;
    file.path = c.files.get(&at.file_id).cloned();
    crate::codebase::source_initializer(
        c.codebase,
        &file,
        at,
        class.name,
        c.php_version,
        c.include_settings.path_constants,
        c.files.get(&class.span.file_id).cloned(),
    )
    .map(|value| super::arrays::initializer_order(c, at, class, c.to_phpstan(&value)))
}

/// ConflictingTraitConstantsRule::processSingleConstant().
fn single_trait_constant<'arena>(
    c: &mut Checker<'_, 'arena>,
    display: &str,
    class_name: &str,
    t: &ClassLikeMetadata,
    theirs: &ClassLikeConstantMetadata,
    k: &ClassLikeConstant<'arena>,
    item: &ClassLikeConstantItem<'arena>,
) {
    let name = text(item.name.value);
    let trait_display = c.display(t);
    let ours = visibility(k.modifiers.as_slice());
    let label = |v: Visibility| match v {
        Visibility::Public => ("Public", "public"),
        Visibility::Protected => ("Protected", "protected"),
        Visibility::Private => ("Private", "private"),
    };
    if ours != theirs.visibility {
        let (ours, _) = label(ours);
        let (_, theirs) = label(theirs.visibility);
        c.report_non_ignorable(
            k,
            "classConstant.visibility",
            format!(
                "{ours} constant {display}::{name} overriding {theirs} constant {trait_display}::{name} should also be {theirs}."
            ),
            None,
        );
    }
    let final_here = is_final(k.modifiers.as_slice());
    // assistant's call: the codex's FINAL flag (also set by `@final`) stands in for ReflectionClassConstant::isFinal()
    if theirs.flags.is_final() {
        if !final_here {
            c.report_non_ignorable(
                k,
                "classConstant.nonFinal",
                format!(
                    "Non-final constant {display}::{name} overriding final constant {trait_display}::{name} should also be final."
                ),
                None,
            );
        }
    } else if final_here {
        c.report_non_ignorable(
            k,
            "classConstant.final",
            format!(
                "Final constant {display}::{name} overriding non-final constant {trait_display}::{name} should also be non-final."
            ),
            None,
        );
    }
    let trait_name = text(t.name.as_bytes());
    let theirs_type = theirs
        .type_declaration
        .as_ref()
        .map(|d| c.native_type(theirs.span.file_id, d.span, &trait_name));
    let ours_type = k
        .hint
        .as_ref()
        .map(|h| c.native_type(c.file.id, h.span(), class_name));
    let describe = |t: &Type| t.describe(Level::TypeOnly);
    match (theirs_type, ours_type) {
        (None, Some(Some(ours))) => c.report_non_ignorable(
            k,
            "classConstant.nativeType",
            format!(
                "Constant {display}::{name} ({}) overriding constant {trait_display}::{name} should not have a native type.",
                describe(&ours)
            ),
            None,
        ),
        (Some(Some(theirs)), None) => {
            let theirs = describe(&theirs);
            c.report_non_ignorable(
                k,
                "classConstant.missingNativeType",
                format!(
                    "Constant {display}::{name} overriding constant {trait_display}::{name} ({theirs}) should also have native type {theirs}."
                ),
                None,
            );
        }
        (Some(Some(theirs)), Some(Some(ours))) if theirs != ours => {
            let theirs = describe(&theirs);
            c.report_non_ignorable(
                k,
                "classConstant.nativeType",
                format!(
                    "Constant {display}::{name} ({}) overriding constant {trait_display}::{name} ({theirs}) should have the same native type {theirs}.",
                    describe(&ours)
                ),
                None,
            );
        }
        _ => {}
    }
}

/// OverridingConstantRule. checkPhpDocMethodSignatures enables the type checks from level 3.
fn overriding_constant<'arena>(
    c: &mut Checker<'_, 'arena>,
    class: &ClassLikeMetadata,
    k: &ClassLikeConstant<'arena>,
) {
    let display = c.display(class);
    for item in k.items.iter() {
        let Some((prototype, ancestor)) = find_prototype(c, class, item.name.value) else {
            continue;
        };
        let key = mago_word::word(item.name.value);
        let (current, context) =
            reflected_constant(c, class, key).expect("a class constant walked in its class");
        let name = text(current.name.as_bytes());
        // an interface's constants are final before PHP 8.1 (BetterReflection's isFinal())
        let interface_final = ancestor.kind.is_interface() && c.php_version < 80100;
        let declaring = c.display(ancestor);
        if prototype.flags.is_final() || interface_final {
            c.report_non_ignorable(
                k,
                "classConstant.final",
                format!("Constant {display}::{name} overrides final constant {declaring}::{name}."),
                None,
            );
        }
        let display = c.display(context);
        let ours = current.visibility;
        if prototype.visibility == Visibility::Public {
            if ours != Visibility::Public {
                let label = if ours == Visibility::Private {
                    "Private"
                } else {
                    "Protected"
                };
                c.report_non_ignorable(
                    k,
                    "classConstant.visibility",
                    format!(
                        "{label} constant {display}::{name} overriding public constant {declaring}::{name} should also be public."
                    ),
                    None,
                );
            }
        } else if ours == Visibility::Private {
            c.report_non_ignorable(
                k,
                "classConstant.visibility",
                format!(
                    "Private constant {display}::{name} overriding protected constant {declaring}::{name} should be protected or public."
                ),
                None,
            );
        }
        if c.level < 3 {
            continue;
        }
        let errors = declared::with_classes(c, || {
            let theirs = constant_native(c, prototype, ancestor);
            let ours = constant_native(c, current, context);
            let mut errors = Vec::new();
            if let Some(theirs) = &theirs {
                let theirs_display = theirs.describe(Level::TypeOnly);
                match &ours {
                    Some(ours) if !is_super_type_of(theirs, ours).yes() => errors.push((
                        "classConstant.nativeType",
                        format!(
                            "Native type {} of constant {display}::{name} is not covariant with native type {theirs_display} of constant {declaring}::{name}.",
                            ours.describe(Level::TypeOnly)
                        ),
                        false,
                    )),
                    None => errors.push((
                        "classConstant.missingNativeType",
                        format!(
                            "Constant {display}::{name} overriding constant {declaring}::{name} ({theirs_display}) should also have native type {theirs_display}."
                        ),
                        false,
                    )),
                    _ => {}
                }
            }
            // RealClassClassConstantReflection::getValueType(): only compare when both constants have a
            // PHPDoc type, but decide that PHPDoc against the native type before comparing their values.
            let theirs_doc = declared::constant(c, prototype.span, item.name.value, ancestor);
            let ours_doc = declared::constant(c, current.span, item.name.value, context);
            if let (Some(theirs_doc), Some(ours_doc)) = (theirs_doc, ours_doc) {
                let effective = |native, doc| match native {
                    Some(native) => decide_type(native, Some(doc)),
                    None => doc,
                };
                let theirs = effective(theirs, theirs_doc);
                let ours = effective(ours, ours_doc);
                if !is_super_type_of(&theirs, &ours).yes() {
                    errors.push((
                        "classConstant.type",
                        format!(
                            "Type {} of constant {display}::{name} is not covariant with type {} of constant {declaring}::{name}.",
                            ours.describe(Level::Value),
                            theirs.describe(Level::Value)
                        ),
                        true,
                    ));
                }
            }
            errors
        });
        for (identifier, message, ignorable) in errors {
            if ignorable {
                c.report(k, identifier, message, None);
            } else {
                c.report_non_ignorable(k, identifier, message, None);
            }
        }
    }
}

/// BetterReflection::getConstantsConsideringAlreadyVisitedClasses(): own declarations, inherited non-private
/// constants, then traits and interfaces. Mago's populated map puts traits before inherited constants.
fn reflected_constant<'a>(
    c: &Checker<'a, '_>,
    class: &'a ClassLikeMetadata,
    name: mago_word::Word,
) -> Option<(&'a ClassLikeConstantMetadata, &'a ClassLikeMetadata)> {
    let ancestors: Vec<_> = std::iter::successors(Some(class), |k| {
        k.direct_parent_class.and_then(|p| c.class_named(&p))
    })
    .take(64)
    .collect();
    ancestors.into_iter().rev().fold(
        None,
        |inherited: Option<(&ClassLikeConstantMetadata, &ClassLikeMetadata)>, class| {
            let current = class.constants.get(&name);
            let own = current.is_some_and(|m| {
                m.span.file_id == class.span.file_id
                    && class.span.start.offset <= m.span.start.offset
                    && m.span.end.offset <= class.span.end.offset
            });
            if !own
                && let Some(inherited) =
                    inherited.filter(|(m, _)| m.visibility != Visibility::Private)
            {
                return Some(inherited);
            }
            current.map(|m| (m, declaring(c, class, m)))
        },
    )
}

/// `ClassConstantReflection::getNativeType()`: the declaration or PHPStan's php-8-stubs metadata, never the
/// inferred initializer type.
fn constant_native(
    c: &Checker,
    constant: &ClassLikeConstantMetadata,
    class: &ClassLikeMetadata,
) -> Option<Type> {
    if let Some(native) = &constant.type_declaration {
        return Some(declared::from_tunion(c, &native.type_union, class));
    }
    if !class.flags.is_built_in() {
        return None;
    }
    super::overriding::natives(c.php_version)
        .get(&c.display(class).to_ascii_lowercase())?
        .constant_types
        .get(&text(constant.name.as_bytes()))
        .cloned()
}

/// OverridingConstantRule::findPrototype(): the constant of the first immediate interface that has it, else the
/// parent's unless private; with the display name of the class declaring it.
fn find_prototype<'a>(
    c: &Checker<'a, '_>,
    class: &'a ClassLikeMetadata,
    name: &[u8],
) -> Option<(&'a ClassLikeConstantMetadata, &'a ClassLikeMetadata)> {
    let name = mago_word::word(name);
    let ancestors: Vec<&ClassLikeMetadata> = std::iter::successors(
        class.direct_parent_class.and_then(|p| c.class_named(&p)),
        |k| k.direct_parent_class.and_then(|p| c.class_named(&p)),
    )
    .take(64)
    .collect();
    // getImmediateInterfaces(): the class's interfaces less those of its parents and those its interfaces extend
    let interfaces: Vec<&ClassLikeMetadata> = class
        .all_parent_interfaces
        .iter()
        .filter_map(|i| c.class_named(i))
        .collect();
    let indirect = |i: &ClassLikeMetadata| {
        ancestors
            .iter()
            .any(|p| p.all_parent_interfaces.contains(&i.name))
            || interfaces
                .iter()
                .any(|o| o.all_parent_interfaces.contains(&i.name))
    };
    for i in interfaces.iter().filter(|i| !indirect(i)) {
        if let Some(constant) = i.constants.get(&name) {
            return Some((constant, declaring(c, i, constant)));
        }
    }
    let parent = *ancestors.first()?;
    reflected_constant(c, parent, name).filter(|(k, _)| k.visibility != Visibility::Private)
}

/// `getDeclaringClass()` of a constant `k` has: the class-like whose declaration holds it, or the class using the
/// trait that does.
pub(super) fn declaring<'a>(
    c: &Checker<'a, '_>,
    k: &'a ClassLikeMetadata,
    constant: &ClassLikeConstantMetadata,
) -> &'a ClassLikeMetadata {
    let holds = |s: Span| {
        s.file_id == constant.span.file_id
            && s.start.offset <= constant.span.start.offset
            && constant.span.end.offset <= s.end.offset
    };
    // a trait's `used_traits` holds the traits it uses, recursively
    let in_traits = |k: &ClassLikeMetadata| {
        k.direct_trait_names()
            .filter_map(|t| c.class_named(&t))
            .any(|t| {
                holds(t.span)
                    || t.used_traits
                        .iter()
                        .filter_map(|u| c.class_named(u))
                        .any(|u| holds(u.span))
            })
    };
    std::iter::successors(Some(k), |k| {
        k.direct_parent_class.and_then(|p| c.class_named(&p))
    })
    .take(64)
    .chain(
        k.all_parent_interfaces
            .iter()
            .filter_map(|i| c.class_named(i)),
    )
    .find(|k| holds(k.span) || in_traits(k))
    .unwrap_or(k)
}
