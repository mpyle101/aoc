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

fn part_one(input: &str) -> i64
{
    use vm::Vm;

    let mut vm = Vm::new(input).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(1);
    vm.exec().unwrap();
    stdout.flush()
}

fn part_two(input: &str) -> i64
{
    use vm::Vm;

    let mut vm = Vm::new(input).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(5);
    vm.exec().unwrap();
    stdout.flush()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 13787043);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3892695);
    }
}