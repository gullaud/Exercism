use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory{
    robot_names : Rc<RefCell<HashSet::<String>>>,
}

pub struct Robot {
    name : String,
    robot_names : Rc<RefCell<HashSet::<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory {robot_names : Rc::new(RefCell::new(HashSet::new()))}
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        Robot::new(rng, Rc::clone(&self.robot_names))
    }
}

impl Robot {

    pub fn new<R: Rng>(rng: &mut R, robot_names:Rc<RefCell<HashSet<String>>>) -> Self {
        let mut robot = Robot{name : String::new(), robot_names };
        robot.reset(rng);
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.robot_names.borrow_mut().remove(&self.name);
        loop {
            self.name.clear();
            self.name.push_str( &(0..2).map(|_| rng.random_range(b'A'..=b'Z') as char ).collect::<String>() );
            self.name.push_str( &(0..3).map(|_| rng.random_range(b'1'..=b'9') as char ).collect::<String>() );
            if ! self.robot_names.borrow().contains(self.name()) {
                break;
            }
        }
        self.robot_names.borrow_mut().insert(self.name().to_string());
    }
}
