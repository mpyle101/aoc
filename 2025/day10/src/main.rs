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
    let mut buttons = vec![];
    let mut patterns: Vec<u16> = vec![];
    input.lines()
        .for_each(|l| {
            let v = l.split_whitespace().collect::<Vec<_>>();
            let pat = v[0][1..v[0].len() - 1].chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .fold(0, |n, (i, _)| n | 1 << i);
            let btns: Vec<u16> = v[1..v.len() - 1].iter()
                .fold(vec![], |mut v, b| {
                    let btns = b[1..b.len() - 1].split(',')
                        .flat_map(|s| s.parse::<usize>())
                        .fold(0, |n, i| n | 1 << i);
                    v.push(btns);
                    v
                });

            patterns.push(pat);
            buttons.push(btns);
        });

    (0..buttons.len())
        .map(|i| lights(patterns[i], &buttons[i]))
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut buttons = vec![];
    let mut joltage = vec![];
    input.lines()
        .for_each(|l| {
            let v = l.split_whitespace().collect::<Vec<_>>();
            let j = v.last().unwrap();
            let jlts = j[1..j.len() - 1].split(',')
                .flat_map(|s| s.parse::<u32>())
                .collect::<Vec<_>>();

            let mut btns = v[1..v.len() - 1].iter()
                .map(|b| {
                    b[1..b.len() - 1].split(',')
                        .flat_map(|s| s.parse::<usize>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            btns.sort_by_key(|v| std::cmp::Reverse(v.len()));

            joltage.push(jlts);
            buttons.push(btns);
        });

    let mut count = 0;
    (0..buttons.len())
        .for_each(|i| {
            println!("{:?}", joltage[i]);
            let c = dfs(&joltage[i], &buttons[i]);
            println!("{c}");
            count += c;
        });

    count
}

fn lights(pat: u16, btns: &[u16]) -> usize
{
    use pathfinding::prelude::bfs;

    let path = bfs(
        &(0, -1),
        |(n, i)| lighters(*n, *i, btns),
        |(n, _)| *n == pat
    );

    path.unwrap().len() - 1
}

fn lighters(n: u16, i: i32, btns: &[u16]) -> Vec<(u16, i32)>
{
    btns.iter()
        .enumerate()
        .filter(|(j, _)| *j != i as usize)
        .map(|(j, v)| (n ^ v, j as i32))
        .collect()
}

fn dfs(joltage: &[u32], btns: &[Vec<usize>]) -> u32
{
    if joltage.iter().sum::<u32>() == 0 {
        0
    } else {
        // Get the index of the jolt with the least number of buttons
        // ignoring any at 0.
        let (_, ix) = (0..joltage.len())
            .filter(|i| joltage[*i] != 0)
            .map(|i| (btns.iter().filter(|v| v.contains(&i)).count(), i))
            .min()
            .unwrap();

        // Partition the buttons into used and remaining based on if
        // they manipulate the jolt from above.
        let (used, rem) = btns.iter()
            .fold((vec![], vec![]), |(mut used, mut rem), v| {
                if v.contains(&ix) { used.push(v.clone()) } else { rem.push(v.clone()) }
                (used, rem)
            });

        let mut count = u32::MAX;
        if !used.is_empty() {
            // Generate new target states by applying the buttons enough
            // times in all combinations to hit the required value for the
            // target jolt. Recurse and do it over again with the reduced set
            // of available buttons. It's possible the filtering will remove
            // all the candiate states.
            let jolt = joltage[ix];
            let coefs = get_coefs(jolt, joltage.len());

            let states = get_states(&coefs, joltage.len(), &used);

            let mut jolts = vec![0;joltage.len()];
            for st in states.iter() {
                if joltage.iter().zip(st).all(|(a, b)| b <= a) {
                    (0..joltage.len()).for_each(|i| jolts[i] = joltage[i] - st[i]);
                    let r = dfs(&jolts, &rem);
                    if r != u32::MAX { count = count.min(jolt + r)}
                }
            }
        }

        count
    }
}

fn get_states(coefs: &[Vec<u32>], size: usize, btns: &[Vec<usize>]) -> Vec<Vec<u32>>
{
    // Use them to generate value vectors.
    coefs.iter()
        .map(|v| {
            let mut cv = vec![0;size];
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c > 0)
                .for_each(|(i, c)| btns[i].iter().for_each(|j| cv[*j] += *c));
            cv
        })
        .collect()
}

fn get_coefs(x: u32, n: usize) -> Vec<Vec<u32>>
{
    use itertools::Itertools;

    let x1 = x as i32;
    let n1 = n as i32;

    let mut states = vec![];
    for bars in (0..x1 + n1 - 1).combinations(n - 1) {
        let mut state = vec![];
        let mut prev = -1;
        for b in bars {
            state.push((b - prev - 1) as u32);
            prev = b;
        }
        state.push((x1 + n1 - 1 - prev - 1) as u32);
        states.push(state)
    }

    states
}

#[allow(dead_code)]
fn calc(size: usize, coef: &[u32], btns: &[Vec<usize>]) -> Vec<u32>
{
    btns.iter()
        .zip(coef.iter())
        .fold(vec![0;size], |mut v, (b, n)| {
            b.iter().for_each(|i| v[*i] += n);
            v
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 527);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 0);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 7);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 33);
    }
}
