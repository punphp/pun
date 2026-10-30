use mago_php_version::PHPVersion;
use mago_php_version::PHPVersionRange;

use mago_span::HasSpan;
use mago_span::Span;
use mago_word::Word;

use crate::metadata::attribute::AttributeMetadata;
use crate::metadata::flags::MetadataFlags;
use crate::metadata::ttype::TypeMetadata;
use crate::metadata::version_constraint::VersionConstraint;
use crate::ttype::get_mixed;
use crate::ttype::union::TUnion;
use crate::visibility::Visibility;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct ClassLikeConstantMetadata {
    pub attributes: Vec<AttributeMetadata>,
    pub name: Word,
    pub span: Span,
    pub visibility: Visibility,
    pub type_declaration: Option<TypeMetadata>,
    pub type_metadata: Option<TypeMetadata>,
    pub inferred_type: Option<TUnion>,
    pub flags: MetadataFlags,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) explicit_docblock_flags: MetadataFlags,
    pub version_constraint: VersionConstraint,
}

impl ClassLikeConstantMetadata {
    #[must_use]
    pub fn new(name: Word, span: Span, visibility: Visibility, flags: MetadataFlags) -> Self {
        Self {
            attributes: Vec::new(),
            name,
            span,
            visibility,
            type_declaration: None,
            type_metadata: None,
            inferred_type: None,
            flags,
            explicit_docblock_flags: MetadataFlags::empty(),
            version_constraint: VersionConstraint::unconstrained(),
        }
    }

    pub fn set_type_declaration(&mut self, type_declaration: TypeMetadata) {
        if self.type_metadata.is_none() {
            self.type_metadata = Some(type_declaration.clone());
        }

        self.type_declaration = Some(type_declaration);
    }

    /// Exact reads use the raw initializer, independently of native/PHPDoc declarations. An overridable
    /// receiver instead exposes the effective declaration type, or mixed for an untyped non-final constant.
    #[must_use]
    pub fn value_type(&self, late_bound: bool) -> TUnion {
        if late_bound {
            if let Some(metadata) = &self.type_metadata {
                return metadata.type_union.clone();
            }
            if !self.flags.is_final() {
                return get_mixed();
            }
        }
        self.inferred_type
            .clone()
            .or_else(|| self.type_metadata.as_ref().map(|metadata| metadata.type_union.clone()))
            .unwrap_or_else(get_mixed)
    }

    /// Returns `true` when this class constant is available in the given PHP
    /// version.
    #[inline]
    #[must_use]
    pub fn is_available_in_version(&self, version: PHPVersion) -> bool {
        self.version_constraint.allows_version(version)
    }

    /// Returns `true` when this class constant is available across the entire
    /// supplied [`PHPVersionRange`].
    #[inline]
    #[must_use]
    pub fn is_available_in_version_range(&self, range: PHPVersionRange) -> bool {
        self.version_constraint.allows_version_range(range)
    }
}

impl HasSpan for ClassLikeConstantMetadata {
    fn span(&self) -> Span {
        self.span
    }
}
