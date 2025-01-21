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

fn part_one(input: &str) -> u32
{
    let program = load(input);

    let mut ip  = 0;
    let mut reg: [u32;2] = [0, 0];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[1]
}

fn part_two(input: &str) -> u32
{
    let program = load(input);

    let mut ip = 0;
    let mut reg: [u32;2] = [1, 0];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[1]
}

fn load(input: &str) -> Vec<Cmd>
{
    use Cmd::*;

    input.lines()
        .map(|l| {
            let mut it = l.split(' ');
            let cmd = it.next().unwrap();
            let reg = it.next().unwrap();
            if cmd == "jmp" {
                let offset = reg.parse::<i32>().unwrap();
                jmp(offset)
            } else {
                let reg = reg.starts_with('b') as usize;
                match cmd {
                    "hlf" => hlf(reg),
                    "tpl" => tpl(reg),
                    "jie" => {
                        let offset = it.next().unwrap().parse::<i32>().unwrap();
                        jie(reg, offset)
                    },
                    "jio" => {
                        let offset = it.next().unwrap().parse::<i32>().unwrap();
                        jio(reg, offset)
                    },
                    _ => inc(reg)
                }
            }
        })
        .collect()
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cmd {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(i32),
    jie(usize, i32),
    jio(usize, i32),
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut [u32;2]) -> usize
    {
        use Cmd::*;

        match self {
            hlf(r) => { reg[*r] /= 2; ip+1 },
            tpl(r) => { reg[*r] *= 3; ip+1 },
            inc(r) => { reg[*r] += 1; ip+1 },
            jmp(n) => (ip as i32 + n) as usize,
            jie(r, n) => if reg[*r] % 2 == 0 { (ip as i32 + n) as usize } else { ip+1 },
            jio(r, n) => if reg[*r] == 1 { (ip as i32 + n) as usize } else { ip+1 },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 255);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 334);
    }
}