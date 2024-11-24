use std::io;

fn main() {
    let mut user_input = String::new();
    let mut total_price: f32 = 0.0;
    let display_menu = "Here's a list of the foods we serve here: \nP = Poundo Yam/Edinkaiko Soup - N3200\nF = Fried Rice and Chicken - N3000\nA = Amala and Ewedu - N2500\nE = Eba and Egusi Soup - N2000\nW = White Rice and Stew - N2500";

    println!("WELCOME TO KUSORO'S CAFE!!");
    println!("{}", display_menu);
    println!("What would you like to buy: ");

    loop {
        let mut price: f32 = 0.0;
        io::stdin()
            .read_line(&mut user_input)
            .expect("Not a valid string");
        let mut fmt_user_input = user_input.trim().to_lowercase();

        if fmt_user_input == "s" {
            break;
        } else if fmt_user_input == "p" {
            price = 3200.0;
        } else if fmt_user_input == "f" {
            price = 3000.0;
        } else if fmt_user_input == "a" || fmt_user_input == "w" {
            price = 2500.0;
        } else if fmt_user_input == "e" {
            price = 2000.0;
        } else {
            println!("We don't sell that here.\n\n{}", display_menu);
            price = 0.0;
        }
        total_price += price;
        println!("Current Price: N{}", total_price);

        println!("What else would you like to buy (Press S to stop): ");

        user_input.clear();
    }

    if total_price > 10000.0 {
        total_price = total_price * 0.95;
    }

    println!("The price of your total purchase is: {}", total_price)
}
