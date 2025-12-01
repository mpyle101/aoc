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

fn part_one(input: &str) -> u32
{
    let mut start = 0;
    let mut graph = Vec::new();

    let mut rowlen = 0;
    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            rowlen = line.len();
            graph.extend(line.chars());
            if let Some(y) = line.find('S') {
                start = x * rowlen + y;
            }
        });

    let (mut p, _) = first_move(start, &graph, rowlen);
    let mut steps = 1;
    while p.pos() != start {
        p = next_move(p, &graph, rowlen);
        steps += 1
    }
    
    steps / 2
}

fn part_two(input: &str) -> usize
{
    use std::collections::HashSet;

    let mut start = 0;
    let mut graph = Vec::new();

    let mut rowlen = 0;
    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            rowlen = line.len();
            graph.extend(line.chars());
            if let Some(y) = line.find('S') {
                start = x * rowlen + y;
            }
        });

    let mut pipes = HashSet::from([start]);
    let (mut p, c) = first_move(start, &graph, rowlen);
    while p.pos() != start {
        pipes.insert(p.pos());
        p = next_move(p, &graph, rowlen);
    }
    
    graph[start] = c;
    (0..graph.len())
        .filter(|p| !pipes.contains(p))
        .filter(|&p| {
            let q = p + rowlen - (p % rowlen);
            (p+1..q)
                .map(|n| (n, graph[n]))
                .filter(|(_, c)| *c == '7' || *c == 'F' || *c == '|')
                .filter(|(n, _)| pipes.contains(n))
                .count() % 2 == 1
        })
        .count()
}

#[derive(Debug, PartialEq)]
enum Move
{
    North(usize),
    South(usize),
    East(usize),
    West(usize),
}
impl Move
{
    fn pos(&self) -> usize
    {
        use Move::*;

        match self {
            North(p) | South(p) | East(p) | West(p) => *p,
        }
    }
}

fn first_move(start: usize, graph: &[char], rowlen: usize) -> (Move, char)
{
    use Move::*;

    let moves = (
        move_up(start, graph, rowlen),
        move_dn(start, graph, rowlen),
        move_lt(start, graph, rowlen),
        move_rt(start, graph, rowlen),
    );
    
    match moves {
        (Some(p), Some(_), None, None) => (North(p), '|'),
        (Some(p), None, Some(_), None) => (North(p), 'J'),
        (Some(p), None, None, Some(_)) => (North(p), 'L'),
        (None, Some(p), Some(_), None) => (South(p), '7'),
        (None, Some(p), None, Some(_)) => (South(p), 'F'),
        (None, None, Some(p), Some(_)) => (West(p),  '-'),
        _ => panic!("Bad start")
    }
}

fn next_move(mv: Move, graph: &[char], rowlen: usize) -> Move
{
    use Move::*;

    match mv {
        North(p) if graph[p] == '|' => North(p - rowlen),
        North(p) if graph[p] == 'F' => East(p + 1),
        North(p) if graph[p] == '7' => West(p - 1),
        South(p) if graph[p] == '|' => South(p + rowlen),
        South(p) if graph[p] == 'L' => East(p + 1),
        South(p) if graph[p] == 'J' => West(p - 1),
        East(p)  if graph[p] == '-' => East(p + 1),
        East(p)  if graph[p] == '7' => South(p + rowlen),
        East(p)  if graph[p] == 'J' => North(p - rowlen),
        West(p)  if graph[p] == '-' => West(p - 1),
        West(p)  if graph[p] == 'F' => South(p + rowlen),
        West(p)  if graph[p] == 'L' => North(p - rowlen),
        _ => panic!("Unsupport movement: {mv:?}")
    }
}

fn move_up(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if pos >= rowlen {
        let c = graph[pos - rowlen];
        if c == '|' ||  c == '7' || c == 'F' {
            return Some(pos - rowlen)
        }
    }

    None
}

fn move_dn(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if pos < graph.len() - rowlen {
        let c = graph[pos + rowlen];
        if c == '|' ||  c == 'L' || c == 'J' {
            return Some(pos + rowlen)
        }
    }

    None
}

fn move_lt(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if !pos.is_multiple_of(rowlen) {
        let c = graph[pos - 1];
        if c == '-' || c == 'L' || c == 'F' {
            return Some(pos - 1)
        }
    }

    None
}

fn move_rt(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if pos % rowlen < rowlen - 1 {
        let c = graph[pos + 1];
        if c == '-' || c == 'J' || c == '7' {
            return Some(pos + 1)
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 7102);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 363);
    }

    #[test]
    fn example1_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn example2_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 4);
    }

    #[test]
    fn example3_part_two()
    {
        let input = include_str!("../example3.txt");
        assert_eq!(part_two(input), 8);
    }

    #[test]
    fn example4_part_two()
    {
        let input = include_str!("../example4.txt");
        assert_eq!(part_two(input), 10);
    }
}
