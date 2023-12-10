use aoc_common_rs::math::lcm;
use aoc_common_rs::ord::Top;

#[derive(Clone, Copy, Default)]
pub enum Operation {
    #[default]
    Identity,
    Add(u64),
    Mul(u64),
    Div(u64),
    Rem(u64),
    Square,
}

impl Operation {
    fn apply(self, value: u64) -> u64 {
        match self {
            Self::Identity => value,
            Self::Add(n) => value + n,
            Self::Mul(n) => value * n,
            Self::Div(n) => value / n,
            Self::Rem(n) => value % n,
            Self::Square => value * value,
        }
    }
}

#[derive(Default)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test_divisible_by: u64,
    pub next_if_true: usize,
    pub next_if_false: usize,
    pub inspections: u64,
}

impl Monkey {
    fn inspect(&mut self, item: u64, post_operation: Operation) -> (u64, usize) {
        let new_item = post_operation.apply(self.operation.apply(item));
        let next = if (new_item % self.test_divisible_by) == 0 {
            self.next_if_true
        } else {
            self.next_if_false
        };
        self.inspections += 1;

        (new_item, next)
    }
}

#[derive(Default)]
pub struct State {
    pub monkeys: Vec<Monkey>,
    post_operation: Operation,
}

impl State {
    pub fn determine_post_operation(&mut self, gold: bool) {
        self.post_operation = if gold {
            let mut divisor: u64 = 1;
            for monkey in &self.monkeys {
                divisor = lcm(divisor, monkey.test_divisible_by);
            }
            Operation::Rem(divisor)
        } else {
            Operation::Div(3)
        }
    }
    pub fn play_round(&mut self) {
        for monkey in 0..self.monkeys.len() {
            self.play_monkey(monkey)
        }
    }
    pub fn play_monkey(&mut self, monkey: usize) {
        loop {
            let items = self.monkeys[monkey].items.split_off(0);
            if items.is_empty() {
                break;
            }
            for item in items {
                let (new_item, next) = self.monkeys[monkey].inspect(item, self.post_operation);
                self.monkeys[next].items.push(new_item);
            }
        }
    }
    pub fn ensure_monkey_exists(&mut self, monkey: usize) {
        for _ in self.monkeys.len()..=monkey {
            self.monkeys.push(Default::default());
        }
    }
    pub fn monkey_business_level(&self) -> u64 {
        let mut most_active: Top<u64, 2> = Default::default();
        for monkey in &self.monkeys {
            most_active.insert(monkey.inspections);
        }

        let [top2, top1] = *most_active;

        top1 * top2
    }
    pub fn dump_items(&self) {
        for (monkey, i) in self.monkeys.iter().zip(0usize..) {
            println!("[-] Monkey {}: {:?}", i, monkey.items);
        }
    }
    pub fn dump_inspections(&self) {
        for (monkey, i) in self.monkeys.iter().zip(0usize..) {
            println!(
                "[-] Monkey {} inspected items {} times.",
                i, monkey.inspections
            );
        }
    }
}
