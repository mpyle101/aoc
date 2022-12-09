
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let tail = part_one(input);
    println!("Part 1: {} ({:?})", tail, t.elapsed());

    let t = Instant::now();
    let tail = part_two(input);
    println!("Part 2: {} ({:?})", tail, t.elapsed());
}

fn part_one(input: &str) -> usize {
    track_tail::<2>(input)
}

fn part_two(input: &str) -> usize {
    track_tail::<10>(input)
}

fn track_tail<const N:usize>(input: &str) -> usize {
    use std::collections::HashSet;

    let mut tail_positions = HashSet::from([(0, 0)]);

    input.lines()
        .flat_map(|s| s.split_once(' '))
        .map(|(dir, steps)| (dir, steps.parse::<i32>().unwrap()))
        .fold([(0, 0);N], |mut pos, (dir, steps)| {
            (0..steps).for_each(|_| {
                let head = pos[0];
                pos[0] = match dir {
                    "U" => (head.0 - 1, head.1),
                    "D" => (head.0 + 1, head.1),
                    "L" => (head.0, head.1 - 1),
                    "R" => (head.0, head.1 + 1),
                    _ => unreachable!()
                };

                (1..N).for_each(|i| pos[i] = follow(pos[i-1], pos[i]));
                tail_positions.insert(pos[N-1]);
            });

            pos
        });
    
    tail_positions.len()
}

fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if tail.0.abs_diff(head.0) <= 1 && tail.1.abs_diff(head.1) <= 1 {
        tail    // touching: don't move
    } else {
        let dy = (head.0 - tail.0).clamp(-1, 1);
        let dx = (head.1 - tail.1).clamp(-1, 1);
    
        (tail.0 + dy, tail.1 + dx)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let tail = part_one(input);
        assert_eq!(tail, 6175);

        let tail = part_two(input);
        assert_eq!(tail, 2578);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let tail = part_one(input);
        assert_eq!(tail, 13);
    }
}
