use std::collections::{HashMap, VecDeque};

type Modules<'a> = HashMap<&'a str, Module<'a>>;
type Network<'a> = VecDeque<(&'a str, &'a str, Pulse)>;

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
    let (mut modules, _, _) = load(input);
    
    let (l, h) = (0..1000)
        .fold((0, 0), |acc, _| {
            let (l, h) = run(&mut modules);
            (acc.0 + l, acc.1 + h)
        });

    l * h
}

fn part_two(input:& str) -> u64
{
    use num::integer::Integer;

    // jt => 510 @ 3918 steps
    // mh => 510 @ 4050 steps
    // pz => 126 @ 3760 steps
    // rn => 126 @ 3906 steps
    // They all reset on the next step so the assumption
    // is at some point they are all at the almost highest
    // value together and the next step would put them all
    // turned on. So, we get the LCM of that next step cycle.
    // Thus: 3819, 4051, 3761 & 3907
    // 233_283_622_908_263

    let mut i = 1;
    let mut needed = 4;
    let mut registers = [0u64;4];

    let mut q = VecDeque::new();

    let (mut modules, _, _) = load(input);
    while needed > 0 {
        press(&mut modules, &mut q);

        let rn = ["bx", "fx", "nx", "kn", "mv", "fk", "rv"].iter()
            .enumerate()
            .fold(0u64, |acc, (i, s)| {
                let m = modules.get(s).unwrap();
                acc | m.bit() << i
            });
        if rn == 126 {
            registers[0] = i + 1;
            needed -= 1;
         }
        
        let pz = ["jp", "dx", "ph", "jc", "ct", "kd", "pp"].iter()
            .enumerate()
            .fold(0u64, |acc, (i, s)| {
                let m = modules.get(s).unwrap();
                acc | m.bit() << i
            });
        if pz == 126 {
            registers[1] = i + 1;
            needed -= 1;
        }
            
        let jt = ["jq", "qt", "lj", "dt", "vp", "jm", "xk", "nk", "vk"].iter()
            .enumerate()
            .fold(0u64, |acc, (i, s)| {
                let m = modules.get(s).unwrap();
                acc | m.bit() << i
            });
        if jt == 510 {
            registers[2] = i + 1;
            needed -= 1;
        }

        let mh = ["nv", "th", "jf", "xm", "gv", "nr", "cj", "vh", "jh"].iter()
            .enumerate()
            .fold(0u64, |acc, (i, s)| {
                let m = modules.get(s).unwrap();
                acc | m.bit() << i
            });
        if mh == 510 {
            registers[3] = i + 1;
            needed -= 1;
        }
    
        i += 1
    }

    registers.iter().cloned()
        .reduce(|acc, n| acc.lcm(&n)).unwrap()
}

fn press<'a>(modules: &mut Modules<'a>, q: &mut Network<'a>)
{
    q.push_back(("button", "broadcaster", Pulse::Low));
    while let Some((src, dst, pulse)) = q.pop_front() {
        if let Some(module) = modules.get_mut(dst) {
            module.process(src, pulse).iter()
                .for_each(|(m, pulse)| q.push_back((dst, m, *pulse)))
        }
    }
}

fn run(modules: &mut Modules) -> (u32, u32)
{
    // Initial pulse from button.
    let mut low_pulses = 1;
    let mut high_pulses = 0;

    let mut q = VecDeque::from([("button", "broadcaster", Pulse::Low)]);
    while let Some((src, dst, pulse)) = q.pop_front() {
        if let Some(module) = modules.get_mut(dst) {
            module.process(src, pulse).iter()
                .for_each(|(m, pulse)| {
                    if let Pulse::Low = pulse {
                        low_pulses += 1;
                    } else {
                        high_pulses += 1;
                    }
                    q.push_back((dst, m, *pulse))
                })
        }
    }

    (low_pulses, high_pulses)
}

#[allow(clippy::manual_strip)]
fn load(input: &str) -> (HashMap<&str, Module>, Vec<&str>, Vec<&str>)
{
    use Module::*;

    let mut flipflops = vec![];
    let mut conjunctions = vec![];

    let mut modules: HashMap<&str, Module> = input.lines()
        .map(|line| {
            let (s1, s2) = line.split_once(" -> ").unwrap();
            let outputs = s2.split(',')
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            if s1 == "broadcaster" {
                ("broadcaster", Broadcaster { outputs })
            } else if s1.starts_with('%') {
                flipflops.push(&s1[1..]);
                (&s1[1..], FlipFlop { outputs, on: false })
            } else {
                conjunctions.push(&s1[1..]);
                (&s1[1..], Conjunction { outputs, inputs: HashMap::new() })
            }
        })
        .collect();

    for (name, module) in modules.clone().iter() {
        for output in module.outputs() {
            if let Some(m) = modules.get_mut(output) {
                m.add_input(name)
            }
        }
    }

    (modules, flipflops, conjunctions)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
    None,
}

#[derive(Clone)]
enum Module<'a> {
    FlipFlop { on: bool, outputs: Vec<&'a str> },
    Conjunction { inputs: HashMap<&'a str, Pulse>, outputs: Vec<&'a str> },
    Broadcaster { outputs: Vec<&'a str> },
}
impl<'a> Module<'a> {
    fn outputs(&self) -> Vec<&str>
    {
        use Module::*;

        match self {
            FlipFlop { outputs, .. } => outputs.clone(),
            Conjunction { outputs, .. } => outputs.clone(),
            Broadcaster { outputs, .. } => outputs.clone(),
        }
    }

    fn add_input(&mut self, name: &'a str)
    {
        use Module::*;

        match self {
            FlipFlop { .. } => (),
            Broadcaster { .. } => (),
            Conjunction { inputs, .. } => { inputs.insert(name, Pulse::Low); },
        }
    }

    fn process(&mut self, src: &str, input: Pulse) -> Vec<(&'a str, Pulse)>
    {
        use Module::*;

        let (pulse, outputs) = match self {
            Broadcaster { outputs } => (input, outputs),
            FlipFlop { on, outputs } => {
                if input == Pulse::Low {
                    *on = !*on;
                    if *on { 
                        (Pulse::High, outputs)
                    } else {
                        (Pulse::Low, outputs)
                    }
                } else {
                    (Pulse::None, outputs)
                }
            },
            Conjunction { inputs, outputs } => {
                *inputs.get_mut(src).unwrap() = input;
                if inputs.values().all(|p| *p == Pulse::High) {
                    (Pulse::Low, outputs)
                } else {
                    (Pulse::High, outputs)
                }
            }
        };

        match pulse {
            Pulse::None => vec![],
            _ => outputs.iter().map(|&m| (m, pulse)).collect(),
        }
    }

    fn bit(&self) -> u64
    {
        use Module::*;

        match self {
            FlipFlop { on, .. } => *on as u64,
            Broadcaster { .. } => 1,
            Conjunction { inputs, .. } => {
                let on = inputs.values().any(|p| *p == Pulse::Low);
                on as u64
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 886347020);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 233_283_622_908_263);
    }

    #[test]
    fn example1_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 32000000);
    }

    #[test]
    fn example2_part_one()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 11687500);
    }
}
