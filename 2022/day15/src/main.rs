
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let positions = part_one(input, 2000000);
    println!("Part 1: {} ({:?})", positions, t.elapsed());

    let t = Instant::now();
    let tuning_frequency = part_two(input, 4000000);
    println!("Part 2: {} ({:?})", tuning_frequency, t.elapsed());
}

fn part_one(input: &str, y: i32) -> i32 {
    let mut x1 = i32::MAX;
    let mut x2 = i32::MIN;

    load(input).iter()
        .filter(|(p, md)| y <= p.1 + md && y >= p.1 - md)
        .for_each(|(p, md)| {
            let xd = md - p.1.abs_diff(y) as i32;
            x1 = x1.min(p.0 - xd);
            x2 = x2.max(p.0 + xd);
        });

    x2 - x1
}

#[allow(clippy::needless_range_loop)]
fn part_two(input: &str, m: i32) -> i64 {
    let sensors = load(input);

    for y in 0..=m {
        let v = covered(&sensors, y);

        let (x1, mut x2) = v[0];
        if x1 == 1 {
            return y as i64
        }
        for i in 1..v.len() {
            if v[i].0 > x2 { 
                return (x2 as i64 + 1) * 4000000 + y as i64;
            }
            if x2 < v[i].1 { x2 = v[i].1 }
        }
        if x2 < m {
            return (x2 as i64 + 1) * 4000000 + y as i64
        }
    }

    0
}

fn load(input: &str) -> Vec<((i32, i32), i32)> {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| {
            let sensor_x = v[2][2..].replace(',',"").parse::<i32>().unwrap();
            let sensor_y = v[3][2..].replace(':',"").parse::<i32>().unwrap();
            let beacon_x = v[8][2..].replace(',',"").parse::<i32>().unwrap();
            let beacon_y = v[9][2..].parse::<i32>().unwrap();
            let md = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
            ((sensor_x, sensor_y), md as i32)
        })
        .collect()
}

fn covered(sensors: &[((i32, i32), i32)], y: i32) -> Vec<(i32, i32)> {
    let mut ranges = vec![];

    sensors.iter()
        .filter(|(p, md)| y <= p.1 + md && y >= p.1 - md)
        .for_each(|(p, md)| {
            let xd = md - p.1.abs_diff(y) as i32;
            ranges.push((p.0 - xd, p.0 + xd));
        });

    ranges.sort();
    ranges
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let positions = part_one(input, 2000000);
        assert_eq!(positions, 5461729);

        let tuning_frequency = part_two(input, 4000000);
        assert_eq!(tuning_frequency, 10621647166538);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let positions = part_one(input, 10);
        assert_eq!(positions, 26);

        let tuning_frequency = part_two(input, 20);
        assert_eq!(tuning_frequency, 56000011);
    }
}
