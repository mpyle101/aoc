use std::hash::{BuildHasherDefault, Hash};
use indexmap::IndexMap;
use rustc_hash::FxHasher;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

// Lifted from the pathfinding crate and modified to our
// specific needs as we don't need the whole path, just the
// length.
pub fn bfs<N, FN, IN>(
    start: N,
    mut successors: FN,
    goal: N) -> Option<usize>
where
    N: Copy + Eq + Hash,
    FN: FnMut(N) -> IN,
    IN: IntoIterator<Item = N>,
{
    use indexmap::map::Entry::Vacant;

    let mut i = 0;
    let mut parents: FxIndexMap<N, usize> = FxIndexMap::default();
    parents.insert(start, usize::MAX);
    while let Some((node, _)) = parents.get_index(i) {
        for successor in successors(*node) {
            if successor == goal {
                return Some(bfs_length(&parents, i));
            }
            if let Vacant(e) = parents.entry(successor) {
                e.insert(i);
            }
        }
        i += 1;
    }

    None
}

fn bfs_length<N>(parents: &FxIndexMap<N, usize>, start: usize) -> usize
{
    let mut count = 0;
    let mut i = start;

    while let Some((_, value)) = parents.get_index(i) {
        count += 1;
        i = *value;
    }

    count
}