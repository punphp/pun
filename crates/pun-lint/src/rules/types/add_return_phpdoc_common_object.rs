//! `lint/types/add-return-phpdoc-common-object`, ported from Rector 2.6.1's
//! `AddReturnDocblockForCommonObjectDenominatorRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a
//! function or method whose one `return` returns an array of objects gets `@return Parent[]` of the class or the
//! first parent class or interface they share.

use mago_syntax::cst::{Node, NodeKind};
use pun_phpstan::types::{ConstArray, Type, mixed};

use super::class_scope;
use super::docblock::{FunctionLike, doc_report, referenced_classes};
use super::facts::{Facts, facts};
use super::silent_void::Place;
use crate::rules::phpdoc_writer::type_mapper::{const_array_key_type, is_useful_array_tag, uniquate_types};
use crate::rules::phpdoc_writer::{decorate_return, expr_type};
use crate::{Context, Report, Rule};

pub struct AddReturnPhpdocCommonObject;

impl Rule for AddReturnPhpdocCommonObject {
    fn id(&self) -> &'static str {
        "lint/types/add-return-phpdoc-common-object"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        // definitely not an array return
        if f.has_non_array_return_type() {
            return;
        }
        let target = f.target(ctx.source);
        let mut info = target.info(ctx.source);
        if is_useful_array_tag(info.return_tag().map(|t| &t.ty)) {
            return;
        }
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let Some(value) = f.only_return_with_expr(&place).and_then(|r| r.value) else {
            return;
        };
        let mut inner = ancestors.to_vec();
        inner.push(node);
        let Some(Type::ConstArray(returned)) = expr_type(ctx, &inner, value) else {
            return;
        };
        let mut referenced = Vec::new();
        for value in &returned.values {
            // each item must refer some classes, and be an object
            let classes = referenced_classes(value);
            if classes.is_empty() || !is_object(value) {
                return;
            }
            referenced.extend(classes);
        }
        // nothing to find here
        if referenced.is_empty() {
            return;
        }
        let mut unique: Vec<String> = Vec::new();
        for class in &referenced {
            if !unique.contains(class) {
                unique.push(class.clone());
            }
        }
        let shared = if unique.len() == 1 {
            Some(unique.remove(0))
        } else {
            first_shared_parent(&facts, &referenced)
        };
        let Some(shared) = shared else {
            return;
        };
        let t = Type::array(resolve_key_type(&returned), Type::Object(shared, vec![]));
        if decorate_return(&mut info, &t) {
            out.extend(doc_report(
                f.name,
                "Add the `@return` array type of the returned objects' shared type.",
                ctx.source,
                &target,
                &info,
            ));
        }
    }
}

/// `$type->isObject()->yes()`.
fn is_object(t: &Type) -> bool {
    match t {
        Type::Object(..) | Type::EnumCase(..) | Type::This(_) | Type::ObjectWithoutClass => true,
        Type::Union(types) => types.iter().all(is_object),
        _ => false,
    }
}

/// `array_intersect(...$parentClassesAndInterfaces)[0]`: the first parent class (or, without one, the first
/// interface) of the first class, when every class has it too.
fn first_shared_parent(facts: &Facts, classes: &[String]) -> Option<String> {
    let lists: Vec<Vec<String>> = classes.iter().map(|c| parents_and_interfaces(facts, c)).collect();
    let first = lists.first()?.first()?;
    lists[1..]
        .iter()
        .all(|l| l.iter().any(|c| c.eq_ignore_ascii_case(first)))
        .then(|| first.clone())
}

/// `resolveParentClassesAndInterfaces()`: `getParentClassesNames()`, then the names of `getInterfaces()`.
fn parents_and_interfaces(facts: &Facts, class: &str) -> Vec<String> {
    let codebase = &facts.codebase;
    let mut out = Vec::new();
    let own = codebase.get_class_like(class.as_bytes());
    let mut current = own;
    while let Some(parent) = current.and_then(|c| c.direct_parent_class) {
        let parent_class = codebase.get_class_like(parent.as_bytes());
        out.push(parent_class.map_or(parent.to_string(), |c| c.original_name.to_string()));
        current = parent_class;
    }
    let mut interfaces: Vec<String> = Vec::new();
    for i in own.iter().flat_map(|own| own.direct_parent_interfaces.iter().chain(own.all_parent_interfaces.iter())) {
        let name = codebase
            .get_class_like(i.as_bytes())
            .map_or(i.to_string(), |c| c.original_name.to_string());
        if !interfaces.contains(&name) {
            interfaces.push(name);
        }
    }
    out.extend(interfaces);
    out
}

/// `resolveKeyType()`: of a union of keys, `string` or `int` when they are all one kind (or their union), else
/// `mixed`.
fn resolve_key_type(a: &ConstArray) -> Type {
    let Type::Union(keys) = const_array_key_type(a) else {
        return mixed();
    };
    // (a constant array's keys are integers and strings)
    let types: Vec<Type> = keys.iter().map(|key| if key.is_string().yes() { Type::String } else { Type::Int }).collect();
    let mut unique = uniquate_types(&types);
    if unique.len() == 1 {
        return unique.remove(0);
    }
    Type::Union(unique)
}
