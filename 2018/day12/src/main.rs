use std::collections::BTreeSet;

type State = BTreeSet<i32>;
type Rules = Vec<(u8, u8)>;

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

fn part_one(input: &str) -> i32
{
    let (state, rules) = load(input);
    let state = (0..20).fold(state, |st, _| cycle(&st, &rules));
    state.iter().sum()
}

fn part_two(input: &str) -> i64
{
    // Via inspection, generation 169 and after are all 75
    // more than the previous generation. So, we get the sum
    // at generation 168 and add 75 to it for each generation
    // after that.

    let (state, rules) = load(input);
    let st = (0..=168).fold(state.clone(), |st, _| {
        cycle(&st, &rules)
    });
    let start = st.iter().sum::<i32>() as i64;

    let n = 50_000_000_000 - 169;
    start + n * 75
}

fn cycle(state: &State, rules: &Rules) -> State
{
    use std::collections::HashSet;

    // Because we have a rule whereby ..... => . we only need to deal
    // with chunks that have plants in them. But we do need to check
    // two plots to the left and two to the right of each potted plant
    // because it may cause one of them to grow or die.
    let mut checked = HashSet::new();

    state.iter()
        .fold(State::new(), |mut st, n| {
            (n-2..=n+2)
                .filter(|n| checked.insert(*n))
                .filter_map(|n| check(n, state, rules).map(|v| (n, *v)))
                .filter(|(_, v)| *v == 1)
                .for_each(|(n, _)| { st.insert(n); });
            st
        })
}

fn check<'a>(n: i32, state: &State, rules: &'a Rules) -> Option<&'a u8>
{
    let pattern = (n-2..=n+2)
        .enumerate()
        .filter(|(_, n)| state.contains(n))
        .fold(0u8, |p, (i, _)| p | 1 << (4 - i));

    rules.iter().
        find(|(p, _)| *p == pattern)
        .map(|(_, v)| v)
}

fn load(input: &str) -> (State, Rules)
{
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let (_, st) = s1.split_once(": ").unwrap();
    let state = st.bytes()
        .enumerate()
        .filter_map(|(i, b)| (b == b'#').then_some(i as i32))
        .collect::<BTreeSet<_>>();

    let rules = s2.lines()
        .flat_map(|line| line.split_once(" => "))
        .map(|(s1, s2)| {
            let pattern = s1.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .fold(0, |n, (i, _)| n | 1 << (4 - i) );
            let plant = if s2.as_bytes()[0] == b'#' { 1 } else { 0 };

            (pattern, plant)
        })
        .collect::<Vec<_>>();

    (state, rules)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3276);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3750000001113);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 325);
    }
}