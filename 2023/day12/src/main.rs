fn main()
{
    use std::time::Instant;

    let input = include_str!("../example1.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

//    let t = Instant::now();
//    let result = part_two(input);
//    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    input.lines()
        .map(|line| {
            let (springs, groups) = parse_record(line);
            arrangements(&springs, &groups)
        })
        .sum()
}

#[allow(dead_code)]
fn part_two(input: &str) -> u32
{
    input.lines()
        .map(|line| {
            let (s, g) = parse_record(line);
            let springs = (0..5)
                .map(|_| s.clone())
                .collect::<Vec<_>>()
                .join("?");
            let groups = (0..5)
                .map(|_| g.clone())
                .collect::<Vec<_>>()
                .join(".");

            arrangements(&springs, &groups)
        })
        .sum()
}


fn parse_record(line: &str) -> (String, String)
{
    let (s, g) = line.split_once(' ').unwrap();
    let springs = s.split('.')
        .filter(|st| ! st.is_empty())
        .collect::<Vec<_>>()
        .join(".");
    let groups = g.split(',')
        .flat_map(|n| n.parse())
        .map(|n| "#".repeat(n))
        .collect::<Vec<_>>()
        .join(".");

    (springs, groups)
}

fn arrangements(springs: &str, groups: &str) -> u32
{
    let sp = springs.as_bytes();
    let gp = groups.as_bytes();

    //count_2d(sp, gp)
    count_arr(sp, gp)
}

#[allow(dead_code)]
fn count_arr(sp: &[u8], gp: &[u8]) -> u32
{
    if let (Ok(s), Ok(g)) = (std::str::from_utf8(sp), std::str::from_utf8(gp)) {
        println!("{s}  {g}");
    }
    
    // Due to stripping, every springs sequence starts with
    // '?' or '#' so we know the first character is always a
    // match.
    let mut dp = vec![0;gp.len()+1];
    dp[0] = 1;

    for b in sp.iter().skip(1) {
        for j in (1..=gp.len()).rev() {
            if *b == gp[j-1] || *b == b'?' {
                dp[j] += dp[j-1]
            }
        }
    }

    println!("{:?}", dp);
    
    dp[dp.len() - 1]
}

#[allow(dead_code)]
fn count_2d(sp: &[u8], gp: &[u8]) -> u32
{
    if let (Ok(s), Ok(g)) = (std::str::from_utf8(sp), std::str::from_utf8(gp)) {
        println!("{s}  {g}");
    }

    let mut dp = (0..=gp.len()).map(|_| vec![0;sp.len()+1]).collect::<Vec<_>>();
    
    // Due to stripping, every springs sequence starts with
    // '?' or '#' so we know the first character is always a
    // match.
    dp[0][0] = 1;

    for i in 1..=gp.len() {
        for j in 1..=sp.len() {
            dp[i][j] = dp[i][j-1];
            if gp[i-1] == sp[j-1] || sp[j-1] == b'?' {
                dp[i][j] += dp[i-1][j-1];
            }
        }
    }

    dp.iter().for_each(|v| println!("{:?}", v));
    
    dp[gp.len()][sp.len()]
}

#[allow(dead_code)]
fn count_recur(sp: &[u8], gp: &[u8]) -> u32
{
    use std::str::from_utf8;

    if let (Ok(s), Ok(g)) = (from_utf8(sp), from_utf8(gp)) {
        print!("{s}  {g} => ");
    }
    let count = if gp.is_empty() {
        sp.iter().all(|c| *c == b'?') as u32
    } else if gp.len() > sp.len() {
        0
    } else if gp.len() == sp.len() && (0..gp.len()).all(|i| gp[i] == sp[i] || sp[i] == b'?') {
        1
    } else if sp[0] == gp[0] || sp[0] == b'?' {
        let i = if gp[0] == b'.' && sp[0] == b'?' { 1 + (sp[1] == b'.') as usize } else { 1 };
        if let (Ok(s), Ok(g)) = (from_utf8(&sp[i..]), from_utf8(&gp[1..])) {
            print!("({s}  {g}), ");
        }
        if let (Ok(s), Ok(g)) = (from_utf8(&sp[i..]), from_utf8(gp)) {
            println!("({s}  {g})");
        }
        count_recur(&sp[i..], &gp[1..]) + count_recur(&sp[i..], gp)
    } else {
        0
    };

    if let (Ok(s), Ok(g)) = (from_utf8(sp), from_utf8(gp)) {
        println!("{s}  {g} => {count}");
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn input_part_one()
    // {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part_one(input), 7307);
    // }

    // #[test]
    // fn example_part_one()
    // {
    //     let input = include_str!("../example.txt");
    //     assert_eq!(part_one(input), 21);
    // }

    #[test]
    fn example_1()
    {
        assert_eq!(arrangements("???.###", "#.#.###"), 1);
    }

    #[test]
    fn example_2()
    {
        assert_eq!(arrangements("??.??.?##", "#.#.###"), 4);
    }

    #[test]
    fn example_3()
    {
        assert_eq!(arrangements("?#?#?#?#?#?#?#?", "#.###.#.######"), 1);
    }

    #[test]
    fn example_4()
    {
        assert_eq!(arrangements("????.#.#", "####.#.#"), 1);
    }

    #[test]
    fn example_5()
    {
        assert_eq!(arrangements("????.######.#####", "#.######.#####"), 4);
    }

    #[test]
    fn example_6()
    {
        assert_eq!(arrangements("?###????????", "###.##.#"), 10);
    }
}
