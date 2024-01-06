use std::ops::Range;
use pathfinding::matrix::Matrix;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let cards = part_one(input);
    println!("Part 1: {} ({:?})", cards, t.elapsed());

    let t = Instant::now();
    let cards = part_two(input);
    println!("Part 2: {} ({:?})", cards, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use pathfinding::prelude::dijkstra;

    let m = Matrix::from_rows(input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap())
        )).unwrap();

    let goal   = (m.rows - 1, m.columns - 1);
    let start  = State { pos: (0, 0), dir: '>', moves: 0 };
    let result = dijkstra(
        &start,
        |state| next_moves(state, 0..3, &m).into_iter().flatten(),
        |&p| p.pos == goal)
        .unwrap();

    result.1
}

fn part_two(input: &str) -> u32
{
    use pathfinding::prelude::dijkstra;

    let m = Matrix::from_rows(input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap())
        )).unwrap();

    let goal   = (m.rows - 1, m.columns - 1);
    let start  = State { pos: (0, 0), dir: '>', moves: 0 };
    let result = dijkstra(
        &start,
        |state| next_moves(state, 4..10, &m).into_iter().flatten(),
        |&st| st.pos == goal && st.moves >= 4)
        .unwrap();

    result.1
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: (usize, usize),
    dir: char,
    moves: u8,
}

fn next_moves(state: &State, valid: Range<u8>, m: &Matrix<u32>) -> [Option<(State, u32)>;3]
{
    let mut moves = [None;3];

    if state.dir == '>' {
        moves[0] = move_rt(state, &valid, m.columns)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[1] = turn_up(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[2] = turn_dn(state, &valid, m.rows)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
    } else if state.dir == '<' {
        moves[0] = move_lt(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[1] = turn_up(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[2] = turn_dn(state, &valid, m.rows)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
    } else if state.dir == '^' {
        moves[0] = move_up(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[1] = turn_lt(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[2] = turn_rt(state, &valid, m.columns)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
    } else if state.dir == 'v' {
        moves[0] = move_dn(state, &valid, m.rows)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[1] = turn_lt(state, &valid)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
        moves[2] = turn_rt(state, &valid, m.columns)
            .and_then(|st| m.get(st.pos).map(|n| (st, *n)));
    }

    moves
}

fn move_up(state: &State, valid: &Range<u8>) -> Option<State>
{
    if state.moves < valid.end && state.pos.0 > 0 {
        let pos = (state.pos.0 - 1, state.pos.1);
        Some(State { pos, dir: '^', moves: state.moves + 1 })
    } else {
        None
    }
}

fn move_dn(state: &State, valid: &Range<u8>, nrows: usize) -> Option<State>
{
    if state.moves < valid.end && state.pos.0 < nrows - 1 {
        let pos = (state.pos.0 + 1, state.pos.1);
        Some(State { pos, dir: 'v', moves: state.moves + 1 })
    } else {
        None
    }
}

fn move_lt(state: &State, valid: &Range<u8>) -> Option<State>
{
    if state.moves < valid.end && state.pos.1 > 0 {
        let pos = (state.pos.0, state.pos.1 - 1);
        Some(State { pos, dir: '<', moves: state.moves + 1 })
    } else {
        None
    }
}

fn move_rt(state: &State, valid: &Range<u8>, ncols: usize) -> Option<State>
{
    if state.moves < valid.end && state.pos.1 < ncols - 1 {
        let pos = (state.pos.0, state.pos.1 + 1);
        Some(State { pos, dir: '>', moves: state.moves + 1 })
    } else {
        None
    }
}

fn turn_up(state: &State, valid: &Range<u8>) -> Option<State>
{
    if state.moves >= valid.start && state.pos.0 > 0 {
        let pos = (state.pos.0 - 1, state.pos.1);
        Some(State { pos, dir: '^', moves: 1 })      
    } else {
        None
    }
}

fn turn_dn(state: &State, valid: &Range<u8>, nrows: usize) -> Option<State>
{
    if state.moves >= valid.start && state.pos.0 < nrows - 1 {
        let pos = (state.pos.0 + 1, state.pos.1);
        Some(State { pos, dir: 'v', moves: 1 })      
    } else {
        None
    }
}

fn turn_lt(state: &State, valid: &Range<u8>) -> Option<State>
{
    if state.moves >= valid.start && state.pos.1 > 0 {
        let pos = (state.pos.0, state.pos.1 - 1);
        Some(State { pos, dir: '<', moves: 1 })      
    } else {
        None
    }
}

fn turn_rt(state: &State, valid: &Range<u8>, ncols: usize) -> Option<State>
{
    if state.moves >= valid.start && state.pos.1 < ncols - 1 {
        let pos = (state.pos.0, state.pos.1 + 1);
        Some(State { pos, dir: '>', moves: 1 })      
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 694);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 829);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 102);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 94);
    }
}
