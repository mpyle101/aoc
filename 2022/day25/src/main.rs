fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let snafu = part_one(input);
    println!("Part 1: {} ({:?})", snafu, t.elapsed());
}

fn part_one(input: &str) -> String
{
    let n = input.lines().map(fubar).sum();
    snafu(n)
}

fn snafu(mut n: i64) -> String
{
    use std::char::from_digit;
    let mut s = String::new();

    while let Some(c) = from_digit((n % 5) as u32, 5) {
        let (c, x) = match c {
            '4' => ('-', n + 5),
            '3' => ('=', n + 5),
             _  => (c, n),
        };
        s.insert(0, c);
        n = x / 5;
        if n == 0 { return s }
    }

    unreachable!()
}

fn fubar(s: &str) -> i64
{
    let (n, _) = s.chars()
        .rev()
        .fold((0, 1), |(n, p), c| {
            let v = match c {
                '2' => 2 * p,
                '1' => p,
                '0' => 0,
                '-' => -p,
                '=' => -2 * p,
                 _  => unreachable!()
            };
            (v + n, p * 5)
        });

    n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        let input = include_str!("../input.txt");

        let snafu = part_one(input);
        assert_eq!(snafu, "2-21=02=1-121-2-11-0");
    }

    #[test]
    fn example()
    {
        let input = include_str!("../example.txt");

        let snafu = part_one(input);
        assert_eq!(snafu, "2=-1=0");
    }

    #[test]
    fn snafu_works() {
        let s = snafu(1);
        assert_eq!(s, "1");

        let s = snafu(2);
        assert_eq!(s, "2");

        let s = snafu(3);
        assert_eq!(s, "1=");

        let s = snafu(4);
        assert_eq!(s, "1-");

        let s = snafu(5);
        assert_eq!(s, "10");

        let s = snafu(6);
        assert_eq!(s, "11");

        let s = snafu(7);
        assert_eq!(s, "12");

        let s = snafu(8);
        assert_eq!(s, "2=");

        let s = snafu(9);
        assert_eq!(s, "2-");

        let s = snafu(10);
        assert_eq!(s, "20");

        let s = snafu(15);
        assert_eq!(s, "1=0");

        let s = snafu(20);
        assert_eq!(s, "1-0");

        let s = snafu(2022);
        assert_eq!(s, "1=11-2");

        let s = snafu(4890);
        assert_eq!(s, "2=-1=0");

        let s = snafu(12345);
        assert_eq!(s, "1-0---0");

        let s = snafu(314159265);
        assert_eq!(s, "1121-1110-1=0");

        let s = snafu(35951702021395);
        assert_eq!(s, "2-21=02=1-121-2-11-0");
    }

    #[test]
    fn fubar_works() {
        let n = fubar("1=-0-2");
        assert_eq!(n, 1747);

        let n = fubar("12111");
        assert_eq!(n, 906);

        let n = fubar("2=0=");
        assert_eq!(n, 198);

        let n = fubar("21");
        assert_eq!(n, 11);

        let n = fubar("2=01");
        assert_eq!(n, 201);

        let n = fubar("111");
        assert_eq!(n, 31);

        let n = fubar("20012");
        assert_eq!(n, 1257);

        let n = fubar("1-12");
        assert_eq!(n, 107);

        let n = fubar("12");
        assert_eq!(n, 7);

        let n = fubar("1=");
        assert_eq!(n, 3);

        let n = fubar("122");
        assert_eq!(n, 37);

        let n = fubar("2=-01");
        assert_eq!(n, 976);

        let n = fubar("2-21=02=1-121-2-11-0");
        assert_eq!(n, 35951702021395);
    }
}
