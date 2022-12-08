type Action = (i32, usize, usize);

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");
    let (stacks, input) = input.split_once("\n\n").unwrap();

    let stacks = load_stacks(stacks);
    let actions = load_actions(input);

    let t = Instant::now();
    let crates = part_one(&actions, &stacks);
    println!("Part 1: {} ({:?})", crates, t.elapsed());

    let t = Instant::now();
    let crates = part_two(&actions, &stacks);
    println!("Part 2: {} ({:?})", crates, t.elapsed());
}

fn load_stacks(input: &str) -> Vec<String> {
    let mut stacks = vec![String::new(); 9];
    input.lines().for_each(|line| {
        let iter = line.chars().skip(1);
        iter.step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c))
    });

    stacks
        .iter_mut()
        .map(|st| {
            st.pop();
            st.chars().rev().collect()
        })
        .collect()
}

fn load_actions(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let v: Vec<_> = line.split_whitespace().collect();
            (
                v[1].parse::<i32>().unwrap(),
                v[3].parse::<usize>().unwrap() - 1,
                v[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect()
}

fn part_one(actions: &[Action], stacks: &[String]) -> String {
    actions
        .iter()
        .fold(stacks.to_vec(), |mut st, (n, from, to)| {
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

fn part_two(actions: &[Action], stacks: &[String]) -> String {
    actions
        .iter()
        .fold(stacks.to_vec(), |mut st, (n, from, to)| {
            let len = st[*from].len();
            let s = st[*from].split_off(len - *n as usize);
            st[*to] += &s;
            st
        })
        .iter()
        .filter_map(|st| st.chars().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");
        let (stacks, input) = input.split_once("\n\n").unwrap();

        let stacks = load_stacks(stacks);
        let actions = load_actions(input);

        let crates = part_one(&actions, &stacks);
        assert_eq!(crates, "PTWLTDSJV");

        let crates = part_two(&actions, &stacks);
        assert_eq!(crates, "WZMFVGGZP");
    }
}
