use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Commissioner {
    sn: usize,
    name: &'static str,
    ministry: &'static str,
    geopolitical_zone: &'static str,
}

fn main() {
    // Separate datasets
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merge datasets into a unified structure
    let mut merged_data = Vec::new();
    for i in 0..commissioners.len() {
        merged_data.push(Commissioner {
            sn: i + 1,
            name: commissioners[i],
            ministry: ministries[i],
            geopolitical_zone: geopolitical_zones[i],
        });
    }

    // Display the merged data
    println!("S/N\tName\t\t\t\tMinistry\t\t\tGeopolitical Zone");
    for commissioner in &merged_data {
        println!(
            "{}\t{}\t{}\t{}",
            commissioner.sn, commissioner.name, commissioner.ministry, commissioner.geopolitical_zone
        );
    }

    // Save to a JSON file
    let file = File::create("merged_data.json").expect("Failed to create file");
    serde_json::to_writer_pretty(file, &merged_data).expect("Failed to write data to file");

    println!("\nData successfully saved to merged_data.json");
}
