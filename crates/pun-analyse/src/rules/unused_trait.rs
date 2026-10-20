//! NotAnalysedTraitRule (level 4) over TraitDeclarationCollector and TraitUseCollector: a trait declared in an
//! analysed file that no analysed class-like uses, directly or through a used trait. PHPStan skips it when every
//! analysed path is a file (`isOnlyFilesAnalysis()`).

use std::collections::HashSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use pun_phpstan::ignore::Error;

use crate::Project;
use crate::codebase::Sources;

pub(crate) fn unused_traits(project: &Project, codebase: &CodebaseMetadata, sources: &Sources) -> Vec<Error> {
    if project.level < 4 || project.only_files {
        return Vec::new();
    }
    let analysed: HashSet<_> = project
        .analysed
        .iter()
        .filter_map(|p| sources.get(p))
        .map(|f| f.id)
        .collect();
    // the traits the analysed class-likes use (a used trait is analysed, so its own uses count)
    let mut used: HashSet<Vec<u8>> = HashSet::new();
    let mut pending: Vec<&[u8]> = codebase
        .class_likes
        .values()
        .filter(|k| k.kind != SymbolKind::Trait && analysed.contains(&k.span.file_id))
        .flat_map(|k| k.used_traits.iter().map(|t| t.as_bytes()))
        .collect();
    while let Some(t) = pending.pop() {
        if used.insert(t.to_ascii_lowercase())
            && let Some(k) = codebase.get_class_like(t)
        {
            pending.extend(k.used_traits.iter().map(|t| t.as_bytes()));
        }
    }
    let mut errors: Vec<Error> = codebase
        .class_likes
        .values()
        .filter(|k| {
            k.kind == SymbolKind::Trait
                && analysed.contains(&k.span.file_id)
                && !used.contains(&k.name.as_bytes().to_ascii_lowercase())
        })
        .filter_map(|k| {
            let file = sources.values().find(|f| f.id == k.span.file_id)?;
            let name = String::from_utf8_lossy(k.original_name.as_bytes());
            // the first line of its declaration, attributes included
            let at = k.span;
            let line = i64::from(file.line_number(at.start.offset)) + 1;
            Some(Error {
                identifier: Some("trait.unused".to_string()),
                tip: Some("See: https://phpstan.org/blog/how-phpstan-analyses-traits".to_string()),
                ..Error::new(
                    format!("Trait {name} is used zero times and is not analysed."),
                    String::from_utf8_lossy(&file.name).into_owned(),
                    Some(line),
                )
            })
        })
        .collect();
    errors.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    errors
}
