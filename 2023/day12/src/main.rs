fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

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
                .flat_map(|_| g.clone())
                .collect::<Vec<_>>();

            arrangements(&springs, &groups)
        })
        .sum()
}


fn parse_record(line: &str) -> (String, Vec<u32>)
{
    let (s, g) = line.split_once(' ').unwrap();
    let springs = s.split('.')
        .filter(|st| ! st.is_empty())
        .collect::<Vec<_>>()
        .join(".");
    let groups = g.split(',')
        .flat_map(|n| n.parse())
        .collect();

    (springs, groups)
}

fn arrangements(springs: &str, groups: &[u32]) -> u32
{
    count(springs, groups, &[0])
}

fn count(springs: &str, groups: &[u32], found: &[u32]) -> u32
{
    let i = found.len() - 1;

    if springs.is_empty() {
        if found[i] == 0 {
            return (groups == &found[..i]) as u32
        } else {
            return (groups == found) as u32
        }
    }

    let c = springs.chars().next().unwrap();
    
    let mut n = 0;
    if c == '.' {
        if found[i] == 0 {
            n += count(&springs[1..], groups, found)
        } else {
            let mut v = found.to_vec(); v.push(0);
            n += count(&springs[1..], groups, &v)
        }
    } else if c == '#' {
        let mut v = found.to_vec(); v[i] += 1;
        n += count(&springs[1..], groups, &v)
    } else {
        // as '#'
        let mut v = found.to_vec(); v[i] += 1;
        n += count(&springs[1..], groups, &v);
 
        // as '.'
        if found[i] == 0 {
            n += count(&springs[1..], groups, found)
        } else {
            let mut v = found.to_vec(); v.push(0);
            n += count(&springs[1..], groups, &v)
        }
    }

    n
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
        assert_eq!(arrangements("???.###", &[1,1,3]), 1);
    }

    #[test]
    fn example_2()
    {
        assert_eq!(arrangements("??.??.?##", &[1,1,3]), 4);
    }

    #[test]
    fn example_3()
    {
        assert_eq!(arrangements("?#?#?#?#?#?#?#?", &[1,3,1,6]), 1);
    }

    #[test]
    fn example_4()
    {
        assert_eq!(arrangements("????.#.#", &[4,1,1]), 1);
    }

    #[test]
    fn example_5()
    {
        assert_eq!(arrangements("????.######.#####", &[1,6,5]), 4);
    }

    #[test]
    fn example_6()
    {
        assert_eq!(arrangements("?###????????", &[3,2,1]), 10);
    }
}
