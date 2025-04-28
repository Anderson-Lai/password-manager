use std::collections::HashMap;



pub fn handle_help() {
    let mut documentation = HashMap::new();

    let create_doc = "create [application_name] [optional_password]\n
    \t --length [length]: Set the length of the randomly generated password\n
    \t -s --special: Include special characters in the password\n
    \t -f --force: Force overwrite of existing password";

    documentation.insert("create", create_doc);
    documentation.insert("read", "Read a password");
    documentation.insert("update", "Update a password");
    documentation.insert("delete", "Delete a password");
    documentation.insert("list", "List all passwords");
    documentation.insert("change-master", "Change the master password");
    documentation.insert("help", "Display this help message");

    println!("Usage: password-manager [command] [arguments]");
    println!("Available commands:");
    
    for key in documentation.keys() {
        println!("\n{}: {}", key, documentation.get(key).unwrap());
    }
}