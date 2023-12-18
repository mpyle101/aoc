use std::collections::{HashMap, HashSet};

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

fn part_one(input: &str) -> u32
{
    let (ncols, nrows, items) = load(input);
    energized(ncols, nrows, &items, ('>', 0))
}

fn part_two(input: &str) -> u32
{
    use rayon::prelude::*;

    let (ncols, nrows, items) = load(input);
    let lrow = nrows - 1;
    let lcol = ncols - 1;

    let tp = (0..ncols).into_par_iter()
        .map(|col| energized(ncols, nrows, &items, ('v', col)))
        .max()
        .unwrap();
    let bt = (0..ncols).into_par_iter()
        .map(|col| energized(ncols, nrows, &items, ('^', lrow * ncols + col)))
        .max()
        .unwrap();
    let lt = (0..nrows).into_par_iter()
        .map(|row| energized(ncols, nrows, &items, ('>', row * ncols)))
        .max()
        .unwrap();
    let rt = (0..nrows).into_par_iter()
        .map(|row| energized(ncols, nrows, &items, ('<', row * ncols + lcol)))
        .max()
        .unwrap();
   
    tp.max(bt).max(lt).max(rt)
}

fn load(input: &str) -> (u32, u32, HashMap<u32, char>)
{
    let mut items = HashMap::new();

    let mut ncols = 0;
    let mut nrows = 0;
    input.lines()
        .zip(0..)
        .for_each(|(line, row)| {
            nrows += 1;
            ncols = line.len() as u32;
            line.chars()
                .zip(0..)
                .for_each(|(c, col)| {
                    let pos = row * ncols + col;
                    if c != '.' {
                        items.insert(pos, c);
                    }
                })
        });

    (ncols, nrows, items)
}

fn energized(
    ncols: u32,
    nrows: u32,
    items: &HashMap<u32, char>,
    start: (char, u32)
) -> u32
{
    use std::collections::VecDeque;

    let mut seen  = HashSet::from([start]);
    let mut tiles = HashSet::from([start.1]);
    let mut beams = VecDeque::from([start]);

    while let Some(state) = beams.pop_front() {
        tiles.insert(state.1);
        calc_next(state, ncols, nrows, start, items).iter()
            .for_each(|st| if *st != start && !seen.contains(st) {
                seen.insert(*st);
                beams.push_back(*st);
            })
    }

    tiles.len() as u32
}


fn calc_next(
    (dir, pos): (char, u32),
    ncols: u32,
    nrows: u32,
    start: (char, u32),
    items: &HashMap<u32, char>,
) -> [(char, u32);2]
{
    let row  = pos / ncols;
    let col  = pos % ncols;
    let lrow = nrows - 1;
    let lcol = ncols - 1;

    let mut states = [start;2];

    if let Some(c) = items.get(&pos) {
        if *c == '|' && (dir == '>' || dir == '<') {
            if row > 0 { states[0] = ('^', pos - ncols); }
            if row < lrow { states[1] = ('v', pos + ncols); }
        } else if *c == '-' && (dir == 'v' || dir == '^') {
            if col > 0 { states[0] = ('<', pos - 1); }
            if col < lcol { states[1] = ('>', pos + 1); }
        } else {
            states[0] = match (dir, c) {
                ('>', '-') if col < lcol => ('>', pos + 1),
                ('>', '/') if row > 0 => ('^', pos - ncols),
                ('<', '-') if col > 0 => ('<', pos - 1),
                ('<', '/') if row < lrow => ('v', pos + ncols),
                ('^', '|') if row > 0 => ('^', pos - ncols),
                ('^', '/') if col < lcol => ('>', pos + 1),
                ('v', '|') if row < lrow => ('v', pos + ncols),
                ('v', '/') if col > 0 => ('<', pos - 1),
                ('>', '\\') if row < lrow => ('v', pos + ncols),
                ('<', '\\') if row > 0 => ('^', pos - ncols),
                ('^', '\\') if col > 0 => ('<', pos - 1),
                ('v', '\\') if col < lcol => ('>', pos + 1),
                _ => start
            }      
        }
    } else {
        states[0] = match dir {
            '<' if col > 0 => ('<', pos - 1),
            '^' if row > 0 => ('^', pos - ncols),
            '>' if col < lcol => ('>', pos + 1),
            'v' if row < lrow => ('v', pos + ncols),
            _ => start
        };
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
