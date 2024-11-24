use std::io;
use chrono::{NaiveDate, Datelike}; // Importing for date handling

// Define a struct to store patient details
struct Patient {
    name: String,
    dob: NaiveDate,
    email: String,
    phone: String,
    num_siblings: u32,
    num_children: u32,
    diagnosis: String,
    village: String,
}

// Function to calculate the discount and the total due amount
fn calculate_discount(patient: &Patient) -> f64 {
    // Define base charges based on diagnosis
    let base_charge = match patient.diagnosis.as_str() {
        "Alzheimer" => 1200000.0,
        "Arrythmia" => 550000.0,
        "CKD" => 1500000.0,
        "Diabetes" => 800000.0,
        "Arthritis" => 450000.0,
        _ => 0.0, // Default to 0 if diagnosis is unknown
    };

    let current_year = 2024;
    let age = current_year - patient.dob.year();
    let mut discount = 0.0;

    // Check conditions to apply discounts
    if patient.diagnosis == "Alzheimer" && age > 50 && patient.num_children > 4 && patient.village == "Akpabom" {
        discount = 0.20; // 20% discount for Alzheimer
    } else if patient.diagnosis == "Arrythmia" && age == 30 && patient.num_siblings > 4 && patient.village == "Ngbauji" {
        discount = 0.05; // 5% discount for Arrythmia
    } else if patient.diagnosis == "CKD" && age > 40 && patient.num_children > 3 && patient.num_siblings > 3 && patient.village == "Atabrikang" {
        discount = 0.15; // 15% discount for CKD
    } else if patient.diagnosis == "Diabetes" && (28..45).contains(&age) && (2..=4).contains(&patient.num_children) && patient.village == "Okorobilom" {
        discount = 0.10; // 10% discount for Diabetes
    } else if patient.diagnosis == "Arthritis" && age > 58 && patient.num_siblings > 5 && patient.num_children > 5 && patient.village == "Emeremen" {
        discount = 0.10; // 10% discount for Arthritis
    }

    // Calculate total after applying discount
    let discount_amount = base_charge * discount;
    base_charge - discount_amount
}

// Function to input patient details from the user
fn get_patient_info() -> Patient {
    // Define a mutable String to hold each input
    let mut input = String::new();

    println!("Enter patient's name:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim().to_string();

    input.clear(); // Clear input for reuse
    println!("Enter patient's date of birth (YYYY-MM-DD):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dob = NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d").expect("Invalid date format");

    input.clear();
    println!("Enter patient's email:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let email = input.trim().to_string();

    input.clear();
    println!("Enter patient's phone number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let phone = input.trim().to_string();

    input.clear();
    println!("Enter number of siblings:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_siblings: u32 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter number of children:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_children: u32 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter medical diagnosis:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let diagnosis = input.trim().to_string();

    input.clear();
    println!("Enter village of residence:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let village = input.trim().to_string();

    // Create and return a Patient struct
    Patient {
        name,
        dob,
        email,
        phone,
        num_siblings,
        num_children,
        diagnosis,
        village,
    }
}

// Function to display patient details and calculated amount
fn display_patient_info(patient: &Patient, total_amount: f64) {
    println!("\nPatient Information:");
    println!("Name: {}", patient.name);
    println!("Date of Birth: {}", patient.dob);
    println!("Email: {}", patient.email);
    println!("Phone Number: {}", patient.phone);
    println!("Number of Siblings: {}", patient.num_siblings);
    println!("Number of Children: {}", patient.num_children);
    println!("Medical Diagnosis: {}", patient.diagnosis);
    println!("Village of Residence: {}", patient.village);
    println!("Total Amount Due: N{}", total_amount);
}

// Main function to process multiple patients
fn main() {
    let mut patients_count = 0; // Counter for the daily patient limit
    let max_patients = 100; // Limit of 100 patients per day

    while patients_count < max_patients {
        println!("\nPatient {}:", patients_count + 1);
        let patient = get_patient_info(); // Get patient information
        let total_amount = calculate_discount(&patient); // Calculate the total with discount if applicable

        // Display the calculated information
        display_patient_info(&patient, total_amount);

        patients_count += 1; // Increment patient count

        // Check if daily limit is reached
        if patients_count >= max_patients {
            println!("\nDaily limit of 100 patients reached.");
            break;
        }
    }
}