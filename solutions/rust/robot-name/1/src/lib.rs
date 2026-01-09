use rand::Rng;
use std::collections::HashSet;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory{
    robot_names : HashSet::<String>,
}

pub struct Robot {
    name : String,
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory {robot_names : HashSet::new()}
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let mut robot = Robot::new(rng);
        while self.robot_names.contains(&robot.name().to_string()) {
            robot.reset(rng);
        }
        self.robot_names.insert(robot.name().to_string());
        robot
    }
}

impl Robot {

    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut robot = Robot{name : String::new() };
        robot.reset(rng);
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.name.clear();
        self.name.push_str( &(0..2).map(|_| rng.random_range(b'A'..=b'Z') as char ).collect::<String>() );
        self.name.push_str( &(0..3).map(|_| rng.random_range(b'1'..=b'9') as char ).collect::<String>() );
        dbg!(&self.name);
    }
}
