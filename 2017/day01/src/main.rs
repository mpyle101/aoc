fn main() {
    use std::{fs, time::Instant};

    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let t = Instant::now();
    let captcha = part_one(&input);
    println!("Part 1: {} {:?}", captcha, t.elapsed());

    let t = Instant::now();
    let captcha = part_two(&input);
    println!("Part 2: {} {:?}", captcha, t.elapsed());
}

fn load(input: &str) -> Vec<u8> {
    input.bytes().map(|b| b - b'0').collect()
}

fn part_one(digits: &[u8]) -> i32 {
    let captcha = digits.iter().enumerate().skip(1)
        .filter_map(|(i, &d)| (digits[i-1] == d).then_some(d as i32))
        .sum();

    if digits.last() == digits.first() {
        captcha + *digits.last().unwrap() as i32
    } else {
        captcha
    }
}

fn part_two(digits: &[u8]) -> i32 {
    let n = digits.len() / 2;
    digits.iter().enumerate()
        .filter_map(|(i, &d)| {
            let ix = (i + n) % digits.len();
            (digits[ix] == d).then_some(d as i32)
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = load(&fs::read_to_string("./input.txt").unwrap());

        let captcha = part_one(&input);
        assert_eq!(captcha, 1119);

        let captcha = part_two(&input);
        assert_eq!(captcha, 1420);
    }
}