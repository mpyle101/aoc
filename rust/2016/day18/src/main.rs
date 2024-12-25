
const TRAPPED: [&[u8]; 4] = [
    b"^^.", b".^^", b"^..", b"..^"
];

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", safe_tiles(input, 40), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", safe_tiles(input, 400000), t.elapsed());
}

fn safe_tiles(input: &str, rows: usize) -> usize {
    let mut tiles = input.as_bytes().to_vec();
    let mut safe = tiles.iter().filter(|&b| *b == b'.').count();
    tiles.insert(0, b'.');
    tiles.push(b'.');

    for _ in 1..rows {
        let mut row = Vec::with_capacity(tiles.len());

        // Add "walls" so the window matching for the next row works.
        row.push(b'.');
        tiles.windows(3)
            .for_each(|arr| row.push(if TRAPPED.contains(&arr) { b'^' } else { b'.' }));
        row.push(b'.');

        // Subtract 2 for the "walls".
        safe += row.iter().filter(|&b| *b == b'.').count() - 2;
        tiles = row;
    }

    safe
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(safe_tiles(input, 40), 1956);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(safe_tiles(input, 400000), 19995121);
    }
}