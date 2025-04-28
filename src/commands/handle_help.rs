use std::collections::HashMap;


pub fn handle_help() {
    let mut documentation = HashMap::new();

    let create_doc = "Either insert a password for an application or randomly generate one for the application\n
    \tUsage: ./password_manager create [application_name] [optional_password]\n
    \t --length [length]: Set the length of the randomly generated password\n
    \t -s --special: Include special characters in the password\n
    \t -f --force: Force overwrite of existing password";

    let read_doc = "Write the password for an application to clipboard or stdout if clipboard is unavailable\n
    \tUsage: ./password_manager read [application_name]";

    let update_doc = "Update the password for an application or randomly generate one for the application\n
    \tUsage: ./password_manager update [application_name] [optional_password]\n
    \t --length [length]: Set the length of the randomly generated password\n
    \t -s --special: Include special characters in the password";

    let delete_doc = "Delete the password for the application\n
    \tUsage: ./password_manager delete [application_name]";

    let list_doc = "List all the applications that have saved passwords; does not list the passwords themselves\n
    \tUsage: ./password_manager list";
    
    let change_master_doc = "Change the master password and re-encrypt all passwords using the new master password\n
    \tUsage: ./password_manager change-master [new_master_password]";

    let help_doc = "Display this help message\n
    \tUsage: ./password_manager help";

    documentation.insert("create", create_doc);
    documentation.insert("read", read_doc);
    documentation.insert("update", update_doc);
    documentation.insert("delete", delete_doc);
    documentation.insert("list", list_doc);
    documentation.insert("change-master", change_master_doc);
    documentation.insert("help", help_doc);

    println!("Available commands:");
    
    for key in documentation.keys() {
        println!("\n{}: {}", key, documentation.get(key).unwrap());
    }
}