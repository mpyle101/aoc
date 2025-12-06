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

fn part_one(input: &str) -> u64
{
    let mut iter = input.lines();
    let mut formulas = iter.next().unwrap()
        .split_whitespace()
        .map(|s| vec![s])
        .collect::<Vec<_>>();
    iter.for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, s)| formulas[i].push(s));
    });
    formulas
        .iter_mut()
        .map(|v| {
            v.reverse();
            if v[0] == "+" {
                v.iter().skip(1).flat_map(|s| s.parse::<u64>()).sum::<u64>()
            } else {
                v.iter().skip(1).flat_map(|s| s.parse::<u64>()).product::<u64>()
            }
        })
        .sum()
}

fn part_two(input: &str) -> u64
{
    let mut lines = input.lines().rev();
    let line = lines.next().unwrap();
    let mut iter = line.chars();
    let c = iter.next().unwrap();

    let mut actions = vec![c];
    let mut spacing = vec![];
    let n = iter.fold(1, |n, c| {
        if c == ' ' {
            n + 1
        } else {
            spacing.push(n - 1);
            actions.push(c);
            1
        }
    });
    spacing.push(n);

    let mut values = vec!{Vec::new(); actions.len()};
    lines
        .for_each(|line| {
            spacing
                .iter()
                .enumerate()
                .fold(0, |n, (ix, i)| {
                    values[ix].push(line[n..n+i].to_string());
                    n + i + 1
                });
        });
    values.iter_mut().for_each(|v| v.reverse());

    (0..values.len())
        .map(|i| {
            let v = &values[i];
            let n = spacing[i];
            (
                i,
                (0..n).rev()
                    .map(|j| {
                        let num = v.iter().flat_map(|s| s.chars().nth(j)).collect::<String>();
                        num.trim().parse::<u64>().unwrap()
                    })
            )
        })
        .map(|(i, v)| {
            if actions[i] == '+' { v.sum::<u64>() } else { v.product() } 
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4805473544166);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 8907730960817);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 4277556);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 3263827);
    }
}
