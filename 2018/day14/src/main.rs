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
    let count = input.parse::<usize>().unwrap();

    let mut recipes: Vec<u8> = Vec::with_capacity(count + 15);
    recipes.push(3);
    recipes.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < count + 10 {
        let score1 = recipes[elf1];
        let score2 = recipes[elf2];
        let n = score1 + score2;
        if n >= 10 { recipes.push(1) }
        recipes.push(n % 10);
        elf1 = (elf1 + 1 + score1 as usize) % recipes.len();
        elf2 = (elf2 + 1 + score2 as usize) % recipes.len();
    }

    recipes[count..count + 10].iter().map(|&n| (n + 48) as char).collect()
}

fn part_two(input: &str) -> u32
{
    let scores = input.bytes()
        .enumerate()
        .fold([0;6], |mut buf, (i, b)| { buf[i] = b - b'0'; buf });
    let mut recipes: Vec<u8> = vec![3,7,1,0,1,0,1];
    let mut len = recipes.len();
    
    let mut elf1 = 6;
    let mut elf2 = 4;
    loop {
        let score1 = recipes[elf1];
        let score2 = recipes[elf2];
        let n = score1 + score2;
        if n >= 10 { 
            len += 1;
            recipes.push(1);
            if recipes[len-6..len] == scores {
                return len as u32 - 6
            }
        }
        len += 1;
        recipes.push(n % 10);
        if recipes[len-6..len] == scores {
            return len as u32 - 6
        }

        elf1 = (elf1 + 1 + score1 as usize) % len;
        elf2 = (elf2 + 1 + score2 as usize) % len;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "1617111014");
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 20321495);
    }
}