
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

#[allow(dead_code)]
fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::yen;
    
    let goal = (3, 3);
    let passcode = input.bytes().collect::<Vec<_>>();
    let steps = yen(
        &((0, 0), passcode),
        |st| doors(st).iter().map(|st| (st.clone(), 1)).collect::<Vec<_>>(),
        |st| st.0 == goal,
        1500    // manually increase until longest path doesn't change
    );

    let mut paths = steps.iter().map(|v| v.0.last().unwrap().1.clone()).collect::<Vec<_>>();
    paths.sort_by_key(|a| a.len());
    let longest = paths.last().unwrap().len() - input.len();

    longest
}

const DELTA: [((i8, i8), u8);4] = [
    (( 0, -1), b'U'),
    (( 0,  1), b'D'),
    ((-1,  0), b'L'),
    (( 1,  0), b'R')
];

fn doors(((x, y), passcode): &State) -> Vec<State>
{
    let h = md5::compute(passcode);
    [
        h[0] >> 4 & 0x0F,
        h[0] & 0x0F,
        h[1] >> 4 & 0x0F,
        h[1] & 0x0F
    ].iter()
        .enumerate()
        .filter_map(move |(i, door)| {
            let ((dx, dy), c) = DELTA[i];
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