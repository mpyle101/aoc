fn main()
{
    use std::time::Instant;

    let input = include_str!("../example3.txt");

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
            if let Some(y) = line.find('S') {
                start = x * rowlen + y;
            }
            rowlen = line.len();
            graph.extend(line.chars());
        });

    let (mut p1, mut p2) = first_move(start, &graph, rowlen);
    let mut steps = 1;
    while p1.pos() != p2.pos() {
        p1 = next_move(p1, &graph, rowlen);
        p2 = next_move(p2, &graph, rowlen);
        steps += 1
    }
    
    steps
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
            if let Some(y) = line.find('S') {
                start = x * rowlen + y;
            }
            rowlen = line.len();
            graph.extend(line.chars());
        });

    let mut circuit = HashSet::from([start]);
    let (mut p1, mut p2) = first_move(start, &graph, rowlen);
    while p1.pos() != p2.pos() {
        circuit.insert(p1.pos());
        circuit.insert(p2.pos());

        p1 = next_move(p1, &graph, rowlen);
        p2 = next_move(p2, &graph, rowlen);
    }
    circuit.insert(p1.pos());
    
    let mut found = Vec::new();
    let tiles = (0..graph.len())
        .filter(|p| !circuit.contains(p))
        .filter(|&p| {
            let q = p + rowlen - (p % rowlen);
            let count = (p+1..q)
                .filter(|n| graph[*n] != '-' && circuit.contains(n))
                .count();
            if count % 2 == 1 { found.push(p); println!("{p}: {count}") }
            count % 2 == 1
        })
        .count();

    (0..graph.len() / rowlen)
        .for_each(|x| {
            (0..rowlen)
                .for_each(|y| {
                    let p = x * rowlen + y;
                    if circuit.contains(&p) {
                        print!("{}", graph[p])
                    } else if found.contains(&p) {
                        print!("*")
                    } else {
                        print!(".")
                    }
                });
            println!();
        });

    tiles
}

#[derive(Debug, PartialEq)]
enum Move
{
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Idle,
}
impl Move
{
    fn pos(&self) -> usize
    {
        use Move::*;

        match self {
            North(p) | South(p) | East(p) | West(p) => *p,
            Idle => panic!("Should never be idle!")
        }
    }
}

fn first_move(start: usize, graph: &[char], rowlen: usize) -> (Move, Move)
{
    use Move::*;

    let mut p1 = Idle;
    let mut p2 = Idle;

    if let Some(p) = move_up(start, graph, rowlen) {
        p1 = North(p);
    }
    if let Some(p) = move_down(start, graph, rowlen) {
        if let Move::Idle = p1 { p1 = South(p) } else { p2 = South(p) }
    }
    if let Move::Idle = p2 {
        if let Some(p) = move_left(start, graph, rowlen) {
            if let Move::Idle = p1 { p1 = West(p) } else { p2 = West(p) }
        }
    }
    if let Move::Idle = p2 {
        if let Some(p) = move_right(start, graph, rowlen) {
            p2 = East(p)
        }
    }

    (p1, p2)
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
        _ => panic!("Unsupport movement: {:?}", mv)
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

fn move_down(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if pos < graph.len() - rowlen {
        let c = graph[pos + rowlen];
        if c == '|' ||  c == 'L' || c == 'J' {
            return Some(pos + rowlen)
        }
    }

    None
}

fn move_left(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
{
    if pos % rowlen != 0 {
        let c = graph[pos - 1];
        if c == '-' || c == 'L' || c == 'F' {
            return Some(pos - 1)
        }
    }

    None
}

fn move_right(pos: usize, graph: &[char], rowlen: usize) -> Option<usize>
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
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 8);
    }
}
