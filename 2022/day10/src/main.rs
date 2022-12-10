
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let signals = part_one(input);
    println!("Part 1: {} ({:?})", signals, t.elapsed());

    let t = Instant::now();
    let image = part_two(input);
    println!("Part 2: {} ({:?})", image, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    let mut x = 1;
    let mut signal = 0;
    let mut signal_multiplier = 1;

    let mut lines = input.lines();
    let mut op = Ops::new(lines.next().unwrap());
    let mut cycles = op.cycles();

    (1..=220)
        .for_each(|cycle| {
            if cycle % (20 * signal_multiplier) == 0 {
                signal += x * cycle;
                signal_multiplier += 2;
            }
            cycles -= 1;
            if cycles == 0 {
                op.exec(&mut x);
                op = Ops::new(lines.next().unwrap());
                cycles = op.cycles();
            }
        });

    signal
}

fn part_two(input: &str) -> String {
    let mut x = 1;

    let mut lines = input.lines();
    let mut op = Ops::new(lines.next().unwrap());
    let mut cycles = op.cycles();

    (0..240)
        .for_each(|cycle| {
            let h_pos = cycle % 40;
            let pixel = if h_pos >= x-1 && h_pos <= x+1 { '#' } else { '.' };
            print!("{pixel}");

            cycles -= 1;
            if cycles == 0 {
                op.exec(&mut x);
                if let Some(line) = lines.next() {
                    op = Ops::new(line);
                    cycles = op.cycles();
                }
            }

            if h_pos == 39 { println!() }
        });

    "PBZGRAZA".into()
}


#[allow(non_camel_case_types)]
enum Ops {
    noop,
    addx (i32),
}

impl Ops {
    fn new(inst: &str) -> Self {
        if inst == "noop" {
            Ops::noop
        } else {
            let (_, v) = inst.split_once(' ').unwrap();
            Ops::addx(v.parse::<i32>().unwrap())
        }
    }

    fn cycles(&self) -> i32 {
        match self {
            Ops::noop => 1,
            Ops::addx {..} => 2,
        }
    }

    fn exec(&self, reg: &mut i32) {
        if let Ops::addx(v) = self {
            *reg += v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let tail = part_one(input);
        assert_eq!(tail, 13440);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let signals = part_one(input);
        assert_eq!(signals, 13140);
    }
}
