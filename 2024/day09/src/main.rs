
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
    let mut v = input.bytes()
        .zip(0..)
        .fold(vec![], |mut v, (c, i)| {
            if i % 2 == 0 {
                v.extend((0..c - b'0').map(|_| i/2));
            } else {
                v.extend((0..c - b'0').map(|_| -1));
            }
            v
        });

    // Start at the first empty block and the end which
    // we know is a file block.
    let mut i = v.iter().position(|n| *n == -1).unwrap();
    let mut j = v.len() - 1;
    while i < j {
        v.swap(i, j);
        i += 1; j -= 1;
        // look for the next empty block
        while i < j && v[i] != -1 {
            i += 1
        }
        // and the next file block
        while i < j && v[j] == -1 {
            j -= 1
        }
    }

    v.iter()
        .take_while(|&&n| n != -1)
        .zip(0..)
        .map(|(n, i)| i * n)
        .sum()
}

fn part_two(input: &str) -> i64
{
    let mut v = input.bytes()
        .zip(0..)
        .fold(vec![], |mut v, (c, i)| {
            let n = (c - b'0') as usize;
            if i % 2 == 0 {
                v.push((n, i/2));
            } else if n > 0 {
                v.push((n, -1))
            }
            v
        });

    let mut id = i64::MAX;
    let mut j = v.len() - 1;
    while j > 0 {
        j = find_file(j, &v, id);
        if j > 0 {
            id = v[j].1;
            let blocks = v[j].0;
            
            if let Some(i) = find_free(j, &v, blocks) {
                j = move_file(i, j, &mut v, blocks, id);
            } else {
                j -= 1;
            }
        }
    }

    let (res, _) = v.iter()
        .fold((0, 0), |(acc, ix), &(c, n)| {
            let res = if n == -1 {
                0
            } else {
                (ix..ix + c)
                    .map(|i| n * i as i64)
                    .sum::<i64>()
            };
            (acc + res, ix + c)
        });

    res
}

fn find_file(mut j: usize, v: &[(usize, i64)], id: i64) -> usize
{
    // Make sure the id of any file blocks found is less than
    // the last one so we don't pickup previously moved files.
    while j > 0 && (v[j].1 == -1 || v[j].1 > id) {
        j -= 1;
    }

    j
}

fn find_free(j: usize, v: &[(usize, i64)], blocks: usize) -> Option<usize>
{
    v[0..j].iter().position(|(c, n)| *n == -1 && *c >= blocks)
}

fn move_file(i: usize, j: usize, v: &mut Vec<(usize, i64)>, blocks: usize, id: i64) -> usize
{
    let (free, _) = v[i];
    if free == blocks {
        v.swap(i, j);
        j
    } else {
        v[j].1 = -1;
        v[i] = (blocks, id);
        v.insert(i+1, (free - blocks, -1));
        j + 1  // because we added a new free block
    }
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
