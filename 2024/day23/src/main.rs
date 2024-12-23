use std::collections::{HashMap, HashSet};

type Network<'a> = HashMap<&'a str, HashSet<&'a str>>;
type NodeSet<'a> = HashSet<&'a str>;

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
    use std::collections::HashSet;

    let network = load(input);
    let mut groups = HashSet::new();
    for key in network.keys().filter(|k| k.starts_with('t')) {
        let v1 = network.get(key).unwrap();
        for k1 in v1 {
            let v2 = network.get(k1).unwrap();
            v2.iter()
                .filter(|k2| v1.contains(*k2))
                .for_each(|k2| {
                    let mut v = vec![key, *k1, *k2];
                    v.sort_unstable();
                    groups.insert(v);
                })
        }
    }

    groups.len()
}

fn part_two(input: &str) -> String
{
    let network = load(input);
    let mut bk = BronKerbosch::default();
    let mut clique = bk.execute(&network).clone();
    clique.sort_unstable();

    clique.join(",")
}

fn load(input: &str) -> Network
{
    input.lines()
        .fold(Network::new(), |mut m, line| {
            let (s1, s2) = line.split_once('-').unwrap();
            m.entry(s1).or_default().insert(s2);
            m.entry(s2).or_default().insert(s1);
            m
        })
}

struct BronKerbosch<'a> {
    clique: Vec<&'a str>
}
impl<'a> BronKerbosch<'a> {
    fn default() -> Self
    {
        BronKerbosch { clique: vec![] }
    }

    fn execute(&'a mut self, graph: &'a Network) -> &'a Vec<&'a str>
    {
        let p = graph.keys().cloned().collect::<HashSet<_>>();
        let x = HashSet::new();
        let mut r = vec![];

        self.bk(graph, &mut r, p, x);

        &self.clique
    }

    fn bk(
        &mut self,
        graph: &'a Network,
        r: &mut Vec<&'a str>,
        mut p: NodeSet<'a>,
        mut x: NodeSet<'a>)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1046);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz");
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 7);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), "co,de,ka,ta");
    }
}
