use serde_json::{Map, Value};

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

fn part_one(input: &str) -> i32
{
    use regex::Regex;

    let re = Regex::new(r"(?-u:\-?\d+)").unwrap();
    re.find_iter(input)
        .flat_map(|v| v.as_str().parse::<i32>())
        .sum()
}

fn part_two(input: &str) -> i64
{
    let json: Vec<Value> = serde_json::from_str(input).unwrap();
    json.iter().map(process).sum()
}

fn process(v: &Value) -> i64
{
    match v {
        Value::Number(n)   => n.as_i64().unwrap(),
        Value::Array(arr)  => arr.iter().map(process).sum(),
        Value::Object(obj) => process_attributes(obj),
        _ => 0,
    }
}

fn process_attributes(obj: &Map<String, Value>) -> i64
{
    let red = obj.values().any(|v| v.as_str() == Some("red"));
    if red { 0 } else { obj.values().map(process).sum() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 191164);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 87842);
    }
}