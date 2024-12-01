use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number between 1 and 6.");
                continue;
            }
        };

        match choice {
            1 => {
                // Area of Trapezium
                println!("Enter height:");
                let height = read_input();
                println!("Enter base1:");
                let base1 = read_input();
                println!("Enter base2:");
                let base2 = read_input();
                let area = height / 2.0 * (base1 + base2);
                println!("Area of Trapezium: {:.2}", area);
            }
            2 => {
                // Area of Rhombus
                println!("Enter diagonal1:");
                let diagonal1 = read_input();
                println!("Enter diagonal2:");
                let diagonal2 = read_input();
                let area = 0.5 * diagonal1 * diagonal2;
                println!("Area of Rhombus: {:.2}", area);
            }
            3 => {
                // Area of Parallelogram
                println!("Enter base:");
                let base = read_input();
                println!("Enter altitude:");
                let altitude = read_input();
                let area = base * altitude;
                println!("Area of Parallelogram: {:.2}", area);
            }
            4 => {
                // Area of Cube
                println!("Enter length of the side:");
                let side = read_input();
                let area = 6.0 * side * side;
                println!("Area of Cube: {:.2}", area);
            }
            5 => {
                // Volume of Cylinder
                println!("Enter radius:");
                let radius = read_input();
                println!("Enter height:");
                let height = read_input();
                let volume = std::f64::consts::PI * radius * radius * height;
                println!("Volume of Cylinder: {:.2}", volume);
            }
            6 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 6."),
        }
    }
}

fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Please enter a valid number")
}
