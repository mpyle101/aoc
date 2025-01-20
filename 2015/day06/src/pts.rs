use std::ops::Range;

pub struct PointsIterator {
    x: i32,
    xs: Range<i32>,
    ys: Range<i32>,
    cy: Range<i32>
}

impl PointsIterator {
    pub fn from(mut xs: Range<i32>, ys: Range<i32>) -> PointsIterator
    {
        let cy = ys.clone();
        let x  = xs.next().unwrap();
        let xs = xs.clone();
    
        PointsIterator { x, xs, ys, cy }
    }    
}

impl Iterator for PointsIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item>
    {
        if let Some(y) = self.cy.next() {
            return Some((self.x, y))
        } else if let Some(x) = self.xs.next() {
            self.x = x;
            self.cy = self.ys.clone();
            if let Some(y) = self.cy.next() {
                return Some((self.x, y))
            }
        }

        None
    }
}
