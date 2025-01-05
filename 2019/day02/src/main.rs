fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, true);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, alarm: bool) -> usize
{
    let mut program = input.split(',')
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();
    if alarm {
        program[1] = 12;
        program[2] = 2;
    }

    execute(&mut program)
}

fn part_two(input: &str) -> usize
{
    let program = input.split(',')
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            if execute(&mut p) == 19690720 {
                return 100 * noun + verb
            }
        }
    }

    0
}

fn execute(program: &mut [usize]) -> usize
{
    for i in (0..program.len()).step_by(4) {
        if program[i] == 99 { break; }
        let (a, b, c) = (program[i+1], program[i+2], program[i+3]);
        program[c] = match program[i] {
            1 => program[a] + program[b],
            2 => program[a] * program[b],
            _ => unreachable!()
        }
    }

    program[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, true), 3850704);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 6718);
    }

    #[test]
    fn example_part_one()
    {
        assert_eq!(part_one("1,0,0,0,99", false), 2);
        assert_eq!(part_one("1,1,1,4,99,5,6,0,99", false), 30);
        assert_eq!(part_one("1,9,10,3,2,3,11,0,99,30,40,50", false), 3500);
    }
}