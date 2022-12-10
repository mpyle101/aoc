use vm::Vm;

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
    let mut vm = Vm::new(input);

    let mut signal = 0;
    let mut signal_multiplier = 1;

    (1..=220)
        .for_each(|cycle| {
            if cycle % (20 * signal_multiplier) == 0 {
                signal += cycle * vm.getx();
                signal_multiplier += 2;
            }

            vm.do_tick();
        });

    signal
}

fn part_two(input: &str) -> String {
    let mut vm = Vm::new(input);

    (0..240)
        .for_each(|cycle| {
            let (x, h_pos) = (vm.getx(), cycle % 40);
            let pixel = if h_pos >= x-1 && h_pos <= x+1 { '#' } else { ' ' };
            if h_pos == 39 { println!("{pixel}") } else { print!("{pixel}")}

            vm.do_tick();
        });

    "PBZGRAZA".into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let signals = part_one(input);
        assert_eq!(signals, 13440);

        let image = part_two(input);
        assert_eq!(image, "PBZGRAZA");
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let signals = part_one(input);
        assert_eq!(signals, 13140);
    }
}
