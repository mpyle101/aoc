use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let root = part_one(input);
    println!("Part 1: {} ({:?})", root, t.elapsed());

    let t = Instant::now();
    let humn = part_two(input);
    println!("Part 2: {} ({:?})", humn, t.elapsed());
}

fn part_one(input: &str) -> i64 {
    let actions = load(input);
    actions.get("root").unwrap().eval(&actions)
}

fn part_two(input: &str) -> i64 {
    let actions = load(input);
    let monkeys = actions.iter()
        .fold(HashMap::new(), |mut m, (monkey, action)| {
            let args = action.args();
            if !args.0.is_empty() {
                m.insert(args.0, (*monkey, *action));
                m.insert(args.1, (*monkey, *action));
            };

            m
        });

    let (monkey, action) = *monkeys.get("humn").unwrap();
    let args = action.args();
    let other = if args.0 == "humn" { (args.1, 'R') } else { (args.0, 'L') };
    let v = actions.get(other.0).unwrap().eval(&actions);

    let mut path = vec![(monkey, action, v, other.1)];
    loop {
        let (monkey, ..) = path.last().unwrap();
        if *monkey == "root" {
            break
        } else {
            let (parent, action) = *monkeys.get(monkey).unwrap();
            let args = action.args();
            let other = if args.0 == *monkey { (args.1, 'R') } else { (args.0, 'L') };
            let v = actions.get(other.0).unwrap().eval(&actions);

            path.push((parent, action, v, other.1))
        }
    }
    path.reverse();

    let start = path[0].2;
    path.iter()
        .skip(1)
        .fold(start, |n, (_, action, v, p)| action.undo(*p, n, *v))
}

fn load(input: &str) -> HashMap<&str, Action> {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| {
            let monkey = v[0].strip_suffix(':').unwrap();
            let action = if v.len() == 2 {
                Action::Literal(v[1].parse::<i64>().unwrap())
            } else {
                match v[2] {
                    "+" => Action::Add(v[1], v[3]),
                    "-" => Action::Sub(v[1], v[3]),
                    "*" => Action::Mul(v[1], v[3]),
                    "/" => Action::Div(v[1], v[3]),
                    _ => unreachable!()
                }
            };

            (monkey, action)
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Action<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Literal(i64),
}

impl<'a> Action<'a> {
    fn eval(&self, actions: &HashMap<&str, Action>) -> i64 {
        use Action::*;

        match self {
            Literal(n) => *n,
            Add(a, b) => {
                let a1 = actions.get(a).unwrap();
                let b1 = actions.get(b).unwrap();
                a1.eval(actions) + b1.eval(actions)
            },
            Sub(a, b) => {
                let a1 = actions.get(a).unwrap();
                let b1 = actions.get(b).unwrap();
                a1.eval(actions) - b1.eval(actions)
            },
            Mul(a, b) => {
                let a1 = actions.get(a).unwrap();
                let b1 = actions.get(b).unwrap();
                a1.eval(actions) * b1.eval(actions)
            },
            Div(a, b) => {
                let a1 = actions.get(a).unwrap();
                let b1 = actions.get(b).unwrap();
                a1.eval(actions) / b1.eval(actions)
            },
        }
    }

    fn args(&self) -> (&str, &str) {
        use Action::*;

        match self {
            Add(a, b) => (a, b),
            Sub(a, b) => (a, b),
            Mul(a, b) => (a, b),
            Div(a, b) => (a, b),
            _ => ("", ""),
        }
    }

    fn undo(&self, p: char, n: i64, v: i64) -> i64 {
        match self {
            Action::Add(_,_) => if p == 'R' && v > n { -(n + v) } else { n - v },
            Action::Sub(_,_) => if p == 'R' { v + n } else { v - n },
            Action::Mul(_,_) => n / v,
            Action::Div(_,_) => if p == 'R' { v * n } else { v / n },
            _ => unreachable!()
        }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let root = part_one(input);
        assert_eq!(root, 21120928600114);

        let humn = part_two(input);
        assert_eq!(humn, 3453748220116);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let root = part_one(input);
        assert_eq!(root, 152);

        let humn = part_two(input);
        assert_eq!(humn, 301);
    }
}
