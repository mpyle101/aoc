
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let quality_level = part_one(input);
    println!("Part 1: {} ({:?})", quality_level, t.elapsed());

    let t = Instant::now();
    let geodes = part_two(input);
    println!("Part 2: {} ({:?})", geodes, t.elapsed());
}

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let blue_prints = load(input);

    blue_prints.iter()
        .map(Factory::new)
        .map(|f| mine(&f, 24))
        .map(|f| f.quality_level())
        .sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    let blue_prints = load(input);

    blue_prints.iter()
        .take(3)
        .map(Factory::new)
        .map(|f| mine(&f, 32))
        .map(|f| f.minerals[3])
        .product::<i32>()
}

fn load(input: &str) -> Vec<BluePrint> {
    input.lines()
    .map(|s| s.split(' ').collect::<Vec<_>>())
    .map(|v| {
        let id   = v[1].strip_suffix(':').unwrap().parse::<usize>().unwrap();
        let ore  = v[6].parse::<i32>().unwrap();
        let clay = v[12].parse::<i32>().unwrap();
        let obsidian = (
            v[18].parse::<i32>().unwrap(),
            v[21].parse::<i32>().unwrap(),
        );
        let geode = (
            v[27].parse::<i32>().unwrap(),
            v[30].parse::<i32>().unwrap(),
        );

        BluePrint { id, ore, clay, geode, obsidian }
    })
    .collect::<Vec<_>>()
}

fn mine(factory: &Factory, minutes: i32) -> Factory {
    use std::cmp::Reverse;
    use std::collections::HashSet;

    let mut seen = HashSet::<Factory>::new();

    let mut states = vec![*factory];
    for _ in 1..=minutes {
        let mut next = vec![];
        for state in states {
            for st in state.states() {
                if seen.insert(st) {
                    next.push(st);
                }
            }
        };

        next.sort_by_key(|st| Reverse(st.minerals[3]));
        states = if next.len() > 10000000 {
            next[0..10000000].to_vec()
        } else {
            next
        };
    }
    
    states[0]
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BluePrint {
    id: usize,
    ore: i32,               // ore
    clay: i32,              // ore
    geode: (i32, i32),      // ore, obsidian
    obsidian: (i32, i32)    // ore, clay
}

#[derive(Clone, Debug)]
enum Robot {
    Ore,
    Clay,
    Geode,
    Obsidian,
}

const ROBOTS: [Robot;4] = [
    Robot::Ore,
    Robot::Clay,
    Robot::Obsidian,
    Robot::Geode,
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Factory {
    bp: BluePrint,
    robots: [i32;4],
    minerals: [i32;4],
}

impl Factory {
    fn new(bp: &BluePrint) -> Factory {
        Factory { bp: *bp, robots: [1,0,0,0], minerals: [0;4] }
    }

    fn with(&self, action: Option<usize>) -> Factory
    {
        let mut st = *self;
        st.minerals.iter_mut()
            .zip(self.robots)
            .for_each(|(m, r)| *m += r);

        if let Some(i) = action {
            st.robots[i] += 1;

            let m = &mut st.minerals;
            let bp = self.bp;
            match ROBOTS[i] {
                Robot::Ore      => m[0] -= bp.ore,
                Robot::Clay     => m[0] -= bp.clay,
                Robot::Obsidian => { m[0] -= bp.obsidian.0; m[1] -= bp.obsidian.1 },
                Robot::Geode    => { m[0] -= bp.geode.0; m[2] -= bp.geode.1 },
            }
        }
        
        st
    }

    fn states(&self) -> Vec<Factory> {
        let mut v = vec![self.with(None)];

        ROBOTS.iter()
            .enumerate()
            .filter(|(_, r)| self.can_build(r))
            .for_each(|(i, _)| v.push(self.with(Some(i))));

        v
    }

    fn quality_level(&self) -> i32 {
        self.bp.id as i32 * self.minerals[3]
    }

    fn can_build(&self, robot: &Robot) -> bool {
        let m  = self.minerals;
        let bp = self.bp;
        match robot {
            Robot::Ore      => m[0] >= bp.ore,
            Robot::Clay     => m[0] >= bp.clay,
            Robot::Obsidian => m[0] >= bp.obsidian.0 && m[1] >= bp.obsidian.1,
            Robot::Geode    => m[0] >= bp.geode.0 && m[2] >= bp.geode.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let quality_level = part_one(input);
        assert_eq!(quality_level, 1192);

        let geodes = part_two(input);
        assert_eq!(geodes, 14725);
    }

    #[test]
    fn example() {
        let input = include_str!("../input.txt");

        let quality_level = part_one(input);
        assert_eq!(quality_level, 33);
    }
}
