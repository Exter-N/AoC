use std::error::Error;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Gesture {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Gesture {
    pub fn play_against(self, theirs: Self) -> Outcome {
        Outcome::try_from((4u8 + (self as u8) - (theirs as u8)) % 3u8).unwrap()
    }
}

impl From<Gesture> for u8 {
    fn from(value: Gesture) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Gesture {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Rock),
            1u8 => Ok(Self::Paper),
            2u8 => Ok(Self::Scissors),
            _ => Err(Box::from("invalid gesture id")),
        }
    }
}

impl TryFrom<char> for Gesture {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(Box::from("invalid gesture character")),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Outcome {
    Lose = 0,
    Draw = 1,
    Win = 2,
}

impl Outcome {
    pub fn what_must_we_play(self, theirs: Gesture) -> Gesture {
        Gesture::try_from((2u8 + (self as u8) + (theirs as u8)) % 3u8).unwrap()
    }
}

impl From<Outcome> for u8 {
    fn from(value: Outcome) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Outcome {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Lose),
            1u8 => Ok(Self::Draw),
            2u8 => Ok(Self::Win),
            _ => Err(Box::from("invalid outcome id")),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(Box::from("invalid outcome character")),
        }
    }
}
