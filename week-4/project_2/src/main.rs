use std::io;

fn main() {
    // Get the experience and age of the employee
    let mut input = String::new();
    println!("Enter the employee's experience in years:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let experience: u32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter the employee's age:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: u32 = input.trim().parse().expect("Please enter a valid number");

    // Determine the incentive based on experience and age
    let incentive = if experience >= 5 {
        // Experienced employees
        if age >= 40 {
            1_560_000 // N1,560,000 for employees 40 or older
        } else if age >= 30 {
            1_480_000 // N1,480,000 for employees between 30 and 39
        } else if age >= 28 {
            1_300_000 // N1,300,000 for employees below 28 with experience
        } else {
            1_300_000 // Default case for employees under 28 with experience
        }
    } else {
        // Inexperienced employees
        100_000 // N100,000 for employees with less than 5 years of experience
    };

    // Output the annual incentive
    println!("The annual incentive is: N{}", incentive);
}
