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
    use std::collections::HashMap;
    use itertools::iproduct;

    let boss = load(input);
    let mut cache = HashMap::new();

    iproduct!(
        0..RINGS.len(),
        0..RINGS.len(),
        0..ARMOR.len(),
        0..WEAPONS.len()
    ).filter_map(|(r1, r2, a, w)| {
        if r1 == r2 && r1 != 0 { 
            None
        } else {
            let cost   = ARMOR[a].cost + WEAPONS[w].cost + RINGS[r1].cost + RINGS[r2].cost;
            let armor  = ARMOR[a].armor + RINGS[r1].armor + RINGS[r2].armor;
            let damage = WEAPONS[w].damage + RINGS[r1].damage + RINGS[r2].damage;
            let player = Player { hp: 100, armor, damage };
            let won = if let Some(result) = cache.get(&(armor, damage)) {
                *result
            } else {
                let result = fight(&player, &boss);
                cache.insert((armor, damage), result);
                result
            };
            won.then_some(cost)
        }
    })
    .min()
    .unwrap()
}

fn part_two(input: &str) -> i32
{
    use std::collections::HashMap;
    use itertools::iproduct;

    let boss = load(input);
    let mut cache = HashMap::new();

    iproduct!(
        0..RINGS.len(),
        0..RINGS.len(),
        0..ARMOR.len(),
        0..WEAPONS.len()
    ).filter_map(|(r1, r2, a, w)| {
        if r1 == r2 && r1 != 0 { 
            None
        } else {
            let cost   = ARMOR[a].cost + WEAPONS[w].cost + RINGS[r1].cost + RINGS[r2].cost;
            let armor  = ARMOR[a].armor + RINGS[r1].armor + RINGS[r2].armor;
            let damage = WEAPONS[w].damage + RINGS[r1].damage + RINGS[r2].damage;
            let player = Player { hp: 100, armor, damage };
            let won = if let Some(result) = cache.get(&(armor, damage)) {
                *result
            } else {
                let result = fight(&player, &boss);
                cache.insert((armor, damage), result);
                result
            };
            (!won).then_some(cost)
        }
    })
    .max()
    .unwrap()
}

fn fight(player: &Player, boss: &Player) -> bool
{
    let players = [player, boss];
    let mut hp  = [player.hp, boss.hp];

    let mut p = 1;  // player goes first
    loop {
        let damage = players[1-p].damage - players[p].armor;
        hp[p] -= if damage < 1 { 1 } else { damage };

        // Return true if the player wins
        if hp[p] < 1 { break p == 1 }

        p = 1 - p;  // switch players
    }
}

struct Player {
    hp: i32,
    armor: i32,
    damage: i32,
}

fn load(input: &str) -> Player
{
    let mut it = input.lines()
        .map(|l| {
            let v = l.split(": ").collect::<Vec<_>>();
            v[1].parse::<i32>().unwrap()
        });
    
    Player {
        hp:     it.next().unwrap(),
        damage: it.next().unwrap(),
        armor:  it.next().unwrap(),
    }
}

struct Item {
    cost: i32,
    armor: i32,
    damage: i32,
}
const ARMOR: [Item;6] = [
    Item { cost:   0, damage: 0, armor: 0 },   // None
    Item { cost:  13, damage: 0, armor: 1 },   // Leather
    Item { cost:  31, damage: 0, armor: 2 },   // Chainmail
    Item { cost:  53, damage: 0, armor: 3 },   // Splintmail
    Item { cost:  75, damage: 0, armor: 4 },   // Bandedmail
    Item { cost: 102, damage: 0, armor: 5 },   // Platemail
];
const WEAPONS: [Item;5] = [
    Item { cost:  8, damage: 4, armor: 0 },    // Dagger
    Item { cost: 10, damage: 5, armor: 0 },    // Shortsword
    Item { cost: 25, damage: 6, armor: 0 },    // Warhammer
    Item { cost: 40, damage: 7, armor: 0 },    // Longsword
    Item { cost: 74, damage: 8, armor: 0 },    // Greataxe
];
const RINGS: [Item;7] = [
    Item { cost:   0, damage: 0, armor: 0 },   // None
    Item { cost:  25, damage: 1, armor: 0 },   // Damage +1
    Item { cost:  50, damage: 2, armor: 0 },   // Damage +2
    Item { cost: 100, damage: 3, armor: 0 },   // Damage +3
    Item { cost:  20, damage: 0, armor: 1 },   // Defense +1
    Item { cost:  40, damage: 0, armor: 2 },   // Defense +2
    Item { cost:  80, damage: 0, armor: 3 },   // Defense +3
];



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 111);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 188);
    }
}