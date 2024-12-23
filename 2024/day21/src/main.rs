#![allow(dead_code)]
use std::collections::HashMap;

type Directions = HashMap<(i32, i32), char>;
type Expansions = HashMap<(char, char), Vec<String>>;
type Numbers    = HashMap<char, (i32, i32)>;
type Memos      = HashMap<(String, usize), usize>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../test.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    input.lines()
        .map(|line| (&line[0..3], sequence(line)))
        .filter_map(|(s, v)| s.parse::<usize>().ok().map(|n| n * v))
        .sum()
}

fn part_two(input: &str) -> usize
{
    let mut memos = Memos::new();

    input.lines()
        .for_each(|line| {
            let seqs = initial_sequences(line);
            let keys = seqs.iter()
                .map(|sq| {
                    let s = concat("A", sq);
                    expand(&s, 3, &mut memos)
                })
                .min()
                .unwrap();
            println!("KEYS: {line} {keys:?}");
        });

    0
}

fn expand(seq: &str, robots: usize, memos: &mut Memos) -> usize
{
    if seq.len() == 1 {
        1
    } else if let Some(n) = memos.get(&(seq.to_string(), robots)) {
        *n
    } else if robots == 0 {
        score2(seq)
    } else {
        let exps = expansions();

        let chars = seq.chars().collect::<Vec<_>>();
        let n = chars.windows(2)
            .map(|w| {
                let exp = exps.get(&(w[0], w[1])).unwrap();
                exp.iter()
                    .map(|s| expand(s, robots - 1, memos))
                    .min()
                    .unwrap()
            })
            .sum();

        memos.insert((seq.to_string(), robots), n);

        n
    }
}

fn score2(seq: &str) -> usize
{
    let exps  = expansions();
    let chars = seq.chars().collect::<Vec<_>>();

    let mut score = 0;
    for w in chars.windows(2) {
        let ex = exps.get(&(w[0], w[1])).unwrap();
        score += ex[0].len();
    }

    score
}

fn sequence(seq: &str) -> usize
{
    let nbrs = numbers();
    let path = seq.chars().collect::<Vec<_>>();

    let start = nbrs.get(&'A').unwrap();
    let goal  = nbrs.get(&path[0]).unwrap();
    let mut possible = solutions(start, goal).iter()
        .flat_map(|s| expand_sequence(s))
        .collect::<Vec<_>>();

    for w in path.windows(2) {
        let start = nbrs.get(&w[0]).unwrap();
        let goal  = nbrs.get(&w[1]).unwrap();
        let slns  = solutions(start, goal).iter()
            .flat_map(|s| expand_sequence(s))
            .collect::<Vec<_>>();
        possible = slns.iter()
            .flat_map(|s| possible.iter().map(|p| concat(p, s)))
            .collect::<Vec<_>>();
    }

    possible.iter()
        .map(|p| score(p))
        .min()
        .unwrap()
}

fn initial_sequences(seq: &str) -> Vec<String>
{
    let nbrs = numbers();
    let path = seq.chars().collect::<Vec<_>>();

    let start = nbrs.get(&'A').unwrap();
    let goal  = nbrs.get(&path[0]).unwrap();
    let mut possible = solutions(start, goal);

    for w in path.windows(2) {
        let start = nbrs.get(&w[0]).unwrap();
        let goal  = nbrs.get(&w[1]).unwrap();
        let slns  = solutions(start, goal);
        possible  = slns.iter()
            .flat_map(|s| possible.iter().map(|p| concat(p, s)))
            .collect::<Vec<_>>();
    }

    possible
}

fn solutions(start: &(i32, i32), goal: &(i32, i32)) -> Vec<String>
{
    use pathfinding::prelude::astar_bag;

    let (slns, _) = astar_bag(
        start, |p| numeric_moves(*p), |p| md(p, goal), |p| p == goal
    ).unwrap();

    let dirs = directions();
    slns
        .map(|sln| {
            let mut s = "".to_string();
            sln.windows(2)
                .for_each(|w| {
                    let delta = (w[1].0 - w[0].0, w[1].1 - w[0].1);
                    let key   = *dirs.get(&delta).unwrap();
                    s.push(key)
                });
            s.push('A');
            s
        })
        .collect()
}

