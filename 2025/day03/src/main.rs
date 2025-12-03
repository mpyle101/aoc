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

#[allow(clippy::needless_range_loop)]
fn part_one(input: &str) -> u32
{
    input.lines()
        .map(|line| {
            let v = line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            let mut n = 0;
            for i in 0..v.len()-1 {
                let n1 = v[i] * 10;
                for j in i+1..v.len() {
                    let n2 = n1 + v[j];
                    n = n.max(n2);
                }
            }

            n
        })
        .sum()
}

fn part_two(input: &str) -> u64
{
    input.lines()
        .map(|line| {
            let v = line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>();

            let mut drops = v.len() - 12;
            let mut st = Vec::with_capacity(line.len());
            st.push(v[0]);

            v.iter()
                .skip(1)
                .for_each(|n| {
                    while drops > 0 && let Some(m) = st.last() && m < n {
                        st.pop();
                        drops -= 1;
                    }
                    st.push(*n);
                });

            st[0..12].iter()
                .rev()
                .zip(0..)
                .fold(0, |acc, (n, i)| acc + n * 10u64.pow(i))
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 17430);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 171975854269367);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 357);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 3121910778619);
    }
}
