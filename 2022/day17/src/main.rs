use std::collections::HashSet;
use lazy_static::lazy_static;

type Shaft = HashSet<(i64, i64)>;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let height = part_one(input);
    println!("Part 1: {} ({:?})", height, t.elapsed());

   let t = Instant::now();
   let height = part_two(input, 1_000_000_000_000);
   println!("Part 2: {} ({:?})", height, t.elapsed());
}

fn part_one(input: &str) -> i64 {
    let v = Rock::rocks();
    let mut rocks = v.iter().cycle();
    let mut wind = input.bytes().cycle();
    
    let mut height = 1;
    let mut shaft = Shaft::new();

    let mut count = 0;
    while count < 2022 {
        let mut rock = rocks.next().unwrap().move_y(height + 3);

        loop {
            let dir = wind.next().unwrap();
            rock = shift(&rock, dir, &shaft).unwrap_or(rock);
            if let Some(r) = fall(&rock, &shaft) { 
                rock = r
            } else {
                height = rock.add(&mut shaft, height);                
                break
            }
        }
        
        count += 1
    }

    height - 1
}

fn part_two(input: &str, num_rocks: u128) -> u128 {
    // There's a cycle in the simulation but it's kinda funky.
    // The first repeated state you hit is not the cycle, it's
    // the begining state of the cycle. When that state is hit
    // again is the actual cycle. So, we need to accumulate the
    // height up to that first state, then capture the height
    // delta to the next state and use that height and count delta
    // to do the modulo arithmetic.

    let mut r_ix = 0usize;
    let rocks = Rock::rocks();

    let mut w_ix = 0usize;
    let wind = input.as_bytes();
    
    let mut shaft = Shaft::new();
    let mut height = 1;
    let mut d_height = 0;
    let mut b_height = 0;
    let mut p_height = 0;

    let mut floor: [i64;7] = [0;7];
    let mut state: [i64;7] = [0;7];
    let mut states = HashSet::new();

    let mut found = false;
    let mut d_count = 0u128;
    let mut p_count = 0u128;

    let mut count = 0u128;
    while count < num_rocks {
        p_height = height;
        let mut rock = rocks[r_ix].move_y(height + 3);

        loop {
            rock = shift(&rock, wind[w_ix], &shaft).unwrap_or(rock);
            w_ix = (w_ix + 1) % wind.len();

            if let Some(r) = fall(&rock, &shaft) { 
                rock = r
            } else {
                height = rock.add(&mut shaft, height);                
                break
            }
        }
        r_ix = (r_ix + 1) % rocks.len();

        rock.iter().for_each(|p| {
            let ix = p.0 as usize;
            floor[ix] = p.1.max(floor[ix]);
        });
        let offset = height - floor.iter().min().unwrap();
        floor.iter().enumerate().for_each(|(i, n)| state[i] = offset - (height - n));
        if !states.insert((r_ix, w_ix, state)) {
            if found {
                d_count = count - p_count;
                d_height = p_height - b_height;
                break;
            } else {
                found = true;
                p_count = count;
                b_height = p_height;
                states.clear();
                states.insert((r_ix, w_ix, state));
            }
        }

        count += 1;
    }

    let m = (num_rocks - p_count) / d_count;
    let base = b_height as u128 + (d_height as u128 * m);
    let num_rocks = (num_rocks - p_count) % d_count;
    let mut count = 1;
    while count < num_rocks {
        let mut rock = rocks[r_ix].move_y(height + 3);
        loop {
            rock = shift(&rock, wind[w_ix], &shaft).unwrap_or(rock);
            w_ix = (w_ix + 1) % wind.len();

            if let Some(r) = fall(&rock, &shaft) { 
                rock = r
            } else {
                height = rock.add(&mut shaft, height);                
                break
            }
        }
        r_ix = (r_ix + 1) % rocks.len();
        count += 1;
    }
    height -= p_height;

    base + height as u128 - 1
}


fn fall(rock: &Rock, shaft: &Shaft) -> Option<Rock> {
    let r = rock.move_y(-1);
    if r.y < 1 || r.iter().any(|p| shaft.contains(&p)) {
        None
    } else {
        Some(r)
    }
}

fn shift(rock: &Rock, dir: u8, shaft: &Shaft) -> Option<Rock> {
    let r = match dir {
        b'>' => rock.move_x(1),
        b'<' => rock.move_x(-1),
        _ => unreachable!()
    };

    if r.x < 0 || r.x + r.wd > 7 || r.iter().any(|p| shaft.contains(&p)) { 
        None
    } else {
        Some(r)
    }
}

lazy_static! {
    static ref SHAPES: [Vec<(i64, i64)>;5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
}

#[derive(Clone, Debug)]
struct Rock {
    x: i64,
    y: i64,
    ht: i64,
    wd: i64,
    ix: usize,
}

impl Rock {
    fn new(ht: i64, wd: i64, ix: usize) -> Rock {
        Rock { ht, wd, ix, x: 2, y: 0 }
    }

    fn move_y(&self, y: i64) -> Rock {
        let mut r = self.clone();
        r.y += y;
        r
    }

    fn move_x(&self, x: i64) -> Rock {
        let mut r = self.clone();
        r.x += x;
        r
    }

    fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        SHAPES[self.ix].iter()
            .map(|p| (p.0 + self.x, p.1 + self.y))
    }

    fn add(&self, shaft: &mut Shaft, height: i64) -> i64 {
        SHAPES[self.ix].iter()
            .map(|p| (p.0 + self.x, p.1 + self.y))
            .for_each(|p| { shaft.insert(p); });
        height.max(self.y + self.ht)
    }

    fn rocks() -> Vec<Rock> {
        vec![
            Rock::new(1, 4, 0),
            Rock::new(3, 3, 1),
            Rock::new(3, 3, 2),
            Rock::new(4, 1, 3),
            Rock::new(2, 2, 4),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let height = part_one(input);
        assert_eq!(height, 3209);

        let height = part_two(input, 10_000);
        assert_eq!(height, 15_841);

        let height = part_two(input, 100_000);
        assert_eq!(height, 158_076);

        let height = part_two(input, 1_000_000);
        assert_eq!(height, 1_580_778);

        let height = part_two(input, 10_000_000);
        assert_eq!(height, 15_807_609);

        let height = part_two(input, 1_000_000_000_000);
        assert_eq!(height, 1_580_758_017_509);
    }

    #[test]
    fn examples() {
        let input = include_str!("../example.txt");

        let height = part_one(input);
        assert_eq!(height, 3068);

        let height = part_two(input, 10_000);
        assert_eq!(height, 15_148);

        let height = part_two(input, 100_000);
        assert_eq!(height, 151_434);

        let height = part_two(input, 1_000_000);
        assert_eq!(height, 1_514_288);

        let height = part_two(input, 10_000_000);
        assert_eq!(height, 15_142_861);

        let height = part_two(input, 1_000_000_000_000);
        assert_eq!(height, 1_514_285_714_288);
    }
}
