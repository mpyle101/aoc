use std::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::FxHasher;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

// Lifted from the pathfinding crate and modified to our
// specific needs as we don't need the whole path, just the
// length.
pub fn bfs<FN, IN>(
    start: u32,
    mut successors: FN,
    goal: u32) -> Option<usize>
where
    FN: FnMut(u32) -> IN,
    IN: IntoIterator<Item = u32>,
{
    use indexmap::map::Entry::Vacant;

    let mut i = 0;
    let mut parents: FxIndexMap<u32, usize> = FxIndexMap::default();
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

fn bfs_length(parents: &FxIndexMap<u32, usize>, start: usize) -> usize
{
    let mut count = 0;
    let mut i = start;

    while let Some((_, value)) = parents.get_index(i) {
        count += 1;
        i = *value;
    }

    count
}