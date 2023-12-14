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

fn part_one(input: &str) -> usize
{
    let patterns = input.split("\n\n")
        .map(|s| load(s.as_bytes()))
        .map(|(ncols, v)| compute_hashes(ncols, &v))
        .collect::<Vec<_>>();

    patterns.iter()
        .map(|(cols, rows)| find_reflection(cols, rows))
        .map(|(_, _, n)| n)
        .sum()
}

fn part_two(input: &str) -> usize
{
    input.split("\n\n")
        .map(|orig| {
            let (ncols, v) = load(orig.as_bytes());
            let (cols, rows) = compute_hashes(ncols, &v);
            let (axis, idx, _) = find_reflection(&cols, &rows);

            let mut count = 0;
            let mut s = orig.as_bytes().to_vec();
            for i in 0..s.len() {
                if s[i] != b'\n' {
                    let b = s[i];
                    if b == b'#' { s[i] = b'.' } else { s[i] = b'#' }
    
                    let (ncols, v) = load(&s);
                    let (cols, rows) = compute_hashes(ncols, &v);
                    if let Some(n) = find_alt_reflection(&cols, &rows, (axis, idx)) {
                        count = n;
                        break;
                    }
    
                    s[i] = b;
                }
            }

            count
        })
        .sum()
}

fn load(input: &[u8]) -> (u32, Vec<u32>)
{
    let mut ncols = 0;

    let pattern = input.split(|b| *b == b'\n')
        .zip(0..)
        .flat_map(|(line, row)| {
            ncols = line.len() as u32;
            line.iter()
                .zip(0..)
                .filter(|(&c, _)| c == b'#')
                .map(|(_, col)| row * ncols + col)
                .collect::<Vec<_>>()
        })
        .collect();

    (ncols, pattern)
}

fn find_reflection(cols: &[u64], rows: &[u64]) -> (char, usize, usize)
{
    if let Some(i) = reflect(cols, 1000) {
        ('c', i, i + 1)
    } else if let Some(i) = reflect(rows, 1000) {
        ('r', i, (i + 1) * 100)
    } else {
        ('*', 0, 0)
    }
}

fn find_alt_reflection(cols: &[u64], rows: &[u64], skip:(char, usize)) -> Option<usize>
{  
    let n = if skip.0 == 'c' { skip.1 } else { 1000 };
    if let Some(i) = reflect(cols, n) {
        return Some(i + 1)
    }

    let n = if skip.0 == 'r' { skip.1 } else { 1000 };
    if let Some(i) = reflect(rows, n) {
        return Some((i + 1) * 100)
    }

    None
}

fn reflect(vals: &[u64], skip: usize) -> Option<usize>
{
    for i in 0..vals.len() - 1 {
        if i != skip && vals[i] == vals[i+1] {
            let len = vals.len() as i32;
            let mut a = i as i32 - 1;
            let mut b = i as i32 + 2;
            while a >= 0 && b < len && vals[a as usize] == vals[b as usize] {
                a -= 1;
                b += 1;
            }
            if i == 0 || a < 0 || b == len {
                return Some(i)
            }
        }
    }

    None
}

fn compute_hashes(ncols: u32, v: &[u32]) -> (Vec<u64>, Vec<u64>)
{
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let nrows = v[v.len() - 1] / ncols + 1;

    let cols: Vec<u64> = (0..ncols)
        .map(|col| {
            let mut hasher = DefaultHasher::new();
            v.iter().for_each(|pos| {
                if pos % ncols == col {
                    (pos / ncols).hash(&mut hasher)
                }
            });
            hasher.finish()
        })
        .collect();

    let rows: Vec<u64> = (0..nrows)
        .map(|row| {
            let mut hasher = DefaultHasher::new();
            v.iter().for_each(|pos| {
                if pos / ncols == row {
                    (pos % ncols).hash(&mut hasher)
                }
            });
            hasher.finish()
        })
        .collect();

    (cols, rows)

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 34100);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 33106);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 405);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 400);
    }
}
