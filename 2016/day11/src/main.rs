
fn main()
{
    use std::time::Instant;

    // Didn't feel like parsing the input (really? sentences?).

    // State values represent the floor a given object is on.
    // state[0] is the elevator; after that each pair of
    // values is the microchip and generator location for a
    // given isotope. The Isotope enum specifies the offset
    // in the state for a given isotope's floor values. From
    // this, the goal state consists of an array of all 4's.

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(), t.elapsed());
}

fn part_one() -> usize
{
    use pathfinding::prelude::bfs;

    let state = [1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1];
    // Cobalt, Polonium, Promethium, Ruthenium, Thulium
    let isotopes = [1, 3, 5, 7, 9];

    let goal = [4u8;11];
    let steps = bfs(&state, |st| next_states(st, &isotopes), |&st| st == goal);

    let v = steps.unwrap();

    // The vector contains the initial state.
    v.len() - 1
}

#[allow(dead_code)]
fn part_two() -> usize
{
    use pathfinding::prelude::bfs;

    let state = [1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    // Cobalt, Polonium, Promethium, Ruthenium, Thulium, Elerium, Dilithium
    let isotopes = [1, 3, 5, 7, 9, 11, 13];

    let goal = [4u8;15];
    let steps = bfs(&state, |st| next_states(st, &isotopes), |&st| st == goal);

    let v = steps.unwrap();

    // The vector contains the initial state.
    v.len() - 1
}

fn next_states<const N: usize>(state: &[u8;N], isotopes: &[usize]) -> Vec<[u8;N]>
{
    // Get the floors the elevator can move to.
    let elevator = state[0];
    let floors: u8 = match elevator {
        1 => 0b0010,
        2 => 0b0101,
        3 => 0b1010,
        4 => 0b0100,
        _ => unreachable!()
    };

    // Get the objects on the current floor (skip the elevator).
    let objects = state.iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, &n)| (n == elevator).then_some(i))
        .collect::<Vec<_>>();

    // Get all possible states of moving one or two objects
    // to the available floors and filter out the ones with
    // unprotected microchips on the same floor as generators.
    get_all(state, &objects, floors)
        .filter(|st| valid(st, isotopes))
        .collect()
}

fn get_all<'a, const N: usize>(
    state: &'a [u8;N],
    objects: &'a [usize],
    floors: u8) -> impl Iterator<Item = [u8;N]> + 'a
{
    use bit_iter::BitIter;
    use itertools::Itertools;

    let iter = BitIter::from(floors);
    iter.flat_map(move |b| {
        let floor = b as u8 + 1;
        objects.iter()
            .map(move |&i| {
                let mut st = *state;
                st[0] = floor;
                st[i] = floor;
                st
            })
        .chain(objects.iter()
            .combinations(2)
            .map(move |v| {
                let mut st = *state;
                st[0] = floor;
                st[*v[0]] = floor;
                st[*v[1]] = floor;
                st
            })
        )
    })
}

fn valid<const N: usize>(state: &[u8;N], isotopes: &[usize]) -> bool
{
    // A state is invalid if there are unprotected microchips
    // on the same floor as a generator for another isotope.
    !isotopes.iter()
        .filter(|&i| state[*i] != state[i + 1])
        .any(|&i| isotopes.iter().any(|n| state[i] == state[n + 1]))
}

#[allow(dead_code)]
fn print(state: &[u8])
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
    
    let mut floor = 4;
    while floor > 0 {
        print!("F{} ", floor);
        state.iter().enumerate()
            .for_each(|(i, n)| if *n == floor {
                    print!("{} ", symbols[i])
                } else {
                    print!(".  ")
                }
            );
        println!();
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