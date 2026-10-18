//! Ports of `PhpCsFixer\Tokenizer\Analyzer\Analysis\*`: plain data, public fields named after
//! upstream's getters (`get_full_name()` -> `.full_name`); computed getters are methods.
//! Strings are bytes (PHP strings). Upstream `?int` is `Option<usize>`.
//! Case/Default/Switch/Match/Enum analyses live with their analyzer in control_case_structures_analyzer.rs.

use pun_tokens::php_compat::{strtolower, trim};

/// `NamespaceUseAnalysis::TYPE_*`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamespaceUseType {
    Class = 1,
    Function = 2,
    Constant = 3,
}

/// `NamespaceUseAnalysis`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamespaceUseAnalysis {
    pub type_: NamespaceUseType,
    pub full_name: Vec<u8>,
    pub short_name: Vec<u8>,
    pub is_aliased: bool,
    pub is_in_multi: bool,
    pub start_index: usize,
    pub end_index: usize,
    pub chunk_start_index: Option<usize>,
    pub chunk_end_index: Option<usize>,
}

impl NamespaceUseAnalysis {
    pub fn is_class(&self) -> bool {
        self.type_ == NamespaceUseType::Class
    }
    pub fn is_function(&self) -> bool {
        self.type_ == NamespaceUseType::Function
    }
    pub fn is_constant(&self) -> bool {
        self.type_ == NamespaceUseType::Constant
    }
}

const RESERVED_TYPES: &[&[u8]] = &[
    b"array",
    b"bool",
    b"callable",
    b"false",
    b"float",
    b"int",
    b"iterable",
    b"list",
    b"mixed",
    b"never",
    b"null",
    b"object",
    b"parent",
    b"resource",
    b"self",
    b"static",
    b"string",
    b"true",
    b"void",
];

/// `TypeAnalysis`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeAnalysis {
    pub name: Vec<u8>,
    pub start_index: Option<usize>,
    pub end_index: Option<usize>,
    pub nullable: bool,
}

impl TypeAnalysis {
    /// `new TypeAnalysis($name, $startIndex, $endIndex)` (PHP >= 8.0 branch).
    pub fn new(name: &[u8], start_index: Option<usize>, end_index: Option<usize>) -> Self {
        let (name, nullable) = if let Some(rest) = name.strip_prefix(b"?") {
            (rest.to_vec(), true)
        } else {
            let lower = strtolower(name);
            (
                name.to_vec(),
                lower.split(|&c| c == b'|').any(|p| trim(p) == b"null"),
            )
        };
        TypeAnalysis {
            name,
            start_index,
            end_index,
            nullable,
        }
    }
    pub fn get_start_index(&self) -> usize {
        self.start_index.expect("TypeAnalysis: no start index.")
    }
    pub fn get_end_index(&self) -> usize {
        self.end_index.expect("TypeAnalysis: no end index.")
    }
    pub fn is_reserved_type(&self) -> bool {
        RESERVED_TYPES.contains(&&strtolower(&self.name)[..])
    }
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
}

/// `ArgumentAnalysis`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgumentAnalysis {
    pub name: Option<Vec<u8>>,
    pub name_index: Option<usize>,
    pub default: Option<Vec<u8>>,
    pub type_analysis: Option<TypeAnalysis>,
}

/// One entry of `AttributeAnalysis::getAttributes()`: `array{start: int, end: int, name: string}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub start: usize,
    pub end: usize,
    pub name: Vec<u8>,
}

/// `AttributeAnalysis`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeAnalysis {
    pub start_index: usize,
    pub end_index: usize,
    pub opening_bracket_index: usize,
    pub closing_bracket_index: usize,
    pub attributes: Vec<Attribute>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_analysis_nullability() {
        assert!(TypeAnalysis::new(b"?int", None, None).is_nullable());
        assert_eq!(TypeAnalysis::new(b"?int", None, None).name, b"int");
        assert!(TypeAnalysis::new(b"int | NULL", None, None).is_nullable());
        assert!(!TypeAnalysis::new(b"int|nullable", None, None).is_nullable());
        assert!(TypeAnalysis::new(b"Self", None, None).is_reserved_type());
    }
}
