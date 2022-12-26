use std::ops::Range;
use itertools::Product;
use pathfinding::matrix::Matrix;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize {
    let trees = Matrix::from_rows(input.lines()
        .map(|line| line.bytes().map(|b| b - b'0' )))
        .unwrap();

    let edges = (trees.rows * 2) + (trees.columns - 2) * 2;
    let visible = trees.inner_iter()
        .filter(|&cell| is_visible(&trees, cell))
        .count();

    edges + visible
}

fn part_two(input: &str) -> usize {
    let trees = Matrix::from_rows(input.lines()
        .map(|line| line.bytes().map(|b| b - b'0' )))
        .unwrap();

    trees.inner_iter()
        .map(|cell| scenic_score(&trees, cell))
        .max()
        .unwrap()
}

trait InnerIter {
    fn inner_iter(&self) -> Product<Range<usize>, Range<usize>>;
}
impl InnerIter for Matrix<u8> {
    fn inner_iter(&self) -> Product<Range<usize>, Range<usize>> {
        use itertools::Itertools;

        (1..self.rows-1).cartesian_product(1..self.columns-1)
    }
}

fn is_visible(m: &Matrix<u8>, curr: (usize, usize)) -> bool {
    let height = m.get(curr).unwrap();
    let is_shorter = |cell| m.get(cell).unwrap() < height;

    m.in_direction(curr, (-1, 0)).all(is_shorter)       // up
    || m.in_direction(curr, (1, 0)).all(is_shorter)     // down
    || m.in_direction(curr, (0, -1)).all(is_shorter)    // left
    || m.in_direction(curr, (0, 1)).all(is_shorter)     // right
}

fn scenic_score(m: &Matrix<u8>, curr: (usize, usize)) -> usize {
    let mut score = viewing_distance(m, curr, (-1, 0));
    if score != 0 { score *= viewing_distance(m, curr, (1, 0)) }
    if score != 0 { score *= viewing_distance(m, curr, (0, -1)) }
    if score != 0 { score *= viewing_distance(m, curr, (0, 1)) }

    score
}

fn viewing_distance(m: &Matrix<u8>, curr: (usize, usize), dir: (isize, isize)) -> usize {
    let height = m.get(curr).unwrap();

    let mut viewable = 0;
    for cell in m.in_direction(curr, dir) {
        viewable += 1;
        if m.get(cell).unwrap() >= height {
            break
        }
    }

    viewable
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1533);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 345744);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 8);
    }
}
