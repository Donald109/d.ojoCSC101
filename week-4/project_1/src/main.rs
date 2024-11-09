use std::io;

fn main() {
    // Input coefficients a, b, and c
    let mut input = String::new();
    println!("Enter coefficient a:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter coefficient b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter coefficient c:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    // Calculate the discriminant (Δ = b² - 4ac)
    let discriminant = b * b - 4.0 * a * c;

    // Check the nature of the roots based on the discriminant
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is exactly one real root: {:.2}", root);
    } else {
        println!("There are no real roots.");
    }
}

