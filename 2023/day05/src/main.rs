use std::ops::Range;
use std::str::Split;

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

#[derive(Debug)]
struct Mapping
{
    src: Range<u64>,
    dst: Range<u64>,
}

fn part_one(input: &str) -> u64
{
    let (first, rest) = input.split_once("\n\n").unwrap();
    let stages = stages(rest);

    let (_, values) = first.split_once(':').unwrap();
    values.trim().split(' ')
        .flat_map(|v| v.parse::<u64>())
        .map(|seed| location_for_seed(seed, &stages))
        .min()
        .unwrap()
}

fn part_two(input: &str) -> u64
{
    let (first, rest) = input.split_once("\n\n").unwrap();
    let stages = stages(rest);

    let (_, values) = first.split_once(':').unwrap();
    let iter: RangeIter = values.trim().into();
    iter.map(|r| location_for_range(&r, &stages))
        .min()
        .unwrap()
}

struct RangeIter<'a>
{
    iter: Split<'a, char>,
}
impl<'a> From<&'a str> for RangeIter<'a>
{
    fn from(s: &'a str) -> Self
    {
        RangeIter { iter: s.split(' ') }
    }
}
impl Iterator for RangeIter<'_>
{
    type Item = Range<u64>;

    fn next(&mut self) -> Option<Range<u64>>
    {
        if let Some(v1) = self.iter.next().map(|n| n.parse::<u64>().unwrap()) {
            self.iter.next()
                .map(|n| n.parse::<u64>().unwrap())
                .map(|v2| v1..v1 + v2)
        } else {
            None
        }
    }
}

fn stages(input: &str) -> Vec<Vec<Mapping>>
{
    input.split("\n\n")
        .map(|mapping| {
            mapping.split('\n')
                .skip(1)
                .map(|s| {
                    let mut it = s.split(' ');
                    let dst = it.next().map(|n| n.parse().unwrap()).unwrap();
                    let src = it.next().map(|n| n.parse().unwrap()).unwrap();
                    let run: u64 = it.next().map(|n| n.parse().unwrap()).unwrap();

                    Mapping {
                        src: src..src + run,
                        dst: dst..dst + run,
                    }
                })
                .collect()
        })
        .collect()
}

fn location_for_seed(seed: u64, stages: &[Vec<Mapping>]) -> u64
{
    let mut v = seed;
    for stage in stages {
        for mapping in stage {
            if mapping.src.contains(&v) {
                v = mapping.dst.start + (v - mapping.src.start);
                break;
            }
        }
    }

    v
}

fn location_for_range(seeds: &Range<u64>, stages: &[Vec<Mapping>]) -> u64
{
    let mut ranges = vec![Range{ start: seeds.start, end: seeds.end }];

    for stage in stages {
        let mut v = vec![];
        for range in ranges {
            let mut src =  range.start..range.end;
            for mapping in stage {
                if mapping.src.contains(&src.start) {
                    let delta = src.start - mapping.src.start;
                    let start = mapping.dst.start + delta;

                    let dst = if mapping.src.contains(&src.end) {
                        let len = src.end - src.start;
                        src.end = src.start;
                        start..start + len
                    } else {
                        src.start = mapping.src.end;
                        start..mapping.dst.end
                    };
                    v.push(dst);
                } else if mapping.src.contains(&src.end) {
                    let delta = src.end - mapping.src.start;
                    let end = mapping.dst.start + delta;
                    src.end = mapping.src.start;
                    v.push(mapping.dst.start..end)
                }
            }
            if !src.is_empty() {
                v.push(src);
            }
        }
        ranges = v;
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    ranges[0].start
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 910845529);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 77435348);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 35);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 46);
    }
}
