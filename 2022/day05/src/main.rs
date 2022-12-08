type Action = (i32, usize, usize);

fn main() {
    use std::{fs, time::Instant};

    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let t = Instant::now();
    let crates = part_one(&input);
    println!("Part 1: {} ({:?})", crates, t.elapsed());

    let t = Instant::now();
    let crates = part_two(&input);
    println!("Part 2: {} ({:?})", crates, t.elapsed());
}

fn load(input: &str) -> Vec<Action> {
    input.lines()
        .map(|line| {
            let v: Vec<_> = line.split_whitespace().collect();
            (
                v[1].parse::<i32>().unwrap(),
                v[3].parse::<usize>().unwrap() - 1,
                v[5].parse::<usize>().unwrap() - 1
            )
        })
        .collect()
}

fn part_one(actions: &[Action]) -> String {
    actions.iter()
        .fold(stacks(), |mut st, (n, from, to)| {
            (0..*n).for_each(|_| {
                let c = st[*from].pop().unwrap();
                st[*to].push(c)
            });
            st
        })
        .iter()
        .filter_map(|st| st.chars().last())
        .collect()
}

fn part_two(actions: &[Action]) -> String {
    actions.iter()
        .fold(stacks(), |mut st, (n, from, to)| {
            let len = st[*from].len();
            let s = st[*from].split_off(len - *n as usize);
            st[*to] += &s;
            st
        })
        .iter()
        .filter_map(|st| st.chars().last())
        .collect()
}

/*
[N]             [R]             [C]
[T] [J]         [S] [J]         [N]
[B] [Z]     [H] [M] [Z]         [D]
[S] [P]     [G] [L] [H] [Z]     [T]
[Q] [D]     [F] [D] [V] [L] [S] [M]
[H] [F] [V] [J] [C] [W] [P] [W] [L]
[G] [S] [H] [Z] [Z] [T] [F] [V] [H]
[R] [H] [Z] [M] [T] [M] [T] [Q] [W]
 1   2   3   4   5   6   7   8   9 
*/
fn stacks() -> Vec<String> {
    vec![
        String::from("RGHQSBTN"),
        String::from("HSFDPZJ"),
        String::from("ZHV"),
        String::from("MZJFGH"),
        String::from("TZCDLMSR"),
        String::from("MTWVHZJ"),
        String::from("TFPLZ"),
        String::from("QVWS"),
        String::from("WHLMTDNC")
    ]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = load(include_str!("../input.txt"));

        let crates = part_one(&input);
        assert_eq!(crates, "PTWLTDSJV");

        let crates = part_two(&input);
        assert_eq!(crates, "WZMFVGGZP");
    }
}