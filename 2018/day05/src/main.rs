fn main() {
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t = Instant::now();
    let units = part_one(input);
    println!("Part 1: {}  ({:?})", units.len(), t.elapsed());

    let t = Instant::now();
    let units = part_two(&units);
    println!("Part 2: {}  ({:?})", units, t.elapsed());
}

fn part_one(input: &str) -> Vec<u8> {
    reduce(input.as_bytes().iter().cloned())
}

#[allow(clippy::almost_complete_range)]
fn part_two(input: &[u8]) -> usize {
    (b'a'..b'z').map(|b| {
        let polymer = input.iter().cloned().filter(|&v| v | 32 != b);
        reduce(polymer)
    })
    .map(|v| v.len())
    .min()
    .unwrap()
}

fn reduce(polymer: impl Iterator<Item = u8>) -> Vec<u8> {
    let mut v = Vec::new();
    polymer.for_each(|b| 
        if v.last().unwrap_or(&0) ^ b == 32 { v.pop(); } else { v.push(b); }
    );
    v
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let units = part_one(input);
    assert_eq!(units.len(), 10368);

    let smallest = part_two(&units);
    assert_eq!(smallest, 4122);
  }
}
