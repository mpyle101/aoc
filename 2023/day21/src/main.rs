#![allow(dead_code)]
mod farm;

use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use farm::Garden;

type Tiles = HashSet<i32>;

pub type Farm = HashMap<(i32, i32), RefCell<farm::Garden>>;

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
    let mut garden = Garden::from(input);
    garden.init();
    march(64, &mut garden);

    garden.steps()
}

fn part_two(input: &str) -> usize
{
    let garden = Garden::from(input);
    teleport(26_501_365, &garden)
}

fn teleport(steps: usize, garden: &Garden) -> usize
{
    let mut corners = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut fill = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let mut g = garden.clone();
    g.init();

    let mut farm = Farm::from([((0, 0), RefCell::new(g))]);
    for i in 1..=391 {
        stride(&mut farm, garden);
        if (66..=260).contains(&i) {
            corners[0].push(farm.get(&(-1,  0)).unwrap().borrow().steps());
            corners[1].push(farm.get(&( 0,  1)).unwrap().borrow().steps());
            corners[2].push(farm.get(&( 1,  0)).unwrap().borrow().steps());
            corners[3].push(farm.get(&( 0, -1)).unwrap().borrow().steps());
        }
        if (132..=391).contains(&i) {
            fill[0].push(farm.get(&(-1, -1)).unwrap().borrow().steps());
            fill[1].push(farm.get(&(-1,  1)).unwrap().borrow().steps());
            fill[2].push(farm.get(&( 1, -1)).unwrap().borrow().steps());
            fill[3].push(farm.get(&( 1,  1)).unwrap().borrow().steps());
        }
    }

    let m  = (steps - 260) % 131;
    let n1 = (steps - 260) / 131;
    let n2 = (steps - 66) % 131;

    let mut count = if m.is_multiple_of(2) {
        (n1 + 2).pow(2) * 7265 + (n1 + 1).pow(2) * 7325
    } else {
        (n1 + 2).pow(2) * 7325 + (n1 + 1).pow(2) * 7265
    };
    count += corners.iter().map(|v| v[n2]).sum::<usize>();
    if m > 2 {
        let f1 = m - 3;
        let f2 = f1 + 131;
        count += fill.iter().map(|v| v[f1] * (n1 + 2)).sum::<usize>();
        count += fill.iter().map(|v| v[f2] * (n1 + 1)).sum::<usize>();
    }

    count
}

fn march(steps: i32, garden: &mut Garden)
{
    let ncols = garden.ncols();
    let nrows = garden.len() / ncols;

    (0..steps).for_each(|_| {
        for pos in 0..garden.len() {
            if garden.is_step(pos) {
                garden.clear(pos);

                let row = pos / ncols;
                let col = pos % ncols;
                if row > 0 { garden.mark(pos - ncols) }
                if col > 0 { garden.mark(pos - 1) }
                if row < nrows - 1 { garden.mark(pos + ncols) }
                if col < ncols - 1 { garden.mark(pos + 1) }
            }
        }
        garden.update();
    });
}

fn sprint(steps: i32, garden: &Garden) -> usize
{
    let mut g = garden.clone();
    g.init();

    let mut farm = Farm::from([((0, 0), RefCell::new(g))]);
    (0..steps).for_each(|_| stride(&mut farm, garden));
    farm.values().map(|g| g.borrow().steps()).sum()
}

fn stride(farm: &mut Farm, rocks: &Garden)
{
    let len   = rocks.len();
    let ncols = rocks.ncols();
    let nrows = rocks.len() / ncols;

    let mut acres = Farm::new();
    for ((r, c), rc) in farm.iter() {
        let mut garden = rc.borrow_mut();
        for pos in 0..len {
            if garden.is_step(pos) { 
                garden.clear(pos);

                let row = pos / ncols;
                let col = pos % ncols;

                // Step up
                if row == 0 {
                    let k = (r - 1, *c);
                    let p = (nrows - 1) * ncols + col;
                    if let Some(g) = farm.get(&k) {
                        g.borrow_mut().mark(p);
                    } else {
                        acres.entry(k)
                            .or_insert(RefCell::new(rocks.clone()))
                            .borrow_mut().step(p);
                    }
                } else {
                    garden.mark(pos - ncols);
                }

                // Step left
                if col == 0 { 
                    let k = (*r, c - 1);
                    let p = row * ncols + ncols - 1;
                    if let Some(g) = farm.get(&k) {
                        g.borrow_mut().mark(p);
                    } else {
                        acres.entry(k)
                            .or_insert(RefCell::new(rocks.clone()))
                            .borrow_mut().step(p);
                    }
                } else {
                    garden.mark(pos - 1);
                }

                // Step down
                if row == nrows - 1 {
                    let k = (r + 1, *c);
                    let p = col;
                    if let Some(g) = farm.get(&k) {
                        g.borrow_mut().mark(p);
                    } else {
                        acres.entry(k)
                            .or_insert(RefCell::new(rocks.clone()))
                            .borrow_mut().step(p)
                    }
                } else {
                    garden.mark(pos + ncols)
                }

                // Step right
                if col == ncols - 1 {
                    let k = (*r, c + 1);
                    let p = row * ncols;
                    if let Some(g) = farm.get(&k) {
                        g.borrow_mut().mark(p);
                    } else {
                        acres.entry(k)
                            .or_insert(RefCell::new(rocks.clone()))
                            .borrow_mut().step(p);
                    }
                } else {
                    garden.mark(pos + 1)
                }
            }
        }
    }
    farm.values().for_each(|g| g.borrow_mut().update());
    farm.extend(acres);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3585);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 597102953699891);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        let mut garden = Garden::from(input);
        garden.init();
        march(6, &mut garden);

        assert_eq!(garden.steps(), 16);
    }

    #[test]
    fn example_part_two_10()
    {
        let input = include_str!("../example.txt");
        let garden = Garden::from(input);

        assert_eq!(sprint(10, &garden), 50);
    }

    #[test]
    fn example_part_two_50()
    {
        let input = include_str!("../example.txt");
        let garden = Garden::from(input);

        assert_eq!(sprint(50, &garden), 1594);
    }

    // #[test]
    // fn example_part_two_100()
    // {
    //     let input = include_str!("../example.txt");
    //     let garden = Garden::from(input);

    //     assert_eq!(sprint(100, &garden), 6536);
    // }
}
