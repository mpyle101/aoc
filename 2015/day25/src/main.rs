fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u64 {

    let target = load(input);

    let mut last = 1;
    let mut cell = (1, 1);
    let mut code: u64 = 20151125;

    while cell != target {
        code *= 252533;
        code %= 33554393;

        if cell.0 == 1 {
            cell.0 = last + 1;
            cell.1 = 1;
            last += 1;
        } else {
            cell.0 -= 1;
            cell.1 += 1;
        }
    }

    code
}

fn load(input: &str) -> (u64, u64)
{
    let mut iter = input.lines();
    iter.next();

    let s = iter.next().unwrap();
    let v = s.split(' ').collect::<Vec<_>>();

    (
        v[5][0..4].parse::<u64>().unwrap(),
        v[7][0..4].parse::<u64>().unwrap()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2650453);
    }
}