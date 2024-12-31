#![allow(clippy::redundant_closure_call)]

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

fn part_one(input: &str) -> i64 {
    input.lines().map(parser::eval1).map(Result::unwrap).sum()
}

fn part_two(input: &str) -> i64 {
    input.lines().map(parser::eval2).map(Result::unwrap).sum()
}

peg::parser!( grammar parser() for str {
    pub rule eval1() -> i64 = precedence!{
        x:(@) ws() "+" ws() y:@ { x + y }
        x:(@) ws() "*" ws() y:@ { x * y }
        --
        n:number() { n }
        "(" e:eval1() ")" { e }
    }

    pub rule eval2() -> i64 = precedence!{
        x:(@) ws() "*" ws() y:@ { x * y }
        --
        x:(@) ws() "+" ws() y:@ { x + y }
        --
        n:number() { n }
        "(" e:eval2() ")" { e }
    }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule ws() = [' ']*
});


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 36382392389406);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 381107029777968);
    }
}