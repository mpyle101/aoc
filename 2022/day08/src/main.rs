use pathfinding::matrix::Matrix;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let visible = part_one(input);
    println!("Part 1: {} ({:?})", visible, t.elapsed());

    let t = Instant::now();
    let score = part_two(input);
    println!("Part 2: {} ({:?})", score, t.elapsed());
}

fn part_one(input: &str) -> usize {
    let trees = Matrix::from_rows(input.lines()
        .map(|line| line.bytes().map(|b| b - b'0' )))
        .unwrap();

    let edges = (trees.rows * 2) + (trees.columns - 2) * 2;
    let visible: usize = (1..trees.rows - 1)
        .map(|r| (1..trees.columns - 1)
            .filter(|c| is_visible(&trees, (r, *c)))
            .count())
        .sum();

    edges + visible
}

fn part_two(input: &str) -> usize {
    let trees = Matrix::from_rows(input.lines()
        .map(|line| line.bytes().map(|b| b - b'0' )))
        .unwrap();

    (1..trees.rows - 1)
        .filter_map(|r| (1..trees.columns - 1)
            .map(|c| score(&trees, (r, c)))
            .max())
        .max()
        .unwrap()
}


fn is_visible(m: &Matrix<u8>, curr: (usize, usize)) -> bool {
    let height = m.get(curr).unwrap();
    let mut visible = m.in_direction(curr, (-1, 0)).all(|cell| m.get(cell).unwrap() < height);
    visible |= m.in_direction(curr, (1, 0)).all(|cell| m.get(cell).unwrap() < height);
    visible |= m.in_direction(curr, (0, -1)).all(|cell| m.get(cell).unwrap() < height);
    visible |= m.in_direction(curr, (0, 1)).all(|cell| m.get(cell).unwrap() < height);

    visible
}

fn score(m: &Matrix<u8>, curr: (usize, usize)) -> usize {
    let mut score = viewing_distance(m, curr, (-1, 0));
    score *= viewing_distance(m ,curr, (1, 0));
    score *= viewing_distance(m, curr, (0, -1));
    score *= viewing_distance(m, curr, (0, 1));

    score
}

fn viewing_distance(m: &Matrix<u8>, curr: (usize, usize), dir: (isize, isize)) -> usize {
    let height = m.get(curr).unwrap();
    let mut viewable = 0;
    for cell in m.in_direction(curr, dir) {
        if let Some(h) = m.get(cell) {
            viewable += 1;
            if h >= height { break }
        }
    }

    viewable
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let visible = part_one(input);
        assert_eq!(visible, 1533);

        let score = part_two(input);
        assert_eq!(score, 345744);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let visible = part_one(input);
        assert_eq!(visible, 21);

        let score = part_two(input);
        assert_eq!(score, 8);
    }
}
