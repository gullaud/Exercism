use std::collections::BTreeMap;

pub struct School {
    roaster : BTreeMap <u32, BTreeMap<String, ()>>,
}

impl School {
    pub fn new() -> School {
        Self {
            roaster : BTreeMap::new(),
        }
    }

    fn connu(&self, student:&str) -> bool {
        for class_roaster in self.roaster.values() {
            if class_roaster.contains_key(student) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.connu(student) {
            return;
        }
        let class = self.roaster.entry(grade).or_default();
        class.insert(student.to_string(), ());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roaster.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(grade_list) = self.roaster.get(&grade) {
             return grade_list.keys().cloned().collect();
        }
        vec![]
    }
}
