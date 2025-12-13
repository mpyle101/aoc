//! Matrix of bits and utilities to rotate, transpose, etc.

#![allow(dead_code)]

use std::ops::{
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
};


#[derive(Clone, Debug)]
pub struct BitMatrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<u64>,
}

impl BitMatrix {
    pub fn new(rows: usize, cols: usize) -> Self
    {
        // Allocate enough space to hold all the entries in a set
        // of 64bit values. This could be bigger than needed so
        // we have to pass the width (cols) to the bit_pos function.
        let words = (rows * cols).div_ceil(64);
        let data = vec![0u64; words];

        Self { rows, cols, data }
    }

    /// Creates a BitGrid from an iterator of rows and a predicate
    /// `rows_iter` can be anything iterable over row items
    /// `is_on` should return true for a cell that should be set
    pub fn from_rows<I, R, C, F>(rows_iter: I, is_on: F) -> Self
        where
            I: IntoIterator<Item = R>,
            R: IntoIterator<Item = C> + Clone,
            F: Fn(&C) -> bool,
    {
        let v: Vec<R> = rows_iter.into_iter().collect();
        let rows = v.len();
        let cols = v
            .first()
            .map(|r| r.clone().into_iter().count())
            .unwrap_or(0);

        let words = (rows * cols).div_ceil(64);
        let mut data = vec![0u64; words];

        for (r, row) in v.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                if is_on(&cell) {
                    let (w, m) = bit_pos(r, c, cols);
                    data[w] |= m;
                }
            }
        }

        BitMatrix { rows, cols, data }
    }
    
    /// Get the bit at (row, col)
    pub fn get(&self, row: usize, col: usize) -> bool
    {
        let (w, m) = bit_pos(row, col, self.cols);
        self.data[w] & m != 0
    }

    /// Set the bit at (row, col) to 1
    pub fn set(&mut self, row: usize, col: usize)
    {
        let (w, m) = bit_pos(row, col, self.cols);
        self.data[w] |= m
    }

    /// Set or clear the entry based on if the value is zero or not.
    pub fn update(&mut self, row: usize, col: usize, val: bool)
    {
        let (w, m) = bit_pos(row, col, self.cols);
        if val { self.data[w] &= !m } else { self.data[w] |= m }
    }

    /// Clear the value at (row, col) - set bit to 0
    pub fn clear(&mut self, row: usize, col: usize)
    {
        let (w, m) = bit_pos(row, col, self.cols);
        self.data[w] &= !m
    }

    /// Return an iterator over ((row, col), bool) for each cell
    pub fn items(&self) -> impl Iterator<Item = ((usize, usize), bool)> + '_
    {
        (0..self.rows).flat_map(move |r| {
            (0..self.cols).map(move |c| {
                let (w, m) = bit_pos(r, c, self.cols);
                ((r, c), (self.data[w] & m) != 0)
            })
        })
    }

    /// Return an iterator over ((row, col), BitRef) for each cell which
    /// can be used to modify the cell value.
    /// 
    /// bm.items_mut(|(r, c), cell| {
    ///     if (r + c) % 2 == 0 { cell.set(); } else { cell.clear(); }
    /// });
    pub fn items_mut<F>(&mut self, mut f: F)
        where F: FnMut((usize, usize), &mut BitRef),
    {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let (w, mask) = bit_pos(r, c, self.cols);
                let mut bit_ref = BitRef { word: &mut self.data[w], mask };
                f((r, c), &mut bit_ref);
            }
        }
    }

    /// Get the bit mask corresponding to a set of position in the matrix
    /// specified by (row, column) tuples. This mask can then be |'d with
    /// another matrix to set the entries or &'d to see if the entries are set.
    pub fn get_mask(&self, tile: &[(u32, u32)], row: usize, col: usize) -> BitMatrix
    {
        let data = tile.iter()
            .fold(vec![0u64; self.data.len()], |mut mask, (r, c)| {
                let (wr, wc) = (row + *r as usize, col + *c as usize);
                let (w, m) = bit_pos(wr, wc, self.cols);
                mask[w] |= m;
                mask
            });

        Self { data, ..*self }
    }

    /// Return a copy of the matrix after transposition.
    pub fn transposed(&self) -> Self
    {
        assert!(
            self.rows != 0 || self.cols == 0,
            "this operation would create a matrix with empty rows"
        );
        let mut m = BitMatrix {
            rows: self.cols,
            cols: self.rows,
            data: vec![0u64; self.data.len()],
        };

        for r in 0..self.rows {
            for c in 0..self.cols {
                m.update(c, r, self.get(r, c));
            }
        }

        m
    }

    /// Flip the matrix around the vertical axis.
    pub fn flip_lr(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols / 2 {
                let opposite = self.cols - 1 - c;

                // swap the bits (left, right)
                let lt = self.get(r, c);
                let rt = self.get(r, opposite);

                self.update(r, c, rt);
                self.update(r, opposite, lt);
            }
        }
    }

    /// Flip the matrix around the horizontal axis.
    pub fn flip_ud(&mut self) {
        for r in 0..self.rows / 2 {
            let opposite = self.rows - 1 - r;

            for c in 0..self.cols {
                // swap the bits (top, bottom)
                let tp = self.get(r, c);
                let bt = self.get(opposite, c);

                self.update(r, c, bt);
                self.update(opposite, c, tp);
            }
        }
    }
}

