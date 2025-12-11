use std::collections::HashMap;

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

#[allow(dead_code)]
fn part_one(input: &str) -> usize
{
    use pathfinding::prelude::count_paths;

    let m = input.lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let s = iter.next().unwrap();
            let src = &s[0..s.len() - 1];

            let dst = iter.collect::<Vec<_>>();
            (src, dst)
        })
        .collect::<HashMap<_,_>>();

    count_paths(
        "you",
        |s| m.get(s).unwrap().iter().cloned(),
        |s| *s == "out"
    )
}

fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::count_paths;

    let m = input.lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let s = iter.next().unwrap();
            let src = &s[0..s.len() - 1];

            let dst = iter.collect::<Vec<_>>();
            (src, dst)
        })
        .collect::<HashMap<_,_>>();

    let empty = vec![];
    let mut count = count_paths(
        "dac",
        |s| m.get(s).unwrap_or(&empty).iter().cloned(),
        |s| *s == "out"
    );
    count *= count_paths(
        "fft",
        |s| m.get(s).unwrap_or(&empty).iter().cloned(),
        |s| *s == "dac"
    );
    count *= count_paths(
        "svr",
        |s| m.get(s).unwrap_or(&empty).iter().cloned(),
        |s| *s == "fft"
    );

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 662);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 429399933071120);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 5);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 2);
    }
}
