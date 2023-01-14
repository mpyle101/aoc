use bitvec::prelude::*;

fn main()
{
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t = Instant::now();
    println!("Part 1: {}  ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {}  ({:?})", part_two(input), t.elapsed());
}

#[allow(dead_code)]
fn part_one(input: &str) -> i32
{
    load(input).play()
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32
{
    let initial = load(input);
    let elves = initial.elves;

    let mut elven_ap = 4;
    let mut game  = initial.with(elven_ap);
    let mut score = game.play();

    while game.elves < elves {
        elven_ap += 1;
        game  = initial.with(elven_ap);
        score = game.play();
    }
    
    score
}

fn load(input: &str) -> Game
{
    use Unit::*;

    let mut tiles = bitvec![0;input.len()];
    let mut units = Units::new();
    let mut elves = 0u8;

    let mut ix = 0;
    let mut rows = 0;
    for line in input.lines() {
        rows += 1;
        for c in line.chars() {
            match c {
                '.' => tiles.set(ix as usize, true),
                'G' => units.push(Goblin { ix, ap: 3, hp: 200 }),
                'E' => { units.push(Elf { ix, ap: 3, hp: 200 }); elves += 1; },
                 _  => {},
            };
            ix += 1;
        }
    }

    let board = Board { tiles, cols: input.len() / rows };
    Game::new(board, units, elves)
}

#[derive(Clone)]
struct Board {
    cols: usize,
    tiles: BitVec,
}
impl Board {
    fn set_tile(&mut self, ix: u16, open: bool)
    {
        self.tiles.set(ix as usize, open)
    }

    fn open(&self, ix: u16) -> bool
    {
        self.tiles[ix as usize]
    }

    fn neighbors(&self, ix: u16, tiles: &mut [(bool, u16);4])
    {
        // The arena is bounded by walls so we don't have to worry
        // about overflow/underflow when calculating neighboring
        // positions.
        let cols = self.cols as u16;

        // above, left, right, below
        tiles[0] = (self.open(ix - cols), ix - cols);
        tiles[1] = (self.open(ix - 1), ix - 1);
        tiles[2] = (self.open(ix + 1), ix + 1);
        tiles[3] = (self.open(ix + cols), ix + cols);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Unit {
    Elf { hp: u8, ix: u16, ap: u16 },
    Goblin { hp: u8, ix: u16, ap: u16 },
}
impl Unit {
    fn tile(&self) -> u16
    {
        match self {
            Unit::Elf { ix, .. } => *ix,
            Unit::Goblin { ix, .. } => *ix,
        }
    }

    fn attack_power(&self) -> u16
    {
        match self {
            Unit::Elf { ap, .. } => *ap,
            Unit::Goblin { ap, .. } => *ap
        }
    }

    fn hit_points(&self) -> u8
    {
        match self {
            Unit::Elf { hp, .. } => *hp,
            Unit::Goblin { hp, .. } => *hp
        }
    }

    fn kill(&mut self)
    {
        match self {
            Unit::Elf { hp, .. } => *hp = 0,
            Unit::Goblin { hp, .. } => *hp = 0,
        }
    }

    fn damage(&mut self, hps: u16)
    {
        match self {
            Unit::Elf { hp, .. } => *hp -= hps as u8,
            Unit::Goblin { hp, .. } => *hp -= hps as u8
        }
    }

    fn set_tile(&mut self, tile: u16)
    {
        match self {
            Unit::Elf { ix, .. } => *ix = tile,
            Unit::Goblin { ix, .. } => *ix = tile,
        }
    }
}
type Units = Vec<Unit>;

#[derive(Clone)]
struct Game {
    board: Board,
    units: Units,
    elves: u8,
    goblins: u8,
}
impl Game {
    fn new(board: Board, units: Units, elves: u8) -> Game
    {
        let goblins = units.len() as u8 - elves;
        Game { board, units, elves, goblins }
    }

    fn with(&self, elven_ap: u16) -> Game
    {
        use Unit::*;

        let mut game = self.clone();
        game.units.iter_mut()
            .for_each(|unit| if let Elf { ap, .. } = unit { *ap = elven_ap });
        game
    }

    fn play(&mut self) -> i32
    {
        let mut round = 0;
        loop {
            round += do_round(self) as i32;
            if self.elves == 0 || self.goblins == 0 {
                break;
            }
        }

        let hp = self.units.iter()
            .map(|unit| unit.hit_points() as i32)
            .sum::<i32>();

        hp * round
    }

    #[allow(dead_code)]
    fn print(&self, round: i32) {
        use Unit::*;

        let rows = (self.board.tiles.len() / self.board.cols) as u16;
        let cols = self.board.cols as u16;

        println!("\nAfter round: {round}");
        for row in 0..rows {
            for col in 0..cols {
                let ix = (row * cols) + col;
                if let Some(i) = self.units.iter().position(|u| u.tile() == ix) {
                    match self.units[i] {
                        Elf { .. } => print!("E"),
                        Goblin { .. } => print!("G"),
                    }
                } else if self.board.open(ix) {
                    print!(".")
                } else {
                    print!("#")
                }
            }
            println!()
        }
        println!("{:?}", self.units)
    }
}

fn do_round(game: &mut Game) -> bool
{
    use Unit::*;

    let mut elves   = vec![];
    let mut goblins = vec![];

    game.units.iter()
        .enumerate()
        .for_each(|(i, unit)| match unit {
            Elf { .. } => elves.push(i),
            Goblin { .. } => goblins.push(i),
        });

    let board = &mut game.board;
    let mut full_round = true;
    let mut units = game.units.iter_mut().collect::<Vec<_>>();
    for i in 0..units.len() {
        let unit = *units[i];
        if unit.hit_points() > 0 {
            let enemies = if let Elf { .. } = unit {
                &mut goblins
            } else {
                &mut elves
            };
            if enemies.is_empty() { full_round = false; }

            if !do_attack(board, i, &mut units, enemies) {
                do_move(board, i, &mut units, enemies);
                do_attack(board, i, &mut units, enemies);
            }
        }
    }

    let mut live_units = Vec::with_capacity(units.len());
    for unit in units {
        if unit.hit_points() > 0 {
            live_units.push(*unit);
        } else {
            match unit {
                Elf { .. }    => game.elves -= 1,
                Goblin { .. } => game.goblins -= 1,
            }
        }
    }
    live_units.sort_by_key(|a| a.tile());
    game.units = live_units;

    full_round
}

fn do_attack(
    board: &mut Board,
    unit: usize,
    units: &mut [&mut Unit],
    enemies: &mut Vec<usize>) -> bool
{
    let enemy = enemy_for(board, unit, units, enemies);
    if let Some(i) = enemy {
        let ap  = units[unit].attack_power();
        let foe = enemies[i];
        if (units[foe].hit_points() as u16) <= ap { 
            enemies.remove(i);
            units[foe].kill();
            board.set_tile(units[foe].tile(), true);
        } else {
            units[foe].damage(ap);
        }
    }

    enemy.is_some()
}

fn enemy_for(
    board: &mut Board,
    unit: usize,
    units: &[&mut Unit],
    enemies: &[usize]) -> Option<usize>
{
    let mut tiles = [(false, u16::MAX);4];
    board.neighbors(units[unit].tile(), &mut tiles);
    let in_range = [
        enemy_at(units, tiles[0], enemies),
        enemy_at(units, tiles[1], enemies),
        enemy_at(units, tiles[2], enemies),
        enemy_at(units, tiles[3], enemies),
    ];
 
    // The indexes are in read order so we need to find
    // a valid unit (Some) with the lowest hp.
    let mut hp = u8::MAX;
    let mut ix = usize::MAX;
    (0..4).for_each(|n| {
        if let Some(i) = in_range[n] {
            let enemy_hp = units[enemies[i]].hit_points();
            if enemy_hp < hp {
                ix = i;
                hp = enemy_hp;
            }
        }
    });

    (ix != usize::MAX).then_some(ix)
}

fn enemy_at(units: &[&mut Unit], (open, ix): (bool, u16), enemies: &[usize]) -> Option<usize>
{
    (!open).then_some(enemies.iter().position(|i| units[*i].tile() == ix)).flatten()
}

fn do_move(
    board: &mut Board,
    unit: usize,
    units: &mut [&mut Unit],
    enemies: &[usize])
{
    let ix = units[unit].tile();
    let cols = board.cols;

    let mut tiles = [(false, u16::MAX);4];
    let mut adjacent = Vec::with_capacity(enemies.len() * 4);
    enemies.iter()
        .map(|i| units[*i].tile())
        .for_each(|tile| {
            board.neighbors(tile, &mut tiles);
            tiles.iter()
                .filter(|(open, _)| *open)
                .for_each(|(_, t)| adjacent.push((manhattan(ix, *t, cols), *t)))
        });

    if !adjacent.is_empty() {
        // Sort by manhattan distance as an approximation of the closest enemies.
        adjacent.sort_unstable();
        if let Some(i) = step_for(board, ix, &adjacent) {
            board.set_tile(ix, true);
            board.set_tile(i, false);
            units[unit].set_tile(i);
        }
    }
}

fn manhattan(a: u16, b: u16, cols: usize) -> u16
{
    // abs(row a - row b) + abs(col a - col b)
    let c = cols as u16;
    (a / c).abs_diff(b / c) + (a % c).abs_diff(b % c)
}

fn step_for(
    board: &Board,
    unit: u16,
    targets: &[(u16, u16)]) -> Option<u16>
{
    use pathfinding::prelude::bfs;

    // Store the length of the path, the enemy index and first step.
    let mut paths = vec![];
    let mut shortest = usize::MAX;
    for (md, goal) in targets {
        if (*md as usize) < shortest {
            let result = bfs(&unit, |i| open_tiles(board, *i), |p| *p == *goal);
            if let Some(path) = result {
                if path.len() <= shortest {
                    shortest = path.len();
                    paths.push((shortest, *goal, path[1]));
                }
            }
        }
    }
    paths.sort_unstable();

    if !paths.is_empty() { Some(paths[0].2) } else { None }
}

fn open_tiles(board: &Board, ix: u16) -> Vec<u16>
{
    let mut tiles = [(false, u16::MAX);4];
    board.neighbors(ix, &mut tiles);
    tiles.iter()
        .filter(|(open, _)| *open)
        .map(|(_, i)| *i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("./input.txt");
        assert_eq!(part_one(input), 181952);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("./input.txt");
        assert_eq!(part_two(input), 47296);
    }

    #[test]
    fn example1_part_one() {
        let input = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(part_one(input), 27730);
    }

    #[test]
    fn example2_part_one() {
        let input = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        assert_eq!(part_one(input), 36334);
    }

    #[test]
    fn example3_part_one() {
        let input = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(part_one(input), 39514);
    }

    #[test]
    fn example4_part_one() {
        let input = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(part_one(input), 27755);
    }

    #[test]
    fn example5_part_one() {
        let input = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        assert_eq!(part_one(input), 28944);
    }

    #[test]
    fn example6_part_one() {
        let input = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(part_one(input), 18740);
    }

    #[test]
    fn example1_part_two() {
        let input = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(part_two(input), 4988);
    }

    #[test]
    fn example3_part_two() {
        let input = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(part_two(input), 31284);
    }

    #[test]
    fn example4_part_two() {
        let input = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(part_two(input), 3478);
    }
}
