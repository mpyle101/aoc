
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

fn part_one(input: &str) -> i32 {
    let mut buf: [char;4] = [' ';4];

    input.chars()
        .take(4)
        .enumerate()
        .for_each(|(i, c)| buf[i] = c);

    let mut offset = 0;
    let mut iter = input.chars().skip(4);
    while !is_marker(&buf, 4) {
        buf.rotate_left(1);
        buf[3] = iter.next().unwrap();
        offset += 1;
    }

    offset + 4
}

fn part_two(input: &str) -> i32 {
    let mut buf: [char;14] = [' ';14];

    input.chars()
        .take(14)
        .enumerate()
        .for_each(|(i, c)| buf[i] = c);

    let mut offset = 0;
    let mut iter = input.chars().skip(14);
    while !is_marker(&buf, 14) {
        buf.rotate_left(1);
        buf[13] = iter.next().unwrap();
        offset += 1;
    }

    offset + 14
}

fn is_marker(buf: &[char], count: usize) -> bool {
    use std::collections::HashSet;

    let set: HashSet<&char> = HashSet::from_iter(buf.iter());
    set.len() == count
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
