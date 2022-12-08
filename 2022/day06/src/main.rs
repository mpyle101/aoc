
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let offset = part_one(input);
    println!("Part 1: {} ({:?})", offset, t.elapsed());

    let t = Instant::now();
    let offset = part_two(input);
    println!("Part 2: {} ({:?})", offset, t.elapsed());
}

fn part_one(input: &str) -> usize {
    find_offset(input, 4)
}

fn part_two(input: &str) -> usize {
    find_offset(input, 14)
}

fn find_offset(input: &str, n: usize) -> usize {
    input.as_bytes()
        .windows(n)
        .take_while(|w| is_not_marker(w))
        .count() + n
}

fn is_not_marker(buf: &[u8]) -> bool {
    (1..buf.len()).any(|i| buf[i..].contains(&buf[i-1]))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let offset = part_one(input);
        assert_eq!(offset, 1542);

        let offset = part_two(input);
        assert_eq!(offset, 3153);
    }

    #[test]
    fn examples() {
        let offset = part_one("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(offset, 5);

        let offset = part_one("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(offset, 6);

        let offset = part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(offset, 10);

        let offset = part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(offset, 11);

        let offset = part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(offset, 19);

        let offset = part_two("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(offset, 23);

        let offset = part_two("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(offset, 23);

        let offset = part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(offset, 29);

        let offset = part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(offset, 26);
    }
}
