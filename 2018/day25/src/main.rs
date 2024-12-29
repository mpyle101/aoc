use petgraph::prelude::UnGraph;

type Point = [i8;4];

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {}  ({:?})", part_one(input), t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use petgraph::algo::connected_components;

    let pts = load(input);
    let mut graph = UnGraph::<(), ()>::default();
    let nodes = pts.iter()
        .map(|_| graph.add_node(()))
        .collect::<Vec<_>>();

    for i in 0..pts.len()-1 {
        for j in i+1..pts.len() {
            if md(&pts[i], &pts[j]) < 4 {
                graph.add_edge(nodes[i], nodes[j], ());
            }
        }
    }

    connected_components(&graph)
}

fn load(input: &str) -> Vec<Point>
{
    input.lines()
        .map(|l| {
            let mut pt = [0;4];
            l.split(',')
                .enumerate()
                .flat_map(|(i, s)| s.parse::<i8>().map(|n| (i, n)))
                .for_each(|(i, n)| pt[i] = n);
            pt
        })
        .collect()
}

fn md(a: &Point, b: &Point) -> i8
{
    a.iter().zip(b.iter()).map(|(v1, v2)| (v1 - v2).abs()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 377);
    }
}
