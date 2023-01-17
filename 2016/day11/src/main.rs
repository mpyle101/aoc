use std::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::FxHasher;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

const BITS: u32 = 0x3;

fn main()
{
    use std::time::Instant;

    // Didn't feel like parsing the input (really? sentences?).

    // State values represent the floor a given object is on.
    // The first two bits are the elevator, after that there
    // are two bits per object with a microchip followed by
    // it's generator.

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(), t.elapsed());
}

fn part_one() -> usize
{
    // Thulium, Ruthenium, Promethium, Polonium, Cobalt, Elevator
    let start: u32 = 0b00_00_00_00_00_01_00_01_00_00_00;
    let goal:  u32 = 0b11_11_11_11_11_11_11_11_11_11_11;

    // Cobalt, Polonium, Promethium, Ruthenium, Thulium, Elevator
    let isotopes = [2, 6, 10, 14, 18];
    let steps = bfs(start, |st| next_states::<5>(st, &isotopes), goal);

    steps.unwrap()
}

#[allow(dead_code)]
fn part_two() -> usize
{
    // Dilithium, Elerium, Thulium, Ruthenium, Promethium, Polonium, Cobalt, Elevator
    let start: u32 = 0b00_00_00_00_00_00_00_00_00_01_00_01_00_00_00;
    let goal:  u32 = 0b11_11_11_11_11_11_11_11_11_11_11_11_11_11_11;

    // Cobalt, Polonium, Promethium, Ruthenium, Thulium, Elerium, Dilithium
    let isotopes = [2, 6, 10, 14, 18, 22, 26];
    let steps = bfs(start, |st| next_states::<7>(st, &isotopes), goal);

    steps.unwrap()
}

fn next_states<const I: usize>(state: u32, isotopes: &[usize]) -> Vec<u32>
{
    // Get the floors the elevator can move to.
    let elevator = state & 0x3;
    let floors: u8 = match elevator {
        0 => 0b0010,
        1 => 0b0101,
        2 => 0b1010,
        3 => 0b0100,
        _ => unreachable!()
    };

    // Get the objects on the current floor (skip the elevator).
    let objects = (1..=I*2)
        .map(|i| i*2)
        .filter(|&i| state & (BITS << i) == elevator << i)
        .collect::<Vec<_>>();

    // Get all possible states of moving one or two objects
    // to the available floors and filter out the ones with
    // unprotected microchips on the same floor as generators.
    get_all(state, &objects, floors)
        .filter(|&st| valid(st, isotopes))
        .collect()
}

fn get_all(
    state: u32,
    objects: &[usize],
    floors: u8) -> impl Iterator<Item = u32> + '_
{
    use bit_iter::BitIter;
    use itertools::Itertools;

    let iter = BitIter::from(floors);
    iter.flat_map(move |b| {
        let floor = b as u32;
        objects.iter()
            .map(move |&i| {
                let st = (state & !BITS) | (floor & BITS);
                let mask = BITS << i;
                (st & !mask) | ((floor << i) & mask)
            })
        .chain(objects.iter()
            .combinations(2)
            .map(move |v| {
                let st = (state & !BITS) | (floor & BITS);
                let mask = (BITS << *v[0]) | (BITS << *v[1]);
                (st & !mask) | ((floor << *v[0]) & mask) | ((floor << *v[1]) & mask)
            })
        )
    })
}

fn valid(state: u32, isotopes: &[usize]) -> bool
{
    // A state is invalid if there are unprotected microchips
    // on the same floor as a generator for another isotope.
    let val = |i| (state & (BITS << i)) >> i;

    !isotopes.iter()
        .filter(|&&i| val(i) != val(i+2))
        .any(|&i| isotopes.iter().any(|&n| val(i) == val(n+2)))
}

// Lifted from the pathfinding crate and modified to our
// specific needs: we don't need the whole path, just the
// length and the first step.
fn bfs<FN, IN>(
    start: u32,
    mut successors: FN,
    goal: u32) -> Option<usize>
where
    FN: FnMut(u32) -> IN,
    IN: IntoIterator<Item = u32>,
{
    use indexmap::map::Entry::Vacant;

    let mut i = 0;
    let mut parents: FxIndexMap<u32, usize> = FxIndexMap::default();
    parents.insert(start, usize::max_value());
    while let Some((node, _)) = parents.get_index(i) {
        for successor in successors(*node) {
            if successor == goal {
                return Some(bfs_length(&parents, i));
            }
            if let Vacant(e) = parents.entry(successor) {
                e.insert(i);
            }
        }
        i += 1;
    }

    None
}

fn bfs_length(parents: &FxIndexMap<u32, usize>, start: usize) -> usize
{
    let mut count = 0;
    let mut i = start;

    while let Some((_, value)) = parents.get_index(i) {
        count += 1;
        i = *value;
    }

    count
}

#[allow(dead_code)]
fn print<const I: usize>(state: u32)
{
    let symbols = [
        "E ",
        "Cm", "Cg",
        "Pm", "Pg",
        "Qm", "Qg",
        "Rm", "Rg",
        "Tm", "Tg",
        "Em", "Eg",
        "Dm", "Dg"
    ];
    
    let val = |i| { let st = state & (0x3u32 << i); st >> i};

    let mut floor = 3;
    loop {
        print!("F{} ", floor + 1);
        (0..=I*4)
            .enumerate()
            .step_by(2)
            .for_each(|(i, n)| if val(n) == floor {
                    print!("{} ", symbols[i/2])
                } else {
                    print!(".  ")
                }
            );
        println!();
        if floor == 0 { break; }
        floor -= 1;
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        assert_eq!(part_one(), 47);
    }

    #[test]
    fn input_part_two()
    {
        assert_eq!(part_two(), 71);
    }
}