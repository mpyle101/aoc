fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1:  {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2:  {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> i32
{
    input.lines()
        .map(|line| {
            let seq: Vec<i32> = line.split(' ')
                .flat_map(|s| s.parse())
                .collect();

            HistoryIter::from(&seq)
                .flat_map(|v| v.last().copied())
                .sum::<i32>()
        })
        .sum()
}

fn part_two(input: &str) -> i32
{
    input.lines()
        .map(|line| {
            let seq: Vec<i32> = line.split(' ')
                .flat_map(|s| s.parse())
                .collect();
            let vals: Vec<_> = HistoryIter::from(&seq)
                .map(|v| v[0])
                .collect();
    
            vals.iter().rev()
                .cloned()
                .reduce(|acc, n| n - acc)
                .unwrap()
            })
        .sum()
}

struct HistoryIter
{
    hist: Vec<i32>,
}
impl From<&Vec<i32>> for HistoryIter
{
    fn from(seq: &Vec<i32>) -> Self
    {
        HistoryIter { hist: seq.clone() }
    }
}
impl Iterator for HistoryIter
{
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>>
    {
        use std::mem::take;

        if self.hist.iter().any(|n| *n != 0) {
            let seq = take(&mut self.hist);
            self.hist = (1..seq.len()).map(|i| seq[i] - seq[i-1]).collect();

            Some(seq)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1939607039);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1041);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 114);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 2);
    }
}
