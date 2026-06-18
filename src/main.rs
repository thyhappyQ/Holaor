use std::env::args;
use std::process::exit;
use colored::Colorize;
use chrono::{NaiveDate, Datelike, Local};

fn main() {
    println!("Start to verify your ID card");

    let id_card = get_id_card();

    // Verify the ID card
    let result = verify_id_card(&id_card);

    // Print the result
    print!("The form of your ID card is ");
    if result {
        // If there is not any error,we print a green result
        print!("{}", "legal".green());
    }
    else{
        // Or we print a red result
        print!("{}!", "not correct".red());
        exit(-3);
    }
    println!("!");

    // Print the info
    if !get_info(&id_card) {
        println!("Your ID card has a few {}","problems".yellow())
    }
    else {
        println!("Your ID card is {}","OK!".green())
    }
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

fn get_info(id_card:&String)->bool{
    if !get_info_from_id_card(&id_card) {
        return false
    }

    true
}

fn get_id_card()->String{
    // Get args
    let mut args = args();

    // We only peek the first arg that user entered
    let id_card = args.nth(1).expect("Please enter a ID card");

    id_card
}

fn get_info_from_id_card(id_card:&String)->bool{
    if let Some(got_age) = get_age(&id_card) {
        println!("The age is {}", got_age.to_string().bright_green());
    }
    else{
        return false
    }

    print!("The sex is ");
    if get_sex(&id_card) {
        println!("{}","man".bright_green())
    }
    else{
        println!("{}","woman".bright_green())
    }

    true
}

fn get_age(id_card:&String)->Option<i16>{
    // Extract part of the string
    let birthday_str = &id_card[6..14];

    // Get the birthday of the holder of the ID card
    let birthday = NaiveDate::parse_from_str(&birthday_str, "%Y%m%d");

    // Do check
    let birthday = match birthday {
        Ok(date) => date,
        Err(_) => {
            println!("{}","Failed to parse date".red());
            exit(-1);
        }
    };

    // Get local time
    let local_time = Local::now().date_naive();

    // Get age
    let age = local_time.year() - birthday.year();
    let age = age as i16; // Turn it into u8

    // Verify the age to check if it is in a normal range
    let result = varify_age(&age);

    // If it is OK,we return the age,or we will return None and print the error
    match result{
        AgeResult::Normal=>{
            return Some(age);
        }

        AgeResult::Few=>{
            println!("{}","The age is too few".red());
        }

        AgeResult::Large=>{
            println!("{}","The age is too large".red());
        }
    }

    None
}

fn get_sex(id_card:&String)->bool{
    // We think true is a man in this function(and false is a woman)

    // Get No.17 char in the card string
    let magic_number = id_card.chars().nth(16).expect("Magic number is not here");

    // Turn magic number to u8
    let magic_number = magic_number.to_digit(10).expect("Failed to parse magic number to u8") as u8;

    // Check if it is an odd
    if magic_number % 2 != 0 {
        // It shows the magic number is an odd,so we return "man"
        return true
    }

    // Or we return "woman"
    false
}

const NORMAL_MAX_AGE:i16 = 130;
const NORMAL_MIN_AGE:i16 = 0;

enum AgeResult{
    Large = 0,
    Normal = 1,
    Few = 2
}

fn varify_age(age:&i16)->AgeResult{
    // Check if the age is in a normal range

    // Check if it is too big
    if *age >= NORMAL_MAX_AGE{
        return AgeResult::Large;
    }

    // Check if it is too few
    if *age < NORMAL_MIN_AGE{
        return AgeResult::Few;
    }

    AgeResult::Normal
}