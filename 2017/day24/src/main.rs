
fn main() {
    use std::{fs, time::Instant};
    
    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let t = Instant::now();
    println!("Part 1: {} {:?}", part_one(&input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} {:?}", part_two(&input), t.elapsed());
}

fn load(input: &str) -> Vec<[i32;2]> {
    input.lines()
        .map(|s| {
            let mut it = s.split('/');
            let a = it.next().unwrap().parse::<i32>().unwrap();
            let b = it.next().unwrap().parse::<i32>().unwrap();

            [a, b]
        })
        .collect()
}

#[derive(Clone)]
struct State {
    port: i32,
    score: i32,
    length: i32,
    components: Vec<[i32;2]>,
}

fn part_one(ports: &[[i32;2]]) -> i32 {
    use std::collections::VecDeque;
    
    let start = State { port: 0, score: 0, length: 0, components: ports.to_vec() };
    
    let mut strongest = start.clone();
    let mut q = VecDeque::from([start]);
    while let Some(st) = q.pop_front() {
        if st.score > strongest.score {
            strongest = st.clone();
        }

        st.components.iter()
            .enumerate()
            .filter_map(|(i, p)| (p[0] == st.port || p[1] == st.port).then_some(i))
            .for_each(|i| {
                let mut components = st.components.clone();
                let ports = components.remove(i);
                let port  = if st.port == ports[0] { ports[1] } else { ports[0] };
                let score = st.score + ports[0] + ports[1];
                let state = State { port, score, components, length: st.length + 1 };
                q.push_back(state)
            })
    }
    
    strongest.score
}

fn part_two(ports: &[[i32;2]]) -> i32 {
    use std::collections::VecDeque;
    
    let start = State { port: 0, score: 0, length: 0, components: ports.to_vec() };
    
    let mut bridges = vec![];
    let mut q = VecDeque::from([start]);
    while let Some(st) = q.pop_front() {
        let v = st.components.iter()
            .enumerate()
            .filter_map(|(i, p)| (p[0] == st.port || p[1] == st.port).then_some(i))
            .collect::<Vec<_>>();

        if v.is_empty() {
            bridges.push(st)
        } else {
            v.iter()
                .for_each(|i| {
                    let mut components = st.components.clone();
                    let ports = components.remove(*i);
                    let port  = if st.port == ports[0] { ports[1] } else { ports[0] };
                    let score = st.score + ports[0] + ports[1];
                    let state = State { port, score, components, length: st.length + 1 };
                    q.push_back(state)
                })
        }
    }
    
    bridges.sort_by_key(|st| (st.length, st.score));
    bridges.last().unwrap().score
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn input_part_one() {
        let input = load(&fs::read_to_string("./input.txt").unwrap());
        assert_eq!(part_one(&input), 1656);
    }

    #[test]
    fn input_part_two() {
        let input = load(&fs::read_to_string("./input.txt").unwrap());
        assert_eq!(part_two(&input), 1642);
    }
}
