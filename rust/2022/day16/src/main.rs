
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> u32 {
    use std::collections::HashMap;

    let valves = load(input);
    let mut states = HashMap::new();
    states.insert(State::new(0), 0);

    for m in 1..=30 {
        let mut next: HashMap<State, u32> = HashMap::new();
        states.iter()
            .for_each(|(st, p)| {
                if !st.is_open() && valves[st.idx()].rate > 0 {
                    let s1 = st.open();
                    let p1 = p + ((30 - m) * valves[st.idx()].rate);
                    match next.get_mut(&s1) {
                        None => { next.insert(s1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }
                valves[st.idx()].iter().chain([st.idx()])
                    .for_each(|t| {
                        let s1 = st.move_to(t);
                        match next.get_mut(&s1) {
                            None => { next.insert(s1, *p); }
                            Some(p1) => if p > p1 { *p1 = *p }
                        }
                    })
            });

        // Don't drag along states which are never going to catch up.
        states = if next.len() < 100 {
            next
        } else {
            let mut v = next.iter().collect::<Vec<_>>();
            v.sort_by(|a, b| b.1.cmp(a.1));
            v[0..100].iter().cloned().map(|(a, b)| (*a, *b)).collect()
        };
    }

    *states.values().max().unwrap()
}

fn part_two(input: &str) -> u32 {
    use std::collections::HashMap;
    
    let valves = load(input);
    let mut states = HashMap::new();
    states.insert((0usize, 0usize, 0usize), 0);

    for m in 1..=26 {
        let mut next: HashMap<(usize, usize, usize), u32> = HashMap::new();
        states.iter()
            .for_each(|((h, e, v), p)| {
                let vh = v & 1 << *h != 0;
                let ve = v & 1 << *e != 0;

                // I open a valve, the elephant stays or moves
                if !vh && valves[*h].rate > 0 {
                    let v1 = v | 1 << *h;
                    let p1 = p + ((26 - m) * valves[*h].rate);
                    valves[*e].iter().chain([*e])
                        .for_each(|e1| {
                            let st1 = (*h, e1, v1);
                            match next.get_mut(&st1) {
                                None => { next.insert(st1, p1); }
                                Some(p2) => if p1 > *p2 { *p2 = p1 }
                            }
                        })
                }

                // Elephant opens a valve, I stay or move
                if e != h && !ve && valves[*e].rate > 0 {
                    let v1 = v | 1 << *e;
                    let p1 = p + ((26 - m) * valves[*e].rate);
                    valves[*h].iter().chain([*h])
                        .for_each(|h1| {
                            let st1 = (h1, *e, v1);
                            match next.get_mut(&st1) {
                                None => { next.insert(st1, p1); }
                                Some(p2) => if p1 > *p2 { *p2 = p1 }
                            }
                        })
                }
                
                // We both open valves.
                if e != h && !vh && valves[*h].rate > 0 && !ve && valves[*e].rate > 0 {
                    let v1 = v | 1 << *h | 1 << *e;
                    let p1 = p + ((26 - m) * valves[*h].rate) + ((26 - m) * valves[*e].rate);
                    let s1 = (*h, *e, v1);
                    match next.get_mut(&s1) {
                        None => { next.insert(s1, p1); }
                        Some(p2) => if p1 > *p2 { *p2 = p1 }
                    }
                }

                // We both move.
                valves[*h].iter().chain([*h])
                    .for_each(|h1| valves[*e].iter().chain([*e])
                        .for_each(|e1| {
                            let st1 = (h1, e1, *v);
                            match next.get_mut(&st1) {
                                None => { next.insert(st1, *p); }
                                Some(p1) => if p > p1 { *p1 = *p }
                            };
                        }))
            });

        // Don't drag along states which are never going to catch up.
        states = if next.len() < 1000 {
            next
        } else {
            let mut v = next.iter().collect::<Vec<_>>();
            v.sort_by(|a, b| b.1.cmp(a.1));
            v[0..1000].iter().cloned().map(|(a, b)| (*a, *b)).collect()
        };
    }

    *states.values().max().unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    ix: usize,
    open: usize,
}
impl State {
    fn new(ix: usize) -> State {
        State { ix: 1 << ix, open: 0 }
    }

    fn idx(&self) -> usize {
        self.ix.trailing_zeros() as usize
    }

    fn is_open(&self) -> bool {
        self.open & self.ix == self.ix
    }

    fn move_to(&self, ix: usize) -> State {
        State { ix: 1 << ix, open: self.open }
    }

    fn open(&self) -> State {
        State { ix: self.ix, open: self.open | self.ix }
    }
}

#[derive(Debug)]
struct Valve {
    rate: u32,
    tunnels: usize,
}
impl Valve {
    fn iter(&self) -> TunnelIter {
        TunnelIter::new(self.tunnels)
    }
}

#[derive(Debug)]
struct TunnelIter {
    ix: u8,
    tunnels: usize,
}
impl TunnelIter {
    fn new(tunnels: usize) -> TunnelIter {
        Self { tunnels, ix: 0 }
    }
}
impl Iterator for TunnelIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tunnels == 0 {
            None
        } else {
            let ix = self.tunnels.trailing_zeros() + 1;
            self.tunnels >>= ix;
            self.ix += ix as u8;
            Some((self.ix - 1) as usize)
        }
    }
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
                .fold(0, |t, i| t | 1 << i);
            Valve { rate: *r, tunnels: idx }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1775);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2351);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 1651);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1707);
    }
}
