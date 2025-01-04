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

    let mut ip  = 0;
    let mut reg = [0;4];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg)
    }

    reg[0]
}

fn part_two(input: &str) -> i32
{
    
    let program = load(input);

    let mut ip  = 0;
    let mut reg = [0, 0, 1, 0];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg)
    }

    reg[0]
}

#[allow(non_camel_case_types)]
enum Cmd {
    cpn(usize, i32),
    cpr(usize, usize),
    inc(usize),
    dec(usize),
    jnz(usize, i32),
    jmp(i32),
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut [i32;4]) -> usize
    {
        use Cmd::*;
        (ip as i32 + match self {
            cpn(r, n) => { reg[*r] = *n; 1 },
            cpr(a, b) => { reg[*b] = reg[*a]; 1 },
            inc(r)    => { reg[*r] += 1; 1 },
            dec(r)    => { reg[*r] -= 1; 1 },
            jnz(r, n) => if reg[*r] != 0 { *n } else { 1 },
            jmp(n)    => *n
        }) as usize
    }
}

fn load(input: &str) -> Vec<Cmd>
{
    use Cmd::*;

    input.lines()
        .map(|line| {
            let mut it = line.split(' ');
            match it.next() {
                Some("cpy") => {
                    let a = it.next().unwrap();
                    let b = it.next().map(|s| s.as_bytes()[0] - b'a').unwrap();
                    if let Ok(n) = a.parse::<i32>() {
                        cpn(b as usize, n)
                    } else {
                        let a = (a.as_bytes()[0] - b'a') as usize;
                        cpr(a, b as usize)
                    }
                },
                Some("inc") => {
                    let r = it.next().map(|s| s.as_bytes()[0] - b'a').unwrap();
                    inc(r as usize)
                },
                Some("dec") => {
                    let r = it.next().map(|s| s.as_bytes()[0] - b'a').unwrap();
                    dec(r as usize)
                },
                Some("jnz") => {
                    let a = it.next().unwrap();
                    let b = it.next().and_then(|s| s.parse::<i32>().ok());
                    if a.parse::<i32>().is_ok() {
                        jmp(b.unwrap())
                    } else {
                        let a = (a.as_bytes()[0] - b'a') as usize;
                        jnz(a, b.unwrap())
                    }
                },
                _ => unreachable!()
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 318083);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 9227737);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 42);
    }
}