pub struct BitRef<'a> {
    word: &'a mut u64,
    mask: u64,
}

impl<'a> BitRef<'a> {
    /// Get the current value of the cell
    pub fn get(&self) -> bool
    {
        (*self.word & self.mask) != 0
    }

    /// Set the cell to true
    pub fn set(&mut self)
    {
        *self.word |= self.mask;
    }

    /// Clear the cell to false
    pub fn clear(&mut self)
    {
        *self.word &= !self.mask;
    }

    /// Set the cell to a specific value
    pub fn set_to(&mut self, val: bool)
    {
        if val { self.set() } else { self.clear() }
    }
}

// let (w, m) = bit_pos(r, c, width)
// bitset[w] |= m to set that cell in the board
// bitset[w] & m != 0 to see if that cell is occupied
fn bit_pos(r: usize, c: usize, w: usize) -> (usize, u64)
{
    let idx = r * w + c;
    (idx / 64, 1u64 << (idx % 64))
}

impl<'a, 'b> BitOr<&'b BitMatrix> for &'a BitMatrix {
    type Output = BitMatrix;

    fn bitor(self, rhs: &'b BitMatrix) -> BitMatrix
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        let data = self.data.iter()
            .zip(&rhs.data)
            .map(|(a, b)| a | b)
            .collect();

        BitMatrix { data, ..*self }
    }
}

impl<'a, 'b> BitAnd<&'b BitMatrix> for &'a BitMatrix {
    type Output = BitMatrix;

    fn bitand(self, rhs: &'b BitMatrix) -> BitMatrix
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        let data = self.data.iter()
            .zip(&rhs.data)
            .map(|(a, b)| a & b)
            .collect();

        BitMatrix { data, ..*self }
    }
}

impl BitOrAssign<&BitMatrix> for BitMatrix {
    fn bitor_assign(&mut self, rhs: &BitMatrix)
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        self.data.iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, b)| *a |= *b);
    }
}

impl BitAndAssign<&BitMatrix> for BitMatrix {
    fn bitand_assign(&mut self, rhs: &BitMatrix)
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        self.data.iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, b)| *a &= *b);
    }
}

impl<'a, 'b> BitXor<&'b BitMatrix> for &'a BitMatrix {
    type Output = BitMatrix;

    fn bitxor(self, rhs: &'b BitMatrix) -> BitMatrix
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        let data = self.data.iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        BitMatrix { data, ..*self }
    }
}

impl BitXorAssign<&BitMatrix> for BitMatrix {
    fn bitxor_assign(&mut self, rhs: &BitMatrix)
    {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.data.len(), rhs.data.len());

        self.data.iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, b)| *a ^= *b);
    }
}
