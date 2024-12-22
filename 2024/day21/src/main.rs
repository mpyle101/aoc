use std::collections::HashMap;

type Directions = HashMap<(i32, i32), char>;
type Expansions = HashMap<(char, char), Vec<String>>;
type Numbers    = HashMap<char, (i32, i32)>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../example2.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    input.lines()
//        .map(|line| (&line[0..3], sequence(line)))
        .map(|line| (&line[0..3], sequence(line)))
        .inspect(|(s, n)| println!("{s} {n}"))
        .filter_map(|(s, v)| s.parse::<usize>().ok().map(|n| n * v))
        .sum()
}

fn sequence(seq: &str) -> usize
{
    use std::collections::HashSet;
    use pathfinding::prelude::astar_bag_collect;
    use rayon::prelude::*;

    let nbrs = numbers();
    let dirs = directions();

    let mut path = seq.chars().collect::<Vec<_>>();
    path.insert(0, 'A');

    let mut possible = HashSet::new();
    for w in path.windows(2) {
        let start = nbrs.get(&w[0]).unwrap();
        let goal  = nbrs.get(&w[1]).unwrap();
        let (slns, _) = astar_bag_collect(
            start, |p| numeric_moves(*p), |p| md(p, goal), |p| p == goal
        ).unwrap();

        let v = slns.par_iter()
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
            .collect::<HashSet<_>>();

        let k = v.par_iter()
            .flat_map(|s| expand(s)).collect::<HashSet<_>>();
        let min = k.iter().map(|s| s.len()).min().unwrap();
        possible = if possible.is_empty() {
            k.iter().filter(|s| s.len() == min).cloned().collect::<HashSet<_>>()
        } else {
            k.iter()
                .filter(|s| s.len() == min)
                .flat_map(|s| possible.iter().map(|p| {
                    let mut q = p.clone();
                    q.push_str(s);
                    q
                }))
                .collect::<HashSet<_>>()
        };

        let k = possible.par_iter()
            .flat_map(|s| expand(s)).collect::<HashSet<_>>();
        let min = k.iter().map(|s| s.len()).min().unwrap();
        possible = k.iter()
            .filter(|s| s.len() == min)
            .flat_map(|s| possible.iter().map(|p| {
                let mut q = p.clone();
                q.push_str(s);
                q
            }))
            .collect::<HashSet<_>>();
    }
    
    dbg!(&possible);
    possible.len()
}

fn expand(seq: &str) -> Vec<String>
{
    let exps  = expansions();
    let chars = seq.chars().collect::<Vec<_>>();
    let mut v = if let Some(ex) = exps.get(&('A', chars[0])) {
        ex.clone()
    } else {
        vec![]
    };

    for w in chars.windows(2) {
        let ex = exps.get(&(w[0], w[1])).unwrap();
        v = ex.iter()
            .flat_map(|e| v.iter().map(|s| {
                let mut s = s.to_string();
                s.push_str(e);
                s
            }))
            .collect::<Vec<_>>()
    }

    v
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
    if x > 0 && !(x == 1 && y == 2) { v.push(((x - 1, y), 1)) }
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
        ('8', (1, 1)),
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
        assert_eq!(part_one(input), 218);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 126384);
    }
}
