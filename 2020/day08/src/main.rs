fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> i32
{
    let program = load(input);
    exec_one(&program).unwrap()
}

fn part_two(input: &str) -> i32
{
    let mut program = load(input);
    let ips = (0..program.len())
        .filter(|ip| program[*ip].0 == "jmp" || program[*ip].0 == "nop")
        .collect::<Vec<_>>();

    for ip in ips {
        let inst = program[ip].0;
        program[ip].0 = if inst == "jmp" { "nop" } else { "jmp" };
        if let Some(n) = exec_two(&program) {
            return n
        }
        program[ip].0 = inst
    }
    
    0
}

fn load(input: &str) -> Vec<(&str, i32)>
{
    input.lines()
        .map(|line| {
            let (op, v) = line.split_once(' ').unwrap();
            let v = v.parse::<i32>().unwrap();
            (op, v)
        })
        .collect()
}

fn exec_one(program: &[(&str, i32)]) -> Option<i32>
{
    use std::collections::HashSet;

    let mut ip   = 0;
    let mut acc  = 0;
    let mut seen = HashSet::new();
    loop {
        if seen.insert(ip) {
            ip = match program[ip] {
                ("acc", n) => { acc += n; ip + 1 },
                ("jmp", n) => (ip as i32 + n) as usize,
                ("nop", _) => ip + 1,
                _ => unreachable!()
            }
        } else {
            return Some(acc)
        }
    }
}

fn exec_two(program: &[(&str, i32)]) -> Option<i32>
{
    use std::collections::HashSet;

    let mut ip   = 0;
    let mut acc  = 0;
    let mut seen = HashSet::new();
    while ip < program.len() {
        if seen.insert(ip) {
            ip = match program[ip] {
                ("acc", n) => { acc += n; ip + 1 },
                ("jmp", n) => (ip as i32 + n) as usize,
                ("nop", _) => ip + 1,
                _ => unreachable!()
            }
        } else {
            return None
        }
    }

    Some(acc)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1489);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1539);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 5);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 8);
    }
}