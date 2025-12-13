use bitmatrix::BitMatrix;

#[derive(Clone, Copy, Debug)]
struct Edges {
    lt: u16,
    rt: u16,
    top: u16,
    bot: u16,
}

struct TileData {
    id: u32,
    mat: BitMatrix,
    edges: Edges,
}

#[derive(Clone, Debug)]
struct Tile {
    idx: usize,
    pos: usize,
    edges: Edges,
}

type Tiles = Vec<[TileData; 8]>;
type Layout = Vec<Tile>;


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

fn part_one(input: &str) -> u64
{
    let tiles = load(input);
    let size = (tiles.len() as f32).sqrt() as usize;
    let layout = layout_tiles(0, size, vec![], &tiles).unwrap();

    // Iterate over the corners: tl, tr, bl, br
    [0, size - 1, size * (size - 1), size * size - 1]
        .iter()
        .map(|i| {
            let Tile { pos, idx, .. } = layout[*i];
            tiles[pos][idx].id as u64
        })
        .product()
}

fn part_two(input: &str) -> usize
{
    let tiles = load(input);
    let size = (tiles.len() as f32).sqrt() as usize;
    let layout = layout_tiles(0, size, vec![], &tiles).unwrap();

    // Can be flipped or rotated with respect to sea monsters.
    let image = build_image(&layout, size, &tiles);
    let images = [
        image.clone(),
        image.rotated_cw(1),
        image.rotated_cw(2),
        image.rotated_cw(3),
        image.flipped_vert(),
        image.flipped_vert().rotated_cw(1),
        image.flipped_vert().rotated_cw(2),
        image.flipped_vert().rotated_cw(3),
    ];

    for img in images {
        let n = sea_monsters(&img);
        if n > 0 { 
            let rough: usize = img.iter()
                .map(|row| row.filter(|c| *c).count())
                .sum();
            return rough - (n * SEA_MONSTER.len())
         }
    }

    0
}

fn load(input: &str) -> Tiles
{
    let scans = input.split("\n\n")
        .map(|s| {
            let (s1, s2) = s.split_once('\n').unwrap();
            let id = s1[5..9].parse::<u32>().unwrap();
            let data = BitMatrix::from_rows(s2.lines().map(|l| l.bytes()), |c| *c == b'#');

            (id, data)
        })
        .collect::<Vec<_>>();

    scans.iter()
        .map(|(id, tile)| {
            [
                make_tile(*id, tile.clone()),
                make_tile(*id, tile.rotated_cw(1)),
                make_tile(*id, tile.rotated_cw(2)),
                make_tile(*id, tile.rotated_cw(3)),
                make_tile(*id, tile.flipped_vert()),
                make_tile(*id, tile.flipped_vert().rotated_cw(1)),
                make_tile(*id, tile.flipped_vert().rotated_cw(2)),
                make_tile(*id, tile.flipped_vert().rotated_cw(3)),
            ]
        })
        .collect()
}

fn make_tile(id: u32, m: BitMatrix) -> TileData
{
    let sl = m.slice(0..m.rows, 0..1).unwrap();
    let lt = make_edge(&sl);
    let sl = m.slice(0..m.rows, m.cols-1..m.cols).unwrap();
    let rt = make_edge(&sl);
    let sl = m.slice(0..1, 0..m.cols).unwrap();
    let top = make_edge(&sl);
    let sl = m.slice(m.rows-1..m.rows, 0..m.cols).unwrap();
    let bot = make_edge(&sl);

    // We don't need the tile content for part one and don't need
    // the edges for part two.
    let mat = m.slice(1..m.rows-1, 1..m.cols-1).unwrap();

    TileData { id, mat, edges: Edges { lt, rt, top, bot } }
}

fn layout_tiles(
    pos: usize,
    size: usize,
    layout: Layout,
    tiles: &Tiles
) -> Option<Layout>
{
    if pos == size * size {
        Some(layout)
    } else {
        for (i, tdata) in tiles.iter().enumerate() {
            if !layout.iter().any(|t| t.pos == i) {
                for (idx, data) in tdata.iter().enumerate() {
                    if place_tile(pos, size, data, &layout) {
                        let mut img = layout.clone();
                        img.push(Tile { pos: i, idx, edges: data.edges });

                        if let Some(img) = layout_tiles(pos + 1, size, img, tiles) {
                            return Some(img)
                        }
                    }
                }
            }
        }

        None
    }
}

fn place_tile(
    pos: usize,
    size: usize,
    tile: &TileData,
    layout: &Layout,
) -> bool
{
    if pos >= size {
        let top = &layout[pos - size];
        if top.edges.bot != tile.edges.top {
            return false
        }
    }

    if !pos.is_multiple_of(size) {
        let lt = &layout[pos - 1];
        if lt.edges.rt != tile.edges.lt {
            return false
        }
    }

    true
}

fn make_edge(m: &BitMatrix) -> u16
{
    m.iter()
        .flatten()
        .enumerate()
        .filter(|(_, c)| *c)
        .fold(0, |n, (i, _)| n | 1 << i)
}

fn build_image(layout: &Layout, size: usize, tiles: &Tiles) -> BitMatrix
{
    // Ugly but we need to know how big the tiles are now.
    let rows = tiles[0][0].mat.rows;
    let dims = rows * size;

    BitMatrix::from_fn(dims, dims, |r, c| {
        let i  = r / rows;  // row of tile in layout
        let ii = r % rows;  // row in tile
        let j  = c / rows;  // col of tile in layout
        let jj = c % rows;  // col in tile

        let tile = &layout[i * size + j];
        let data = &tiles[tile.pos][tile.idx];
        data.mat.get(ii, jj)
    })
}

// 3x20
// ..................#.
// #....##....##....###
// .#..#..#..#..#..#...
const SEA_MONSTER: [(usize, usize);15] = [
    (0, 18),
    (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
    (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
];

fn sea_monsters(m: &BitMatrix) -> usize
{
    (0..m.rows - 2)
        .flat_map(|r| (0..m.cols - 19).map(move |c| (r, c)))
        .filter(|(r, c)| {
            SEA_MONSTER.iter().all(|(dr, dc)| m.get(r + dr, c + dc))
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 18482479935793);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2118);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 20899048083289);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 273);
    }
}