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
    let (nums, boards) = load(input);
    let mut marked = vec![0;boards.len()];

    let mut it = nums.iter();
    let (n, i) = loop {
        if let Some(&n) = it.next() {
            mark(n, &boards, &mut marked);
            if let Some(i) = bingo(&marked) {
                break (n, i)
            }
        } else {
            break (0, 0)
        }
    };

    let board = &boards[i];
    let mut sum = 0;
    let mut unmarked = !marked[i] & 0x1FFFFFF;
    while let Some(i) = bit(&mut unmarked) {
        sum += board[i]
    }

    n * sum
}

fn part_two(input: &str) -> u32
{
    let (nums, mut boards) = load(input);
    let mut marks = vec![0;boards.len()];

    let mut num    = 0;
    let mut board  = vec![];
    let mut marked = 0;
    for n in nums {
        mark(n, &boards, &mut marks);
        while let Some(i) = bingo(&marks) {
            num    = n;
            board  = boards.remove(i);
            marked = marks.remove(i);
        }
    }

    let mut sum = 0;
    let mut unmarked = !marked & 0x1FFFFFF;
    while let Some(i) = bit(&mut unmarked) {
        sum += board[i]
    }

    num * sum
}

static WINS: [u32;10] = [
    0b0000000000000000000011111,
    0b0000000000000001111100000,
    0b0000000000111110000000000,
    0b0000011111000000000000000,
    0b1111100000000000000000000,
    0b0000100001000010000100001,
    0b0001000010000100001000010,
    0b0010000100001000010000100,
    0b0100001000010000100001000,
    0b1000010000100001000010000,
];

fn mark(num: u32, boards: &[Vec<u32>], marked: &mut [u32])
{
    boards.iter()
        .enumerate()
        .flat_map(|(i, b)| b.iter().position(|n| *n == num).map(|p| (i, p)))
        .for_each(|(i, p)| marked[i] |= 1 << p);
}

fn bingo(marked: &[u32]) -> Option<usize>
{
    marked.iter()
        .position(|n| WINS.iter().any(|&w| w & n == w))
}

fn bit(n: &mut u32) -> Option<usize> {
    match n {
        0 => None,
        _ => {
            let i = n.trailing_zeros();
            *n &= !(1 << i);
            Some(i as usize)
        }
    }
}

fn load(input: &str) -> (Vec<u32>, Vec<Vec<u32>>)
{
    let mut it = input.split("\n\n");
    let nums = it.next()
        .map(|s| s.split(',')
            .flat_map(|n| n.parse::<u32>())
            .collect::<Vec<_>>()
        )
        .unwrap();

    let boards = it
        .map(|board| board.lines()
            .fold(vec![], |mut v, line| {
                v.extend(line.split_whitespace()
                    .flat_map(|s| s.parse::<u32>()));
                v
            })
        )
        .collect::<Vec<_>>();

    (nums, boards)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 58838);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 6256);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 4512);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1924);
    }

}