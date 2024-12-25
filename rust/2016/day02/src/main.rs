fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t = Instant::now();
    let code = part_one(&input);
    println!("Part 1: {} ({:?})", code, t.elapsed());

    let t = Instant::now();
    let code = part_two(&input);
    println!("Part 2: {} ({:?})", code, t.elapsed());
}

fn part_one(input: &str) -> String {
    let mut key = (1i8, 1i8);

    input.lines().map(|l| {
        key = l.chars().fold(key, |p, c|
            match c {
                'U' => (0.max(p.0 - 1), p.1),
                'D' => (2.min(p.0 + 1), p.1),
                'L' => (p.0, 0.max(p.1 - 1)),
                'R' => (p.0, 2.min(p.1 + 1)),
                _ => panic!("Unknown direction: {c}")
            }
        );
        (key.0 * 3 + key.1 + 1) as u32
    })
    .map(|n| char::from_digit(n, 10).unwrap())
    .collect()
}

fn part_two(input: &str) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    input.lines().map(|l| {
        let (r, c) = l.chars().fold((2i8, 0i8), |(kr, kc), c| {
            let (r, c) = match c {
                'U' => (0.max(kr - 1), kc),
                'D' => (4.min(kr + 1), kc),
                'L' => (kr, 0.max(kc - 1)),
                'R' => (kr, 4.min(kc + 1)),
                _ => unreachable!()
            };
            let ch = keypad[r as usize][c as usize];
            if ch == ' ' { (kr, kc) } else { (r, c) }
        });
        keypad[r as usize][c as usize]
    })
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let code = part_one(&input);
        assert_eq!(code, "12578");

        let code = part_two(&input);
        assert_eq!(code, "516DD");
    }
}