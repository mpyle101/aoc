use std::collections::{HashMap, HashSet};
use z3::{ast::Int, Optimize};

type BotMap = HashMap<Bot, HashSet<Bot>>;
type BotSet = HashSet<Bot>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {}  ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {}  ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize
{
    let bots = load(input);

    let mut bots = bots.to_vec();
    bots.sort_by(|a, b| b.r.cmp(&a.r));

    let bot = bots[0];
    bots.iter()
        .filter(|b| bot.md(b) <= bot.r)
        .count()
}

fn part_two(input: &str) -> i64
{
    let bots = load(input);

    let x = Int::new_const("x");
    let y = Int::new_const("y");
    let z = Int::new_const("z");

    let one  = Int::from_i64(1);
    let zero = Int::from_i64(0);

    let mut count = Int::from_i64(0);
    for b in bots {
        let bx = Int::from_i64(b.x);
        let by = Int::from_i64(b.y);
        let bz = Int::from_i64(b.z);
        let br = Int::from_i64(b.r);

        let dx = bx - &x;
        let dx = dx.le(&zero).ite(&dx.unary_minus(), &dx);
        let dy = by - &y;
        let dy = dy.le(&zero).ite(&dy.unary_minus(), &dy);
        let dz = bz - &z;
        let dz = dz.le(&zero).ite(&dz.unary_minus(), &dz);
        let md = &dx + &dy + &dz;
        count += md.le(&br).ite(&one, &zero);
    }

    let optimizer = Optimize::new();
    optimizer.maximize(&count);

    let dx = x.le(&zero).ite(&x.unary_minus(), &x);
    let dy = y.le(&zero).ite(&y.unary_minus(), &y);
    let dz = z.le(&zero).ite(&z.unary_minus(), &z);
    let md = &dx + &dy + &dz;
    optimizer.minimize(&md);

    optimizer.check(&[]);
    let model = optimizer.get_model().unwrap();
    let res = model.eval(&md, true).unwrap();

    res.as_i64().unwrap()
}

#[allow(dead_code)]
fn part_two_bron_kerbosch(input: &str) -> i64
{
    // :sad-panda:
    // Unfortunately, this approach doesn't work for our input set
    // because using manhattan distance to determine that if a group
    // has full pair wise overlap, it has some mutual overlap only
    // works in spaces up to 2D. The overlap.png file shows an example
    // of a data set where all the points touch each other but there
    // is no point in common to all of them. We can see we got unlucky
    // with the input data because this code produces: 47141438 and
    // the correct answer is 47141479. Soooo close.

    let bots = load(input);
    let mut m = BotMap::new();

    // Build up a map of each bot to its neighbors based overlapping
    // space covered by the bots. 
    for (i, a) in bots.iter().enumerate().take(bots.len()-1) {
        for b in bots.iter().skip(i+1) {
            if a.overlaps(b) {
                m.entry(*a).or_default().insert(*b);
                m.entry(*b).or_default().insert(*a);
            }
        }
    }

    // Use Bron-Kerbosch to find the "maximal clique" which gives
    // us the largest set of bots all overlapping each other. That
    // set must contain the point in range of most bots.
    let origin = Bot { x: 0, y: 0, z: 0, r: 0 };
    let mut bk = BronKerbosch::default();
    let clique = bk.execute(&m);

    // Find the point closest to the origin of the bot whose range
    // ends the farthest away. That has to be the shortest distance
    // to the point in range of all the bots in the set. Tellingly,
    // we don't know where the point is, just it's distance.
    clique.iter()
        .map(|bot| bot.md(&origin) - bot.r)
        .max()
        .unwrap()
}

fn load(input: &str) -> Vec<Bot>
{
    input.lines().map(|l| {
        let (s1, s2) = l.split_once(">, r=").unwrap();

        let mut it = s1[5..].split(',');
        let x = read(&mut it);
        let y = read(&mut it);
        let z = read(&mut it);

        let r = s2.parse::<i64>().unwrap();

        Bot { x, y, z, r }
    })
    .collect()
}

fn read<'a>(mut it: impl Iterator<Item=&'a str>) -> i64
{
    it.next().map(|v| v.parse::<i64>().unwrap()).unwrap()
}

struct BronKerbosch {
    clique: Vec<Bot>
}
impl BronKerbosch {
    fn default() -> Self
    {
        BronKerbosch { clique: vec![] }
    }

    fn execute<'a>(&'a mut self, graph: &BotMap) -> &'a Vec<Bot>
    {
        let p = graph.keys().cloned().collect::<BotSet>();
        let x = BotSet::new();
        let mut r = vec![];

        self.bk(graph, &mut r, p, x);

        &self.clique
    }

    fn bk(
        &mut self,
        graph: &BotMap,
        r: &mut Vec<Bot>,
        mut p: BotSet,
        mut x: BotSet)
    {
        if p.is_empty() && x.is_empty() {
            if r.len() > self.clique.len() {
                self.clique = r.clone()
            }
        } else if !p.is_empty() {
            let pivot = p.union(&x)
                .map(|v| graph.get(v).unwrap())
                .max_by_key(|v| v.len())
                .unwrap();

            for v in &p - pivot {
                let neighbors = graph.get(&v).unwrap();

                r.push(v);
                self.bk(graph, r, &p & neighbors, &x & neighbors);
                r.pop();

                p.remove(&v);
                x.insert(v);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Bot {
    r: i64,
    x: i64,
    y: i64,
    z: i64,
}
impl Bot {
    fn md(&self, other: &Self) -> i64
    {
        (self.x.abs_diff(other.x) +
         self.y.abs_diff(other.y) +
         self.z.abs_diff(other.z)) as i64
    }

    fn overlaps(&self, other: &Self) -> bool
    {
        self.md(other) <= self.r + other.r
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 481);
    }

    // #[test]
    // fn input_part_two()
    // {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part_two(input), 47141479);
    // }
}
