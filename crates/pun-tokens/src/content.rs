//! A token's content: its bytes inline when they fit in 22, else on the heap. Same 24 bytes as a `Vec<u8>`, but most
//! tokens (whitespace, names, operators) need no allocation to build, clone or drop.
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

const INLINE: usize = 22;

#[derive(Clone)]
pub enum Content {
    Inline { len: u8, buf: [u8; INLINE] },
    Heap(Box<[u8]>),
}

impl Content {
    fn from_slice(bytes: &[u8]) -> Content {
        if bytes.len() <= INLINE {
            let mut buf = [0; INLINE];
            buf[..bytes.len()].copy_from_slice(bytes);
            Content::Inline {
                len: bytes.len() as u8,
                buf,
            }
        } else {
            Content::Heap(bytes.into())
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content::from_slice(b"")
    }
}

impl Deref for Content {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            Content::Inline { len, buf } => &buf[..*len as usize],
            Content::Heap(b) => b,
        }
    }
}

impl Borrow<[u8]> for Content {
    fn borrow(&self) -> &[u8] {
        self
    }
}


impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[u8] as fmt::Debug>::fmt(self, f)
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Content) -> bool {
        **self == **other
    }
}
impl Eq for Content {}

impl Hash for Content {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl PartialEq<&[u8]> for Content {
    fn eq(&self, other: &&[u8]) -> bool {
        **self == **other
    }
}
impl<const N: usize> PartialEq<[u8; N]> for Content {
    fn eq(&self, other: &[u8; N]) -> bool {
        **self == other[..]
    }
}
impl<const N: usize> PartialEq<&[u8; N]> for Content {
    fn eq(&self, other: &&[u8; N]) -> bool {
        **self == other[..]
    }
}

impl From<&[u8]> for Content {
    fn from(b: &[u8]) -> Self {
        Content::from_slice(b)
    }
}
impl<const N: usize> From<&[u8; N]> for Content {
    fn from(b: &[u8; N]) -> Self {
        Content::from_slice(b)
    }
}
impl From<Vec<u8>> for Content {
    fn from(b: Vec<u8>) -> Self {
        if b.len() <= INLINE {
            Content::from_slice(&b)
        } else {
            Content::Heap(b.into_boxed_slice())
        }
    }
}
impl From<&str> for Content {
    fn from(s: &str) -> Self {
        Content::from_slice(s.as_bytes())
    }
}
impl From<String> for Content {
    fn from(s: String) -> Self {
        Content::from(s.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 22 bytes stay inline, 23 go to the heap; both read back the same bytes, and equality and hashing ignore where
    // the bytes live (a heap-built short content equals an inline one).
    #[test]
    fn inline_up_to_22_bytes() {
        assert_eq!(std::mem::size_of::<Content>(), 24);
        let short = Content::from(&[b'a'; 22]);
        assert!(matches!(short, Content::Inline { len: 22, .. }));
        assert_eq!(&*short, &[b'a'; 22][..]);
        let long = Content::from(vec![b'b'; 23]);
        assert!(matches!(long, Content::Heap(_)));
        assert_eq!(&*long, &[b'b'; 23][..]);
        assert_eq!(&*Content::from("b".repeat(23)), &[b'b'; 23][..]);
        let heap_short = Content::Heap(Box::from(&b"ab"[..]));
        assert_eq!(heap_short, Content::from("ab"));
        let hash = |c: &Content| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            c.hash(&mut h);
            h.finish()
        };
        assert_eq!(hash(&heap_short), hash(&Content::from("ab")));
        assert_eq!(Content::default(), *b"");
        assert_eq!(format!("{:?}", Content::from("ab")), "[97, 98]");
    }
}
