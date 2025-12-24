pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let num_of_students = diagram.len() / 4;
    let mut student_index = 0; // before i learned about position, keeping the try_fold silliness 
    students.iter().try_fold(0, |i, &s| if s == student { student_index = i; None } else { Some(i+1) });

    let pretify = |p| {
         match p {
            Some('G') => {"grass"}
            Some('C') => {"clover"}
            Some('R') => {"radishes"}
            Some('V') => {"violets"}
            _ => {panic!("Unknown Plant");}
        }
    };

    let plants = vec![
        pretify(diagram.chars().nth(student_index*2)),
        pretify(diagram.chars().nth(student_index*2+1)),
        pretify(diagram.chars().nth(student_index*2+num_of_students*2+1)),
        pretify(diagram.chars().nth(student_index*2+num_of_students*2+1+1)),
    ];
    plants
} 