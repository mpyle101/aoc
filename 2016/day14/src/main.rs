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

fn part_one(input: &str) -> usize {
    get_keys(input, 1)
}

fn part_two(input: &str) -> usize {
    get_keys(input, 2017)
}

fn get_keys(salt: &str, n: i32) -> usize {
    let mut keys = vec![];
    let mut candidates = vec![];

    let mut index = 0;
    while keys.len() < 64 {
        let key  = format!("{salt}{index}");
        let hash = mash(key, n);

        if let Some(c1) = check_hash(&hash, 5) {
            candidates.iter()
                .filter(|(c2, ix)| (index - ix) <= 1000 && c1 == *c2)
                .for_each(|(_, ix)| keys.push(*ix));
        }
        if let Some(c) = check_hash(&hash, 3) {
            candidates.push((c, index));
        }

        index += 1;
    }
    keys.sort_unstable();

    keys[63]
}

fn mash(key: String, n: i32) -> Vec<char> {
    let result = (0..n).fold(key, |k, _| {
        format!("{:x}", md5::compute(k))
    });
    
    result.chars().collect::<Vec<char>>()
}

fn check_hash(hash: &[char], i: usize) -> Option<char> {
    for w in hash.windows(i) {
        if w[1..].iter().all(|n| *n == w[0]) {
            return Some(w[0])
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 15035);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 19968);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 22728);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 22551);
    }
}