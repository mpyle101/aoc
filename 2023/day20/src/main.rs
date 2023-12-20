use std::{collections::HashMap, vec};

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let mut modules = load(input);
    
    let (l, h) = (0..1000)
        .fold((0, 0), |acc, _| {
            let (l, h) = run(&mut modules);
            (acc.0 + l, acc.1 + h)
        });

    l * h
}

fn run(modules: &mut HashMap<&str, Module<'_>>) -> (u32, u32)
{
    use std::collections::VecDeque;

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
fn load(input: &str) -> HashMap<&str, Module>
{
    use Module::*;

    let mut modules: HashMap<&str, Module> = input.lines()
        .map(|line| {
            let (s1, s2) = line.split_once(" -> ").unwrap();
            let outputs = s2.split(',')
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            if s1 == "broadcaster" {
                ("broadcaster", Broadcaster { outputs })
            } else if s1.starts_with('%') {
                (&s1[1..], FlipFlop { outputs, on: false })
            } else {
                (&s1[1..], Conjunction { outputs, inputs: HashMap::new() })
            }
        })
        .collect();

    for (name, module) in modules.clone().iter() {
        for o in module.outputs() {
            if let Some(m) = modules.get_mut(o) {
                m.add_input(name)
            }
        }
    }

    modules
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
