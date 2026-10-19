//! Mago 1.57.0's variable-usage walker (mago-linter src/rule/utils/variable_usage.rs) as its `UsageCollector`
//! sees it, for the rules ported from Mago's `no-unused-global` and `no-unused-static`: every direct variable a
//! function body names, apart from `global` and `static` declarations, the bodies of nested functions, classes and
//! closures (whose `use` list does count), and the parameters of nested arrow functions. `compact('name')` names
//! `$name`; a variable variable or an `extract()` call makes the result unknown.

use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

/// The direct variable names a body references, or `None` when it can't be known.
pub(crate) fn used_names<'arena>(
    body: &Block<'arena>,
    interest: &[&'arena [u8]],
) -> Option<Vec<&'arena [u8]>> {
    let mut walker = Usage {
        interest: interest.to_vec(),
        referenced: Vec::new(),
        bailed: false,
        excluded: Vec::new(),
    };
    walker.walk_block(body, &mut ());
    (!walker.bailed).then_some(walker.referenced)
}

/// The parts of a function, method or closure Mago's `function_like_parts` returns: its body, if it has one.
pub(crate) fn function_like_body<'ast, 'arena>(
    node: Node<'ast, 'arena>,
) -> Option<&'ast Block<'arena>> {
    match node {
        Node::Function(f) => Some(&f.body),
        Node::Method(m) => match &m.body {
            MethodBody::Concrete(b) => Some(b),
            MethodBody::Abstract(_) => None,
        },
        Node::Closure(c) => Some(&c.body),
        _ => None,
    }
}

/// Mago's `is_silenced_name`: `$_` and names that start with `$_`.
pub(crate) fn is_silenced_name(name: &[u8]) -> bool {
    name.strip_prefix(b"$").unwrap_or(name).starts_with(b"_")
}

struct Usage<'arena> {
    interest: Vec<&'arena [u8]>,
    referenced: Vec<&'arena [u8]>,
    bailed: bool,
    excluded: Vec<&'arena [u8]>,
}

impl<'arena> Usage<'arena> {
    fn reference(&mut self, name: &'arena [u8]) {
        if !self.referenced.contains(&name) {
            self.referenced.push(name);
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Usage<'arena> {
    fn walk_in_direct_variable(&mut self, d: &'ast DirectVariable<'arena>, _: &mut ()) {
        if !self.excluded.contains(&d.name) {
            self.reference(d.name);
        }
    }

    fn walk_in_indirect_variable(&mut self, _: &'ast IndirectVariable<'arena>, _: &mut ()) {
        self.bailed = true;
    }

    fn walk_in_nested_variable(&mut self, _: &'ast NestedVariable<'arena>, _: &mut ()) {
        self.bailed = true;
    }

    fn walk_global(&mut self, g: &'ast Global<'arena>, _: &mut ()) {
        if g.variables
            .iter()
            .any(|v| !matches!(v, Variable::Direct(_)))
        {
            self.bailed = true;
        }
    }

    fn walk_static(&mut self, s: &'ast Static<'arena>, ctx: &mut ()) {
        for item in s.items.iter() {
            if let StaticItem::Concrete(c) = item {
                self.walk_expression(c.value, ctx);
            }
        }
    }

    fn walk_function_call(&mut self, fc: &'ast FunctionCall<'arena>, ctx: &mut ()) {
        let local = match fc.function {
            Expression::Identifier(Identifier::Local(local)) => local.value,
            _ => b"",
        };
        if local.eq_ignore_ascii_case(b"extract") {
            self.bailed = true;
            return;
        }
        if local.eq_ignore_ascii_case(b"compact") {
            for arg in fc.argument_list.arguments.iter() {
                let Expression::Literal(Literal::String(lit)) = arg.value() else {
                    continue;
                };
                let raw = lit.raw;
                if raw.len() <= 2 {
                    continue;
                }
                let inner = &raw[1..raw.len() - 1];
                for key in self.interest.clone() {
                    if key.strip_prefix(b"$") == Some(inner) {
                        self.reference(key);
                    }
                }
            }
            return;
        }
        self.walk_expression(fc.function, ctx);
        self.walk_argument_list(&fc.argument_list, ctx);
    }

    fn walk_closure(&mut self, c: &'ast Closure<'arena>, _: &mut ()) {
        if let Some(use_clause) = &c.use_clause {
            for cap in use_clause.variables.iter() {
                self.reference(cap.variable.name);
            }
        }
    }

    fn walk_arrow_function(&mut self, a: &'ast ArrowFunction<'arena>, ctx: &mut ()) {
        let saved = self.excluded.len();
        self.excluded
            .extend(a.parameter_list.parameters.iter().map(|p| p.variable.name));
        self.walk_expression(a.expression, ctx);
        self.excluded.truncate(saved);
    }

    fn walk_anonymous_class(&mut self, class: &'ast AnonymousClass<'arena>, ctx: &mut ()) {
        let Some(arguments) = &class.argument_list else {
            return;
        };
        for argument in arguments.arguments.iter() {
            let value = match argument {
                PartialArgument::Positional(p) => p.value,
                PartialArgument::Named(n) => n.value,
                _ => continue,
            };
            self.walk_expression(value, ctx);
        }
    }

    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
}

/// The `global` or `static` declarations of a body, outside nested functions and classes: each declared name with its
/// span.
pub(crate) fn declarations<'arena>(
    body: &Block<'arena>,
    statics: bool,
) -> Vec<(&'arena [u8], mago_span::Span)> {
    let mut collector = Declarations {
        statics,
        items: Vec::new(),
    };
    collector.walk_block(body, &mut ());
    collector.items
}

struct Declarations<'arena> {
    statics: bool,
    items: Vec<(&'arena [u8], mago_span::Span)>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Declarations<'arena> {
    fn walk_global(&mut self, g: &'ast Global<'arena>, _: &mut ()) {
        if self.statics {
            return;
        }
        for v in g.variables.iter() {
            if let Variable::Direct(d) = v {
                self.items.push((d.name, d.span));
            }
        }
    }

    fn walk_static(&mut self, s: &'ast Static<'arena>, _: &mut ()) {
        if !self.statics {
            return;
        }
        for item in s.items.iter() {
            let var = item.variable();
            self.items.push((var.name, var.span));
        }
    }

    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
}
