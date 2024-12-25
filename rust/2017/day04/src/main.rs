fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t = Instant::now();
    let valid = part_one(&input);
    println!("Part 1: {} {:?}", valid, t.elapsed());

    let t = Instant::now();
    let valid = part_two(&input);
    println!("Part 2: {} {:?}", valid, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    use std::collections::HashSet;

    input.lines()
        .map(|s| {
            let words  = s.split(' ').collect::<Vec<_>>();
            let unique = HashSet::<&&str>::from_iter(words.iter());
            unique.len() == words.len()
        })
        .filter(|valid| *valid)
        .count() as i32
}

fn part_two(input: &str) -> i32 {
    use itertools::Itertools;

    input.lines()
        .map(|s| s.split(' ')
            .combinations(2)
            .any(|v| v[0].len() == v[1].len() &&
                (*v[0]).chars().sorted().eq((*v[1]).chars().sorted())
            )
        )
        .filter(|invalid| !*invalid)
        .count() as i32
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let valid = part_one(&input);
    assert_eq!(valid, 325);

    let valid = part_two(&input);
    assert_eq!(valid, 119);
  }
}