//! Larastan's CollectionHelper: a model's own Eloquent collection class (`#[CollectedBy]`, `newCollection()`), and
//! ModelRelationsExtension's relation properties, whose collections it replaces.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::expander::{StaticClassType, TypeExpansionOptions, expand_union};
use mago_codex::misc::GenericParent;
use mago_codex::ttype::{TType, TypeRef};
use mago_codex::ttype::template::{TemplateResult, inferred_type_replacer};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{get_int, get_specialized_template_type};
use mago_word::{Word, word};

pub(crate) const ELOQUENT_COLLECTION: &[u8] = b"Illuminate\\Database\\Eloquent\\Collection";
const RELATION: &[u8] = b"Illuminate\\Database\\Eloquent\\Relations\\Relation";
const COLLECTED_BY: &[u8] = b"Illuminate\\Database\\Eloquent\\Attributes\\CollectedBy";

fn named(class: Word, parameters: Option<Vec<TUnion>>) -> TAtomic {
    TAtomic::Object(TObject::Named(TNamedObject::new_with_type_parameters(class, parameters)))
}

/// `determineCollectionClassName()`: the class `#[CollectedBy]` names, else the one `newCollection()` returns.
fn collection_class_name(codebase: &CodebaseMetadata, model: &[u8]) -> Word {
    let fallback = || word(ELOQUENT_COLLECTION);
    let Some(k) = codebase.get_class_like(model) else { return fallback() };
    if let Some(attribute) = k.attributes.iter().find(|a| a.name.as_bytes().eq_ignore_ascii_case(COLLECTED_BY))
        && let Some(TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value }))) =
            attribute.arguments.first().and_then(|a| a.value_type.as_ref()).and_then(|t| t.types.first())
    {
        return *value;
    }
    let returned = codebase
        .get_declaring_method(model, b"newCollection")
        .and_then(|f| f.return_type_metadata.as_ref())
        .map(|t| &t.type_union);
    match returned.map(|t| t.types.as_ref()) {
        Some([TAtomic::Object(TObject::Named(n))]) => n.name,
        _ => fallback(),
    }
}

/// `determineCollectionClass()`: the model's collection, given its key and value when it is generic over both, its
/// value when its one template is `TModel`.
pub(crate) fn collection_class(codebase: &CodebaseMetadata, model: &[u8]) -> TAtomic {
    let class = collection_class_name(codebase, model);
    let Some(k) = codebase.get_class_like(class.as_bytes()) else { return named(class, None) };
    let model = codebase.get_class_like(model).map_or_else(|| word(model), |m| m.original_name);
    let value = TUnion::from_atomic(named(model, None));
    let parameters = match k.template_types.len() {
        2 => Some(vec![get_int(), value]),
        1 if k.template_types.contains_key(&word("TModel")) => Some(vec![value]),
        _ => None,
    };
    named(k.original_name, parameters)
}

/// `getObjectClassNames()` of a type whose parts are objects.
fn object_class_names(ty: &TUnion) -> Vec<Word> {
    ty.types
        .iter()
        .filter_map(|t| match t {
            TAtomic::Object(TObject::Named(n)) => Some(n.name),
            _ => None,
        })
        .collect()
}

/// `getTemplateType(Collection::class, 'TModel')->getObjectClassNames()` of an Eloquent collection type.
fn collection_models(codebase: &CodebaseMetadata, n: &TNamedObject) -> Option<Vec<Word>> {
    let class = n.name.as_bytes();
    if !class.eq_ignore_ascii_case(ELOQUENT_COLLECTION) && !codebase.is_instance_of(class, ELOQUENT_COLLECTION) {
        return None;
    }
    let k = codebase.get_class_like(class)?;
    let models = get_specialized_template_type(
        codebase,
        word("TModel"),
        codebase.get_class_like(ELOQUENT_COLLECTION)?.name,
        k,
        n.type_parameters.as_deref(),
    )?;
    Some(object_class_names(&models))
}

/// Whether the type names `Illuminate\Database\Eloquent\Collection` (`getReferencedClasses()`).
pub(crate) fn references_collection(ty: &TUnion) -> bool {
    ty.types.iter().any(|t| match t {
        TAtomic::Object(TObject::Named(n)) => {
            n.name.as_bytes().eq_ignore_ascii_case(ELOQUENT_COLLECTION)
                || n.type_parameters.iter().flatten().any(references_collection)
        }
        _ => false,
    })
}

/// `replaceCollectionsInType()`: each Eloquent collection of models becomes the models' own collections.
pub(crate) fn replace_collections(codebase: &CodebaseMetadata, ty: &TUnion) -> TUnion {
    if !references_collection(ty) {
        return ty.clone();
    }
    let mut types = Vec::new();
    for t in ty.types.iter() {
        let TAtomic::Object(TObject::Named(n)) = t else {
            types.push(t.clone());
            continue;
        };
        match collection_models(codebase, n) {
            Some(models) if !models.is_empty() => {
                types.extend(models.iter().map(|m| collection_class(codebase, m.as_bytes())));
            }
            Some(_) => types.push(t.clone()),
            None => {
                let mut n = n.clone();
                if let Some(parameters) = n.type_parameters.as_mut() {
                    for p in parameters.iter_mut() {
                        *p = replace_collections(codebase, p);
                    }
                }
                types.push(TAtomic::Object(TObject::Named(n)));
            }
        }
    }
    TUnion::from_vec(types)
}

