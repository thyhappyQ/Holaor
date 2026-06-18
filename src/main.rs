use std::env::args;
use colored::Colorize;

fn main() {
    println!("Start to verify your ID card");

    let id_card = get_id_card();

    // Print the result
    print!("Your ID card is ");
    if verify_id_card(&id_card) {
        // If there is not any error,we print a green result
        print!("{}", "legal".green());
    }
    else{
        // Or we print a red result
        print!("{}", "not correct".red());
    }
    println!("!");
}

const STANDARD_ID_CARD_LENGTH:u8 = 18;

fn verify_id_card(id_card:&String)->bool{
    // Pack all the functions up

    // Check the length of the ID card
    if let Some(error_info) = verify_length(&id_card) {
        println!("{}", error_info);
        return false;
    }

    // If everything is OK,surely we return true
    true
}

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

fn get_info_from_id_card(id_card:&String)->Option<String>{
    None
}