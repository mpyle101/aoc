use std::collections::HashSet;
use lazy_static::lazy_static;

type Shaft = HashSet<(i32, i32)>;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let calories = part_one(input);
    println!("Part 1: {} ({:?})", calories, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    let v = Rock::rocks();
    let mut rocks = v.iter().cycle();
    let mut wind = input.chars().cycle();
    
    let mut count = 0;
    let mut height = 0;
    let mut shaft = Shaft::new();

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

    height
}

fn fall(rock: &Rock, shaft: &Shaft) -> Option<Rock> {
    let r = rock.move_y(-1);
    if r.y < 0 || r.iter().any(|p| shaft.contains(&p)) {
        None
    } else {
        Some(r)
    }
}

fn shift(rock: &Rock, dir: char, shaft: &Shaft) -> Option<Rock> {
    let r = match dir {
        '>' => rock.move_x(1),
        '<' => rock.move_x(-1),
        _ => unreachable!()
    };

    if r.x < 0 || r.x + r.wd > 7 || r.iter().any(|p| shaft.contains(&p)) { 
        None
    } else {
        Some(r)
    }
}

#[allow(dead_code)]
fn print_shaft(ht: i32, shaft: &Shaft) {
    for y in 0..ht {
        print!("|");
        for x in 0..7 {
            if shaft.contains(&(x, ht - y - 1)) { print!("#") } else { print!(".") };
        }
        println!("|");
    }
    println!("+-------+")
}

lazy_static! {
    static ref SHAPES: [Vec<(i32, i32)>;5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
}

#[derive(Clone, Debug)]
struct Rock {
    x: i32,
    y: i32,
    ht: i32,
    wd: i32,
    ix: usize,
}

impl Rock {
    fn new(ht: i32, wd: i32, ix: usize) -> Rock {
        Rock { ht, wd, ix, x: 2, y: 0 }
    }

    fn move_y(&self, y: i32) -> Rock {
        let mut r = self.clone();
        r.y += y;
        r
    }

    fn move_x(&self, x: i32) -> Rock {
        let mut r = self.clone();
        r.x += x;
        r
    }

    fn iter(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        SHAPES[self.ix].iter()
            .map(|p| (p.0 + self.x, p.1 + self.y))
    }

    fn add(&self, shaft: &mut Shaft, height: i32) -> i32 {
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

        let calories = part_one(input);
        assert_eq!(calories, 3209);
    }

    #[test]
    fn examples() {
        let input = include_str!("../example.txt");

        let calories = part_one(input);
        assert_eq!(calories, 3068);
    }
}
