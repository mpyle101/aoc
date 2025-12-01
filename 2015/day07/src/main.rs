use std::collections::HashMap;

type Gates<'a>   = HashMap<&'a str, Gate<'a>>;
type Signals<'a> = HashMap<&'a str, i32>;

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
    let gates = load(input);
    let mut signals = Signals::new();

    eval("a", &gates, &mut signals)
}

fn part_two(input: &str) -> i32
{
    let gates = load(input);
    let mut signals = Signals::new();
    let a = eval("a", &gates, &mut signals);
    let mut signals = Signals::from([("b", a)]);

    eval("a", &gates, &mut signals)
}

#[allow(non_camel_case_types)]
enum Gate<'a> {
    set(i32),
    sig(&'a str),
    not(&'a str),
    or(&'a str, &'a str),
    and(&'a str, &'a str),
    lsh(&'a str, i32),
    rsh(&'a str, i32),
    iand(&'a str, i32),
}

impl<'a> Gate<'a> {
    fn eval(&self, gates: &Gates<'a>, signals: &mut Signals<'a>) -> i32
    {
        use Gate::*;

        match self {
            set(n)     => *n,
            sig(a)     => eval(a, gates, signals),
            not(a)     => !eval(a, gates, signals),
            or(a, b)   => eval(a, gates, signals) | eval(b, gates, signals),
            and(a, b)  => eval(a, gates, signals) & eval(b, gates, signals),
            lsh(a, n)  => eval(a, gates, signals) << n,
            rsh(a, n)  => eval(a, gates, signals) >> n,
            iand(a, n) => eval(a, gates, signals) & n
        }
    }
}

fn eval<'a>(w: &'a str, gates: &Gates<'a>, signals: &mut Signals<'a>) -> i32
{
    if let Some(n) = signals.get(w) {
        *n
    } else {
        let g = gates.get(w).unwrap();
        let n = g.eval(gates, signals);
        signals.insert(w, n);
        n
    }
}

fn load(input: &str) -> Gates<'_>
{
    use Gate::*;

    input.lines()
        .flat_map(|line| line.split_once(" -> "))
        .map(|(s, w)| {
            let v = s.split(' ').collect::<Vec<_>>();
            let g = if v.len() == 1 {
                if let Ok(n) = v[0].parse::<i32>() {
                    set(n)
                } else {
                    sig(v[0])
                }
            } else if v.len() == 2 {
                not(v[1])
            } else {
                match v[1] {
                    "OR"     => or(v[0], v[2]),
                    "AND"    => if v[0] == "1" { iand(v[2], 1) } else { and(v[0], v[2]) },
                    "LSHIFT" => {
                        let n = v[2].parse::<i32>().unwrap();
                        lsh(v[0], n)
                    },
                    "RSHIFT" => {
                        let n = v[2].parse::<i32>().unwrap();
                        rsh(v[0], n)
                    },
                    _ => unreachable!()
                }
            };
            (w, g)
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
        assert_eq!(part_one(input), 16076);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2797);
    }
}