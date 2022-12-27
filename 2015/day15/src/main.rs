fn main() {
    use std::time::Instant;

    let ingredients = load(include_str!("../input.txt"));

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(&ingredients), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(&ingredients), t.elapsed());
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
    permutations(ingredients.len()).iter()
        .map(|v| ingredients.iter()
            .enumerate()
            .fold((0, 0, 0, 0), |acc, (i, ingredient)|
                (
                    acc.0 + ingredient.capacity * v[i],
                    acc.1 + ingredient.durability * v[i],
                    acc.2 + ingredient.flavor * v[i],
                    acc.3 + ingredient.texture * v[i]
                )
            ))
        .map(|t| limit(t.0) * limit(t.1) * limit(t.2) * limit(t.3))
        .max()
        .unwrap()
}

fn part_two(ingredients: &[Ingredient]) -> i32 {
    permutations(ingredients.len()).iter()
        .map(|v| ingredients.iter()
            .enumerate()
            .fold((0, 0, 0, 0, 0), |acc, (i, ingredient)|
                (
                    acc.0 + ingredient.capacity * v[i],
                    acc.1 + ingredient.durability * v[i],
                    acc.2 + ingredient.flavor * v[i],
                    acc.3 + ingredient.texture * v[i],
                    acc.4 + ingredient.calories * v[i],
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

fn permutations(count: usize) -> Vec<[i32;4]> {
    if count == 2 {
        (0..=100)
            .map(|a| [a, 100 - a, 0, 0])
            .collect()
    } else {
        (0..=100)
            .flat_map(|a| (0..=100)
                .flat_map(move |b| (0..=100)
                    .flat_map(move |c| (0..=100)
                        .filter(move |d| a + b + c + d == 100)
                        .map(move |d| [a, b, c, d]))))
            .collect()  
    }
}

#[derive(Debug)]
struct Ingredient {
    calories: i32,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
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