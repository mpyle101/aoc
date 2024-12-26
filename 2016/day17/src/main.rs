use std::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::FxHasher;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

fn main()
{
    use std::time::Instant;

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one("veumntbg"), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two("veumntbg"), t.elapsed());
}

type State = ((i8, i8), Vec<u8>);

fn part_one(input: &str) -> String
{
    use pathfinding::prelude::bfs;
    
    let goal = (3, 3);
    let passcode = input.bytes().collect::<Vec<_>>();
    let steps = bfs(&((0, 0), passcode), doors, |st| st.0 == goal).unwrap();
    let (_, v) = steps.last().unwrap();
    
    v[input.len()..].iter().map(|&b| b as char).collect::<String>()
}

fn part_two(input: &str) -> usize
{
    let goal = (3, 3);
    let passcode = input.bytes().collect::<Vec<_>>();
    
    bfs_longest(&((0, 0), passcode), doors, goal)
}

const DOORS: [((i8, i8), u8);4] = [
    (( 0, -1), b'U'),
    (( 0,  1), b'D'),
    ((-1,  0), b'L'),
    (( 1,  0), b'R')
];

fn doors(((x, y), passcode): &State) -> Vec<State>
{
    // Each set of 4 bits is the hex value we want.
    let h = md5::compute(passcode);
    [
        h[0] >> 4 & 0x0F,
        h[0] & 0x0F,
        h[1] >> 4 & 0x0F,
        h[1] & 0x0F
    ].iter()
        .enumerate()
        .filter_map(move |(i, door)| {
            let ((dx, dy), c) = DOORS[i];
            let pt = (x + dx, y + dy);
            (is_open(door) && in_bounds(pt)).then_some({
                let mut v = passcode.clone();
                v.push(c);
                (pt, v)
            })
        })
        .collect()
}

fn in_bounds((x, y): (i8, i8)) -> bool
{
    (0..4).contains(&x) && (0..4).contains(&y)
}

fn is_open(b: &u8) -> bool
{
    // hex b, c, d, e & f
    (11..16).contains(b)
}

fn bfs_longest<FN, IN>(
    start: &State,
    mut successors: FN,
    goal: (i8, i8)) -> usize
where
    FN: FnMut(&State) -> IN,
    IN: IntoIterator<Item = State>,
{
    use indexmap::map::Entry::*;

    let mut i = 0;
    let mut longest = 0;
    let mut parents: FxIndexMap<State, usize> = FxIndexMap::default();
    parents.insert(start.clone(), usize::MAX);
    while let Some((node, _)) = parents.get_index(i) {
        for st in successors(node) {
            if st.0 == goal {
                longest = longest.max(bfs_length(&parents, i));
            } else if let Vacant(e) = parents.entry(st) { 
                e.insert(i);
            }
        }
        i += 1;
    }

    longest
}

fn bfs_length(parents: &FxIndexMap<State, usize>, start: usize) -> usize
{
    let mut count = 0;
    let mut i = start;

    while let Some((_, value)) = parents.get_index(i) {
        count += 1;
        i = *value;
    }

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        assert_eq!(part_one("veumntbg"), "DDRRULRDRD");
    }

    #[test]
    fn input_part_two()
    {
        assert_eq!(part_two("veumntbg"), 536);
    }
}