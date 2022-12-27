
fn main() {
    use std::time::Instant;

	let t = Instant::now();
    println!("Part 1: {} ({:?})", compute("yzbqklnj", "00000"), t.elapsed());

	let t = Instant::now();
    println!("Part 2: {} ({:?})", compute("yzbqklnj", "000000"), t.elapsed());
}

fn compute(secret: &str, tag: &str) -> u32 {
    let mut n = 0;
    loop {
		let key = secret.to_owned() + &n.to_string();
		let digest = md5::compute(key);
		if format!("{:x}", digest).starts_with(tag) {
			break n
		}
		n += 1
    }
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn input_part_one() {
		assert_eq!(compute("yzbqklnj", "00000"), 282749);
	}

	#[test]
	fn input_part_two() {
		assert_eq!(compute("yzbqklnj", "000000"), 9962624);
	}
}