use std::env::args;
use std::process::exit;
use colored::Colorize;
use chrono::{NaiveDate, Datelike, Local};

fn main() {
    println!("开始读取...");

    let id_card = get_id_card();

    // Verify the ID card
    let result = verify_id_card(&id_card);

    // Print the result
    print!("身份证格式：");
    if result {
        // If there is not any error,we print a green result
        println!("{}!", "正确".green());
    }
    else{
        // Or we print a red result
        println!("{}!", "错误".red());
        exit(-3);
    }

    // Print the info
    if !get_info(&id_card) {
        println!("身份证信息有问题，这是个{}","警告".yellow())
    }
    else {
        println!("身份证{}","正常!".green())
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
        return Some("身份证的长度是：".to_string()
                + length.to_string().as_str()
                + ",但我们期望："
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
    let id_card = args.nth(1).expect("请输入一个身份证号");

    id_card
}

fn get_info_from_id_card(id_card:&String)->bool{
    if let Some(got_age) = get_age(&id_card) {
        println!("年龄是：{}", got_age.to_string().bright_green());
    }
    else{
        return false
    }

    print!("性别是：");
    if get_sex(&id_card) {
        println!("{}","男".bright_green())
    }
    else{
        println!("{}","女".bright_green())
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
            println!("{}","解析日期失败".red());
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
            println!("{}","年龄是个负数".red());
        }

        AgeResult::Large=>{
            println!("{}","年龄似乎太大了".red());
        }
    }

    None
}

fn get_sex(id_card:&String)->bool{
    // We think true is a man in this function(and false is a woman)

    // Get No.17 char in the card string
    let magic_number = id_card.chars().nth(16).expect("序列数读取失败");

    // Turn magic number to u8
    let magic_number = magic_number.to_digit(10).expect("转换序列数失败") as u8;

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