use std::env::args;

fn main() {
    println!("Hello, world!");
}

const STANDARD_ID_CARD_LENGTH:u8 = 18;

fn verify_length(source:&String)->Option<String>{
    // Turn the source string to the chars
    let chars = source.chars();

    // Get the length of the ID card
    let length = chars.count();

    // Check if the length of the ID card is correct
    if STANDARD_ID_CARD_LENGTH != length as u8 {
        return Some("The length of this ID card is ".to_string()
                + length.to_string().as_str()
                + ",but we except "
                + STANDARD_ID_CARD_LENGTH.to_string().as_str());
    }

    // If everything is OK,we return "None" to show that there is not any error
    None
}

fn get_id_card()->String{
    // Get args
    let mut args = args();

    // We only peek the first arg that user entered
    let id_card = args.nth(1).expect("Please enter a ID card");

    id_card
}