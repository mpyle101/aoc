
fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());
}

fn part_one(input: &str) -> String
{
    let n = input.lines().map(fubar).sum();
    snafu(n)
}

fn snafu(mut n: i64) -> String
{
    let mut s = String::new();
    while n > 0 {
        let c = (n % 5) as u8 + b'0';
        let (c, x) = match c {
            b'4' => ('-', n + 5),
            b'3' => ('=', n + 5),
              _  => (c as char, n),
        };
        s.insert(0, c);
        n = x / 5;
    }

    s
}

fn fubar(s: &str) -> i64
{
    let (n, _) = s.chars()
        .rev()
        .fold((0, 0), |(n, p), c| {
            let v = match c {
                '2' =>  2,
                '1' =>  1,
                '0' =>  0,
                '-' => -1,
                '=' => -2,
                 _  => unreachable!()
            };
            (n + v * 5i64.pow(p), p + 1)
        });

    n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "2-21=02=1-121-2-11-0");
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), "2=-1=0");
    }

    #[test]
    fn snafu_works()
    {
        assert_eq!(snafu(1), "1");
        assert_eq!(snafu(2), "2");
        assert_eq!(snafu(3), "1=");
        assert_eq!(snafu(4), "1-");
        assert_eq!(snafu(5), "10");
        assert_eq!(snafu(6), "11");
        assert_eq!(snafu(7), "12");
        assert_eq!(snafu(8), "2=");
        assert_eq!(snafu(9), "2-");
        assert_eq!(snafu(10), "20");
        assert_eq!(snafu(15), "1=0");
        assert_eq!(snafu(20), "1-0");
        assert_eq!(snafu(2022), "1=11-2");
        assert_eq!(snafu(4890), "2=-1=0");
        assert_eq!(snafu(12345), "1-0---0");
        assert_eq!(snafu(314159265), "1121-1110-1=0");
        assert_eq!(snafu(35951702021395), "2-21=02=1-121-2-11-0");
    }

    #[test]
    fn fubar_works()
    {
        assert_eq!(fubar("1=-0-2"), 1747);
        assert_eq!(fubar("12111"), 906);
        assert_eq!(fubar("2=0="), 198);
        assert_eq!(fubar("21"), 11);
        assert_eq!(fubar("2=01"), 201);
        assert_eq!(fubar("111"), 31);
        assert_eq!(fubar("20012"), 1257);
        assert_eq!(fubar("1-12"), 107);
        assert_eq!(fubar("12"), 7);
        assert_eq!(fubar("1="), 3);
        assert_eq!(fubar("122"), 37);
        assert_eq!(fubar("2=-01"), 976);
        assert_eq!(fubar("2-21=02=1-121-2-11-0"), 35951702021395);
    }
}
