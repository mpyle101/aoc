
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let pressure = part_one(input);
    println!("Part 1: {} ({:?})", pressure, t.elapsed());

    let t = Instant::now();
    let pressure = part_two(input);
    println!("Part 2: {} ({:?})", pressure, t.elapsed());
}

fn part_one(input: &str) -> u32 {
    use std::collections::HashMap;

    let valves = load(input);
    let mut states = HashMap::new();
    states.insert(State::new(0, &valves), 0);

    for m in 1..=30 {
        let mut next: HashMap<State, u32> = HashMap::new();
        states.iter()
            .for_each(|(st, p)| {
                if !st.is_open() && valves[st.loc].rate > 0 {
                    let st1 = st.open();
                    let p1  = p + ((30 - m) * valves[st.loc].rate);
                    match next.get_mut(&st1) {
                        None => { next.insert(st1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }
                [st.loc].iter().chain(valves[st.loc].tunnels.iter())
                    .for_each(|t| {
                        let st1 = st.move_to(*t);
                        match next.get_mut(&st1) {
                            None => { next.insert(st1, *p); }
                            Some(p1) => if p > p1 { *p1 = *p }
                        }
                    })
            });

        states = next
    }

    *states.values().max().unwrap()
}

fn part_two(input: &str) -> u32 {
    use std::collections::HashMap;

    let valves = load(input);
    let mut states = HashMap::new();
    states.insert((0usize, 0usize, vec![false;valves.len()]), 0);

    for m in 1..=26 {
        let mut next: HashMap<(usize, usize, Vec<bool>), u32> = HashMap::new();
        states.iter()
            .for_each(|((l, e, v), p)| {
                if !v[*l] && valves[*l].rate > 0 {
                    let mut v1 = v.clone();
                    v1[*l] = true;
                    let st1 = (*l, *e, v1);
                    let p1 = p + ((30 - m) * valves[*l].rate);
                    match next.get_mut(&st1) {
                        None => { next.insert(st1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }
                if !v[*e] && valves[*e].rate > 0 {
                    let mut v1 = v.clone();
                    v1[*e] = true;
                    let st1 = (*l, *e, v1);
                    let p1 = p + ((30 - m) * valves[*e].rate);
                    match next.get_mut(&st1) {
                        None => { next.insert(st1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }
                if e != l && !v[*l] && valves[*l].rate > 0 && !v[*e] && valves[*e].rate > 0 {
                    let mut v1 = v.clone();
                    v1[*l] = true;
                    v1[*e] = true;
                    let st1 = (*l, *e, v1);
                    let mut p1 = *p;
                    p1 += (30 - m) * valves[*l].rate;
                    p1 += (30 - m) * valves[*e].rate;
                    match next.get_mut(&st1) {
                        None => { next.insert(st1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }

                [*l].iter().chain(valves[*l].tunnels.iter())
                    .for_each(|l1| [*e].iter().chain(valves[*e].tunnels.iter())
                        .for_each(|e1| {
                            let st1 = (*l1, *e1, v.clone());
                            match next.get_mut(&st1) {
                                None => { next.insert(st1, *p); }
                                Some(p1) => if p > p1 { *p1 = *p }
                            };
                        }))
            });

        states = next;
    }

    *states.values().max().unwrap()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    loc: usize,
    open: Vec<bool>,
}
impl State {
    fn new(loc: usize, valves: &[Valve]) -> State {
        State { loc, open: vec![false; valves.len()] }
    }

    fn is_open(&self) -> bool {
        self.open[self.loc]
    }

    fn move_to(&self, loc:usize) -> State {
        let mut st = self.clone();
        st.loc = loc;
        st
    }

    fn open(&self) -> State {
        let mut st = self.clone();
        st.open[st.loc] = true;
        st
    }
}

#[derive(Debug)]
struct Valve {
    rate: u32,
    tunnels: Vec<usize>,
}

fn load(input: &str) -> Vec<Valve> {
    let mut tunnels: Vec<_> = input.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|v| {
            let label = v[1];
            let rate = v[4][5..].replace(';', "").parse::<u32>().unwrap();
            let tunnels = v[9..].iter()
                .filter_map(|&s| s.strip_suffix(',').or(Some(s)))
                .collect::<Vec<_>>();

            (label, rate, tunnels)
        })
        .collect();

    tunnels.sort();
    tunnels.iter()
        .map(|(_, r, v)| {
            let idx = v.iter()
                .filter_map(|t| tunnels.iter().position(|(label, _, _)| t == label))
                .collect();
            Valve { rate: *r, tunnels: idx }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let calories = part_one(input);
        assert_eq!(calories, 1775);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let pressure = part_one(input);
        assert_eq!(pressure, 1651);

        let pressure = part_two(input);
        assert_eq!(pressure, 1707);
    }
}
