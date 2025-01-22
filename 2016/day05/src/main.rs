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
    let mut password = ['-';8];

    let marker  = ['0';5];
    let mut buf = ['0';6];
    let mut index = 0;

    let mut i = 0;
    while i < 8 {
        let s = format!("{input}{index}");
        let digest = md5::compute(s);
        let hash = hex::encode(digest.iter());
        hash.chars().enumerate().take(6).for_each(|(n, c)| buf[n] = c);
        if buf[0..5] == marker {
            password[i] = buf[5];
            i += 1;
        }

        index += 1;
    }

    password.iter().collect()
}

fn part_two(input: &str) -> String
{
    let mut password = ['-';8];

    let marker  = ['0';5];
    let mut buf = ['0';7];
    let mut index = 0;

    let mut i = 0;
    while i < 8 {
        let s = format!("{input}{index}");
        let digest = md5::compute(s);
        let hash = hex::encode(digest.iter());
        hash.chars().enumerate().take(7).for_each(|(n, c)| buf[n] = c);
        if buf[0..5] == marker && (buf[5] as u8) > 47 && (buf[5] as u8) < 56 {
            let ix = buf[5].to_digit(10).unwrap() as usize;
            if password[ix] == '-' { 
                password[ix] = buf[6];
                i += 1 ;
                println!("{}", password.iter().collect::<String>());
            }
        }

        index += 1;
    }

    password.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "801b56a7");
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "424a0197");
    }
}