use std::cmp::Ordering;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let indices = part_one(input);
    println!("Part 1: {} ({:?})", indices, t.elapsed());

    let t = Instant::now();
    let decoder_key = part_two(input);
    println!("Part 2: {} ({:?})", decoder_key, t.elapsed());
}

fn part_one(input: &str) -> usize {
    use std::cmp::Ordering::Greater;

    input.split("\n\n")
        .enumerate()
        .filter_map(|(i, packets)| packets.split_once('\n').map(|v| (i, v)))
        .filter_map(|(i, (a, b))| (compare_packets(a, b) != Greater).then_some(i+1))
        .sum()
}

fn part_two(input: &str) -> usize {
    let markers = "[[2]]\n[[6]]";

    let mut packets: Vec<_> = markers.split('\n')
        .chain(input.split('\n'))
        .filter(|s| !s.is_empty())
        .collect();
    packets.sort_by(|a, b| compare_packets(a, b));
    
    let i2 = packets.iter().position(|&s| s == "[[2]]").unwrap() + 1;
    let i6 = packets.iter().position(|&s| s == "[[6]]").unwrap() + 1;

    i2 * i6
}

fn compare_packets(a: &str, b: &str) -> Ordering {
    compare_lists(
        &mut a.chars().skip(1).peekable(),
        &mut b.chars().skip(1).peekable()
    )
}

fn compare_lists<I, K>(
    a: &mut std::iter::Peekable<I>,
    b: &mut std::iter::Peekable<K>
) -> Ordering
    where I: Iterator<Item=char>, K: Iterator<Item=char>
{
    loop {
        let mut ca = a.next().unwrap();
        let mut cb = b.next().unwrap();
        if ca == ',' { ca = a.next().unwrap() }
        if cb == ',' { cb = b.next().unwrap() }

        if ca == ']' && cb == ']' {
            break Ordering::Equal
        } else if ca == ']' || cb == ']' {
            break if ca == ']' { Ordering::Less } else { Ordering::Greater }
        } else if ca == '[' && cb == '[' {
            let v = compare_lists(a, b);
            if v != Ordering::Equal { break v }
        } else if ca == '[' {
            let s = format!("{}]", token(cb, b));
            let v = compare_lists(a, &mut s.chars().peekable());
            if v != Ordering::Equal { break v }
        } else if cb == '[' {
            let s = format!("{}]", token(ca, a));
            let v = compare_lists(&mut s.chars().peekable(), b);
            if v != Ordering::Equal { break v }
        } else {
            match (number(ca, a), number(cb, b)) {
                (va, vb) if va > vb => break Ordering::Greater,
                (va, vb) if va < vb => break Ordering::Less,
                _ => ()
            }
        }
    }
}

fn token<I>(c1: char, iter: &mut std::iter::Peekable<I>) -> String
    where I: Iterator<Item=char>
{
    if c1.is_ascii_digit() {
        let v = number(c1, iter);
        format!("{v}")
    } else {
        c1.into()
    }
}

fn number<I>(c1: char, iter: &mut std::iter::Peekable<I>) -> u8 
    where I: Iterator<Item=char>    
{
    let mut v = c1 as u8 - b'0';
    if let Some(&c2) = iter.peek() {
        if c2.is_ascii_digit() {
            iter.next(); 
            v *= 10;
            v += c2 as u8 - b'0'
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let indices = part_one(input);
        assert_eq!(indices, 5555);

        let decoder_key = part_two(input);
        assert_eq!(decoder_key, 22852);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let indices = part_one(input);
        assert_eq!(indices, 13);

        let decoder_key = part_two(input);
        assert_eq!(decoder_key, 140);
    }
}
