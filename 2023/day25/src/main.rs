use std::collections::HashMap;
use petgraph::prelude::{NodeIndex, UnGraph};

type Graph<'a> = UnGraph::<&'a str, usize>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    let (mut graph, nodes, edges) = load(input);

    let mut g = karger(&graph);
    while g.edge_count() != 3 {
        g = karger(&graph)
    }

    let cut: Vec<_> = g.edge_weights()
        .map(|w| edges[*w])
        .flat_map(|(a, b)| {
            let n1 = nodes.get(a).unwrap();
            let n2 = nodes.get(b).unwrap();
            graph.find_edge(*n1, *n2)
        })
        .collect();
    cut.iter().for_each(|e| { graph.remove_edge(*e); });

    connected_components(&graph)
        .iter()
        .product()
}

fn load(input: &str) -> (Graph, HashMap<&str, NodeIndex>, Vec<(&str, &str)>)
{
    let mut nodes = HashMap::new();
    let mut edges = vec![];
    let mut graph = Graph::new_undirected();

    let mut i = -1;
    input.lines()
        .for_each(|line| {
            let (s1, s2) = line.split_once(": ").unwrap();
            let ix = *nodes.entry(s1).or_insert_with(|| graph.add_node(s1));
            let edges: Vec<_> = s2.split(' ')
                .inspect(|n| edges.push((s1, *n)))
                .map(|n| {i += 1; (n, i as usize)})
                .map(|(n, w)| (ix, *nodes.entry(n).or_insert_with(|| graph.add_node(n)), w))
                .collect();

            graph.extend_with_edges(edges);
        });

    (graph, nodes, edges)
}

fn karger<'a>(graph: &'a Graph) -> Graph<'a>
{
    use petgraph::visit::EdgeRef;
    use rand::Rng;

    let mut g = graph.clone();
    while g.node_count() > 2 {
        let n = rand::thread_rng().gen_range(0..g.edge_count());
        let edge = g.edge_references().nth(n).unwrap();
        let (source, target) = (edge.source(), edge.target());

        let v = g.edges(target)
            .filter(|e| *e != edge)
            .map(|e| (e.target(), *e.weight()))
            .collect::<Vec<_>>();
        v.iter()
            .filter(|(t, _)| *t != source)
            .for_each(|(t, w)| { g.add_edge(source, *t, *w); });
        g.remove_node(target);
    }

    g
}

fn connected_components(g: &Graph) -> Vec<usize>
{
    use petgraph::unionfind::UnionFind;
    use petgraph::visit::{EdgeRef, NodeIndexable};

    let mut vertex_sets = UnionFind::new(g.node_bound());
    for edge in g.edge_references() {
        let (a, b) = (edge.source(), edge.target());

        // union the two vertices of the edge
        vertex_sets.union(g.to_index(a), g.to_index(b));
    }
    let mut labels = vertex_sets.into_labeling();
    labels.sort_unstable();

    let mut c = 1;
    let mut n = labels[0];
    let mut counts = vec![];
    labels.iter().skip(1)
        .for_each(|i| {
            if n == *i {
                c += 1;
            } else {
                counts.push(c);
                c = 1;
                n = *i;
            }
        });
    counts.push(c);

    counts
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 613870);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 54);
    }
}
