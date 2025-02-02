mod input;
mod output;
mod user_access;
mod user_type;

fn main() {
    println!("Welcome to the User Access Program");
    println!("Please enter your user type from the following options: \na.Admin\nb.Project Manager\nc.Employee\nd.Customer\ne.Vendor\n");

    get_user_content();
}

//Gets the input from the user and matches it to their appropriate user type
fn get_user_content() {
    let user_type =
        user_type::UserType::as_str(input::get_input("Enter your user type: ".to_string()));

    //Matches the user type to the file that is meant to be printed out
    user_type::match_user_type(user_type);
}
