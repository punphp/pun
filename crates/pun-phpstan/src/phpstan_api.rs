//! PHPStan 2.3.5's own class-likes (`data/phpstan-api.php`, written by scripts/export-phpstan-api-stub.php).

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub const PHPSTAN_API: &[u8] = include_bytes!("../data/phpstan-api.php");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassKind {
    Class,
    Interface,
    Trait,
}

struct Declared {
    roots: HashSet<String>,
    classes: HashMap<String, ClassKind>,
}

static DECLARED: LazyLock<Declared> = LazyLock::new(|| {
    let mut declared = Declared {
        roots: HashSet::new(),
        classes: HashMap::new(),
    };
    let mut namespace = String::new();
    for line in String::from_utf8_lossy(PHPSTAN_API).lines() {
        if let Some(name) = line.strip_prefix("namespace ") {
            namespace = name.trim_end_matches(';').to_ascii_lowercase();
            let root = namespace.split('\\').next().unwrap_or_default();
            declared.roots.insert(root.to_string());
            continue;
        }
        let mut words = line
            .split(' ')
            .skip_while(|w| matches!(*w, "final" | "abstract" | "readonly"));
        let kind = match words.next() {
            Some("class") => ClassKind::Class,
            Some("interface") => ClassKind::Interface,
            Some("trait") => ClassKind::Trait,
            _ => continue,
        };
        let name = words.next().unwrap_or_default();
        let name = name.split(':').next().unwrap_or_default();
        let class = format!("{namespace}\\{}", name.to_ascii_lowercase());
        declared.classes.insert(class, kind);
    }
    declared
});

/// Whether the PHAR's autoloader answers for `name`: its first namespace segment is one the API declares.
pub fn owns(name: &str) -> bool {
    let name = name.trim_start_matches('\\').to_ascii_lowercase();
    name.split_once('\\')
        .is_some_and(|(root, _)| DECLARED.roots.contains(root))
}

pub fn class_kind(name: &str) -> Option<ClassKind> {
    let name = name.trim_start_matches('\\').to_ascii_lowercase();
    DECLARED.classes.get(&name).copied()
}
