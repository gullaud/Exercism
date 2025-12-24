pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let num_of_students = diagram.len() / 4;
    let mut student_index = 0;
    students.iter().try_fold(0, |i, &s| if s == student { student_index = i; None } else { Some(i+1) });

    let mut plants = Vec::new();
    plants.push(diagram.chars().nth(student_index*2));
    plants.push(diagram.chars().nth(student_index*2+1));
    plants.push(diagram.chars().nth(student_index*2+num_of_students*2+1));
    plants.push(diagram.chars().nth(student_index*2+num_of_students*2+1+1));

    let mut res = Vec::new();
    for p in plants {
        match p {
            Some('G') => {res.push("grass");}
            Some('C') => {res.push("clover");}
            Some('R') => {res.push("radishes");}
            Some('V') => {res.push("violets");}
            _ => {panic!("Unknown Plant");}
        }
    }
    res
}
