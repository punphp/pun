use std::sync::Arc;

use foldhash::HashMap;
use mago_database::file::FileId;
use mago_span::Span;

use super::function_like::FunctionLikeMetadata;
use super::shared::Shared;
use crate::symbol::SymbolIdentifier;

type Entries = HashMap<SymbolIdentifier, Box<FunctionLikeMetadata>>;

#[derive(Clone, Debug, PartialEq)]
struct FileClosures {
    file: FileId,
    entries: Entries,
}

/// An effective function-like table: shared populated declarations plus fresh file closures.
/// Lookup and iteration both hide every replaced file's old anonymous declarations.
#[derive(Clone, Debug, Default)]
pub struct FunctionLikeTable {
    base: Shared<Entries>,
    files: Vec<Arc<FileClosures>>,
    // Ephemeral ownership coordinates for analysis of edited bodies. Populated
    // named signatures remain shared; this is not part of the persisted table.
    declaration_spans: Shared<HashMap<Span, Span>>,
}

impl PartialEq for FunctionLikeTable {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|(key, metadata)| other.get(key) == Some(metadata))
    }
}

impl std::ops::Index<&SymbolIdentifier> for FunctionLikeTable {
    type Output = Box<FunctionLikeMetadata>;

    fn index(&self, key: &SymbolIdentifier) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl FunctionLikeTable {
    pub(super) fn replace_closures(&mut self, file: FileId, entries: Entries, declaration_spans: HashMap<Span, Span>) {
        self.files.retain(|overlay| overlay.file != file);
        self.files.push(Arc::new(FileClosures { file, entries }));
        self.declaration_spans.retain(|span, _| span.file_id != file);
        self.declaration_spans.extend(declaration_spans);
    }

    pub(super) fn declaration_span(&self, span: Span) -> Span {
        self.declaration_spans.get(&span).copied().unwrap_or(span)
    }

    fn hides(&self, metadata: &FunctionLikeMetadata) -> bool {
        (metadata.kind.is_closure() || metadata.kind.is_arrow_function())
            && self.files.iter().any(|overlay| overlay.file == metadata.span.file_id)
    }

    pub fn get(&self, key: &SymbolIdentifier) -> Option<&Box<FunctionLikeMetadata>> {
        self.files
            .iter()
            .find_map(|overlay| overlay.entries.get(key))
            .or_else(|| self.base.get(key).filter(|metadata| !self.hides(metadata)))
    }

    pub fn contains_key(&self, key: &SymbolIdentifier) -> bool {
        self.get(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SymbolIdentifier, &Box<FunctionLikeMetadata>)> {
        self.files
            .iter()
            .flat_map(|overlay| overlay.entries.iter())
            .chain(self.base.iter().filter(|(_, metadata)| !self.hides(metadata)))
    }

    pub fn keys(&self) -> impl Iterator<Item = &SymbolIdentifier> {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl Iterator<Item = &Box<FunctionLikeMetadata>> {
        self.iter().map(|(_, metadata)| metadata)
    }

    pub fn len(&self) -> usize {
        if self.files.is_empty() {
            self.base.len()
        } else {
            self.iter().count()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    pub fn get_mut(&mut self, key: &SymbolIdentifier) -> Option<&mut Box<FunctionLikeMetadata>> {
        self.materialize();
        self.base.get_mut(key)
    }

    pub fn insert(
        &mut self,
        key: SymbolIdentifier,
        value: Box<FunctionLikeMetadata>,
    ) -> Option<Box<FunctionLikeMetadata>> {
        self.materialize();
        self.base.insert(key, value)
    }

    pub fn remove(&mut self, key: &SymbolIdentifier) -> Option<Box<FunctionLikeMetadata>> {
        self.materialize();
        self.base.remove(key)
    }

    pub fn entry(
        &mut self,
        key: SymbolIdentifier,
    ) -> std::collections::hash_map::Entry<'_, SymbolIdentifier, Box<FunctionLikeMetadata>> {
        self.materialize();
        self.base.entry(key)
    }

    pub fn retain(&mut self, f: impl FnMut(&SymbolIdentifier, &mut Box<FunctionLikeMetadata>) -> bool) {
        self.materialize();
        self.base.retain(f);
    }

    pub fn extend(&mut self, entries: impl IntoIterator<Item = (SymbolIdentifier, Box<FunctionLikeMetadata>)>) {
        self.materialize();
        self.base.extend(entries);
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&SymbolIdentifier, &mut Box<FunctionLikeMetadata>)> {
        self.materialize();
        self.base.iter_mut()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Box<FunctionLikeMetadata>> {
        self.iter_mut().map(|(_, value)| value)
    }

    pub fn into_inner(mut self) -> Entries {
        self.materialize();
        self.base.into_inner()
    }

    // Mutation explicitly detaches a snapshot, preserving copy-on-write semantics. Analysis
    // never takes this path: installing a replacement only appends file-sized metadata.
    fn materialize(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let files = std::mem::take(&mut self.files);
        self.base.retain(|_, metadata| {
            !(metadata.kind.is_closure() || metadata.kind.is_arrow_function())
                || !files.iter().any(|overlay| overlay.file == metadata.span.file_id)
        });
        for overlay in files {
            self.base.extend(Arc::unwrap_or_clone(overlay).entries);
        }
    }
}

impl IntoIterator for FunctionLikeTable {
    type Item = (SymbolIdentifier, Box<FunctionLikeMetadata>);
    type IntoIter = <Entries as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}

impl<'a> IntoIterator for &'a FunctionLikeTable {
    type Item = (&'a SymbolIdentifier, &'a Box<FunctionLikeMetadata>);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl<'a> IntoIterator for &'a mut FunctionLikeTable {
    type Item = (&'a SymbolIdentifier, &'a mut Box<FunctionLikeMetadata>);
    type IntoIter = <&'a mut Entries as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.materialize();
        (&mut self.base).into_iter()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for FunctionLikeTable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, metadata) in self {
            map.serialize_entry(key, metadata)?;
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FunctionLikeTable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Entries::deserialize(deserializer).map(|entries| Self {
            base: entries.into(),
            files: Vec::new(),
            declaration_spans: Default::default(),
        })
    }
}
