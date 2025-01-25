// For part 2, the position and velocity variables are independent so you
// can find where each set of x's, y's and z's repeat on their own. Also,
// since you get to a value via stepping, you know the first repeat has to
// be when the velocities are all zero.
// Finding when each set of velocity values hit zero together is waaaay faster
// then trying to find when all the velocities are zero across the board. Once
// you find each cycle point, you then find the least common multiple which will
// tell you the iteration when all of them have cycled to zero together.

use itertools::Itertools;
use num::integer::Integer;
use regex::Regex;
use std::hash::Hash;
use std::ops::{AddAssign, Neg};

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

fn part_one(input: &str) -> i32
{
    let mut moons = load(input);
    (0..1000)
        .for_each(|_| {
            apply_gravity(&mut moons);
            apply_velocity(&mut moons);
        });

    moons.iter()
        .map(|m| m.energy())
        .sum()
}

fn part_two(input: &str) -> u64
{
    let mut moons = load(input);
    let vx = find_cycle(&mut moons.clone(), 0);
    let vy = find_cycle(&mut moons.clone(), 1);
    let vz = find_cycle(&mut moons, 2);

    vx.lcm(&vy).lcm(&vz)
}

fn load(moons: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    moons
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| Moon::new( 
            c.get(1).unwrap().as_str().parse::<i32>().unwrap(), 
            c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            c.get(3).unwrap().as_str().parse::<i32>().unwrap()
        ))
        .collect::<Vec<Moon>>()
}

fn find_cycle(moons: &mut [Moon], index: usize) -> u64
{
    let mut iterations = 1u64;

    apply_gravity(moons);
    apply_velocity(moons);
    while !(0..4).all(|i| moons[i].vel.get(index) == 0) {
        apply_gravity(moons);
        apply_velocity(moons);
        iterations += 1;
    }

    iterations * 2
}

fn apply_gravity(moons: &mut [Moon])
{
    (0..moons.len()).combinations(2)
        .for_each(|v| {
            let delta = velocity_delta(&moons[v[0]].pos, &moons[v[1]].pos);
            // Only one mutable reference allowed at a time
            let moon: &mut Moon = &mut moons[v[0]];
            moon.vel += delta.clone();

            let moon: &mut Moon = &mut moons[v[1]];
            moon.vel += -delta;
        })
}

fn apply_velocity(moons: &mut [Moon])
{
    moons.iter_mut().for_each(|m| m.update());
}

fn velocity_delta(a: &Triplet, b: &Triplet) -> Triplet
{
    Triplet {
        x: b.x.cmp(&a.x) as i32,
        y: b.y.cmp(&a.y) as i32,
        z: b.z.cmp(&a.z) as i32,
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Triplet {
    x: i32,
    y: i32,
    z: i32,
}
impl Triplet {
    fn get(&self, index: usize) -> i32
    {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!()
        }
    }
}
impl AddAssign for Triplet {
    fn add_assign(&mut self, other: Self)
    {
            *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
impl Neg for Triplet {
    type Output = Self;

    fn neg(self) -> Self::Output
    {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Moon {
    pos: Triplet,
    vel: Triplet,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self
    {
        Moon {
            pos: Triplet { x, y, z },
            vel: Triplet { x: 0, y: 0, z: 0 }
        }
    }

    fn update(&mut self)
    {
        self.pos += self.vel.clone()
    }

    fn kinetic(&self) -> i32
    {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn potential(&self) -> i32
    {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn energy(&self) -> i32
    {
        self.potential() * self.kinetic()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 8538);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 506_359_021_038_056);
    }

}
