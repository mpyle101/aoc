
fn main() {
    use std::time::Instant;

	let t = Instant::now();
    println!("Part 1: {} ({:?})", compute(b"yzbqklnj", 5), t.elapsed());

	let t = Instant::now();
    println!("Part 2: {} ({:?})", compute(b"yzbqklnj", 6), t.elapsed());
}

fn compute(secret: &[u8;8], zeros: u8) -> u32 {
	use md5_rs::Context;

    let mut n = 1;
    loop {
		let mut ctx = Context::new();
		ctx.read(secret);
		ctx.read(n.to_string().as_bytes());
		let d = ctx.finish();

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
	fn input_part_one() {
		assert_eq!(compute(b"yzbqklnj", 5), 282749);
	}

	#[test]
	fn input_part_two() {
		assert_eq!(compute(b"yzbqklnj", 6), 9962624);
	}
}