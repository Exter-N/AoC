use std::{cell::RefCell, collections::HashMap, error::Error, ops::Range};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandlerOnce,
};

struct Day19Start();

fn insert_pattern(
    patterns: &mut HashMap<char, Vec<Range<usize>>>,
    rest: &str,
    start: usize,
    len: usize,
) {
    let head = rest.chars().next().unwrap();
    let pats = patterns.entry(head).or_insert_with(|| Vec::new());
    pats.push(start..(start + len));
}

impl LineStreamHandlerOnce for Day19Start {
    fn update(
        self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let mut patterns = HashMap::new();
        let mut start = 0usize;
        let mut rest = line;
        while let Some(separator) = rest.find(", ") {
            if separator > 0 {
                insert_pattern(&mut patterns, rest, start, separator);
            }
            start += separator + 2;
            rest = &rest[(separator + 2)..];
        }
        insert_pattern(&mut patterns, rest, start, rest.len());
        Ok(Box::new(Day19::new(line.into(), patterns)))
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

#[derive(Debug)]
struct Day19 {
    line: String,
    patterns: HashMap<char, Vec<Range<usize>>>,
    matching_inputs: usize,
    matches: usize,
    match_cache: RefCell<HashMap<String, usize>>,
}

impl Day19 {
    fn new(line: String, patterns: HashMap<char, Vec<Range<usize>>>) -> Day19 {
        Self {
            line,
            patterns,
            matching_inputs: 0,
            matches: 0,
            match_cache: RefCell::new(HashMap::new()),
        }
    }

    fn count_matches(&self, input: &str) -> usize {
        if input.is_empty() {
            return 1;
        }

        let head = input.chars().next().unwrap();
        if let Some(pats) = self.patterns.get(&head) {
            if let Some(cached) = self.match_cache.borrow().get(input) {
                return *cached;
            }
            let mut matches = 0;
            for pat in pats {
                let len = pat.len();
                if len > input.len() {
                    continue;
                }
                if &input[0..len] == &self.line[pat.clone()] {
                    matches += self.count_matches(&input[len..]);
                }
            }
            self.match_cache
                .borrow_mut()
                .insert(String::from(input), matches);
            matches
        } else {
            0
        }
    }
}

impl LineStreamHandlerOnce for Day19 {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if !line.is_empty() {
            let matches = self.count_matches(line);
            if matches > 0 {
                self.matching_inputs += 1;
            }
            self.matches += matches;
        }
        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Possible designs: {}",
            SILVER_ANSI, self.matching_inputs
        );
        println!("[{}] Different ways:   {}", GOLD_ANSI, self.matches);
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(19, "Linen Layout", Day19Start()))
}
