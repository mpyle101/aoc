use vm::Vm;

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

fn part_one(input: &str) -> usize
{
    let mut vm = Vm::new(input).unwrap();
    let (_, mut stdout) = vm.pipes();

    vm.exec().unwrap();
    stdout.drain()
        .chunks(3)
        .filter(|c| c[2] == 2)
        .count()
}

fn part_two(input: &str) -> i64
{
    use std::collections::HashSet;

    let mut vm = Vm::new(input).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    vm.set_addr(0, 2);
    vm.exec().unwrap();
    let tiles = stdout.drain();
    let mut blocks: HashSet<(i64, i64)> = tiles.chunks(3)
        .filter(|c| c[2] == 2)
        .map(|c| (c[0], c[1]))
        .collect();

    let mut ball   = find_tile(&tiles, 4).unwrap();
    let mut paddle = find_tile(&tiles, 3).unwrap();
    let offset = ball.0 - paddle.0;

    let joystick = offset.cmp(&0) as i32;
    stdin.write(joystick);

    let mut score = 0;
    while !blocks.is_empty() {
        vm.cont().unwrap();
        for tile in stdout.drain().chunks(3) {
            match tile {
                [-1, 0, n] => score = *n,
                [x,  y, 0] => { blocks.remove(&(*x, *y)); },
                [x,  y, 3] => paddle = (*x, *y),
                [x,  y, 4] => ball = (*x, *y),
                _ => panic!()
            }
        };

        let offset = ball.0 - paddle.0;
        let joystick = offset.cmp(&0) as i32;
        stdin.write(joystick);
        vm.cont().unwrap();
    };

    score
}

fn find_tile(screen: &[i64], id: i64) -> Option<(i64, i64)>
{
    let tiles = screen.chunks(3)
        .collect::<Vec<&[i64]>>();
    let idx = tiles.iter()
        .position(|t| t[2] == id)?;

    Some((tiles[idx][0], tiles[idx][1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 427);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 21426);
    }
}