use std::ops::Index;

fn main() {
    use std::time::Instant;

    let input = load_strategy1(include_str!("../input.txt"));
    let t = Instant::now();
    let score = calc_score(&input);
    println!("Part 1: {} ({:?})", score, t.elapsed());

    let input = load_strategy2(include_str!("../input.txt"));
    let t = Instant::now();
    let score = calc_score(&input);
    println!("Part 2: {} ({:?})", score, t.elapsed());
}

#[derive(Clone, Copy)]
enum Shape {
    Rock = 0,
    Paper,
    Scissors,
}

impl Shape {
    fn from(byte: u8) -> Self {
        use Shape::*;

        match byte {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scissors,
            _ => panic!("Unknown shape: {byte}"),
        }
    }

    fn with_result(&self, result: u8) -> Self {
        use Shape::*;

        match (self, result) {
            (Rock, b'X') => Scissors,
            (Rock, b'Y') => Rock,
            (Rock, b'Z') => Paper,

            (Paper, b'X') => Rock,
            (Paper, b'Y') => Paper,
            (Paper, b'Z') => Scissors,

            (Scissors, b'X') => Paper,
            (Scissors, b'Y') => Scissors,
            (Scissors, b'Z') => Rock,
            _ => panic!("Unknown combination!"),
        }
    }
}

impl Index<&Shape> for [[i32; 3]; 3] {
    type Output = [i32; 3];

    fn index(&self, shape: &Shape) -> &Self::Output {
        &self[*shape as usize]
    }
}

impl Index<&Shape> for [i32; 3] {
    type Output = i32;

    fn index(&self, shape: &Shape) -> &Self::Output {
        &self[*shape as usize]
    }
}

fn score_matrix() -> [[i32; 3]; 3] {
    [
        [4, 8, 3], // rock
        [1, 5, 9], // paper
        [7, 2, 6], // scissors
    ]
}

fn load_strategy1(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|s| {
            let bytes = s.as_bytes();
            (Shape::from(bytes[0]), Shape::from(bytes[2]))
        })
        .collect()
}

fn load_strategy2(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|s| {
            let bytes = s.as_bytes();
            let shape = Shape::from(bytes[0]);
            (shape, shape.with_result(bytes[2]))
        })
        .collect()
}

fn calc_score(rounds: &[(Shape, Shape)]) -> i32 {
    let scoring = score_matrix();
    rounds.iter().map(|(s1, s2)| scoring[s1][s2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = load_strategy1(include_str!("../input.txt"));
        let score = calc_score(&input);
        assert_eq!(score, 8933);

        let input = load_strategy2(include_str!("../input.txt"));
        let score = calc_score(&input);
        assert_eq!(score, 11998);
    }
}
