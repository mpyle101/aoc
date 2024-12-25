use std::collections::HashMap;

type Workflows<'a> = HashMap<&'a str, Vec<Rule>>;
type Ratings = [(u64, u64);4];
type State<'a> = (&'a str, Ratings);

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

fn part_one(input: &str) -> u64
{
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let wfs = load_workflows(s1);
    let parts = load_parts(s2);

    parts.iter()
        .filter(|part| run(part, &wfs) == "A")
        .map(|part| part.iter().sum::<u64>())
        .sum()
}

fn part_two(input: &str) -> u64
{
    use std::collections::VecDeque;

    let (s1, _) = input.split_once("\n\n").unwrap();
    let wfs = load_workflows(s1);

    let mut accepted = vec![];

    let mut q = VecDeque::from([("in", [(1u64, 4000u64);4])]);
    while let Some((wf, parts)) = q.pop_front() {
        let mut ratings = parts;
        for rule in wfs.get(wf).unwrap() {
            let (st, rat) = process(&ratings, rule);
            if st.0 == "A" {
                accepted.push(st.1);
            } else if st.0 != "R" {
                q.push_back(st);
            }
            ratings = rat;
        }
    }

    //accepted.iter().for_each(|a| println!("{:?}", a));
    
    accepted.iter()
        .map(|ratings| ratings.iter()
            .filter(|(a, b)| a <= b)
            .map(|(a, b)| b - a + 1)
            .product::<u64>()
        )
        .sum()
}

fn process<'a>(ratings: &Ratings, rule: &'a Rule) -> (State<'a>, Ratings)
{
    match rule.op {
        '=' => ((&rule.wf, *ratings), *ratings),
        '>' => {
            let mut rat = *ratings;
            let rng = rat[rule.rat];
            rat[rule.rat] = (rng.0.max(rule.val + 1), rng.1);
            let st: State = (&rule.wf, rat);

            let mut rat = *ratings;
            let rng = rat[rule.rat];
            rat[rule.rat] = (rng.0, rng.1.min(rule.val));
            (st, rat)
        },
        '<' => {
            let mut rat = *ratings;
            let rng = rat[rule.rat];
            rat[rule.rat] = (rng.0, rng.1.min(rule.val - 1));
            let st: State = (&rule.wf, rat);

            let mut rat = *ratings;
            let rng = rat[rule.rat];
            rat[rule.rat] = (rng.0.max(rule.val), rng.1);
            (st, rat)
        },
        _ => panic!("Unknown operation: {}", rule.op)
    }
}

#[derive(Debug)]
struct Rule {
    op: char,
    wf: String,
    val: u64,
    rat: usize,
}

fn run<'a>(part: &[u64;4], wfs: &'a Workflows) -> &'a str
{
    let mut name = "in";

    while name != "R" && name != "A" {
        if let Some(rules) = wfs.get(name) {
            let mut iter = rules.iter().peekable();
            while iter.next_if(|rule|
                match rule.op {
                    '=' => false,
                    '<' => part[rule.rat] >= rule.val,
                    '>' => part[rule.rat] <= rule.val,
                    _ => panic!("Unknown operation: {}", rule.op)
                }).is_some() {}
            name = &iter.next().unwrap().wf;
        } else {
            panic!("Workflow not found: {name}")
        }
    }

    name
}

fn load_workflows(wf: &str) -> HashMap<&str, Vec<Rule>>
{
    use regex::Regex;

    let last_re = Regex::new(r",(?<wf>\w+)}").unwrap();
    let rules_re = Regex::new(r"([x|m|a|s])([<|>])(?<val>\d+):(?<wf>\w+)").unwrap();

    wf.lines()
        .map(|line| {
            let (name, s) = line.split_once('{').unwrap();
            let mut rules = rules_re.captures_iter(s)
                .map(|caps| {
                    let rat = category(&caps[1]);
                    let op  = caps[2].chars().next().unwrap();
                    let val = caps[3].parse::<u64>().unwrap();
                    let wf  = caps[4].to_string();

                    Rule { wf, rat, op, val }
                })
                .collect::<Vec<_>>();
            if let Some(caps) = last_re.captures(s) {
                let wf = caps[1].to_string();
                rules.push(Rule { wf, rat: 5, op: '=', val: 0 })
            } else {
                panic!("Default rule not found!")
            }

            (name, rules)
        })
        .collect()
}

fn category(c: &str) -> usize
{
    match c {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("Unknown category: {c}")
    }
}

fn load_parts(parts: &str) -> Vec<[u64;4]>
{
    parts.lines()
        .map(|line| {
            let mut xmas = [0;4];
            let s = &line[1..line.len()-1];
            s.split(',')
                .enumerate()
                .for_each(|(i, c)| {
                    let (_, n) = c.split_once('=').unwrap();
                    xmas[i] = n.parse::<u64>().unwrap();
                });
            xmas
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 362930);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 116365820987729);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 19114);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 167_409_079_868_000);
    }
}
