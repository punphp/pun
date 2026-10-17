//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase;
use mago_codex::scanner::scan_program;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_syntax::parser::parse_file;

use crate::Project;

static PRELUDE: LazyLock<Prelude> = LazyLock::new(Prelude::build);

/// PHPStan's unprefixed classes and functions (scripts/export-phpstan-api-stub.php): PHPStan always has them loaded.
const PHPSTAN_API: &[u8] = include_bytes!("../data/phpstan-api.php");

/// Whether `code` names PHPStan's own namespaces, so the (large) API stub is worth scanning.
fn names_phpstan(code: &[u8]) -> bool {
    code.windows(8)
        .any(|w| w.eq_ignore_ascii_case(b"phpstan\\") || w.eq_ignore_ascii_case(b"phpparse"))
}

pub fn php_version(id: u32) -> PHPVersion {
    PHPVersion::new(id / 10000, id / 100 % 100, id % 100)
}

pub fn read(path: &Path, file_type: FileType) -> Option<File> {
    let contents = std::fs::read(path).ok()?;
    let name = path.to_string_lossy().into_owned().into_bytes();
    Some(File::new(
        Cow::Owned(name),
        file_type,
        Some(path.to_path_buf()),
        Cow::Owned(contents),
    ))
}

fn scan(metadata: &mut CodebaseMetadata, file: &File, version: PHPVersion) {
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    let names = NameResolver::new(&arena).resolve(program);
    // ponytail: Mago's scanner trips a debug assertion on some hints (an empty TUnion::from_vec, debug builds
    // only); the file's symbols are then unknown
    let scanned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scan_program(&arena, file, program, &names, version)
    }));
    if let Ok(scanned) = scanned {
        metadata.extend(scanned);
    }
}

pub fn build(project: &Project) -> CodebaseMetadata {
    let mut metadata = PRELUDE.metadata.clone();
    let mut references = PRELUDE.symbol_references.clone();
    let version = php_version(project.php_version);
    let mut seen = HashSet::new();
    let mut api = false;
    let files = project
        .analysed
        .iter()
        .map(|p| (p, FileType::Host))
        .chain(project.scanned.iter().map(|p| (p, FileType::Vendored)));
    for (path, file_type) in files {
        if !seen.insert(path) {
            continue;
        }
        let Some(file) = read(path, file_type) else {
            continue;
        };
        api |= file_type == FileType::Host && names_phpstan(&file.contents);
        scan(&mut metadata, &file, version);
    }
    if api {
        let stub = File::new(
            Cow::Borrowed(b"phpstan-api.php"),
            FileType::Vendored,
            None,
            Cow::Borrowed(PHPSTAN_API),
        );
        scan(&mut metadata, &stub, version);
    }
    populate_codebase(
        &mut metadata,
        &mut references,
        Default::default(),
        Default::default(),
    );
    metadata
}
