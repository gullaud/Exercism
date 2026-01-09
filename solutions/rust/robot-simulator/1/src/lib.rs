// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    d : Direction,
    x : i32,
    y : i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {x,y,d}
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.d {
            Direction::North => {self.d = Direction::East}
            Direction::East => {self.d = Direction::South}
            Direction::South => {self.d = Direction::West}
            Direction::West => {self.d = Direction::North}
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.d {
            Direction::North => {self.d = Direction::West}
            Direction::East => {self.d = Direction::North}
            Direction::South => {self.d = Direction::East}
            Direction::West => {self.d = Direction::South}
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => { self.y += 1; }
            Direction::West => {self.x -= 1;}
            Direction::East => { self.x += 1}
            Direction::South => { self.y -=1; }
        }
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for inst in instructions.chars() {
            self = match inst {
                'A' => { self.advance() }
                'R' => { self.turn_right() }
                'L' => { self.turn_left() }
                _ => { panic!("Invalid Command") }
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
