use std::default;

use crate::user_access;

//enum for different user types
pub enum UserType {
    Admin,
    ProjectManager,
    Employee,
    Customer,
    Vendor,
}

//converts a user type into a string
impl UserType {
    //Match the appropriate user enums to their string counter parts
    pub fn as_str(user_type: String) -> UserType {
        match user_type.to_lowercase().as_str() {
            "admin" => UserType::Admin,
            "project manager" => UserType::ProjectManager,
            "employee" => UserType::Employee,
            "customer" => UserType::Customer,
            "vendor" => UserType::Vendor,
            default => {
                println!("Invalid User Type, Please enter a valid user type.");
                UserType::Customer
            }
        }
    }
}

//matches the user type to their access level
pub fn match_user_type(user_type: UserType) {
    match user_type {
        UserType::Admin => user_access::admin_access(),
        UserType::ProjectManager => user_access::project_manager_access(),
        UserType::Employee => user_access::employee_access(),
        UserType::Customer => user_access::customer_access(),
        UserType::Vendor => user_access::vendor_access(),
    };
}
