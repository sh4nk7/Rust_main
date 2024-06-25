fn get_phone_number(name: String, phone_book: &[(String, i64)]) -> Result<i64, String> {
    for (entry_name, &phone_number) in phone_book {
        if entry_name == &name {
            return Ok(phone_number);
        }
    }
    Err(format!("Name '{}' not found in the phone book.", name))
}

fn main() {
    let phone_book = vec![
        (String::from("Alice"), 1234567890),
        (String::from("Bob"), 2345678901),
        (String::from("Carol"), 3456789012),
    ];

    match get_phone_number(String::from("Alice"), &phone_book) {
        Ok(number) => println!("Alice's phone number is {}", number),
        Err(e) => println!("Error: {}", e),
    }

    match get_phone_number(String::from("Dave"), &phone_book) {
        Ok(number) => println!("Dave's phone number is {}", number),
        Err(e) => println!("Error: {}", e),
    }
}
