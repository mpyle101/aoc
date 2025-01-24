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
    input.lines()
        .fold([0, 0], |[n2, n3], line| {
            let letters = line.bytes()
                .fold([0;26], |mut a, b| {
                    a[(b - b'a') as usize] += 1; a
                });
            [
                letters.iter().find(|&n| *n == 2).map_or(n2, |_| n2 + 1),
                letters.iter().find(|&n| *n == 3).map_or(n3, |_| n3 + 1)
            ]
        })
        .iter()
        .product()
}

fn part_two(input: &str) -> String
{
    let ids = input.lines().collect::<Vec<_>>();

    for i in 0..ids.len() - 1 {
        for j in i..ids.len() {
            if ids[i].bytes()
                .zip(ids[j].bytes())
                .filter(|(a, b)| a != b)
                .count()  == 1
            {
                return ids[i].bytes()
                    .zip(ids[j].bytes())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a as char)
                    .collect::<String>()
            }
        }
    }

    "".into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 5368);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "cvgywxqubnuaefmsljdrpfzyi");
    }
}