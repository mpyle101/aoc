use std::collections::HashMap;

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
    let mut iter = input.split("\n\n");
    let wfs = load_workflows(iter.next().unwrap());
    let parts = load_parts(iter.next().unwrap());

    parts.iter()
        .filter(|part| run(part, &wfs) == "A")
        .map(|part| part.iter().sum::<u32>())
        .sum()
}

fn part_two(input: &str) -> u64
{
    let mut iter = input.split("\n\n");
    let wfs = load_workflows(iter.next().unwrap());
    let parts = load_parts(iter.next().unwrap());

    parts.iter()
        .filter(|part| run(part, &wfs) == "A")
        .map(|part| part.iter().sum::<u32>() as u64)
        .sum::<u64>()
}

struct Rule {
    op: char,
    wf: String,
    val: u32,
    cat: usize,
}

fn run<'a>(part: &[u32;4], wfs: &'a HashMap<&str, Vec<Rule>>) -> &'a str
{
    let mut name = "in";

    while name != "R" && name != "A" {
        if let Some(rules) = wfs.get(name) {
            let mut iter = rules.iter().peekable();
            while iter.next_if(|rule|
                match rule.op {
                    '=' => false,
                    '<' => part[rule.cat] >= rule.val,
                    '>' => part[rule.cat] <= rule.val,
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
                    let cat = category(&caps[1]);
                    let op  = caps[2].chars().next().unwrap();
                    let val = caps[3].parse::<u32>().unwrap();
                    let wf  = caps[4].to_string();

                    Rule { wf, cat, op, val }
                })
                .collect::<Vec<_>>();
            if let Some(caps) = last_re.captures(s) {
                let wf = caps[1].to_string();
                rules.push(Rule { wf, cat: 5, op: '=', val: 0 })
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

fn load_parts(parts: &str) -> Vec<[u32;4]>
{
    parts.lines()
        .map(|line| {
            let mut xmas = [0;4];
            let s = &line[1..line.len()-1];
            s.split(',')
                .enumerate()
                .for_each(|(i, c)| {
                    let (_, n) = c.split_once('=').unwrap();
                    xmas[i] = n.parse::<u32>().unwrap();
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
        assert_eq!(part_two(input), 362930);
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
        assert_eq!(part_two(input), 167409079868000);
    }
}
