
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let calories = part_one(input);
    println!("Part 1: {} ({:?})", calories, t.elapsed());

    let t = Instant::now();
    let calories = part_two(input);
    println!("Part 2: {} ({:?})", calories, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    input.split("\n\n")
        .map(|group| group.lines()
            .map(|s| s.parse::<i32>().unwrap())
            .sum())
        .max()
        .unwrap()
}

fn part_two(input: &str) -> i32 {
    let mut res: [i32; 3] = [0; 3];

    input.split("\n\n")
        .map(|group| group.lines()
            .map(|s| s.parse::<i32>().unwrap())
            .sum())
        .for_each(|n|
            (0..3).for_each(|i|
                if n > res[i] { 
                    if i > 0 { res[i-1] = res[i] }
                    res[i] = n;
                }
            )
        );

    return res.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let calories = part_one(input);
        assert_eq!(calories, 70720);

        let calories = part_two(input);
        assert_eq!(calories, 207148);
    }
}