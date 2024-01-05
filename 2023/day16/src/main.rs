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
    let (ncols, tiles) = load(input);
    energized(ncols, tiles, (RT, 0))
}

fn part_two(input: &str) -> usize
{
    use rayon::prelude::*;

    let (ncols, tiles) = load(input);
    let nrows = tiles.len() / ncols;
    let lcol  = ncols - 1;
    let lrow  = nrows - 1;

    let mut start = Vec::with_capacity(500);
    start.extend((0..ncols).map(|col| (DN, col)));
    start.extend((0..ncols).map(|col| (UP, lrow * ncols + col)));
    start.extend((0..nrows).map(|row| (RT, row * ncols)));
    start.extend((0..nrows).map(|row| (LT, row * ncols + lcol)));
    start.into_par_iter()
        .map(|st| energized(ncols, tiles.clone(), st))
        .max()
        .unwrap()
}

// Type of tile.
const OPEN_SPACE: u8 = 0b00000000;
const SPLITTER_H: u8 = 0b00000001;
const SPLITTER_V: u8 = 0b00000010;
const MIRROR_FWD: u8 = 0b00000100;
const MIRROR_BWD: u8 = 0b00001000;
const TILE_MASK:  u8 = 0b00001111;

// Direction the beam is traveling when it
// enters the tile.
const UP: u8 = 0b00010000;
const DN: u8 = 0b00100000;
const LT: u8 = 0b01000000;
const RT: u8 = 0b10000000;
const DIR_MASK: u8 = 0b11110000;

fn load(input: &str) -> (usize, Vec<u8>)
{
    let mut ncols = 0;
    let tiles = input.lines()
        .flat_map(|line| {
            ncols = line.len();
            line.chars()
                .map(|c| match c {
                    '.'  => OPEN_SPACE,
                    '-'  => SPLITTER_H,
                    '|'  => SPLITTER_V,
                    '/'  => MIRROR_FWD,
                    '\\' => MIRROR_BWD,
                    _ => panic!("Unknown tile type: {c}")
                })
        })
        .collect();

    (ncols, tiles)
}

fn energized(
    ncols: usize,
    mut tiles: Vec<u8>,
    start: (u8, usize)
) -> usize
{
    let mut beams = vec![start];

    while let Some((dir, ix)) = beams.pop() {
        if tiles[ix] & dir == 0 {
            tiles[ix] |= dir;
            beams.extend(
                radiate(ncols, &tiles, ix, dir).iter().flatten()
            )
        }
    }

    tiles.iter().filter(|t| *t & DIR_MASK > 0).count()
}

fn radiate(ncols: usize, tiles: &[u8], ix: usize, dir: u8) -> [Option<(u8, usize)>;2]
{
    let mut states = [None;2];

    let row  = ix / ncols;
    let col  = ix % ncols;
    let lrow = (tiles.len() / ncols) - 1;
    let lcol = ncols - 1;

    let tile = tiles[ix] & TILE_MASK;
    let curr = tile | dir;
    if curr == SPLITTER_V | RT || curr == SPLITTER_V | LT {
        if row > 0 { states[0] = Some((UP, ix - ncols)) }
        if row < lrow { states[1] = Some((DN, ix + ncols ))}
    } else if curr == SPLITTER_H | UP || curr == SPLITTER_H | DN {
        if col > 0 { states[0] = Some((LT, ix - 1)) }
        if col < lcol { states[1] = Some((RT, ix + 1)) }
    } else if tile < MIRROR_FWD {
        states[0] = match dir {
            UP if row > 0    => Some((UP, ix - ncols)),
            DN if row < lrow => Some((DN, ix + ncols)),
            LT if col > 0    => Some((LT, ix - 1)),
            RT if col < lcol => Some((RT, ix + 1)),
            _ => None
        }
    } else {
        states[0] = match (dir, tile) {
            (RT, MIRROR_FWD) if row > 0    => Some((UP, ix - ncols)),
            (RT, MIRROR_BWD) if row < lrow => Some((DN, ix + ncols)),
            (LT, MIRROR_FWD) if row < lrow => Some((DN, ix + ncols)),
            (LT, MIRROR_BWD) if row > 0    => Some((UP, ix - ncols)),
            (UP, MIRROR_FWD) if col < lcol => Some((RT, ix + 1)),
            (UP, MIRROR_BWD) if col > 0    => Some((LT, ix - 1)),
            (DN, MIRROR_FWD) if col > 0    => Some((LT, ix - 1)),
            (DN, MIRROR_BWD) if col < lcol => Some((RT, ix + 1)),
            _ => None
        }
    }

    states
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 8021);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 8216);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 46);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 51);
    }
}
