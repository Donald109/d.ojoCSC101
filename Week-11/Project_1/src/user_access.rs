use crate::output;

//Print the globacom dbase file content
pub fn admin_access() {
    println!("Admin Access Granted");
    output::print_file("globacom_dbase.sql".to_string());
}

//Print the project tb file content
pub fn project_manager_access() {
    println!("Project Manager Access Granted");
    output::print_file("project_tb.sql".to_string());
}

//Print the staff tb file content
pub fn employee_access() {
    println!("Employee Access Granted");
    output::print_file("staff_tb.sql".to_string());
}

//Print the customer tb file content
pub fn customer_access() {
    println!("Customer Access Granted");
    output::print_file("customer_tb.sql".to_string());
}

//Print the dataplan tb file content
pub fn vendor_access() {
    println!("Vendor Access Granted");
    output::print_file("dataplan_tb.sql".to_string());
}
