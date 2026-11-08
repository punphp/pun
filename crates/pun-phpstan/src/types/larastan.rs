//! Larastan's string types: `GenericModelPropertyType` (`model property of T`) and `ViewStringType` (`view-string`).
//! Their checks ask the analysed app (a model's properties, a table's columns, the views) through
//! [`with_larastan_strings()`]; without an answer a name is taken as there.

use std::cell::Cell;

use super::{Accessory, StrictMixed, Tri, Type, accepts_with, is_super_type_of};

/// What the string types ask the app.
pub enum StringQuery<'a> {
    /// `$model->hasInstanceProperty($name)`
    Property(&'a Type, &'a str),
    /// `ModelPropertyHelper::hasDatabaseProperty($table, $column)`
    Column(&'a str, &'a str),
    /// `view()->exists($name)`
    View(&'a str),
}

type Answer = dyn Fn(StringQuery<'_>) -> Tri;

thread_local! {
    static ANSWER: Cell<Option<*const Answer>> = const { Cell::new(None) };
}

/// Runs `f` with `answer` answering the string types' questions on this thread.
pub fn with_larastan_strings<R>(
    answer: &dyn Fn(StringQuery<'_>) -> Tri,
    f: impl FnOnce() -> R,
) -> R {
    struct Restore(Option<*const Answer>);
    impl Drop for Restore {
        fn drop(&mut self) {
            ANSWER.with(|a| a.set(self.0));
        }
    }
    let ptr: *const (dyn Fn(StringQuery<'_>) -> Tri + '_) = answer;
    #[expect(
        clippy::transmute_ptr_to_ptr,
        reason = "erases the trait object's lifetime, which `as` cannot"
    )]
    // SAFETY: as in `symbols::with_classes()`: only the lifetime is erased, and `Restore` puts the previous answer
    // back before this function returns, also on unwinding.
    let ptr: *const Answer = unsafe { std::mem::transmute(ptr) };
    let _restore = Restore(ANSWER.with(|a| a.replace(Some(ptr))));
    f()
}

fn ask(query: StringQuery<'_>) -> Tri {
    let Some(answer) = ANSWER.with(Cell::get) else {
        return Tri::Yes;
    };
    // SAFETY: set by `with_larastan_strings()`, whose borrow outlives every call made while it is set.
    let answer = unsafe { &*answer };
    answer(query)
}

/// The `model property of` type `t` is (or has as an intersection member).
fn model_property(t: &Type) -> Option<&Type> {
    match t {
        Type::Accessory(Accessory::ModelProperty(m)) => Some(m),
        Type::Intersection(ts) => ts.iter().find_map(model_property),
        _ => None,
    }
}

fn is_view_string(t: &Type) -> bool {
    match t {
        Type::Accessory(Accessory::ViewString) => true,
        Type::Intersection(ts) => ts.iter().any(is_view_string),
        _ => false,
    }
}

/// `count($type->getConstantStrings()) === 1`
fn single_constant(t: &Type) -> Option<String> {
    match t.constant_strings().as_slice() {
        [one] => Some(String::from_utf8_lossy(one).into_owned()),
        _ => None,
    }
}

/// `GenericModelPropertyType::accepts()`, before the compound given type answers itself (Pun's `model property
/// of` is an intersection, which PHPStan's own class is not).
pub(super) fn model_property_accepts(
    t: &Type,
    b: &Type,
    strict: bool,
    compound: impl FnOnce() -> Tri,
) -> Tri {
    if let Some(given) = model_property(b) {
        return accepts_with(t, given, strict, StrictMixed::default());
    }
    if b.is_compound() {
        return compound();
    }
    if let Some(given) = single_constant(b) {
        if matches!(t, Type::Mixed { .. }) || t.object_class_names().is_empty() {
            return Tri::Yes;
        }
        let given = given.split(" as ").next().unwrap_or_default();
        if given.contains('.') {
            return match given.split('.').collect::<Vec<_>>().as_slice() {
                [table, column] => ask(StringQuery::Column(table, column)),
                _ => Tri::Yes,
            };
        }
        return ask(StringQuery::Property(t, given));
    }
    // an arbitrary string cannot be checked
    if b.is_string().yes() {
        Tri::Yes
    } else {
        Tri::No
    }
}

/// `GenericModelPropertyType::isSuperTypeOf()`
pub(super) fn model_property_super(t: &Type, b: &Type) -> Tri {
    if let Some(given) = single_constant(b) {
        return Tri::from(ask(StringQuery::Property(t, &given)).yes());
    }
    match b {
        Type::Accessory(Accessory::ModelProperty(given)) => is_super_type_of(t, given),
        _ => Tri::No,
    }
}

/// `ViewStringType::accepts()`
pub(super) fn view_string_accepts(b: &Type, compound: impl FnOnce() -> Tri) -> Tri {
    if b.is_compound() && !is_view_string(b) {
        return compound();
    }
    view_string_super(b)
}

/// `ViewStringType::isSuperTypeOf()`
pub(super) fn view_string_super(b: &Type) -> Tri {
    if let Some(given) = single_constant(b) {
        return ask(StringQuery::View(&given));
    }
    if is_view_string(b) {
        Tri::Yes
    } else if b.is_string().yes() {
        Tri::Maybe
    } else {
        Tri::No
    }
}
