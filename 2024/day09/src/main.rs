
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

fn part_one(input: &str) -> i64
{
    let mut id = 0;
    let mut v = input.bytes()
        .enumerate()
        .fold(vec![], |mut v, (i, c)| {
            if i % 2 == 0 {
                (0..c - b'0').for_each(|_| v.push(id));
                id += 1;
            } else {
                (0..c - b'0').for_each(|_| v.push(-1))
            }
            v
        });

    let mut i = v.iter().position(|n| *n == -1).unwrap();
    let mut j = v.len() - 1;
    while i != j {
        v.swap(i, j);
        i += 1;
        j -= 1;
        while v[i] != -1 && i != j {
            i += 1
        }
        while v[j] == -1 && i != j {
            j -= 1
        }
    }

    v.iter()
        .take_while(|n| **n != -1)
        .enumerate()
        .map(|(i, n)| i as i64 * n)
        .sum()
}

fn part_two(input: &str) -> i64
{
    let mut id = 0;
    let mut v = input.bytes()
        .enumerate()
        .fold(vec![], |mut v, (i, c)| {
            let count = c - b'0';
            if i % 2 == 0 {
                v.push((count, id));
                id += 1;
            } else if count > 0 {
                v.push((count, -1))
            }
            v
        });

    let mut n = i64::MAX;
    let mut j = v.len() - 1;
    while j > 0 {
        while j > 0 && (v[j].1 == -1 || v[j].1 > n) {
            j -= 1;
        }
        if j > 0 {
            n = v[j].1;
            let blocks = v[j].0;
    
            if let Some(i) = find_free(j, &v, blocks) {
                let (free, _) = v[i];
                if free == blocks {
                    v.swap(i, j);
                } else {
                    v[j].1 = -1;
                    v[i] = (blocks, n);
                    v.insert(i+1, (free - blocks, -1));
                    j += 1
                }
            } else {
                j -= 1;
            }
        }
    }

    let mut ix = 0;
    v.iter()
        .map(|(c, n)| {
            (0..*c)
                .map(|_| {
                    let val = if *n == -1 { 0 } else { n * ix };
                    ix += 1;
                    val
                })
                .sum::<i64>()
        })
        .sum()
}

fn find_free(j: usize, v: &[(u8, i64)], blocks: u8) -> Option<usize>
{
    v[0..j].iter()
        .position(|(c, n)| *n == -1 && *c >= blocks)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 6346871685398);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 6373055193464);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 1928);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 2858);
    }
}
