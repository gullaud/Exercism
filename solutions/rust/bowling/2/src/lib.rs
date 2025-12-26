#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls : Vec<u16>,
    in_frame : bool,     // only for validating 2 rolls withing a frame <= 10 pins ...
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls : Vec::new(), in_frame : false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.in_frame && self.rolls.last().unwrap() + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.rolls.push(pins);
        self.in_frame = ! self.in_frame && pins < 10;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls.is_empty() {
            return None;
        }
        let mut current_roll = 0;
        let mut score = 0;
        for _ in 0..10 {
            if current_roll >= self.rolls.len() { return None; }            
            let first_roll = self.rolls[current_roll];
            match first_roll {
                r if r < 10 => { 
                    score += r; 
                    if current_roll + 1 >= self.rolls.len() { return None; }
                    let second_roll = self.rolls[current_roll+1];
                    score += second_roll;
                    if first_roll + second_roll == 10 { // spare
                        if current_roll + 2 >= self.rolls.len() { return None; }
                        score += self.rolls[current_roll + 2];
                    }
                    current_roll += 2;
                }
                10 => {
                    if current_roll + 2 >= self.rolls.len() { return None; }
                    score += 10 + self.rolls[current_roll+1] + self.rolls[current_roll+2];
                    current_roll += 1;
                }
                _ => panic!("Unexpected Pins")
            }
        }
        Some(score)
    }
}
