use std::collections::HashSet;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let sand = part_one(input);
    println!("Part 1: {} ({:?})", sand, t.elapsed());

    let t = Instant::now();
    let sand = part_two(input);
    println!("Part 2: {} ({:?})", sand, t.elapsed());
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
                let (a, b) = (w[0], w[1]);
                if a.0 == b.0 {
                    let y0 = b.1.min(a.1);
                    let y1 = b.1.max(a.1);
                    (y0..=y1).for_each(|y| { acc.insert((a.0, y)); })
                } else {
                    let x0 = b.0.min(a.0);
                    let x1 = b.0.max(a.0);
                    (x0..=x1).for_each(|x| { acc.insert((x, a.1)); })
                }
            });
            acc
        })
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
    fn it_works() {
        let input = include_str!("../input.txt");

        let sand = part_one(input);
        assert_eq!(sand, 805);

        let sand = part_two(input);
        assert_eq!(sand, 25161);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let sand = part_one(input);
        assert_eq!(sand, 24);

        let sand = part_two(input);
        assert_eq!(sand, 93);
    }
}
