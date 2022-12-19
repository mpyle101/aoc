use std::collections::HashSet;

fn main() {
    use std::time::Instant;

    let input = include_str!("../example.txt");

    let t = Instant::now();
    let surface_area = part_one(input);
    println!("Part 1: {} ({:?})", surface_area, t.elapsed());

    let t = Instant::now();
    let surface_area = part_two(input);
    println!("Part 2: {} ({:?})", surface_area, t.elapsed());
}

fn part_one(input: &str) -> usize {
    let cubes = input.lines()
        .map(|s| s.split(','))
        .map(Cube::new)
        .collect::<Vec<_>>();
    
    let v = cubes[0].faces();
    let mut faces: HashSet<Face> = HashSet::from_iter(v.iter().copied());
    cubes.iter().skip(1)
        .for_each(|cube| {
            let v = cube.faces();
            v.iter().for_each(|f| if !faces.remove(f) { faces.insert(*f); })
        });

    faces.len()
}

fn part_two(input: &str) -> usize {
    use std::collections::VecDeque;

    let cubes = input.lines()
        .map(|s| s.split(','))
        .map(Cube::new)
        .collect::<Vec<_>>();

    let max_pt = cubes.iter()
        .map(|c| c.x.max(c.y).max(c.z))
        .max()
        .unwrap();
    let min_pt = cubes.iter()
        .map(|c| c.x.min(c.y).min(c.z))
        .min()
        .unwrap();
    let start = Cube::from(min_pt, min_pt, min_pt);

    let mut q = VecDeque::from([start]);
    let mut seen = HashSet::new();
    let mut steam = HashSet::from([start]);

    let lava = cubes.iter().collect::<HashSet<_>>();
    while let Some(cube) = q.pop_front() {
        if !lava.contains(&cube) && !seen.contains(&cube) {
            steam.insert(cube);
            cube.neighbors(min_pt, max_pt).iter()
                .for_each(|c| q.push_back(*c));
        }
        seen.insert(cube);
    }

    cubes.iter()
        .map(|cube| cube.neighbors(min_pt, max_pt).iter()
            .filter(|c| !lava.contains(c) && steam.contains(c))
            .count())
        .sum()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Face {
    pts: [(usize, usize, usize);4],
    plane: char,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn new<'a>(mut it: impl Iterator<Item = &'a str>) -> Cube {
        let x = it.next().unwrap().parse::<usize>().unwrap();
        let y = it.next().unwrap().parse::<usize>().unwrap();
        let z = it.next().unwrap().parse::<usize>().unwrap();

        Cube { x, y, z }
    }

    fn from(x: usize, y: usize, z: usize) -> Cube {
        Cube { x, y, z }
    }

    fn inside(&self, min_pt: usize, max_pt: usize) -> bool {
        let range = min_pt..=max_pt;
        range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
    }

    fn faces(&self) -> Vec<Face> {
        let (x, y, z) = (self.x, self.y, self.z);
        vec![
            Face { plane: 'X', pts: [(x, y-1, z-1), (x, y-1, z), (x, y, z-1), (x, y, z)] },
            Face { plane: 'Y', pts: [(x-1, y, z-1), (x-1, y, z), (x, y, z-1), (x, y, z)] },
            Face { plane: 'Z', pts: [(x-1, y-1, z), (x-1, y, z), (x, y-1, z), (x, y, z)] },
            Face { plane: 'X', pts: [(x-1, y-1, z-1), (x-1, y-1, z), (x-1, y, z-1), (x-1, y, z)] },
            Face { plane: 'Y', pts: [(x-1, y-1, z-1), (x-1, y-1, z), (x, y-1, z-1), (x, y-1, z)] },
            Face { plane: 'Z', pts: [(x-1, y-1, z-1), (x-1, y, z-1), (x, y-1, z-1), (x, y, z-1)] },
        ]
    }

    fn neighbors(&self, min_pt: usize, max_pt: usize) -> Vec<Cube> {
        [
            Cube::from(self.x+1, self.y, self.z),
            Cube::from(self.x-1, self.y, self.z),
            Cube::from(self.x, self.y+1, self.z),
            Cube::from(self.x, self.y-1, self.z),
            Cube::from(self.x, self.y, self.z+1),
            Cube::from(self.x, self.y, self.z-1),
        ]
        .into_iter()
        .filter(|cube| cube.inside(min_pt, max_pt))
        .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let surface_area = part_one(input);
        assert_eq!(surface_area, 4500);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let surface_area = part_one(input);
        assert_eq!(surface_area, 64);

        let surface_area = part_two(input);
        assert_eq!(surface_area, 58);
    }
}
