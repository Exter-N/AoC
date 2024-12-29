use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use aoc_common_rs::{cc::ThreeCC, some_or_break, some_or_return_none};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wire {
    Literal(bool),
    And(ThreeCC, ThreeCC),
    Or(ThreeCC, ThreeCC),
    Xor(ThreeCC, ThreeCC),
}

impl Wire {
    fn evaluate(self, circuit: &mut Circuit, code: ThreeCC) -> Option<bool> {
        match self {
            Wire::Literal(value) => Some(value),
            Wire::And(op1, op2) => {
                let res1 = circuit.evaluate(op1);
                let res2 = circuit.evaluate(op2);
                let res = some_or_return_none!(res1) && some_or_return_none!(res2);
                circuit.define(code, Wire::Literal(res));
                Some(res)
            }
            Wire::Or(op1, op2) => {
                let res1 = circuit.evaluate(op1);
                let res2 = circuit.evaluate(op2);
                let res = some_or_return_none!(res1) || some_or_return_none!(res2);
                circuit.define(code, Wire::Literal(res));
                Some(res)
            }
            Wire::Xor(op1, op2) => {
                let res1 = circuit.evaluate(op1);
                let res2 = circuit.evaluate(op2);
                let res = some_or_return_none!(res1) ^ some_or_return_none!(res2);
                circuit.define(code, Wire::Literal(res));
                Some(res)
            }
        }
    }

    pub fn is_and(self) -> bool {
        matches!(self, Wire::And(_, _))
    }

    pub fn is_or(self) -> bool {
        matches!(self, Wire::Or(_, _))
    }

    pub fn is_xor(self) -> bool {
        matches!(self, Wire::Xor(_, _))
    }

    pub fn swap_operands(self) -> Wire {
        match self {
            Wire::Literal(value) => Wire::Literal(value),
            Wire::And(op1, op2) => Wire::And(op2, op1),
            Wire::Or(op1, op2) => Wire::Or(op2, op1),
            Wire::Xor(op1, op2) => Wire::Xor(op2, op1),
        }
    }

