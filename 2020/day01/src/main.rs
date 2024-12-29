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
    let v = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();

    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            if v[i] + v[j] == 2020 { return v[i] * v[j] }
        }
    }

    0
}

fn part_two(input: &str) -> u32
{
    let v = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();

    for i in 0..v.len()-2 {
        for j in i+1..v.len()-1 {
            for k in j+1..v.len() {
                if v[i] + v[j] + v[k] == 2020 { return v[i] * v[j] * v[k]}
            }
        }
    }

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 878724);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 201251610);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 514579);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 241861950);
    }

}