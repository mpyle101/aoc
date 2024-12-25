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
            let (raw, bid) = s.split_once(' ').unwrap();
            raw.bytes()
                .zip(cards.iter_mut())
                .for_each(|(c, card)| {
                    *card = match c {
                        b'A' => 12,
                        b'K' => 11,
                        b'Q' => 10,
                        b'J' => 9,
                        b'T' => 8,
                           _ => c - b'2',
                    }
                });

            Hand {
                cards,
                bid: bid.parse::<u32>().unwrap(),
                rank: rank(&cards),
            }
        })
        .collect();
    hands.sort();
    
    hands.iter()
        .zip(1..)
        .map(|(hand, i)| i * hand.bid)
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut hands: Vec<_> = input.split('\n')
        .map(|s| {
            let mut cards = [0u8;5];
            let (raw, bid) = s.split_once(' ').unwrap();
            raw.bytes()
                .zip(cards.iter_mut())
                .for_each(|(c, card)| {
                    *card = match c {
                        b'A' => 12,
                        b'K' => 11,
                        b'Q' => 10,
                        b'T' => 9,
                        b'J' => 0,
                           _ => c - b'1',
                    }
                });

            Hand {
                cards,
                bid: bid.parse::<u32>().unwrap(),
                rank: rank_joker(&cards),
            }
        })
        .collect();
    hands.sort();
    
    hands.iter()
        .zip(1..)
        .map(|(hand, i)| i * hand.bid)
        .sum()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Rank
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug, PartialEq)]
struct Hand
{
    bid: u32,
    rank: Rank,
    cards: [u8;5],
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
        let ordering = self.rank.cmp(&other.rank);
        if ordering == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            ordering
        }
    }
}

fn rank(cards: &[u8;5]) -> Rank
{
    use Rank::*;

    let mut counts = [0u8;13];
    cards.iter().for_each(|c| counts[*c as usize] += 1);
    counts.sort_by(|a, b| b.cmp(a));

    match &counts[..2] {
        [5, 0] => FiveOfAKind,
        [4, 1] => FourOfAKind,
        [3, 2] => FullHouse,
        [3, 1] => ThreeOfAKind,
        [2, 2] => TwoPair,
        [2, 1] => OnePair,
             _ => HighCard
    }
}

fn rank_joker(cards: &[u8;5]) -> Rank
{
    use Rank::*;
    
    let mut counts = [0u8;13];
    cards.iter().for_each(|c| counts[*c as usize] += 1);
    let jokers = counts[0];

    counts.sort_by(|a, b| b.cmp(a));
    counts[2] = jokers;

    match &counts[..3] {
        [5, 0, 0] => FiveOfAKind,
        [5, 0, 5] => FiveOfAKind,
        [4, 1, 4] => FiveOfAKind,
        [4, 1, 1] => FiveOfAKind,
        [4, 1, 0] => FourOfAKind,
        [3, 2, 3] => FiveOfAKind,
        [3, 2, 2] => FiveOfAKind,
        [3, 2, 0] => FullHouse,
        [3, 1, 3] => FourOfAKind,
        [3, 1, 1] => FourOfAKind,
        [3, 1, 0] => ThreeOfAKind,
        [2, 2, 2] => FourOfAKind,
        [2, 2, 1] => FullHouse,
        [2, 2, 0] => TwoPair,
        [2, 1, 2] => ThreeOfAKind,
        [2, 1, 1] => ThreeOfAKind,
        [2, 1, 0] => OnePair,
        [1, 1, 1] => OnePair,
                _ => HighCard,
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
