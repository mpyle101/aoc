
fn main() {
    use std::fs;
    use std::time::Instant;

    let input = load_strategy1(&fs::read_to_string("./input.txt").unwrap());
    let t = Instant::now();
    let score = calc_score(&input);
    println!("Part 1: {} ({:?})", score, t.elapsed());

    let input = load_strategy2(&fs::read_to_string("./input.txt").unwrap());
    let t = Instant::now();
    let score = calc_score(&input);
    println!("Part 2: {} ({:?})", score, t.elapsed());
}

enum Shape {
    Rock = 0,
    Paper,
    Scissors
}

fn score_matrix() -> [[i32; 3]; 3] {
    [
        [4, 8, 3],  // rock
        [1, 5, 9],  // paper
        [7, 2, 6]   // scissors
    ]
}

fn shape_index(b: u8) -> usize {
    use Shape::*;

    match b {
        b'A' | b'X' => Rock as usize,
        b'B' | b'Y' => Paper as usize,
        b'C' | b'Z' => Scissors as usize,
        _ => panic!("Unknown shape: {b}")
    }
}

fn result_index(b1: u8, b2: u8) -> usize {
    use Shape::*;

    match (b1, b2) {
        (b'A', b'X') => Scissors as usize,
        (b'A', b'Y') => Rock as usize,
        (b'A', b'Z') => Paper as usize,

        (b'B', b'X') => Rock as usize,
        (b'B', b'Y') => Paper as usize,
        (b'B', b'Z') => Scissors as usize,

        (b'C', b'X') => Paper as usize,
        (b'C', b'Y') => Scissors as usize,
        (b'C', b'Z') => Rock as usize,
        _ => panic!("Unknown combination: ({b1}, {b2})")
    }
}

fn load_strategy1(input: &str) -> Vec<(usize, usize)> {
    input.split("\n").map(|s| {
        let bytes = s.as_bytes();
        (shape_index(bytes[0]), shape_index(bytes[2]))
    }).collect::<Vec<_>>()
}

fn load_strategy2(input: &str) -> Vec<(usize, usize)> {
    input.split("\n").map(|s| {
        let bytes = s.as_bytes();
        (shape_index(bytes[0]), result_index(bytes[0], bytes[2]))
    }).collect::<Vec<_>>()
}

fn calc_score(rounds: &[(usize, usize)]) -> i32 {
    let scoring = score_matrix();
    rounds.iter()
        .map(|(s1, s2)| scoring[*s1][*s2])
        .sum()
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