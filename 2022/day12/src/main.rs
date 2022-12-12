use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let steps = part_one(input);
    println!("Part 1: {} ({:?})", steps, t.elapsed());

    let t = Instant::now();
    let steps = part_two(input);
    println!("Part 2: {} ({:?})", steps, t.elapsed());
}

fn part_one(input: &str) -> usize {
    use pathfinding::prelude::bfs;

    let hm = HeightMap::new(input);
    let hike = bfs(&hm.start, |&p| hm.steps(p), |&p| p == hm.end);

    hike.unwrap().len() - 1
}

fn part_two(input: &str) -> usize {
    use pathfinding::prelude::bfs;

    let hm = HeightMap::new(input);
    hm.map.iter()
        .filter(|(_, h)| **h == 0)
        .filter_map(|(p, _)| bfs(p, |pos| hm.steps(*pos), |pos| *pos == hm.end))
        .map(|v| v.len() - 1)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct HeightMap {
    start: (i32, i32),
    end: (i32, i32),
    map: HashMap<(i32, i32), u8>,
}

impl HeightMap {
    fn new(input: &str) -> HeightMap {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                let r = row as i32;
                let c = col as i32;

                let height = match byte {
                    b'S' => { start = (r, c); 0 },
                    b'E' => { end = (r, c); 25 }
                    b    => { b - b'a' }
                };
                map.insert((r, c), height);
            }
        }

        HeightMap { start, end, map }
    }

    const STEPS: [(i32, i32);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn steps(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
        let h = self.map.get(&p).unwrap();

        HeightMap::STEPS.iter()
            .map(|&st| (p.0 + st.0, p.1 + st.1))
            .filter_map(|p1| self.map.get(&p1)
                .and_then(|h1| ((*h + 1) >= *h1).then_some(p1)))
            .collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let steps = part_one(input);
        assert_eq!(steps, 456);

        let steps = part_one(input);
        assert_eq!(steps, 454);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let steps = part_one(input);
        assert_eq!(steps, 31);

        let steps = part_two(input);
        assert_eq!(steps, 29);
    }
}
