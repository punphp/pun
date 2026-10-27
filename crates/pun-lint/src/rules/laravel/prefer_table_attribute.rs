//! `lint/laravel/prefer-table-attribute`, ported from rector-laravel 2.5.0's `TablePropertyToTableAttributeRector`
//! (src/Rector/Class_) and `TableAttributeFactory` (src/NodeFactory).
//!
//! A model's `protected $table = '...'` (or, when the class has `#[Table]` already, that attribute's `name:`) becomes
//! `#[\Illuminate\Database\Eloquent\Attributes\Table(name: ..., key: ..., keyType: ..., incrementing: ...)]`, with the
//! defaults of the protected `$primaryKey`, `$keyType` and `$incrementing` properties (or the existing attribute's
//! arguments of those names, which the properties override). The properties it reads go; an existing `#[Table]`
//! group is replaced in place, the other attributes of that group with it.

use mago_syntax::cst::{Expression, Node, NodeKind, PartialArgument};

use super::property_attribute::{ClassView, MODEL, is_string_node, moved_value, report, rewrite};
use crate::rules::rector_batch5::resolve_class_name;
use crate::{Context, Report, Rule};

const TABLE: &str = "Illuminate\\Database\\Eloquent\\Attributes\\Table";

pub struct PreferTableAttribute;

impl Rule for PreferTableAttribute {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-table-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let class = ClassView::of(node).unwrap();
        if !class.is_object_type(ctx, ancestors, MODEL) {
            return;
        }
        let has_existing = class.has_attribute(ancestors, TABLE);
        // Resolve the table value: from $table property, or from existing attribute
        let table_property = class.property("table");
        let mut table_value = None;
        if let Some((_, property)) = &table_property {
            if !property.is_protected() {
                return;
            }
            match property.default() {
                Some(v) if is_string_node(v) => table_value = Some(v),
                _ => return,
            }
        }
        if table_value.is_none() && has_existing {
            match existing_arg(&class, ancestors, "name") {
                Some(v) if is_string_node(v) => table_value = Some(v),
                _ => return,
            }
        }
        let Some(table_value) = table_value else {
            return;
        };
        // Seed options from existing attribute args (properties take precedence below)
        let mut options = Vec::new();
        if has_existing {
            for name in ["key", "keyType", "incrementing"] {
                if let Some(arg) = existing_arg(&class, ancestors, name) {
                    set(&mut options, name, arg);
                }
            }
        }
        let mut removed: Vec<usize> = table_property.iter().map(|(i, _)| *i).collect();
        let mut any_removed = table_property.is_some();
        for (property, option) in [
            ("primaryKey", "key"),
            ("keyType", "keyType"),
            ("incrementing", "incrementing"),
        ] {
            if let Some((index, p)) = class.property(property)
                && p.is_protected()
                && let Some(value) = p.default()
            {
                set(&mut options, option, value);
                removed.push(index);
                any_removed = true;
            }
        }
        // If the attribute exists but there are no properties to remove, there is nothing to do
        if has_existing && !any_removed {
            return;
        }
        let mut args = vec![format!(
            "name: {}",
            moved_value(ctx.source, &class, table_value)
        )];
        args.extend(
            options
                .iter()
                .map(|(name, value)| format!("{name}: {}", moved_value(ctx.source, &class, value))),
        );
        let group = format!("#[\\{TABLE}({})]", args.join(", "));
        let replaced: Vec<usize> = class
            .attribute_lists
            .iter()
            .position(|list| {
                list.attributes
                    .iter()
                    .any(|a| resolve_class_name(ancestors, &a.name).eq_ignore_ascii_case(TABLE))
            })
            .into_iter()
            .collect();
        let Some(edits) = rewrite(ctx.source, &class, &[group], &replaced, &removed) else {
            return;
        };
        out.push(report(
            &class,
            "Changes model table-related properties to use the Table attribute.",
            edits,
        ));
    }
}

/// `$options[$name] = $value`: a new name goes last, a known one keeps its place.
fn set<T>(options: &mut Vec<(&'static str, T)>, name: &'static str, value: T) {
    match options.iter_mut().find(|(n, _)| *n == name) {
        Some(option) => option.1 = value,
        None => options.push((name, value)),
    }
}

/// `getExistingAttributeArg()`: the value of the first argument named `name` of a `#[Table]` attribute.
fn existing_arg<'ast, 'arena>(
    class: &ClassView<'ast, 'arena>,
    ancestors: &[Node<'_, '_>],
    name: &str,
) -> Option<&'ast Expression<'arena>> {
    class.attribute_lists.iter().find_map(|list| {
        list.attributes
            .iter()
            .filter(|a| resolve_class_name(ancestors, &a.name).eq_ignore_ascii_case(TABLE))
            .find_map(|a| {
                a.argument_list
                    .as_ref()?
                    .arguments
                    .iter()
                    .find_map(|arg| match arg {
                        PartialArgument::Named(n) if n.name.value == name.as_bytes() => {
                            Some(crate::syntax::unparenthesize(n.value))
                        }
                        _ => None,
                    })
            })
    })
}
