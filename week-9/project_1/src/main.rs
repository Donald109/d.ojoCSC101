use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Drinks {
    lager: Vec<&'static str>,
    stout: Vec<&'static str>,
    non_alcoholic: Vec<&'static str>,
}

fn main() {
    // Define the categories and drinks
    let drinks = Drinks {
        lager: vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"],
        stout: vec!["Legend", "Turbo King", "Williams"],
        non_alcoholic: vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"],
    };

    // Serialize the data to JSON
    let json_data = serde_json::to_string_pretty(&drinks).expect("Failed to serialize data");

    // Write the JSON data to a file
    let mut file = File::create("drinks.json").expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");

    println!("Data successfully written to drinks.json");
}
