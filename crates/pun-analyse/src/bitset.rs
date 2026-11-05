//! A sequence of flags stored one bit each.

/// A sequence of `len` flags, one bit each.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitSet {
    words: Vec<u64>,
    len: usize,
}

impl BitSet {
    /// `len` flags, all clear.
    pub fn new(len: usize) -> BitSet {
        BitSet {
            words: vec![0; len.div_ceil(64)],
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Flag `i`; a flag past the end is clear.
    pub fn get(&self, i: usize) -> bool {
        i < self.len && self.words[i / 64] & (1 << (i % 64)) != 0
    }

    /// Sets flag `i` to `value`, growing the sequence to hold it.
    pub fn set(&mut self, i: usize, value: bool) {
        if i >= self.len {
            self.len = i + 1;
            self.words.resize(self.len.div_ceil(64), 0);
        }
        if value {
            self.words[i / 64] |= 1 << (i % 64);
        } else {
            self.words[i / 64] &= !(1 << (i % 64));
        }
    }

    pub fn push(&mut self, value: bool) {
        self.set(self.len, value);
    }

    pub fn pop(&mut self) -> Option<bool> {
        let last = self.last()?;
        self.truncate(self.len - 1);
        Some(last)
    }

    pub fn last(&self) -> Option<bool> {
        self.len.checked_sub(1).map(|i| self.get(i))
    }

    /// Keeps the first `len` flags.
    pub fn truncate(&mut self, len: usize) {
        for i in len..self.len.min(self.words.len() * 64) {
            self.words[i / 64] &= !(1 << (i % 64));
        }
        self.len = self.len.min(len);
        self.words.truncate(self.len.div_ceil(64));
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator + '_ {
        (0..self.len).map(|i| self.get(i))
    }
}

impl IntoIterator for BitSet {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter { set: self, next: 0 }
    }
}

/// The flags of a [`BitSet`], in order.
pub struct IntoIter {
    set: BitSet,
    next: usize,
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let i = self.next;
        self.next += 1;
        (i < self.set.len).then(|| self.set.get(i))
    }
}

impl FromIterator<bool> for BitSet {
    fn from_iter<I: IntoIterator<Item = bool>>(flags: I) -> BitSet {
        let mut out = BitSet::default();
        for flag in flags {
            out.push(flag);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn flags_across_words() {
        let mut flags = BitSet::new(70);
        flags.set(0, true);
        flags.set(64, true);
        flags.set(69, true);
        flags.set(0, false);
        assert_eq!(flags.len(), 70);
        assert_eq!(
            flags
                .iter()
                .enumerate()
                .filter(|(_, f)| *f)
                .map(|(i, _)| i)
                .collect::<Vec<_>>(),
            [64, 69]
        );
        assert!(!flags.get(70));
        assert_eq!(flags.last(), Some(true));
        assert_eq!(flags.pop(), Some(true));
        assert_eq!(flags.last(), Some(false));
        flags.truncate(65);
        assert_eq!(flags.len(), 65);
        assert_eq!(BitSet::default().pop(), None);
        flags.push(false);
        flags.set(70, false);
        assert_eq!(flags.len(), 71);
        assert_eq!(flags.iter().filter(|f| *f).count(), 1);
        let collected: BitSet = [true, false, true].into_iter().collect();
        assert_eq!(
            collected.into_iter().collect::<Vec<_>>(),
            [true, false, true]
        );
    }
}
