struct Candidate {
    name: String,
    years_of_experience: u32,
}

fn main() {
    let candidates = [
        Candidate { name: String::from("Alice"), years_of_experience: 5 },
        Candidate { name: String::from("Bob"), years_of_experience: 8 },
        Candidate { name: String::from("Charlie"), years_of_experience: 3 },
    ];

    let mut most_experienced = &candidates[0];

    for candidate in &candidates {
        if candidate.years_of_experience > most_experienced.years_of_experience {
            most_experienced = candidate;
        }
    }

    println!(
        "The most experienced candidate is {} with {} years of experience.",
        most_experienced.name, most_experienced.years_of_experience
    );
}

