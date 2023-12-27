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

fn part_one(input: &str) -> u32
{
    let stones = load(input);

    crossings(&stones, 200000000000000.0, 400000000000000.0)
}

fn part_two(input: &str) -> i64
{
    use z3::ast::{Ast, Int};
    use z3::{Config, Context, SatResult, Solver};

    let stones = load(input);

    let ctx = Context::new(&Config::new());
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");
    
    let zero = Int::from_i64(&ctx, 0);

    for (n, st) in stones.iter().enumerate() {
        let t = Int::new_const(&ctx, format!("t{n}"));

        let st_px = Int::from_i64(&ctx, st.p[0]);
        let st_py = Int::from_i64(&ctx, st.p[1]);
        let st_pz = Int::from_i64(&ctx, st.p[2]);
        let st_vx = Int::from_i64(&ctx, st.v[0]);
        let st_vy = Int::from_i64(&ctx, st.v[1]);
        let st_vz = Int::from_i64(&ctx, st.v[2]);

        let x1 = &px + (&vx * &t);
        let y1 = &py + (&vy * &t);
        let z1 = &pz + (&vz * &t);
        let x2 = st_px + (st_vx * &t);
        let y2 = st_py + (st_vy * &t);
        let z2 = st_pz + (st_vz * &t);

        solver.assert(&t.ge(&zero));
        solver.assert(&x1._eq(&x2));
        solver.assert(&y1._eq(&y2));
        solver.assert(&z1._eq(&z2));
    }

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model().unwrap();
    let res_px = model.eval(&px, true).unwrap();
    let res_py = model.eval(&py, true).unwrap();
    let res_pz = model.eval(&pz, true).unwrap();
    let x = res_px.as_i64().unwrap();
    let y = res_py.as_i64().unwrap();
    let z = res_pz.as_i64().unwrap();

    x + y + z
}

fn load(input: &str) -> Vec<Stone>
{
    input.lines()
        .map(|line| {
            let (sp, sv) = line.split_once(" @ ").unwrap();

            let mut iter = sp.split(',');
            let x = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let y = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let z = iter.next().unwrap().trim().parse::<i64>().unwrap();

            let mut iter = sv.split(',');
            let xv = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let yv = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let zv = iter.next().unwrap().trim().parse::<i64>().unwrap();

            Stone { p: [x, y, z], v: [xv, yv, zv] }
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Stone {
    p: [i64;3],
    v: [i64;3],
}

fn crossings(stones: &[Stone], min: f32, max: f32) -> u32
{
    let r = min..=max;
    let is_valid = |p: &(f32, f32)| r.contains(&p.0) && r.contains(&p.1);

    stones.iter()
        .enumerate()
        .map(|(i, s1)| stones.iter()
            .skip(i + 1)
            .filter_map(|s2| intersection_2d(s1, s2))
            .filter(is_valid)
            .count()
        )
        .sum::<usize>() as u32
}

fn intersection_2d(s1: &Stone, s2: &Stone) -> Option<(f32, f32)>
{
    let dx = s2.p[0] - s1.p[0];
    let dy = s2.p[1] - s1.p[1];
    let dt = s2.v[0] * s1.v[1] - s2.v[1] * s1.v[0];
    if dt == 0 {
        return None
    }

    let u = (dy * s2.v[0] - dx * s2.v[1]) as f32 / dt as f32;
    let v = (dy * s1.v[0] - dx * s1.v[1]) as f32 / dt as f32;
    if u < 0.0 || v < 0.0 {
        return None
    }

    Some((
        s1.p[0] as f32 + s1.v[0] as f32 * u,
        s1.p[1] as f32 + s1.v[1] as f32 * u)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 16050);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 669042940632377);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        let stones = load(input);
        assert_eq!(crossings(&stones, 7.0, 27.0), 2);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 47);
    }
}
