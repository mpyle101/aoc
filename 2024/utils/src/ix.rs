use std::ops::Range;

pub fn from(len: usize) -> IndexCombinations
{
    let outer = 0..len - 1;
    let inner = 0..0;

    IndexCombinations { curr: 0, outer, inner, len }
}

pub struct IndexCombinations {
    len: usize,
    curr: usize,
    outer: Range<usize>,
    inner: Range<usize>,
}

impl Iterator for IndexCombinations {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item>
    {
        if let Some(j) = self.inner.next() {
            return Some((self.curr, j))
        } else if let Some(i) = self.outer.next() {
            self.curr  = i;
            self.inner = i + 1..self.len;
            if let Some(j) = self.inner.next() {
                return Some((self.curr, j))
            }
        }

        None
    }
}