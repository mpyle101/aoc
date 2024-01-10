fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2 (mb): {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two_z3(input);
    println!("Part 2 (z3): {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let stones = load(input);

    crossings(&stones, 200000000000000.0, 400000000000000.0)
}

fn part_two(input: &str) -> i64
{
    use std::collections::HashSet;

    let stones = load(input);

    // Find pairs of hail stones whose velocity in a given
    // axis is the same. These two stones will always have
    // the same relative position to one another (ie, the 
    // distance between them remains the same forever). In
    // order to hit both stones, the rock must be traveling
    // at a speed intersecting both potential axis positions.
    // That means the difference between the rock speed and
    // the stone speed must be a multiple of the difference
    // between the stone positions. Find a set of values
    // making this true across a likely range. The intersection
    // of all potential velocities is the one (or the few but
    // reelistically the one).
    let mut vx = HashSet::new();
    let mut vy = HashSet::new();
    let mut vz = HashSet::new();

    for (a, i) in stones.iter().zip(1..) {
        for b in stones.iter().skip(i) {
            if vx.len() != 1 && a.v[0] == b.v[0] {
                let dv = (-1000..=1000)
                    .filter(|v| *v != a.v[0])
                    .filter(|v| (a.p[0] - b.p[0]) % (v - a.v[0]) == 0)
                    .collect();
                vx = if vx.is_empty() { dv } else { &dv & &vx }
            }
            if vy.len() != 1 && a.v[1] == b.v[1] {
                let dv = (-1000..=1000)
                    .filter(|v| *v != a.v[1])
                    .filter(|v| (a.p[1] - b.p[1]) % (v - a.v[1]) == 0)
                    .collect();
                vy = if vy.is_empty() { dv } else { &dv & &vy }
            }
            if vz.len() != 1 && a.v[2] == b.v[2] {
                let dv = (-1000..=1000)
                    .filter(|v| *v != a.v[2])
                    .filter(|v| (a.p[2] - b.p[2]) % (v - a.v[2]) == 0)
                    .collect();
                vz = if vz.is_empty() { dv } else { &dv & &vz }
            }
        }
    }

    let rv = [
        *vx.iter().next().unwrap() as f64,
        *vy.iter().next().unwrap() as f64,
        *vz.iter().next().unwrap() as f64
    ];

    // Get slopes of lines formed by subtracting rock velocity
    // from stone velocity.
    let a = stones[0].real();
    let b = stones[1].real();
    let ma = (a.1[1] - rv[1]) / (a.1[0] - rv[0]);
    let mb = (b.1[1] - rv[1]) / (b.1[0] - rv[0]);

    // y = mx + b => b = y - mx
    let ba = a.0[1] - ma * a.0[0];
    let bb = b.0[1] - mb * b.0[0];

    // This part I don't quite get.
    let x = (bb - ba) / (ma - mb);
    let y = ma * x + ba;
    let t = (x - a.0[0]) / (a.1[0] - rv[0]);
    let z = a.0[2] + (a.1[2] - rv[2]) * t;

    x.round() as i64 +
    y.round() as i64 +
    z.round() as i64
}

fn part_two_z3(input: &str) -> i64
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

        let x1 = &px + (&vx * &t);
        let y1 = &py + (&vy * &t);
        let z1 = &pz + (&vz * &t);
        let x2 = st.p[0] + (st.v[0] * &t);
        let y2 = st.p[1] + (st.v[1] * &t);
        let z2 = st.p[2] + (st.v[2] * &t);

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
impl Stone {
    fn real(&self) -> ([f64;3], [f64;3])
    {
        (
            [self.p[0] as f64, self.p[1] as f64, self.p[2] as f64],
            [self.v[0] as f64, self.v[1] as f64, self.v[2] as f64],
        )
    }
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
        assert_eq!(part_two_z3(input), 669042940632377);
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
        assert_eq!(part_two_z3(input), 47);
    }
}
