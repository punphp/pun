//! `VerbosityLevel::getRecommendedLevelByType()`: how precisely a message describes an accepting and an accepted type,
//! so that two different types never describe the same.

use std::cell::Cell;

use super::{Accessory, Level, Type, symbols};

#[derive(Default)]
struct Flags {
    more: Cell<bool>,
    very: Cell<bool>,
}

impl Flags {
    fn visit(&self, t: &Type) {
        if self.very.get() {
            return;
        }
        let children = || {
            t.traverse(&|i| {
                self.visit(i);
                i.clone()
            });
        };
        if t.is_callable().yes() {
            self.more.set(true);
            // ponytail: a ClosureType's isStaticClosure() is maybe here (PHPDoc closures); a closure expression's
            // known staticness makes it precise
            children();
            return;
        }
        if t.is_constant_array().yes() {
            self.more.set(true);
            children();
            return;
        }
        if t.is_constant_value().yes() && t.is_null().no() {
            self.more.set(true);
            if !t.is_array().no() {
                children();
            }
            return;
        }
        match t {
            Type::Accessory(Accessory::LowercaseString | Accessory::UppercaseString) => {
                self.more.set(true);
                self.very.set(true);
            }
            // Larastan's GenericModelPropertyType and ViewStringType are StringType subclasses, no accessories
            Type::Accessory(Accessory::ModelProperty(_) | Accessory::ViewString) => children(),
            Type::Accessory(_) | Type::IntRange(..) => self.more.set(true),
            _ => children(),
        }
    }
}

/// A generic object of a class with an invariant template.
fn invariant_template(t: &Type) -> bool {
    let found = Cell::new(false);
    fn go(t: &Type, found: &Cell<bool>) {
        if found.get() {
            return;
        }
        if let Type::Object(class, args, _) | Type::Static(class, args) = t
            && !args.is_empty()
            // ponytail: the symbols carry no template variance, so every class template counts as invariant
            && symbols::lookup(class).is_some_and(|c| !c.templates.is_empty())
        {
            found.set(true);
            return;
        }
        t.traverse(&|i| {
            go(i, found);
            i.clone()
        });
    }
    go(t, &found);
    found.get()
}

pub fn recommended_level(accepting: &Type, accepted: Option<&Type>) -> Level {
    let f = Flags::default();
    f.visit(accepting);
    if f.very.get() {
        return Level::Precise;
    }
    let verbosity = f.more.get().then_some(Level::Value);
    let Some(accepted) = accepted else {
        return verbosity.unwrap_or(Level::TypeOnly);
    };
    if !invariant_template(accepting) {
        return verbosity.unwrap_or(Level::TypeOnly);
    }
    let f = Flags::default();
    f.visit(accepted);
    if f.very.get() {
        return Level::Precise;
    }
    if f.more.get() {
        Level::Value
    } else {
        verbosity.unwrap_or(Level::TypeOnly)
    }
}
