use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Student {
    name: &'static str,
    matric_number: &'static str,
    department: &'static str,
    level: u32,
}

fn main() {
    // Define the student details
    let students = vec![
        Student {
            name: "Oluchi Mordi",
            matric_number: "ACC10211111",
            department: "Accounting",
            level: 300,
        },
        Student {
            name: "Adams Aliyu",
            matric_number: "ECO10110101",
            department: "Economics",
            level: 100,
        },
        Student {
            name: "Shania Bolade",
            matric_number: "CSC10328828",
            department: "Computer",
            level: 200,
        },
        Student {
            name: "Adekunle Gold",
            matric_number: "EEE11020202",
            department: "Electrical",
            level: 100,
        },
        Student {
            name: "Blanca Edemoh",
            matric_number: "MEE10202001",
            department: "Mechanical",
            level: 100,
        },
    ];

    // Display student details
    println!("PAU SMIS");
    println!("Student Name\tMatric Number\tDepartment\tLevel");
    for student in &students {
        println!("{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level);
    }

    // Serialize student data to CSV format
    let mut wtr = csv::Writer::from_writer(File::create("students.csv").expect("Failed to create file"));
    for student in &students {
        wtr.serialize(student).expect("Failed to write student data");
    }

    wtr.flush().expect("Failed to flush data to file");

    println!("Student details successfully saved to students.csv");
}
