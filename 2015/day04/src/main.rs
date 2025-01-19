
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

fn part_one(input: &str) -> u32
{
    compute(input.as_bytes(), 5)
}

fn part_two(input: &str) -> u32
{
    compute(input.as_bytes(), 6)
}

fn compute(secret: &[u8], zeros: u8) -> u32
{
	use md5::Context;

	let mut context = Context::new();
	context.consume(secret);

    let mut n: u32 = 1;
    loop {
		let mut ctx = context.clone();
		ctx.consume(n.to_string().as_bytes());
		let d = ctx.compute();

		// Bytes are 8 bits, hexadecimal values only use 4.
		if (d[0] | d[1] == 0) && (d[2] == 0 || zeros == 5 && d[2] & 0xF0 == 0) {
			return n
		}

		n += 1
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 282749);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 9962624);
    }
}