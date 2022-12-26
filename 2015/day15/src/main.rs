fn main() {
    use std::time::Instant;

    let ingredients = load(include_str!("../input.txt"));

    let t = Instant::now();
    let score = part_one(&ingredients);
    println!("Part 1: {} ({:?})", score, t.elapsed());

    let t = Instant::now();
    let score = part_two(&ingredients);
    println!("Part 2: {} ({:?})", score, t.elapsed());
}

#[derive(Debug)]
#[allow(dead_code)]
struct Ingredient {
    capacity: i32,
    calories: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

fn load(input: &str) -> Vec<Ingredient> {
    input.lines().map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| Ingredient {
            capacity:   v[2][0..v[2].len() - 1].parse::<i32>().unwrap(),
            durability: v[4][0..v[4].len() - 1].parse::<i32>().unwrap(),
            flavor:     v[6][0..v[6].len() - 1].parse::<i32>().unwrap(),
            texture:    v[8][0..v[8].len() - 1].parse::<i32>().unwrap(),
            calories:   v.last().unwrap().parse::<i32>().unwrap(),
        })
        .collect()
}

fn part_one(ingredients: &[Ingredient]) -> i32 {
    use itertools::Itertools;

    (0..=100).permutations(ingredients.len())
        .filter(|v| v.iter().sum::<i32>() == 100)
        .map(|v| v.iter().enumerate().fold(
            (0, 0, 0, 0), |t, (i, n)| (
                t.0 + ingredients[i].capacity * n,
                t.1 + ingredients[i].durability * n,
                t.2 + ingredients[i].flavor * n,
                t.3 + ingredients[i].texture * n
            )
        ))
        .map(|t| limit(t.0) * limit(t.1) * limit(t.2) * limit(t.3))
        .max()
        .unwrap()
}

fn part_two(ingredients: &[Ingredient]) -> i32 {
    use itertools::Itertools;

    (0..=100).permutations(ingredients.len())
        .filter(|v| v.iter().sum::<i32>() == 100)
        .map(|v| v.iter().enumerate().fold(
            (0, 0, 0, 0, 0), |t, (i, n)| (
                t.0 + ingredients[i].capacity * n,
                t.1 + ingredients[i].durability * n,
                t.2 + ingredients[i].flavor * n,
                t.3 + ingredients[i].texture * n,
                t.4 + ingredients[i].calories * n,
            )
        ))
        .filter(|t| t.4 == 500)
        .map(|t| limit(t.0) * limit(t.1) * limit(t.2) * limit(t.3))
        .max()
        .unwrap()
}


fn limit(n: i32) -> i32 {
    if n > 0 { n } else { 0 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let ingredients = load(include_str!("../input.txt"));
        assert_eq!(part_one(&ingredients), 222870);
    }

    #[test]
    fn input_part_two() {
        let ingredients = load(include_str!("../input.txt"));
        assert_eq!(part_two(&ingredients), 117936);
    }

    #[test]
    fn example_part_one() {
        let ingredients = load(include_str!("../example.txt"));
        assert_eq!(part_one(&ingredients), 62842880);
    }

    #[test]
    fn example_part_two() {
        let ingredients = load(include_str!("../example.txt"));
        assert_eq!(part_two(&ingredients), 57600000);
    }
}