    pub fn other_operand(self, known_operand: ThreeCC) -> Option<ThreeCC> {
        match self {
            Wire::Literal(_) => None,
            Wire::And(op1, op2) => {
                if op1 == known_operand {
                    Some(op2)
                } else if op2 == known_operand {
                    Some(op1)
                } else {
                    None
                }
            }
            Wire::Or(op1, op2) => {
                if op1 == known_operand {
                    Some(op2)
                } else if op2 == known_operand {
                    Some(op1)
                } else {
                    None
                }
            }
            Wire::Xor(op1, op2) => {
                if op1 == known_operand {
                    Some(op2)
                } else if op2 == known_operand {
                    Some(op1)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Circuit {
    wires: HashMap<ThreeCC, Wire>,
}

fn two_digit_numbers() -> impl Iterator<Item = (char, char)> {
    ('0'..='9').cartesian_product('0'..='9')
}

fn make_number_bit_code(prefix: char, i: usize) -> ThreeCC {
    assert!(i < 100);
    ThreeCC::new(
        prefix,
        char::from_digit((i / 10) as u32, 10).unwrap(),
        char::from_digit((i % 10) as u32, 10).unwrap(),
    )
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
        }
    }

    pub fn define(&mut self, code: ThreeCC, definition: Wire) -> Option<Wire> {
        self.wires.insert(code, definition)
    }

    pub fn get(&self, code: ThreeCC) -> Option<Wire> {
        match self.wires.get(&code) {
            Some(definition) => Some(*definition),
            None => None,
        }
    }

    pub fn remove(&mut self, code: ThreeCC) -> Option<Wire> {
        self.wires.remove(&code)
    }

    pub fn define_opt(&mut self, code: ThreeCC, definition: Option<Wire>) -> Option<Wire> {
        match definition {
            Some(def) => self.define(code, def),
            None => self.remove(code),
        }
    }

    pub fn swap(&mut self, code1: ThreeCC, code2: ThreeCC) {
        let def1 = self.get(code1);
        let def2 = self.define_opt(code2, def1);
        self.define_opt(code1, def2);
    }

    pub fn find(&self, definition: Wire) -> Option<ThreeCC> {
        let swapped_definition = definition.swap_operands();
        for (code, def) in self.wires.iter() {
            if *def == definition || *def == swapped_definition {
                return Some(*code);
            }
        }

        None
    }

    pub fn evaluate(&mut self, code: ThreeCC) -> Option<bool> {
        let definition = some_or_return_none!(self.get(code));
        definition.evaluate(self, code)
    }

    pub fn get_number_width(&self, prefix: char) -> usize {
        for ((hi, lo), i) in two_digit_numbers().zip(0usize..) {
            if self.get(ThreeCC::new(prefix, hi, lo)).is_none() {
                return i;
            }
        }

        100
    }

    pub fn evaluate_number(&mut self, prefix: char) -> u128 {
        let mut result = 0u128;
        for ((hi, lo), i) in two_digit_numbers().zip(0usize..) {
            let bit = some_or_break!(self.evaluate(ThreeCC::new(prefix, hi, lo)));
            if bit {
                result |= 1 << i;
            }
        }
        result
    }

    fn fix_adder_stage(
        &mut self,
        i: usize,
        sum: ThreeCC,
        carry: ThreeCC,
        errors: &mut HashSet<ThreeCC>,
    ) {
        let x_in = make_number_bit_code('x', i);
        let y_in = make_number_bit_code('y', i);
        let mut in_xor = self.find(Wire::Xor(x_in, y_in)).unwrap();
        let mut in_and = self.find(Wire::And(x_in, y_in)).unwrap();
        if i > 0 {
            let mut sum_def = self.get(sum).unwrap();
            let mut carry_def = self.get(carry).unwrap();
            // sum = in_xor ^ carry_in
            // carry = in_and || (in_xor && carry_in)
            let carry_in;
            let carry_and;
            if sum_def.is_or() {
                errors.insert(sum);
                errors.insert(carry);
                self.swap(sum, carry);
                swap(&mut sum_def, &mut carry_def);
            }
            if sum_def.is_xor() {
                if let Some(sum_def_other) = sum_def.other_operand(in_xor) {
                    carry_in = sum_def_other;
                } else if let Some(sum_def_other) = sum_def.other_operand(in_and) {
                    errors.insert(in_and);
                    errors.insert(in_xor);
                    self.swap(in_and, in_xor);
                    swap(&mut in_and, &mut in_xor);
                    carry_in = sum_def_other;
                } else {
                    todo!();
                }
                if carry_def.is_or() {
                    carry_and = carry_def.other_operand(in_and).unwrap();
                } else {
                    todo!();
                }
            } else if let Wire::Or(carry_op1, carry_op2) = carry_def {
                let carry_op1_def = self.get(carry_op1).unwrap();
                let carry_op2_def = self.get(carry_op2).unwrap();
                if let Some(carry_other) = carry_def.other_operand(in_and) {
                    let carry_other_def = self.get(carry_other).unwrap();
                    if carry_other_def.is_xor() && sum_def.is_and() {
                        errors.insert(carry_other);
                        errors.insert(sum);
                        self.swap(carry_other, sum);
                        // sum_def = carry_other_def;
                        carry_and = carry_other;
                        carry_in = carry_other_def.other_operand(in_xor).unwrap();
                    } else {
                        todo!();
                    }
                } else {
                    if carry_op1_def.is_and() {
                        if let Some(carry_op1_other) = carry_op1_def.other_operand(in_xor) {
                            carry_and = carry_op1;
                            carry_in = carry_op1_other;
                        } else {
                            todo!();
                        }
                    } else if carry_op2_def.is_and() {
                        if let Some(carry_op2_other) = carry_op2_def.other_operand(in_xor) {
                            carry_and = carry_op2;
                            carry_in = carry_op2_other;
                        } else {
                            todo!();
                        }
                    } else {
                        todo!();
                    }
                    let expected_in_and = carry_def.other_operand(carry_and).unwrap();
                    let expected_in_and_def = self.get(expected_in_and).unwrap();
                    if expected_in_and_def.is_xor() && sum_def.is_and() {
                        errors.insert(expected_in_and);
                        errors.insert(sum);
                        self.swap(expected_in_and, sum);
                        // sum_def = expected_in_and_def;
                        // in_and = expected_in_and;
                    } else {
                        todo!();
                    }
                }
            } else {
                todo!();
            }
            let carry_and_def = self.get(carry_and).unwrap();
            if !carry_and_def.is_and() {
                todo!();
            }
            self.fix_adder_stage(i - 1, make_number_bit_code('z', i - 1), carry_in, errors);
        } else {
            // sum = in_xor
            // carry = in_and
            if sum != in_xor || carry != in_and {
                todo!();
            }
        }
    }

    pub fn fix_adder(&mut self) -> HashSet<ThreeCC> {
        let mut errors = HashSet::new();
        let z_width = self.get_number_width('z');
        self.fix_adder_stage(
            z_width - 2,
            make_number_bit_code('z', z_width - 2),
            make_number_bit_code('z', z_width - 1),
            &mut errors,
        );
        errors
    }
}
