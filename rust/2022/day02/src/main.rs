use std::ops::Index;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    let scoring = scoring();
    input.lines()
        .map(|s| {
            let bytes = s.as_bytes();
            let s1 = Shape::from(bytes[0]);
            let s2 = Shape::from(bytes[2]);
            scoring[s1][s2]
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let scoring = scoring();
    input.lines()
        .map(|s| {
            let bytes = s.as_bytes();
            let s1 = Shape::from(bytes[0]);
            let s2 = s1.with_result(bytes[2]);
            scoring[s1][s2]
        })
        .sum()
}

fn scoring() -> [[i32; 3]; 3] {
    [
        [4, 8, 3], // rock
        [1, 5, 9], // paper
        [7, 2, 6], // scissors
    ]
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

impl Index<Shape> for [[i32; 3]; 3] {
    type Output = [i32; 3];

    fn index(&self, shape: Shape) -> &Self::Output {
        &self[shape as usize]
    }
}

impl Index<Shape> for [i32; 3] {
    type Output = i32;

    fn index(&self, shape: Shape) -> &Self::Output {
        &self[shape as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 8933);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 11998);
    }
}
