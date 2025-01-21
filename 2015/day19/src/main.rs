type Rules<'a> = Vec<(&'a str, &'a str)>;

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

fn part_one(input: &str) -> usize
{
    use std::collections::HashSet;

    let (rules, molecule) = load(input);
    let mut molecules = HashSet::new();
    rules.iter().for_each(|(k, s)|
        molecule.match_indices(k).for_each(|(i, _)| {
            let mut m = molecule.to_string();
            m.replace_range(i..i+k.len(), s);
            molecules.insert(m);
        })
    );

    molecules.len()
}

fn part_two(input: &str) -> u32
{
    use rand::seq::SliceRandom;

    let (rules, molecule) = load(input);

    let rrules = rules.iter().map(|(k, v)| (*v, *k)).collect::<Vec<_>>();

    let mut cnt = 0;
    let mut m = molecule.to_string();
    while m != "e" {
        // Pick a completely random rule to apply. Sometimes we get stuck
        // sometimes we find the answer: 207.
        let rule = rrules.choose(&mut rand::thread_rng()).unwrap();
        while let Some(i) = m.find(rule.0) {
            m.replace_range(i..(i+rule.0.len()), rule.1);
            cnt += 1;
        }
    }

    cnt
}

fn load(input: &str) -> (Rules, &str)
{
    let mut it = input.split("\n\n");
    let rules = it.next()
        .map(|v| v.lines()
            .fold(Vec::new(), |mut rules, s| {
                let kv = s.split(" => ").collect::<Vec<_>>();
                rules.push((kv[0], kv[1]));
                rules
            })
        )
        .unwrap();

    (rules, it.next().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 576);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 207);
    }
}