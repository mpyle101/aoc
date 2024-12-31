use std::ops::RangeInclusive;

type Ticket = Vec<u32>;
type Rules<'a> = Vec<(&'a str, RangeInclusive<u32>, RangeInclusive<u32>)>;
type RI32 = RangeInclusive<u32>;

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
    let (rules, _, nearby) = load(input);

    let invalid = |n| !rules.iter().any(|(_, r1, r2)| r1.contains(n) || r2.contains(n));
    nearby.iter()
        .flat_map(|v| v.iter().filter(|n| invalid(*n)))
        .sum::<u32>()
}

fn part_two(input: &str) -> u64
{
    let (rules, ticket, nearby) = load(input);

    let valid = |n| rules.iter().any(|(_, r1, r2)| r1.contains(n) || r2.contains(n));
    let nearby = nearby.iter()
        .filter(|v| v.iter().all(valid))
        .collect::<Vec<_>>();

    // The mask is a bit pattern matching the possible ticket columns.
    // There's one for each rule in the order vector. We iterate over
    // the valid tickets and for each one test the rules against the
    // column value. If we find a rule which can't handle a given value,
    // put a 0 at that column position in the bit mask.
    let mask: u32 = 0b0000_0000_0000_1111_1111_1111_1111_1111;
    let mut order = vec![mask; ticket.len()];

    let invalid = |n, r1: &RI32, r2: &RI32| !(r1.contains(n) || r2.contains(n));
    nearby.iter()
        .for_each(|tkt| tkt.iter()
            .enumerate()
            .for_each(|(i, n)| {
                rules.iter()
                    .enumerate()
                    .filter(|(_, (_, r1, r2))| invalid(n, r1, r2))
                    .for_each(|(r, _)| order[r] &= !(1 << i))
            })
        );

    // For it to work, when we're done with the above, there must be
    // at least one rule with only one possible column. We find it,
    // and remove that bit from the rest, find the next one, remove
    // it's bit and so on until we've found all the column for all
    // the rules.
    let mut order = order.iter().zip(0usize..).map(|(n, i)| (*n, i)).collect::<Vec<_>>();
    let mut rules = [0u32;20];
    while let Some(i) = order.iter().position(|(n, _)| n.count_ones() == 1) {
        let (n, p) = order.remove(i);
        rules[p] = n;
        order.iter_mut().for_each(|(m, _)| *m &= !n)
    }

    // The first 6 rules are the departure rules. Turn those column
    // masks into indexes and get the values from the ticket.
    rules.iter()
        .take(6)
        .map(|n| (31 - n.leading_zeros()) as usize)
        .map(|i| ticket[i] as u64)
        .product()
}

fn load(input: &str) -> (Rules, Ticket, Vec<Ticket>)
{
    let mut it = input.split("\n\n");
    let s = it.next().unwrap();
    let rules = s.lines()
        .map(|l| {
            let (name, s2) = l.split_once(": ").unwrap();
            let (s1, s2)   = s2.split_once(" or ").unwrap();
            let r1 = parse_range(s1);
            let r2 = parse_range(s2);
            (name, r1, r2)
        })
        .collect::<Rules>();

    let s = it.next().unwrap();
    let ticket = s.lines()
        .skip(1)
        .flat_map(|l| l.split(',').flat_map(|s| s.parse::<u32>()))
        .collect::<Vec<_>>();

    let s = it.next().unwrap();
    let nearby = s.lines()
        .skip(1)
        .map(|l| l.split(',').flat_map(|s| s.parse::<u32>()).collect())
        .collect::<Vec<_>>();

    (rules, ticket, nearby)
}

fn parse_range(s: &str) -> RangeInclusive<u32>
{
    let (n1, n2) = s.split_once('-').unwrap();
    n1.parse::<u32>().unwrap()..=n2.parse::<u32>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 25059);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3253972369789);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 71);
    }
}