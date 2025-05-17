use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let aunts = load(include_str!("./input.txt"));
    let clues = sues_clues();

    let t = Instant::now();
    let sue = part_one(&aunts, &clues);
    println!("Part 1: {} ({:?})", sue, t.elapsed());

    let t = Instant::now();
    let sue = part_two(&aunts, &clues);
    println!("Part 2: {} ({:?})", sue, t.elapsed());
}

fn load(input: &str) -> Vec<HashMap<&str, i32>> {
    input.lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| HashMap::from([
            ("no", v[1][0..v[1].len() - 1].parse::<i32>().unwrap()),
            (v[2], v[3][0..v[3].len() - 1].parse::<i32>().unwrap()),
            (v[4], v[5][0..v[5].len() - 1].parse::<i32>().unwrap()),
            (v[6], v[7].parse::<i32>().unwrap()),
        ]))
        .collect()
}

fn part_one(aunts: &[HashMap<&str, i32>], clues: &[(&str, i32); 10]) -> i32 {
    let sue = clues.iter().fold(aunts.to_vec(), |vec, (clue, value)|
        vec.iter()
            .filter(|m| m.get(clue).is_none_or(|v| v == value))
            .cloned()
            .collect::<Vec<_>>()
    );

    *sue.first().unwrap().get("no").unwrap()
}

fn part_two(aunts: &[HashMap<&str, i32>], clues: &[(&str, i32); 10]) -> i32 {
    let sue = clues.iter().fold(aunts.to_vec(), |vec, (clue, value)|
        vec.iter().filter(|map|
            map.get(clue).is_none_or(|v|
                if *clue == "cats:" || *clue == "trees:" {
                    v > value
                } else if *clue == "pomeranians:" || *clue == "goldfish:" {
                    v < value
                } else {
                    v == value
                }
            )
        )
        .cloned()
        .collect::<Vec<_>>()
    );

    *sue.first().unwrap().get("no").unwrap()
}

fn sues_clues() -> [(&'static str, i32); 10] {
    [
        ("children:", 3),
        ("cats:", 7),
        ("samoyeds:", 2),
        ("pomeranians:", 3),
        ("akitas:", 0),
        ("vizslas:", 0),
        ("goldfish:", 5),
        ("trees:", 3),
        ("cars:", 2),
        ("perfumes:", 1),
    ]
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let aunts = load(include_str!("./input.txt"));
    let clues = sues_clues();

    let sue = part_one(&aunts, &clues);
    assert_eq!(sue, 103);

    let sue = part_two(&aunts, &clues);
    assert_eq!(sue, 405);
  }
}