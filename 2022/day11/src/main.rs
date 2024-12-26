
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize {
    let mut monkeys: Vec<_> = input.split("\n\n")
        .map(Monkey::new)
        .collect();

    // To keep the borrow checker happy.
    let mut items: Vec<_> = monkeys.iter_mut()
        .map(|m| std::mem::take(&mut m.items))
        .collect();
    
    (0..20).for_each(|_|
        monkeys.iter_mut().enumerate()
            .for_each(|(i, m)| {
                let m_items = std::mem::take(&mut items[i]);        
                m_items.iter()
                    .for_each(|&v| {
                        let wl = m.inspect(v) / 3;
                        let monkey = m.throw_to(wl);
                        items[monkey].push(wl);
                    });
            })
    );
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

fn part_two(input: &str) -> usize {
    let mut monkeys: Vec<_> = input.split("\n\n")
        .map(Monkey::new)
        .collect();

    // To keep the borrow checker happy.
    let mut items: Vec<_> = monkeys.iter_mut()
        .map(|m| std::mem::take(&mut m.items))
        .collect();
    
    let divisor: u64 = monkeys.iter().map(|m| m.test_divisor).product();
    
    (0..10000).for_each(|_| {
        monkeys.iter_mut()
            .enumerate()
            .for_each(|(i, m)| {
                let m_items = std::mem::take(&mut items[i]);        
                m_items.iter()
                    .for_each(|&v| {
                        let wl = m.inspect(v) % divisor;
                        let monkey = m.throw_to(wl);
                        items[monkey].push(wl);
                    });
            });
    });
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op_value: Option<u64>,
    operation: fn(u64, Option<u64>) -> u64,
    inspections: usize,
    true_monkey: usize,
    false_monkey: usize,
    test_divisor: u64,
}

impl Monkey {
    fn new(s: &str) -> Self {
        let inspections = 0;

        // Skip the label
        let mut iter = s.split('\n').skip(1);

        let s = iter.next().unwrap();
        let items: Vec<_> = s[18..].split(", ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let s = iter.next().unwrap();
        let v: Vec<_> = s[22..].split(' ').collect();
        let operation = if v[1] == "+" {
            |old, v: Option<u64>| old + v.unwrap_or(old)
        } else {
            |old, v: Option<u64>| old * v.unwrap_or(old)
        };
        let op_value = if v[2] == "old" 
            { None } else { Some(v[2].parse::<u64>().unwrap()) };

        let s = iter.next().unwrap();
        let test_divisor = s[21..].parse::<u64>().unwrap();

        let s = iter.next().unwrap();
        let v: Vec<_> = s.split(' ').collect();
        let true_monkey = v[9].parse::<usize>().unwrap();

        let s = iter.next().unwrap();
        let v: Vec<_> = s.split(' ').collect();
        let false_monkey = v[9].parse::<usize>().unwrap();

        Monkey {
            items,
            op_value, 
            operation,
            inspections,
            true_monkey,
            false_monkey,
            test_divisor
        }
    }

    fn inspect(&mut self, item: u64) -> u64 {
        self.inspections += 1;
        (self.operation)(item, self.op_value)
    }

    fn throw_to(&self, wl: u64) -> usize {
        if wl % self.test_divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 50616);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 11309046332);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 10605);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 2713310158);
    }
}
