use std::ops::{Index, IndexMut};
use lazy_static::lazy_static;

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
    use std::collections::VecDeque;

    let boss = load(input);
    let state = State {
        mana: 0,
        hero: Wizard { hp: 50, mana: 500, armor: 0 },
        boss,
        effects: [0i32;3],
    };

    let mut mana = i32::MAX;
    let mut q = VecDeque::from([state]);
    while let Some(st) = q.pop_front() {
        if st.boss.hp <= 0 {
            mana = mana.min(st.mana);
        } else {
            do_move(&st, false).iter().for_each(|s| q.push_back(*s));
        }
    }

    mana
}

fn part_two(input: &str) -> i32
{
    use std::collections::VecDeque;

    let boss = load(input);
    let state = State {
        mana: 0,
        hero: Wizard { hp: 50, mana: 500, armor: 0 },
        boss,
        effects: [0i32;3],
    };

    let mut mana = i32::MAX;
    let mut q = VecDeque::from([state]);
    while let Some(st) = q.pop_front() {
        if st.boss.hp <= 0 {
            mana = mana.min(st.mana);
        } else {
            do_move(&st, true).iter().for_each(|s| q.push_back(*s));
        }
    }

    mana
}

fn load(input: &str) -> Monster
{
    let mut it = input.lines()
        .map(|l| {
            let v = l.split(": ").collect::<Vec<_>>();
            v[1].parse::<i32>().unwrap()
        });
    
    Monster {
        hp:     it.next().unwrap(),
        damage: it.next().unwrap(),
    }
}

enum Spell {
    Drain(i32),
    Shield(i32),
    Poison(i32),
    Recharge(i32),
    MagicMissle(i32),
}

lazy_static! {
    static ref SPELLS: [Spell;5] = [
        Spell::MagicMissle(53),
        Spell::Drain(73),
        Spell::Shield(113),
        Spell::Poison(173),
        Spell::Recharge(229),
    ];
}

#[derive(Clone, Copy)]
struct Monster {
    hp: i32,
    damage: i32,
}

#[derive(Clone, Copy)]
struct Wizard {
    hp: i32,
    mana: i32,
    armor: i32,
}

enum Effect {
    Shield = 0,
    Poison = 1,
    Recharge = 2,
}

type Effects = [i32;3];

impl Index<Effect> for Effects {
    type Output = i32;

    fn index(&self, effect: Effect) -> &Self::Output
    {
        match effect {
            Effect::Shield   => &self[0],
            Effect::Poison   => &self[1],
            Effect::Recharge => &self[2],
        }
    }
}

impl IndexMut<Effect> for Effects {
    fn index_mut(&mut self, effect: Effect) -> &mut Self::Output
    {
        match effect {
            Effect::Shield   => &mut self[0],
            Effect::Poison   => &mut self[1],
            Effect::Recharge => &mut self[2],
        }
    }
}


#[derive(Clone, Copy)]
struct State {
    mana: i32,
    hero: Wizard,
    boss: Monster,
    effects: Effects,
}

impl Spell {
    fn cast(&self, state: &State) -> State
    {
        use Effect::*;
    
        let mut st = *state;
        match self {
            Spell::Drain(n) => {
                st.mana += n;
                st.hero.mana -= n;
                st.hero.hp += 2;
                st.boss.hp -= 2;
            },
            Spell::Shield(n) => {
                st.mana += n;
                st.hero.mana -= n;
                st.effects[Shield] = 6;
            },
            Spell::Poison(n) => {
                st.mana += n;
                st.hero.mana -= n;
                st.effects[Poison] = 6;
            },
            Spell::Recharge(n) => {
                st.mana += n;
                st.hero.mana -= n;
                st.effects[Recharge] = 5;
            },
            Spell::MagicMissle(n) => {
                st.mana += n;
                st.hero.mana -= n;
                st.boss.hp -= 4;
            },
        }

        st
    }

    fn can_cast(&self, st: &State) -> bool
    {
        use Effect::*;

        match self {
            Spell::Drain(n)       => st.hero.mana >= *n,
            Spell::Shield(n)      => st.hero.mana >= *n && st.effects[Shield]   == 0,
            Spell::Poison(n)      => st.hero.mana >= *n && st.effects[Poison]   == 0,
            Spell::Recharge(n)    => st.hero.mana >= *n && st.effects[Recharge] == 0,
            Spell::MagicMissle(n) => st.hero.mana >= *n,
        }
    }
}

fn do_move(state: &State, hard: bool) -> Vec<State>
{
    let mut st0 = *state;
    if hard {
        if st0.hero.hp == 1 {
            return vec![]
        }
        st0.hero.hp -= 1;
    }

    let st = apply_effects(&st0);
    if st.boss.hp <= 0 {
        return vec![st]
    }

    // See if the hero can cast a spell
    // If so, cast the spell and see if the boss is dead
    // If not, do the boss's turn so first apply any effects
    // If the boss is dead, return the state
    // Otherwise, let the boss attack and if our hero still
    // lives, return that state.
    SPELLS.iter()
        .filter_map(|spell| {
            if spell.can_cast(&st) {
                let s = spell.cast(&st);
                if s.boss.hp <= 0 {
                    Some(s)
                } else {
                    let mut s = apply_effects(&s);
                    if s.boss.hp <= 0 {
                        Some(s)
                    } else {
                        let damage = s.boss.damage - s.hero.armor;
                        s.hero.hp -= if damage < 1 { 1 } else { damage };
                        if s.hero.hp > 0 {
                            Some(s)
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            }
        })
        .collect()
}

fn apply_effects(state: &State) -> State
{
    use Effect::*;

    let mut st = *state;
    if st.effects[Shield] > 0 {
        st.hero.armor = 7;
        st.effects[Shield] -= 1;
    } else {
        st.hero.armor = 0;
    }
    if st.effects[Poison] > 0 {
        st.boss.hp -= 3;
        st.effects[Poison] -= 1;
    }
    if st.effects[Recharge] > 0 {
        st.hero.mana += 101;
        st.effects[Recharge] -= 1;
    }

    st
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1824);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1937);
    }
}