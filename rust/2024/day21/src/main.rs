use std::collections::HashMap;

type Expansions<'a> = HashMap<(char, char), Vec<&'a str>>;
type Directions = HashMap<(i32, i32), char>;
type Numbers    = HashMap<char, (i32, i32)>;
type Memos      = HashMap<(String, usize), usize>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 25);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    solve(input, 2)
}

fn part_two(input: &str, robots: usize) -> usize
{
    solve(input, robots)
}

fn solve(input: &str, robots: usize) -> usize
{
    use std::str::FromStr;

    let mut memos = Memos::new();

    input.lines()
        .map(|line| {
            let n = usize::from_str(&line[0..3]).unwrap();
            let seqs = sequences(line);
            let keys = seqs.iter()
                .map(|sq| expand(sq, robots, &mut memos))
                .min().unwrap();
            n * keys
        })
        .sum()
}

fn expand(seq: &str, robots: usize, memos: &mut Memos) -> usize
{
    if let Some(n) = memos.get(&(seq.into(), robots)) {
        return *n
    }

    let n = if robots == 0 {
        seq.len()
    } else {
        let exps = expansions();
        let mut chars = seq.chars().collect::<Vec<_>>();
        chars.insert(0, 'A');

        chars.windows(2)
            .fold(0, |acc, w| {
                acc + if w[0] == w[1] {
                    1
                } else {
                    let exp = exps.get(&(w[0], w[1])).unwrap();
                    exp.iter()
                        .map(|sq| expand(sq, robots - 1, memos))
                        .min().unwrap()
                }
            })
    };

    memos.insert((seq.into(), robots), n);

    n
}

fn sequences(seq: &str) -> Vec<String>
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
            .flat_map(|s| possible.iter().map(|p| p.to_owned() + s))
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
    slns.map(|sln| {
        let mut s = "".to_string();
        sln.windows(2)
            .for_each(|w| {
                let delta = (w[1].0 - w[0].0, w[1].1 - w[0].1);
                let key   = *dirs.get(&delta).unwrap();
                s.push(key)
            });
        s += "A";
        s
    })
    .collect()
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

fn expansions<'a>() -> &'static Expansions<'a>
{
    use std::sync::OnceLock;

    static EXPANSIONS: OnceLock<Expansions> = OnceLock::new();
    EXPANSIONS.get_or_init(|| HashMap::from([
        (('A', 'A'), vec!["A"]), 
        (('A', '^'), vec!["<A"]),
        (('A', '>'), vec!["vA"]),
        (('A', 'v'), vec!["v<A", "<vA"]),   
        (('A', '<'), vec!["v<<A"]),
    
        (('^', '^'), vec!["A"]),
        (('^', 'A'), vec![">A"]),
        (('^', 'v'), vec!["vA"]), 
        (('^', '>'), vec!["v>A", ">vA"]),
        (('^', '<'), vec!["v<A"]),
    
        (('>', '>'), vec!["A"]),
        (('>', 'A'), vec!["^A"]),
        (('>', 'v'), vec!["<A"]),
        (('>', '^'), vec!["^<A", "<^A"]),
        (('>', '<'), vec!["<<A"]),
    
        (('v', 'v'), vec!["A"]),
        (('v', '^'), vec!["^A"]),
        (('v', '>'), vec![">A"]), 
        (('v', '<'), vec!["<A"]), 
        (('v', 'A'), vec!["^>A", ">^A"]),
    
        (('<', '<'), vec!["A"]),
        (('<', 'v'), vec![">A"]),
        (('<', '>'), vec![">>A"]),
        (('<', '^'), vec![">^A"]),
        (('<', 'A'), vec![">>^A"]),
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
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 25), 294585598101704);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 126384);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 2), 126384);

        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 2), 237342);   
    }
}
