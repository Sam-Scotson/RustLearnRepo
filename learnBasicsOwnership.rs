fn main() {
    // Create an empty vector and store student names
    let mut student_names: Vec<String> = Vec::new();
    let name1 = String::from("Alice");
    let name2 = String::from("Bob");

    student_names.push(name1);
    student_names.push(name2);

    // Attempt to access name1 after it has been moved
    // Uncommenting the following line should result in a compilation error:
    // println!("Name1: {}", name1);

    // Access and print student names from the vector
    for name in &student_names {
        println!("Student Name: {}", name);
    }

    // Modify a student name
    let mut third_name = String::from("Charlie");
    student_names.push(third_name);

    // Attempt to access third_name after it has been moved
    // Uncommenting the following line should result in a compilation error:
    // println!("Third Name: {}", third_name);
}
