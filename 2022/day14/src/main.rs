use std::collections::HashSet;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> i32 {
    let mut used = load(input);
    let &(_, lowest) = used.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    let mut count = 0;
    'outer: loop {
        let mut pos = (500, 0);
        while let Some(p) = fall(pos, &mut used) {
            pos = p;
            if p.1 >= lowest { break 'outer }
        }
        count += 1;
        used.insert(pos);
    }

    count
}

fn part_two(input: &str) -> i32 {
    let mut used = load(input);
    let &(_, lowest) = used.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let floor = lowest + 2;

    let mut count = 0;
    'outer: loop {
        let mut pos = (500, 0);
        while let Some(p) = fall(pos, &mut used) {
            pos = p;
            if p.1 == floor - 1 { break }
        }
        count += 1;
        used.insert(pos);

        // If it didn't move, the cave is full.
        if pos == (500, 0) { break 'outer }
    }

    count
}

fn load(input: &str) -> HashSet<(u32, u32)> {
    input.split('\n')
        .map(|line| line.split(" -> ")
            .flat_map(|s| s.split_once(','))
            .map(|(s1, s2)| (s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()))
            .collect::<Vec<_>>())
        .fold(HashSet::new(), |mut acc, v| {
            v.windows(2).for_each(|w| {
                let [a, b] = sort(w);
                (a.0..=b.0).for_each(|x| (a.1..=b.1)
                    .for_each(|y| { acc.insert((x, y)); })
                );
            });
            acc
        })
}

fn sort(arr: &[(u32, u32)]) -> [(u32, u32);2] {
    // This works because we know either the x's or the y's are the same.
    if arr[0] > arr[1] { [arr[1], arr[0]] } else { [arr[0], arr[1]]}
}

fn fall(p: (u32, u32), used: &mut HashSet<(u32, u32)>) -> Option<(u32, u32)> {
    if !used.contains(&(p.0, p.1+1)) {
        Some((p.0, p.1 + 1))
    } else if !used.contains(&(p.0-1, p.1+1)) {
        Some((p.0-1, p.1+1))
    } else if !used.contains(&(p.0+1, p.1+1)) {
        Some((p.0+1, p.1+1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 805);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 25161);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 24);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 93);
    }
}
