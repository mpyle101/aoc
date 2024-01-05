type Action = (usize, usize, usize);

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> String {
    let (stacks, input) = input.split_once("\n\n").unwrap();
    let actions = load_actions(input);

    let mut stacks = load_stacks(stacks);
    actions.iter()
        .for_each(|(n, src, dst)| {
            (0..*n).for_each(|_| {
                let c = stacks[*src].pop().unwrap();
                stacks[*dst].push(c)
            });
        });
        
    stacks.iter()
        .filter_map(|st| st.last())
        .collect()
}

fn part_two(input: &str) -> String {
    let (stacks, input) = input.split_once("\n\n").unwrap();
    let actions = load_actions(input);

    let mut stacks = load_stacks(stacks);
    actions.iter()
        .for_each(|(n, src, dst)| {
            let len = stacks[*src].len();
            let mut s = stacks[*src].split_off(len - *n);
            stacks[*dst].append(&mut s);
        });
        
    stacks.iter()
        .filter_map(|st| st.last())
        .collect()
}

fn load_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::with_capacity(50);9];
    input.lines()
        .for_each(|line| {
            let iter = line.chars().skip(1);
            iter.step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| stacks[i].push(c))
        });

    stacks.iter_mut()
        .for_each(|st| {
            st.pop();   // remove stack number from last line
            st.reverse()
        });

    stacks
}

fn load_actions(input: &str) -> Vec<Action> {
    input.lines()
        .map(|line| {
            let v: Vec<_> = line.split_whitespace().collect();
            (
                v[1].parse::<usize>().unwrap(),
                v[3].parse::<usize>().unwrap() - 1,
                v[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "PTWLTDSJV");
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "WZMFVGGZP");
    }
}
