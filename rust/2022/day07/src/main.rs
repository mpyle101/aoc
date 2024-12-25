use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    use std::time::Instant;

    let input = include_str!("../example.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize {
    let sizes = calc_sizes(input);
    sizes.values().filter(|&&n| n <= 100000).sum()
}

fn part_two(input: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    vec!["/"].hash(&mut hasher);

    let sizes  = calc_sizes(input);
    let unused = 70000000 - *sizes.get(&hasher.finish()).unwrap();
    let needed = 30000000 - unused;

    *sizes.values()
        .filter(|&&n| n > needed)
        .min()
        .unwrap()
}

fn calc_sizes(input: &str) -> HashMap<u64, usize> {
    let mut pwd = vec!["/"];

    input.lines()
        .fold(HashMap::new(), |mut sizes, s| {
            if s.starts_with("$ c") {
                match &s[5..] {
                    "/"  => pwd = vec!["/"],
                    ".." => { pwd.pop(); },
                    dir  => pwd.push(dir)
                }
            } else {
                let c = s.chars().next().unwrap();
                if c != '$' && c != 'd' {
                    if let Some((n, _)) = s.split_once(' ') {
                        if let Ok(size) = n.parse::<usize>() {
                            (0..pwd.len()).for_each(|i| {
                                let mut hasher = DefaultHasher::new();
                                pwd[..i+1].hash(&mut hasher);
                                *sizes.entry(hasher.finish()).or_insert(0) += size;
                            })
                        }
                    }
                }
            }

            sizes
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1453349);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2948823);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 95437);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 24933642);
    }
}
