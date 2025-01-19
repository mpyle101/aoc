fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    input.lines()
        .filter(|word| nice(word))
        .count()
}

fn part_two(input: &str) -> usize
{
    input.lines()
        .filter(|word| nicer(word))
        .count()
}

fn nice(word: &str) -> bool {
    let mut vowels = 0;
    let mut double = 0;

    let bad = [b"ab", b"cd", b"pq", b"xy"];

    let bytes = word.as_bytes();
    for s in bytes.windows(2) {
        if bad.contains(&&[s[0], s[1]]) {
            return false;
        }

        double += (s[0] == s[1]) as u32;
        vowels += is_vowel(s[0]) as u32;
    }
    vowels += is_vowel(*bytes.last().unwrap()) as u32;

    vowels > 2 && double > 0
}

fn nicer(word: &str) -> bool {
    use std::collections::HashMap;

    let mut pairs = HashMap::new();
    let (repeats, _, _) = word.chars()
        .fold((0, '^', '^'),
            |(r, c0, c1), c| {
                if !(c0 == c1 && c1 == c) {
                    *pairs.entry((c1, c)).or_insert(0) += 1;
                }
                (r + (c0 == c) as i32, c1, c)
            }
        );

    *pairs.values().max().unwrap() > 1 && repeats > 0
}

fn is_vowel(c: u8) -> bool
{
    c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 258);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 53);
    }

    #[test]
    fn nice_1() {
        assert!(nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn nice_2() {
        assert!(nice("aaa"));
    }

    #[test]
    fn nice_3() {
        assert!(!nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn nice_4() {
        assert!(!nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn nice_5() {
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn nicer_1() {
        assert!(nicer("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn nicer_2() {
        assert!(nicer("xxyxx"));
    }

    #[test]
    fn nicer_3() {
        assert!(!nicer("uurcxstgmygtbstg"));
    }

    #[test]
    fn nicer_4() {
        assert!(!nicer("ieodomkazucvgmuy"));
    }
}