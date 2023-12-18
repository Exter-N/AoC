use std::ops::Range;

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(u32),
    Symbol(char),
    None,
}

impl Token {
    fn is_none(self) -> bool {
        matches!(self, Token::None)
    }
    fn new(ch: char) -> Self {
        if ch.is_digit(10) {
            Self::Number((ch as u32) - ('0' as u32))
        } else if ch == '.' {
            Self::None
        } else {
            Self::Symbol(ch)
        }
    }
    fn append(self, ch: char) -> Self {
        match self {
            Self::Number(num) => {
                if ch.is_digit(10) {
                    Self::Number(num * 10 + (ch as u32) - ('0' as u32))
                } else {
                    Self::None
                }
            }
            _ => Self::None,
        }
    }
}

#[derive(Debug)]
struct Number {
    position: Range<usize>,
    value: u32,
}

impl Number {
    fn new(position: Range<usize>, value: u32) -> Self {
        Self { position, value }
    }
}

#[derive(Debug)]
struct Symbol {
    position: usize,
    value: char,
    numbers_around: usize,
    product_around: u32,
}

impl Symbol {
    fn new(position: usize, value: char) -> Self {
        Self {
            position,
            value,
            numbers_around: 0,
            product_around: 1,
        }
    }
    fn add_number_around(&mut self, number: u32) {
        self.numbers_around += 1;
        self.product_around *= number;
    }
}

fn symbols_around_range(symbols: &[Symbol], position: &Range<usize>) -> Range<usize> {
    let start =
        match symbols.binary_search_by_key(&position.start.saturating_sub(1), |sym| sym.position) {
            Ok(i) => i,
            Err(i) => i,
        };
    let end = match symbols.binary_search_by_key(&position.end, |sym| sym.position) {
        Ok(i) => i + 1,
        Err(i) => i,
    };
    start..end
}

fn symbols_around<'a>(symbols: &'a [Symbol], position: &Range<usize>) -> &'a [Symbol] {
    let range = symbols_around_range(&symbols, position);
    if range.start > range.end {
        &[]
    } else {
        &symbols[range]
    }
}

fn symbols_around_mut<'a>(symbols: &'a mut [Symbol], position: &Range<usize>) -> &'a mut [Symbol] {
    let range = symbols_around_range(&symbols, position);
    if range.start > range.end {
        &mut []
    } else {
        &mut symbols[range]
    }
}

#[derive(Default, Debug)]
pub struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Line {
    pub fn parse(line: &str) -> Self {
        let mut result: Self = Default::default();
        let mut current_token = Token::None;
        let mut current_start = None;
        for (ch, i) in line.chars().chain(".".chars()).zip(0usize..) {
            if current_start.is_some() {
                let token = current_token.append(ch);
                if token.is_none() {
                    match current_token {
                        Token::Number(num) => {
                            result
                                .numbers
                                .push(Number::new(current_start.unwrap()..i, num));
                        }
                        Token::Symbol(ch) => {
                            result.symbols.push(Symbol::new(current_start.unwrap(), ch));
                        }
                        _ => {}
                    }
                    current_start = None;
                }
                current_token = token;
            }
            if current_start.is_none() {
                current_token = Token::new(ch);
                if !current_token.is_none() {
                    current_start = Some(i);
                }
            }
        }
        result.inner_update_gears();
        result
    }
    fn semi_sum_of_parts(&self, other: &Self) -> u32 {
        let mut result = 0;
        for number in &other.numbers {
            let symbols = symbols_around(&self.symbols, &number.position);
            if symbols.len() > 0 {
                result += number.value;
            }
        }
        result
    }
    pub fn inner_sum_of_parts(&self) -> u32 {
        self.semi_sum_of_parts(self)
    }
    pub fn outer_sum_of_parts(&self, other: &Self) -> u32 {
        self.semi_sum_of_parts(other) + other.semi_sum_of_parts(self)
    }
    fn inner_update_gears(&mut self) {
        for number in &self.numbers {
            for sym in symbols_around_mut(&mut self.symbols, &number.position) {
                sym.add_number_around(number.value);
            }
        }
    }
    pub fn update_gears(&mut self, other: &Self) {
        for number in &other.numbers {
            for sym in symbols_around_mut(&mut self.symbols, &number.position) {
                sym.add_number_around(number.value);
            }
        }
    }
    pub fn into_gear_ratios(self) -> u32 {
        let mut result = 0;
        for symbol in &self.symbols {
            if symbol.value == '*' && symbol.numbers_around == 2 {
                result += symbol.product_around;
            }
        }
        result
    }
}
