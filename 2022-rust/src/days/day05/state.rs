use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;

#[derive(Default)]
pub struct ShipWithCrane {
    pub reverse_on_move: bool,
    pub verbose: bool,
    stacks: Vec<VecDeque<char>>,
}

impl ShipWithCrane {
    pub fn new(reverse_on_move: bool, verbose: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            reverse_on_move,
            verbose,
            ..Default::default()
        }))
    }

    pub fn add_bottom_layer(&mut self, crates: Vec<char>) {
        for (cr, i) in crates.into_iter().zip(0usize..) {
            if i >= self.stacks.len() {
                self.stacks.push(VecDeque::new());
            }
            if ' ' != cr {
                self.stacks[i].push_front(cr);
            }
        }
    }
    pub fn move_top(&mut self, num: usize, from: usize, to: usize) -> Result<(), Box<dyn Error>> {
        if self.reverse_on_move {
            for _ in 0..num {
                if let Some(cr) = self.stacks[from].pop_back() {
                    self.stacks[to].push_back(cr);
                } else {
                    return Err(Box::from("moving too many elements"));
                }
            }
        } else {
            let from_len = self.stacks[from].len();
            if num > from_len {
                return Err(Box::from("moving too many elements"));
            }
            let mut top = self.stacks[from].split_off(from_len - num);
            self.stacks[to].append(&mut top);
        };

        Ok(())
    }
    pub fn tops(&self) -> Result<String, Box<dyn Error>> {
        let mut tops = String::with_capacity(self.stacks.len());
        for stack in &self.stacks {
            if let Some(cr) = stack.back() {
                tops.push(*cr);
            } else {
                return Err(Box::from("empty stack"));
            }
        }

        Ok(tops)
    }

    pub fn dump(&self) {
        let mut max_height = 0;
        for stack in &self.stacks {
            if stack.len() > max_height {
                max_height = stack.len()
            }
        }
        for i in (0..max_height).rev() {
            let mut j: usize = 0;
            for stack in &self.stacks {
                if j > 0 {
                    print!(" ");
                }
                if i >= stack.len() {
                    print!("   ");
                } else if i > 0 {
                    print!("[{}]", stack[i]);
                } else {
                    print!(" {} ", stack[i]);
                }
                j = j + 1;
            }
            println!();
        }
    }
}
