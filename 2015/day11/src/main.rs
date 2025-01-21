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

fn part_one(input: &str) -> String
{
    generate(input)
}

fn part_two(input: &str) -> String
{
    let password = generate(input);
    generate(&password)
}

fn generate(pword: &str) -> String
{
    // Turn our input into an array of zero based bytes.
    // This will make incrementing trivial using mod 26.
    // hepxcrrq
    let mut password = [0u8;8];
    pword.as_bytes().iter()
        .enumerate()
        .for_each(|(i, b)| password[i] = *b - b'a');

    increment(&mut password);
    while !is_valid(&password) {
        increment(&mut password);
    }

    // Rehydrate back into 'a' based characters.
    password.iter().map(|b| (b + b'a') as char).collect::<String>()
}

fn increment(s: &mut [u8;8])
{
    let mut i = 7;
    s[i] = (s[i] + 1) % 26;
    while s[i] == 0 && i > 0 {
        i -= 1;
        s[i] = (s[i] + 1) % 26;
    }
}

fn is_valid(s: &[u8;8]) -> bool
{
    let mut pairs = 0;

    let mut straight = false;
    for i in 0..6 {
        straight |= s[i+1] == s[i] + 1 && s[i+2] == s[i] + 2
    }
    if straight {
        let mut i = 0;
        while i < 7 && pairs < 2 {
            i = if s[i] == s[i+1] {
                pairs += 1;
                i + 2
            } else {
                i + 1
            }
        }
    }

    pairs > 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "hepxxyzz");
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "heqaabcc");
    }
}