//! One tag of a doc comment as phpstan/phpdoc-parser gives it to Rector (`PhpDocTagNode`), read with Mago's phpdoc
//! parser: the child's text, wrapped in a doc comment of its own, parses to a tag whose value is the typed value
//! phpdoc-parser builds (`VarTagValueNode`, `ParamTagValueNode`, `ReturnTagValueNode`), or an invalid one
//! (`InvalidTagValueNode`) where the type does not parse.

use mago_phpdoc_syntax::PHPDocParser;
use mago_phpdoc_syntax::cst::{Tag, TagValue};

/// `f` on the first tag of `text` (one child of a doc comment, from its `@`), in a doc comment of its own; `None`
/// when `text` holds no tag. Spans in the tag are offsets into `/** {text} */`, so 4 bytes after `text`'s.
pub(crate) fn with_tag<R>(text: &str, f: impl FnOnce(&Tag<'_>) -> R) -> Option<R> {
    let arena = mago_allocator::LocalArena::new();
    let content = format!("/** {text} */");
    let content: &[u8] = mago_allocator::Arena::alloc_slice_copy(&arena, content.as_bytes());
    let document = PHPDocParser::parse(&arena, mago_database::file::FileId::zero(), content);
    document.tags().next().map(f)
}

/// Whether the tag is named exactly `name` (without its `@`), with no vendor prefix: phpdoc-parser keeps the name as
/// written and Rector compares it as is.
pub(crate) fn named(tag: &Tag<'_>, name: &str) -> bool {
    tag.vendor.is_none() && tag.name.value == name.as_bytes()
}

/// Whether the tag's value parsed (no `InvalidTagValueNode`).
pub(crate) fn is_valid(tag: &Tag<'_>) -> bool {
    !matches!(tag.value, TagValue::Invalid(_))
}