fn expand_sequence(seq: &str) -> Vec<String>
{
    let exps  = expansions();

    let chars = seq.chars().collect::<Vec<_>>();
    let mut v = exps.get(&('A', chars[0])).unwrap().clone();
    for w in chars.windows(2) {
        let ex = exps.get(&(w[0], w[1])).unwrap();
        v = ex.iter()
            .flat_map(|e| v.iter().map(|s| concat(s, e)))
            .collect::<Vec<_>>()
    }

    v
}

fn score(seq: &str) -> usize
{
    let exps  = expansions();
    let chars = seq.chars().collect::<Vec<_>>();
    let v = exps.get(&('A', chars[0])).unwrap();

    let mut score = v[0].len();
    for w in chars.windows(2) {
        let ex = exps.get(&(w[0], w[1])).unwrap();
        score += ex[0].len();
    }

    score
}

fn concat(s1: &str, s2: &str) -> String
{
    let mut s = s1.to_string();
    s.push_str(s2);
    s
}

fn md((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32
{
    (x1.abs_diff(*x2) + y1.abs_diff(*y2)) as i32
}

fn numeric_moves((x, y): (i32, i32)) -> Vec<((i32, i32), i32)>
{
    let mut v = Vec::with_capacity(4);

    if x < 2 { v.push(((x + 1, y), 1)) }
    if y > 0 { v.push(((x, y - 1), 1)) }
    if x > 0 && !(x == 1 && y == 3) { v.push(((x - 1, y), 1)) }
    if y < 3 && !(x == 0 && y == 2) { v.push(((x, y + 1), 1)) }

    v
}

fn directions() -> &'static Directions
{
    use std::sync::OnceLock;

    static DIRECTIONS: OnceLock<Directions> = OnceLock::new();
    DIRECTIONS.get_or_init(|| HashMap::from([
        (( 1,  0), '>'),
        ((-1,  0), '<'),
        (( 0,  1), 'v'),
        (( 0, -1), '^')
    ]))
}

fn expansions() -> &'static Expansions
{
    use std::sync::OnceLock;

    static EXPANSIONS: OnceLock<Expansions> = OnceLock::new();
    EXPANSIONS.get_or_init(|| HashMap::from([
        (('A', 'A'), vec!["A".into()]), 
        (('A', '^'), vec!["<A".into()]),
        (('A', '>'), vec!["vA".into()]),
        (('A', 'v'), vec!["<vA".into(), "v<A".into()]),   
        (('A', '<'), vec!["v<<A".into(), "<v<A".into()]),
    
        (('^', '^'), vec!["A".into()]),
        (('^', 'A'), vec![">A".into()]),
        (('^', 'v'), vec!["vA".into()]), 
        (('^', '>'), vec![">vA".into(), "v>A".into()]),
        (('^', '<'), vec!["v<A".into()]),
    
        (('>', '>'), vec!["A".into()]),
        (('>', 'A'), vec!["^A".into()]),
        (('>', 'v'), vec!["<A".into()]),
        (('>', '^'), vec!["^<A".into(), "<^A".into()]),
        (('>', '<'), vec!["<<A".into()]),
    
        (('v', 'v'), vec!["A".into()]),
        (('v', '^'), vec!["^A".into()]),
        (('v', '>'), vec![">A".into()]), 
        (('v', '<'), vec!["<A".into()]), 
        (('v', 'A'), vec!["^>A".into(), ">^A".into()]),
    
        (('<', '<'), vec!["A".into()]),
        (('<', 'v'), vec![">A".into()]),
        (('<', '>'), vec![">>A".into()]),
        (('<', '^'), vec![">^A".into()]),
        (('<', 'A'), vec![">>^A".into(), ">^>A".into()]),
    ]))
}

fn numbers() -> &'static Numbers
{
    use std::sync::OnceLock;

    static NUMBERS: OnceLock<Numbers> = OnceLock::new();
    NUMBERS.get_or_init(|| HashMap::from([
        ('A', (2, 3)),
        ('0', (1, 3)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0))
    ]))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 237342);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 126384);
    }
}
