use std::cmp::Ordering;

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
    let mut hands: Vec<_> = input.split('\n')
        .map(|s| {
            let mut cards = [0u8;5];
            let (c, bid) = s.split_once(' ').unwrap();
            c.bytes()
                .zip(cards.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = match src {
                        b'A' => 12,
                        b'K' => 11,
                        b'Q' => 10,
                        b'J' => 9,
                        b'T' => 8,
                           _ => src - b'2',
                    }
                });

            Hand {
                cards,
                bid: bid.parse::<u32>().unwrap(),
                strength: strength(&cards),
            }
        })
        .collect();
    hands.sort();
    
    hands.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut hands: Vec<_> = input.split('\n')
        .map(|s| {
            let mut cards = [0u8;5];
            let (c, bid) = s.split_once(' ').unwrap();
            c.bytes()
                .zip(cards.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = match src {
                        b'A' => 12,
                        b'K' => 11,
                        b'Q' => 10,
                        b'T' => 9,
                        b'J' => 0,
                           _ => src - b'1',
                    }
                });

            Hand {
                cards,
                bid: bid.parse::<u32>().unwrap(),
                strength: strength_joker(&cards),
            }
        })
        .collect();
    hands.sort();
    
    hands.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

#[derive(Debug, PartialEq)]
struct Hand
{
    bid: u32,
    cards: [u8;5],
    strength: u8,
}

impl Eq for Hand {}
impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}
impl Ord for Hand
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        let ordering = self.strength.cmp(&other.strength);
        if ordering == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            ordering
        }
    }
}

fn strength(cards: &[u8;5]) -> u8
{
    let mut counts = [0u8;13];
    cards.iter().for_each(|c| counts[*c as usize] += 1);
    counts.sort_by(|a, b| b.cmp(a));

    match &counts[..4] {
        [5, 0, 0, 0] => 6,
        [4, 1, 0, 0] => 5,
        [3, 2, 0, 0] => 4,
        [3, 1, 1, 0] => 3,
        [2, 2, 1, 0] => 2,
        [2, 1, 1, 1] => 1,
                   _ => 0
    }
}

fn strength_joker(cards: &[u8;5]) -> u8
{
    let mut counts = [0u8;13];
    cards.iter().for_each(|c| counts[*c as usize] += 1);
    let jokers = counts[0];

    counts.sort_by(|a, b| b.cmp(a));
    counts[4] = jokers;

    match &counts[..5] {
        [5, 0, 0, 0, 0] => 6,    // five of a kind
        [5, 0, 0, 0, 5] => 6,    // five of a kind
        [4, 1, 0, 0, 4] => 6,    // five of a kind
        [4, 1, 0, 0, 1] => 6,    // five of a kind
        [4, 1, 0, 0, 0] => 5,    // four of a kind
        [3, 2, 0, 0, 3] => 6,    // five of a kind
        [3, 2, 0, 0, 2] => 6,    // five of a kind
        [3, 2, 0, 0, 0] => 4,    // full house
        [3, 1, 1, 0, 3] => 5,    // four of a kind
        [3, 1, 1, 0, 1] => 5,    // four of a kind
        [3, 1, 1, 0, 0] => 3,    // three of a kind
        [2, 2, 1, 0, 2] => 5,    // four of a kind
        [2, 2, 1, 0, 1] => 4,    // full house
        [2, 2, 1, 0, 0] => 2,    // two pair
        [2, 1, 1, 1, 2] => 3,    // three of a kind
        [2, 1, 1, 1, 1] => 3,    // three of a kind
        [2, 1, 1, 1, 0] => 1,    // one pair
        [1, 1, 1, 1, 1] => 1,    // one pair
                      _ => 0,    // high card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 249726565);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 251135960);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 6440);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 5905);
    }
}
