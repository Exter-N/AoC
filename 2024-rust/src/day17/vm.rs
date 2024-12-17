use std::{
    error::Error,
    ops::{BitAnd, BitXorAssign, Shr, ShrAssign},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn try_decode_operand(self, operand: u8) -> Result<Operand, Box<dyn Error>> {
        match self {
            Self::Adv | Self::Bst | Self::Out | Self::Bdv | Self::Cdv => operand.try_into(),
            Self::Bxl | Self::Jnz | Self::Bxc => Ok(Operand::Literal(operand)),
        }
    }

    fn execute<T, E>(self, vm: &mut VirtualMachine<'_, T>, operand: T) -> Result<(), E>
    where
        T: Copy
            + Default
            + PartialEq
            + BitAnd<Output = T>
            + BitXorAssign
            + Shr<Output = T>
            + ShrAssign
            + TryInto<usize, Error = E>
            + TryInto<u8, Error = E>
            + From<u8>,
    {
        match self {
            Instruction::Adv => {
                vm.a >>= operand;
            }
            Instruction::Bxl => {
                vm.b ^= operand;
            }
            Instruction::Bst => {
                vm.b = operand & T::from(7);
            }
            Instruction::Jnz => {
                if vm.a != Default::default() {
                    vm.ip = operand.try_into()?;
                }
            }
            Instruction::Bxc => {
                vm.b ^= vm.c;
            }
            Instruction::Out => {
                vm.output.push((operand & T::from(7)).try_into()?);
            }
            Instruction::Bdv => {
                vm.b = vm.a >> operand;
            }
            Instruction::Cdv => {
                vm.c = vm.a >> operand;
            }
        }
        Ok(())
    }
}

impl TryFrom<u8> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(Box::from("invalid instruction")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    A,
    B,
    C,
}

impl Operand {
    fn evaluate<T>(self, vm: &VirtualMachine<'_, T>) -> T
    where
        T: Copy + From<u8>,
    {
        match self {
            Self::Literal(value) => value.into(),
            Self::A => vm.a,
            Self::B => vm.b,
            Self::C => vm.c,
        }
    }
}

impl TryFrom<u8> for Operand {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 | 1 | 2 | 3 => Ok(Self::Literal(value)),
            4 => Ok(Self::A),
            5 => Ok(Self::B),
            6 => Ok(Self::C),
            _ => Err(Box::from("invalid combo operand")),
        }
    }
}

#[derive(Debug)]
pub struct VirtualMachine<'a, T> {
    a: T,
    b: T,
    c: T,
    ip: usize,
    program: &'a Vec<u8>,
    output: Vec<u8>,
}

impl<'a, T> VirtualMachine<'a, T> {
    pub fn new(a: T, b: T, c: T, program: &'a Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::with_capacity(program.len()),
        }
    }

    pub fn into_output(self) -> Vec<u8> {
        self.output
    }
}

impl<'a, T, E> VirtualMachine<'a, T>
where
    T: Copy
        + Default
        + PartialEq
        + BitAnd<Output = T>
        + BitXorAssign
        + Shr<Output = T>
        + ShrAssign
        + TryInto<usize, Error = E>
        + TryInto<u8, Error = E>
        + From<u8>,
    E: Error + 'static,
{
    pub fn try_execute_instruction(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.ip > self.program.len() - 2 {
            return Ok(false);
        }
        let instruction = Instruction::try_from(self.program[self.ip])?;
        let operand = instruction.try_decode_operand(self.program[self.ip + 1])?;
        self.ip += 2;
        instruction.execute(self, operand.evaluate(&self))?;
        Ok(true)
    }

    pub fn try_run_to_completion(&mut self) -> Result<(), Box<dyn Error>> {
        while self.try_execute_instruction()? {}
        Ok(())
    }
}
