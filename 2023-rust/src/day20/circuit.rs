use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    ops::Not,
};

use aoc_common_rs::{cc::TwoCC, mapping::Mapping, math::lcm, ord::insert_sorted};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Pulse {
    Low = 0,
    High = 1,
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Module {
    FlipFlop(Pulse),
    Conjunction(Vec<Pulse>, usize),
    Counter(usize, usize),
    Broadcaster,
    Sink,
}

impl Module {
    pub fn new_flip_flop() -> Self {
        Self::FlipFlop(Pulse::Low)
    }

    pub fn new_conjunction() -> Self {
        Self::Conjunction(Vec::new(), 0)
    }

    pub fn new_counter() -> Self {
        Self::Counter(0, 0)
    }

    fn process(&mut self, pulse: Pulse, from: usize) -> Option<Pulse> {
        match self {
            Module::FlipFlop(last) => match pulse {
                Pulse::Low => {
                    *last = !*last;
                    Some(*last)
                }
                Pulse::High => None,
            },
            Module::Conjunction(inputs, num_low) => {
                let last = &mut inputs[from];
                if *last != pulse {
                    *last = pulse;
                    match pulse {
                        Pulse::Low => {
                            *num_low += 1;
                        }
                        Pulse::High => {
                            *num_low -= 1;
                        }
                    }
                }
                Some(if *num_low == 0 {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Module::Counter(num_low, num_high) => {
                match pulse {
                    Pulse::Low => {
                        *num_low += 1;
                    }
                    Pulse::High => {
                        *num_high += 1;
                    }
                }
                None
            }
            Module::Broadcaster => Some(pulse),
            Module::Sink => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Circuit {
    modules: Vec<(Module, Vec<(usize, usize)>, Vec<usize>)>,
}

impl Circuit {
    pub fn export(&self, mapping: &Mapping<TwoCC>) {
        println!("digraph g {{");
        for ((module, outputs, _), index) in self.modules.iter().zip(0usize..) {
            let mut label = mapping.label(index).to_string_lax();
            if label.is_empty() {
                label.push_str("broadcaster");
            }
            println!(
                "    {} [style = filled, fillcolor = \"#{}\"]",
                label,
                match module {
                    Module::FlipFlop(_) => "ffff80",
                    Module::Conjunction(_, _) => "80ffff",
                    Module::Counter(_, _) => "80ff80",
                    Module::Broadcaster => "ff80ff",
                    Module::Sink => "ff8080",
                }
            );
            for (output, _) in outputs.iter() {
                println!(
                    "    {} -> {}",
                    label,
                    mapping.label(*output).to_string_lax()
                );
            }
        }
        println!("}}");
    }

    pub fn trigger(&mut self, index: usize) -> (usize, usize) {
        let mut num_low = 1;
        let mut num_high = 0;
        let mut queue = VecDeque::new();
        queue.push_back((index, usize::MAX, Pulse::Low));
        while let Some((to, from, pulse)) = queue.pop_front() {
            let (module, outputs, _) = &mut self.modules[to];
            if let Some(out_pulse) = module.process(pulse, from) {
                match out_pulse {
                    Pulse::Low => {
                        num_low += outputs.len();
                    }
                    Pulse::High => {
                        num_high += outputs.len();
                    }
                }
                for (to, from) in outputs.iter() {
                    queue.push_back((*to, *from, out_pulse));
                }
            }
        }
        (num_low, num_high)
    }

    pub fn triggers_before_low(&mut self, from_index: usize, to_index: usize) -> usize {
        if !matches!(self.modules[to_index].0, Module::Counter(_, _)) {
            panic!("not a counter");
        }
        if let [before_to_index] = *self.modules[to_index].2 {
            if matches!(self.modules[before_to_index].0, Module::Conjunction(_, _)) {
                let mut clone = self.clone();
                clone.modules[before_to_index].0 = Module::new_counter();
                let mut total_iterations = 1usize;
                let mut remaining = clone.modules[before_to_index].2.len();
                let mut last_counter = 0usize;
                let mut iterations = 0usize;
                loop {
                    match clone.modules[before_to_index].0 {
                        Module::Counter(_, counter) => {
                            if counter > last_counter {
                                last_counter = counter;
                                let lcm = lcm(total_iterations, iterations);
                                if lcm != total_iterations {
                                    total_iterations = lcm;
                                    remaining -= 1;
                                    if remaining == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                    clone.trigger(from_index);
                    iterations += 1;
                }
                return total_iterations;
            }
        }
        let mut iterations = 0usize;
        while matches!(self.modules[to_index].0, Module::Counter(0, _)) {
            self.trigger(from_index);
            iterations += 1;
        }
        iterations
    }
}

#[derive(Debug)]
pub struct CircuitBuilder {
    modules: HashMap<TwoCC, (Module, Vec<TwoCC>)>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, id: TwoCC, module: Module, outputs: Vec<TwoCC>) {
        self.modules.insert(id, (module, outputs));
    }

    pub fn build(mut self) -> Result<(Circuit, Mapping<TwoCC>), Box<dyn Error>> {
        let mut sinks = HashSet::new();
        for (_, outputs) in self.modules.values() {
            for output in outputs.iter() {
                if !self.modules.contains_key(output) {
                    sinks.insert(*output);
                }
            }
        }
        for sink in sinks.into_iter() {
            eprintln!("[-] Adding sink {:?}", sink);
            self.modules.insert(sink, (Module::Sink, vec![]));
        }
        let mapping = self.modules.keys().map(|key| *key).collect::<Mapping<_>>();
        let mut inputs = vec![Vec::new(); mapping.len()];
        for (input, (_, outputs)) in self.modules.iter() {
            for output in outputs.iter() {
                let module_inputs = &mut inputs[mapping.index(output).unwrap()];
                insert_sorted(module_inputs, mapping.index(input).unwrap());
            }
        }
        let modules = inputs
            .iter()
            .zip(mapping.iter())
            .map(|(module_inputs, (module_id, module_index))| {
                let (mut module, outputs) = self.modules.remove(module_id).unwrap();
                match &mut module {
                    Module::Conjunction(inputs, num_low) => {
                        *inputs = vec![Pulse::Low; module_inputs.len()];
                        *num_low = module_inputs.len();
                    }
                    _ => {}
                }
                (
                    module,
                    outputs
                        .into_iter()
                        .map(|output| {
                            let index = mapping.index(&output).unwrap();
                            (
                                index,
                                inputs[index]
                                    .iter()
                                    .position(|input| *input == module_index)
                                    .unwrap(),
                            )
                        })
                        .collect::<Vec<_>>(),
                    inputs[module_index].clone(),
                )
            })
            .collect::<Vec<_>>();
        assert!(self.modules.len() == 0);
        for (module, _, inputs) in modules.iter() {
            if matches!(module, Module::Counter(_, _)) && inputs.is_empty() {
                return Err("unreachable counter module".into());
            }
        }
        Ok((Circuit { modules }, mapping))
    }
}
