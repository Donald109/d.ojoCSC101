use std::{io, ptr::null};

fn main() {
    staff_level_logic();
}

fn staff_position() -> Vec<(&'static str, &'static str)> {
    let staff_position = vec![
        ("APS 1-2", "Intern"),
        ("APS 1-2", "Paralegal"),
        ("APS 1-2", "Placement"),
        //
        ("APS 3-5", "Administrator"),
        ("APS 3-5", "Research Assitant"),
        ("APS 3-5", "Junior Associate"),
        ("APS 3-5", "Classroom Teacher"),
        //
        ("APS 5-8", "Senior Administator"),
        ("APS 5-8", "PhD Candidate"),
        ("APS 5-8", "Associate"),
        ("APS 5-8", "Senior Teacher"),
        //
        ("EL1 8-10", "Office Manager"),
        ("EL1 8-10", "Post Doc Researcher"),
        ("EL1 8-10", "Senior Associate 1-2"),
        ("EL1 8-10", "Leading Teacher"),
        //
        ("EL1 10-13", "Director"),
        ("EL1 10-13", "Senior Lecturer"),
        ("EL1 10-13", "Senior Associate 3-4"),
        ("EL1 10-13", "Deputy Principal"),
        //
        ("SES", "CEO"),
        ("SES", "Dean"),
        ("SES", "Partner"),
        ("SES", "Principal"),
    ];

    return staff_position;
}

fn staff_level_logic() {
    let mut input = String::new();

    //Initalizes the staff position function
    let staff_position = staff_position();

    println!("What is your staff role?(\"Intern\", e.t.c). Note: Your input is case sensitive ");
    io::stdin().read_line(&mut input).expect("");
    let user_position = input.trim();

    //This is basically the whole logic here. Finds the user_input in the vector and assigns it to a new vector
    let aps_lvl  //Note: This is a vector
    = staff_position
        .iter()
        .find(|&&(aps, position)| position == user_position);

    //Gets the aps lvl from the vector and prints it to the user
    match aps_lvl {
        Some((aps, _)) => println!(
            "The APS level is {} for staff position 
            {}",
            aps, user_position
        ),
        None => println!("Position not found"),
    }
}
