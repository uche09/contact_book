use std::{io, process::exit};

// All structures definition are organized here at the top
// Functions declaration are organized at the bottom (after main())
struct Contact {
    name: String,
    phone: String,
    email: String
}




fn main() {
    // Storage for saving contacts
    let mut storage: Vec<Contact> = Vec::new();

    println!("\n\nCLI PHONE BOOK\n");
    

    loop {

        println!("\nSelect your action:");

        println!("1. Add a contact \n2. View all contacts \n3. Delete contact by name \n4. Exit");
    
        let mut action: String = String::new();
        io::stdin().read_line(&mut action).expect("Input failed");
    
        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a valid action number.\n");
                continue;  
            },
        };
    
        match action {
            1 => {
                let mut name: String = String::new();
                let mut phone: String = String::new();
                let mut email: String = String::new();

                println!("\nContact Name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Input failed");
                
                let name = name.trim().to_string();
                
                if !validate_name(&name) {
                    println!("\nName must consist of only alphabet and must not be empty\n");
                    continue;
                }



                println!("\nContact Number:");
                io::stdin()
                    .read_line(&mut phone)
                    .expect("Input failed");

                let phone = phone.trim().to_string();

                if !validate_number(&phone) {
                    println!("\nNumber must be 10 digits and above. Digits only\n");
                    continue;
                }


                println!("\nContact Email:");
                io::stdin()
                    .read_line(&mut email)
                    .expect("Input failed");

                let email = email.trim().to_string();

                if !validate_email(&email) {
                    println!("\nInvalid email address\n");
                    continue;
                }


                let new_contact = Contact {
                    name: name,
                    phone: phone,
                    email: email
                };

                // Check if number already exist in a contact.
                if storage.iter().find(|cont| cont.phone == new_contact.phone
                || cont.name == new_contact.name).is_some() {
                    println!("\nA contact already exist with this name or number. Please check name and number\n");
                    continue;
                }


                
                let consent: bool = loop {
                    println!("\nDo you want to save this contact? \n1. Yes \n2. No");

                    let mut consent: String = String::new();
                    io::stdin()
                        .read_line(&mut consent)
                        .expect("Input failed");

                    let consent: u8 = match consent.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nPlease enter a valid action number.");
                            continue;
                        },
                    };
                    
                    let mut feedback: bool = true;
                    if consent == 2 {feedback = false;}
                    break feedback
                };
                
                if !consent {println!("\nOperation aborted"); continue;}

                
                storage.push(new_contact);

                println!("\nContact saved!\n\n");
            }
    
            2 => {
                if storage.is_empty() {
                    println!("\nYou do not have any contact\n");
                    continue;
                }

                println!("\n\nYOUR CONTACTS\n");

                for contact in storage.iter() {
                    println!("Name {}\nNumber: {}\nEmail: {}\n",
                    contact.name, contact.phone, contact.email);
                    println!();
                }
    
            }
    
            3 => {
                println!("\nEnter Contact name to delete:");
                let mut rm_contact: String = String::new();

                io::stdin()
                    .read_line(&mut rm_contact)
                    .expect("Input failed");
                
                rm_contact = rm_contact.trim().to_string();

                let index = match storage.iter().position(|cont| cont.name == rm_contact) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };

                
                let consent: bool = loop {
                    println!("\nDo you want to DELETE this contact? \n1. Yes \n2. No\n");

                    let mut consent: String = String::new();
                    io::stdin()
                        .read_line(&mut consent)
                        .expect("Input failed");

                    let consent: u8 = match consent.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nPlease enter a valid action number.\n");
                            continue;
                        },
                    };
                    
                    let mut feedback: bool = true;
                    if consent == 2 {feedback = false;}
                    break feedback
                };
                
                if !consent {println!("\nOperation aborted"); continue;}

                // Delete contact
                let removed = storage.remove(index);

                println!("{} \n{} \n{} \n^^^^^^^^^ \nContact has been delected from Contact Book\n",
                    removed.name, removed.phone, removed.email);
                
            }
    
            4 => {
                println!("Bye!");
                exit(0);
            }

            _ => {
                println!("Please enter a valid action number.\n");
                continue;
            }
        }

    }

}





fn validate_name(name: &String) -> bool {
    // Must be alphabetic and non-empty
    // Name may contain spaces between alphabets
    name.chars().count() > 0
    &&
    name.chars().all(|c| c.is_alphabetic()|| c.is_whitespace())
}

fn validate_number(phone: &String) -> bool {
    // Must be at least 10 digits and digits only
    !(phone.chars().count() < 10)
    &&
    phone.chars().all(|c| c.is_ascii_digit())
}

fn validate_email(email: &String) -> bool {
    // Must contain '@' and '.'
    email.contains('@')
    &&
    email.contains('.')
}