// Define a struct to represent the laptop with brand and cost
struct Laptop {
    brand: String,
    cost_per_unit: u32,
}

impl Laptop {
    // Method to calculate total cost based on quantity purchased
    fn total_cost(&self, quantity: u32) -> u32 {
        self.cost_per_unit * quantity
    }
}

fn main() {
    // Creating instances of the Laptop struct for each brand and price
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        cost_per_unit: 650_000,
    };
    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        cost_per_unit: 755_000,
    };
    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        cost_per_unit: 550_000,
    };
    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        cost_per_unit: 850_000,
    };

    // Quantity of laptops purchased from each brand
    let quantity = 3;

    // Calculate the total cost for each brand and sum them up
    let total_hp_cost = hp_laptop.total_cost(quantity);
    let total_ibm_cost = ibm_laptop.total_cost(quantity);
    let total_toshiba_cost = toshiba_laptop.total_cost(quantity);
    let total_dell_cost = dell_laptop.total_cost(quantity);

    let total_cost = total_hp_cost + total_ibm_cost + total_toshiba_cost + total_dell_cost;

    // Print the result
    println!("Total cost for 3 laptops from each brand: {:?}", total_cost);
}

