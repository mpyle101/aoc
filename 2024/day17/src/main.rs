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

fn part_one(input: &str) -> String
{
    let (a, b, c, program) = load(input);
    execute([a, b, c], &program).iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_two(input: &str) -> u64
{
    use pathfinding::directed::dfs::dfs;

    let (_, b, c, p) = load(input);

    let mut g = p.clone();
    g.reverse();

    // DFS works over straight find the number based on the
    // next 3 bits because there are multiple over all values
    // that will give you a sequence matching the program and
    // dfs will back track to the next one when the matching
    // fails.
    let result = dfs(
        (0, vec![]),
        |(n, _)| neighbors(*n, b, c, &p, &g),
        |(_, v)| v.len() == p.len()
    ).unwrap();

    result.last().unwrap().0
}

fn neighbors(n: u64, b: u64, c: u64, p: &[u64], g: &[u64]) -> Vec<(u64, Vec<u64>)>
{
    // It's a 3 bit computer. Each loop produces one output and
    // them the value in the A register loses 3 bits (divided by 8).
    // So each 3 bits of the starting A value turns into the output
    // value where the highest bits affect the last value. So, we
    // build up by finding an input that generates the program
    // instructions 3 bits at a time by taking the current value,
    // adding all possible combinations of 3 bits to the right and
    // running that through the program looking for a match.
    (0..8)
        .map(|i| {
            let a = (n << 3) | i;
            (a, execute([a, b, c], p))
        })
        .filter(|(_, v)| {
            v.len() <= p.len() &&
            v.iter()
                .rev()
                .zip(g)
                .all(|(a, b)| a == b)
        })
        .collect()
}

fn execute(mut regs: [u64; 3], program: &[u64]) -> Vec<u64>
{
    let mut ip = 0;
    let mut stdout = Vec::with_capacity(16);
    while ip < program.len() {
        let (nx, out) = process(ip, &mut regs, program);
        if let Some(n) = out { stdout.push(n) }
        ip = nx
    }

    stdout
}

fn process(mut ip: usize, regs: &mut[u64; 3], program: &[u64]) -> (usize, Option<u64>)
{
    let mut out = None;

    match program[ip] {
        0 => {  // adv
            ip += 1;
            let d = 2_u64.pow(combo(program[ip], regs) as u32);
            regs[0] /= d;
        },
        1 => {  // bxl
            ip += 1;
            regs[1] ^= program[ip];
        },
        2 => {  // bst
            ip += 1;
            regs[1] = combo(program[ip], regs) % 8;
        },
        3 => {  // jnz
            ip += 1;
            if regs[0] != 0 { 
                ip = program[ip] as usize;
                return (ip, None);
            }
        },
        4 => {  // bxc
            ip += 1;
            regs[1] ^= regs[2];
        },
        5 => {  // out
            ip += 1;
            out = Some(combo(program[ip], regs) % 8);
        },
        6 => {  // bdv
            ip += 1;
            let d = 2_u64.pow(combo(program[ip], regs) as u32);
            regs[1] = regs[0] / d;
        },
        7 => {  // cdv
            ip += 1;
            let d = 2_u64.pow(combo(program[ip], regs) as u32);
            regs[2] = regs[0] / d;
        },
        _ => unreachable!()
    }

    (ip + 1, out)
}

fn combo(n: u64, regs: &[u64; 3]) -> u64
{
    match n {
        0..=3 => n,
        4..=6 => regs[n as usize - 4],
        _ => unreachable!()
    }
}

fn load(input: &str) -> (u64, u64, u64, Vec<u64>)
{
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut line = s1.lines();
    let s = line.next().unwrap();
    let a = s[12..].parse::<u64>().unwrap();
    let s = line.next().unwrap();
    let b = s[12..].parse::<u64>().unwrap();
    let s = line.next().unwrap();
    let c = s[12..].parse::<u64>().unwrap();

    let p = s2[9..].split(',')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    (a, b, c, p)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "7,5,4,3,4,5,3,4,6");
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 164278899142333);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 117440);
    }

}