/// Whether every part of a declared return type is a relation.
pub(crate) fn is_relation(codebase: &CodebaseMetadata, ty: &TUnion) -> bool {
    !ty.types.is_empty()
        && ty.types.iter().all(|a| {
            matches!(a, TAtomic::Object(TObject::Named(n)) if codebase.is_instance_of(n.name.as_bytes(), RELATION))
        })
}

/// ModelRelationsExtension::getProperty(): a relation method's `TResult`, its collections the related models' own.
pub(crate) fn relation_property(codebase: &CodebaseMetadata, object: &TNamedObject, property: &str) -> Option<TUnion> {
    let class = object.name.as_bytes();
    let method = codebase.get_declaring_method(class, property.as_bytes())?;
    let declared = &method.return_type_metadata.as_ref()?.type_union;
    if !is_relation(codebase, declared) {
        return None;
    }
    let mut returned = declared.clone();
    let options = TypeExpansionOptions {
        self_class: Some(codebase.get_class_like(class)?.name),
        static_class_type: StaticClassType::Object(TObject::Named(object.clone())),
        ..Default::default()
    };
    expand_union(codebase, &mut returned, &options);
    let relation = codebase.get_class_like(RELATION)?.name;
    let mut types = Vec::new();
    for t in returned.types.iter() {
        let TAtomic::Object(TObject::Named(n)) = t else {
            types.push(t.clone());
            continue;
        };
        let k = codebase.get_class_like(n.name.as_bytes())?;
        let mut result =
            get_specialized_template_type(codebase, word("TResult"), relation, k, n.type_parameters.as_deref())?;
        // `getTemplateType()` through every ancestor: an intermediate class's templates (BelongsToMany's, under
        // MorphToMany) are the relation's own once more
        for _ in 0..4 {
            let pending: Vec<_> = result
                .get_all_child_nodes()
                .into_iter()
                .filter_map(|node| match node {
                    TypeRef::Atomic(TAtomic::GenericParameter(p)) => match p.defining_entity {
                        GenericParent::ClassLike(class) if class != k.name && class != relation => Some((p.parameter_name, class)),
                        _ => None,
                    },
                    _ => None,
                })
                .collect();
            if pending.is_empty() {
                break;
            }
            let mut templates = TemplateResult::default();
            for (name, class) in pending {
                if let Some(t) = get_specialized_template_type(codebase, name, class, k, n.type_parameters.as_deref()) {
                    templates.add_lower_bound(name, GenericParent::ClassLike(class), t);
                }
            }
            result = inferred_type_replacer::replace(&result, &templates, codebase);
        }
        types.extend(result.types.iter().cloned());
    }
    Some(replace_collections(codebase, &TUnion::from_vec(types)))
}

/// HigherOrderCollectionProxyHelper::determineReturnType(): the result of the collection method `name` proxied over
/// `value`, whose method or property gives `returned`; `None` for its ErrorType.
pub(crate) fn proxy_result(
    codebase: &CodebaseMetadata,
    name: &str,
    value: &TUnion,
    returned: impl FnOnce() -> Option<TUnion>,
    collection: &TUnion,
) -> Option<TUnion> {
    use mago_codex::ttype::comparator::{ComparisonResult, union_comparator};
    let collection_class = match collection.types.as_ref() {
        [TAtomic::Object(TObject::Named(c))] => c.name,
        _ => word(ELOQUENT_COLLECTION),
    };
    let generic = |class: Word, key: TUnion, value: TUnion| -> Option<TUnion> {
        let k = codebase.get_class_like(class.as_bytes())?;
        let parameters = match k.template_types.len() {
            2 => Some(vec![key, value]),
            1 if k.template_types.contains_key(&word("TModel")) => Some(vec![value]),
            _ => None,
        };
        Some(TUnion::from_atomic(named(k.original_name, parameters)))
    };
    let benevolent = || {
        let mut t = mago_codex::ttype::get_int_or_string();
        t.set_ignore_nullable_issues(true);
        t.set_ignore_falsable_issues(true);
        t
    };
    let support = word("Illuminate\\Support\\Collection");
    match name {
        "average" | "avg" => Some(mago_codex::ttype::get_float()),
        "contains" | "every" | "some" => Some(mago_codex::ttype::get_bool()),
        "each" | "filter" | "reject" | "skipUntil" | "skipWhile" | "sortBy" | "sortByDesc" | "takeUntil"
        | "takeWhile" | "unique" => generic(collection_class, get_int(), value.clone()),
        "keyBy" => generic(collection_class, benevolent(), value.clone()),
        "first" => Some(TUnion::from_vec(value.types.iter().cloned().chain([TAtomic::Null]).collect())),
        "flatMap" => generic(support, get_int(), mago_codex::ttype::get_mixed()),
        "groupBy" | "partition" => generic(collection_class, get_int(), generic(collection_class, get_int(), value.clone())?),
        "map" => generic(support, benevolent(), returned()?),
        "max" | "min" => returned(),
        "sum" => union_comparator::is_contained_by(codebase, &get_int(), &returned()?, false, false, true, &mut ComparisonResult::new())
            .then(get_int),
        _ => None,
    }
}
