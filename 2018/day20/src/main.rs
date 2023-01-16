use std::collections::HashMap;
use petgraph::prelude::{UnGraph, NodeIndex};

type Room = (i32, i32);

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {}  ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {}  ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use petgraph::algo::dijkstra;

    let mut regex = load(input);
    let mut facility = Facility::default();
    walk(&mut regex, (0, 0), &mut facility);

    // Find shortest path from start to all rooms, filter by
    // deadends and keep the longest.
    let paths = dijkstra(&facility.graph, facility.start(), None, |_| 1);
    *facility.deadends()
        .filter_map(|ix| paths.get(&ix))
        .max()
        .unwrap()
}

fn part_two(input: &str) -> usize
{
    use petgraph::algo::dijkstra;

    let mut regex = load(input);
    let mut facility = Facility::default();
    walk(&mut regex, (0, 0), &mut facility);

    dijkstra(&facility.graph, facility.start(), None, |_| 1)
        .values()
        .filter(|n| **n > 999)
        .count()
}

fn load(input: &str) -> impl Iterator<Item = char> + '_
{
    // We don't need the begining and end characters (^, $).
    input.chars().skip(1).take(input.len() - 2)
}

struct Facility {
    graph: UnGraph<Room, Room>,
    rooms: HashMap<Room, NodeIndex>,
}
impl Facility {
    fn default() -> Facility
    {
        let mut graph = UnGraph::<Room, Room>::default();
        let rooms = HashMap::from([((0, 0), graph.add_node((0, 0)))]);
        
        Facility { graph, rooms }
    }

    fn start(&self) -> NodeIndex
    {
        *self.rooms.get(&(0, 0)).unwrap()
    }

    fn deadends(&self) -> impl Iterator<Item = NodeIndex> + '_
    {
        self.graph.node_indices()
            .filter(|&ix| self.graph.neighbors(ix).count() == 1)
    }

    fn add_door(&mut self, a: Room, b: Room)
    {
        let a_ix = *self.rooms.get(&a).unwrap();
        let b_ix = *self.rooms.entry(b).or_insert_with(|| self.graph.add_node(b));
        self.graph.update_edge(a_ix, b_ix, (0, 0));
    }
}

fn walk<I>(iter: &mut I, start: Room, facility: &mut Facility)
    where I: Iterator<Item = char>
{
    let mut room = start;

    while let Some(c) = iter.next() {
        if c == '(' {
            walk(iter, room, facility);
        } else if c == ')' {
            return
        } else if c == '|' {
            room = start
        } else {
            let (dx, dy) = match c {
                'N' => ( 0, -1),
                'S' => ( 0,  1),
                'E' => ( 1,  0),
                'W' => (-1,  0),
                _ => unreachable!()
            };
            let next = (room.0 + dx, room.1 + dy);
            facility.add_door(room, next);
            room = next
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
        assert_eq!(part_one(input), 4018);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 8581);
    }

    #[test]
    fn example1_part_one()
    {
        let input = include_str!("../examples/example1.txt");
        assert_eq!(part_one(input), 10);
    }

    #[test]
    fn example2_part_one()
    {
        let input = include_str!("../examples/example2.txt");
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn example3_part_one()
    {
        let input = include_str!("../examples/example3.txt");
        assert_eq!(part_one(input), 23);
    }

    #[test]
    fn example4_part_one()
    {
        let input = include_str!("../examples/example4.txt");
        assert_eq!(part_one(input), 31);
    }
}
