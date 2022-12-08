use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let total_size = part_one(input);
    println!("Part 1: {} ({:?})", total_size, t.elapsed());

    let t = Instant::now();
    let total_size = part_two(input);
    println!("Part 2: {} ({:?})", total_size, t.elapsed());
}

fn part_one(input: &str) -> usize {
    let sizes = calc_sizes(input);
    sizes.values().filter(|&&n| n <= 100000).sum()
}

fn part_two(input: &str) -> usize {
    let sizes  = calc_sizes(input);
    let unused = 70000000 - *sizes.get("/").unwrap();
    let needed = 30000000 - unused;

    *sizes.values()
        .filter(|&&n| n > needed)
        .min()
        .unwrap()
}

fn calc_sizes(input: &str) -> HashMap<String, usize> {
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
                                let path = pwd[..i+1].join("/");
                                *sizes.entry(path).or_insert(0) += size;
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
    fn it_works() {
        let input = include_str!("../input.txt");

        let total_size = part_one(input);
        assert_eq!(total_size, 1453349);

        let total_size = part_two(input);
        assert_eq!(total_size, 2948823);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let total_size = part_one(input);
        assert_eq!(total_size, 95437);
    }
}
