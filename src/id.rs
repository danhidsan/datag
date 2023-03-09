use rand::{self, Rng};
use regex::Regex;

const NIF_RAND_START: i32 = 10000000;
const NIF_RAND_END: i32 = 100000000;

const NIE_RAND_START: i32 = 1000000;
const NIE_RAND_END: i32 = 10000000;

const CIF_RAND_START: i32 = 10000;
const CIF_RAND_END: i32 = 100000;

fn calculate_letter(number: &i32) -> char {
    let letters = "TRWAGMYFPDXBNJZSQVHLCKE".to_string();
    let letter = letters.chars().nth((number % 23).try_into().unwrap()).unwrap();
    
    letter
}

pub fn generate_nif() -> String {
    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(NIF_RAND_START..NIF_RAND_END);
    let nif_letter = calculate_letter(&rand_number);

    let res_nif = format!("{}{}", rand_number, nif_letter);

    res_nif
}

pub fn generate_nie() -> String {
    let mut rng = rand::thread_rng();
    let initial_random = rng.gen_range(0..3);

    let nie_rand_number = rng.gen_range(NIE_RAND_START..NIE_RAND_END);

    let complete_number: i32 = format!("{}{}", initial_random, nie_rand_number).parse().unwrap();

    let end_letter = calculate_letter(&complete_number);


    let res_nie: String;
    
    if initial_random == 0 {
        res_nie = format!("X{}{}", nie_rand_number, end_letter);
    } else if initial_random == 1 {
        res_nie = format!("Y{}{}", nie_rand_number, end_letter);
    } else {
        res_nie = format!("Z{}{}", nie_rand_number, end_letter);
    }

    res_nie
}

fn cif_first_letter() -> char {
    let letters = "ABCDEFGHJNPQRSUVW".to_string();

    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(0..17);
    let letter = letters.chars().nth((rand_number).try_into().unwrap()).unwrap();
    
    letter
}


fn cif_control_letter(number_str: &String) -> (char, u32) {
    let letters = "JABCDEFGHI".to_string();
    let mut sum_a = 0;
    let mut sum_b = 0; 
    for (i, el) in number_str.chars().enumerate() {
        if i % 2 == 0 {
            sum_a += el.to_string().parse::<i32>().unwrap()
        } else {
            let digit_product_str = (el.to_string().parse::<i32>().unwrap() * 2).to_string();
            let mut digit_sum = 0;
            for digit in digit_product_str.chars() {
                digit_sum += digit.to_string().parse::<i32>().unwrap();
            }
            sum_b += digit_sum;
        }
    }

    let sum_c = sum_a + sum_b;
    let digit_e = sum_c.to_string().chars().last().unwrap().to_digit(10).unwrap();

    let digit_d: u32;
    if digit_e != 0 {
        digit_d = 10 - digit_e;
    } else {
        digit_d = 0;
    }

    let letter = letters.chars().nth((digit_d).try_into().unwrap()).unwrap();

    (letter, digit_d)
}

fn province_number () -> String {
    let province_numbers = [
        "00", "01", "02" ,"03", "53", "54", "04", "05", "06", "07", "57", "08", 
        "58", "59", "60", "61", "62", "63", "64", "65", "66", "68", "09", "10", 
        "11", "72", "12", "13", "14", "56", "15", "70", "16", "17", "55", "67", 
        "18", "19", "20", "71", "21", "22", "23", "24", "25", "26", "27", "28", 
        "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "29", 
        "92", "93", "30", "73", "31", "71", "32", "33", "74", "34", "35", "75", 
        "36", "37", "94", "37", "38", "76", "39", "40", "41", "90", "91", "42", 
        "43", "77", "44", "45", "46", "96", "97", "98", "47", "48", "95", "49", 
        "50", "99", "51", "52"
        ];
    let mut rng = rand::thread_rng();
    let province_rand_number = rng.gen_range(0..100);

    province_numbers[province_rand_number].to_string()
}


pub fn generate_cif() -> String {
    let province = province_number();
    let mut rng = rand::thread_rng();
    let cif_rand_number = rng.gen_range(CIF_RAND_START..CIF_RAND_END).to_string();

    let first_letter = cif_first_letter();
    let complete_number = format!("{}{}", province, cif_rand_number);
    let (control_letter, control_digit) = cif_control_letter(&complete_number);

    let res_cif: String;

    if "PQRSW".contains(first_letter) || province == "00" {
        res_cif = format!("{}{}{}", first_letter, complete_number, control_letter)
    } else {
        res_cif = format!("{}{}{}", first_letter, complete_number, control_digit)
    }

    res_cif
}

pub fn validate_nif(nif: String) -> bool {
    let re = Regex::new(r"^(\d{8})([A-Z])$").unwrap();

    if !re.is_match(nif.as_str()) {
        return false;
    }

    let number_substring = nif[..8].to_string();
    let letter_substring = nif[8..].to_string();
    let number_parsed = number_substring.parse::<i32>().unwrap();

    let computed_letter = calculate_letter(&number_parsed);

    return computed_letter.to_string() == letter_substring;
}

pub fn validate_nie(nie: String) -> bool {
    let re = Regex::new(r"^[XYZ]\d{7,8}[A-Z]$").unwrap();

    if !re.is_match(nie.as_str()) {
        return false;
    }

    let number_substring = nie[1..8].to_string();
    let letter_substring = nie[8..].to_string();
    let number_parsed = number_substring.parse::<i32>().unwrap();

    let computed_letter = calculate_letter(&number_parsed);

    return computed_letter.to_string() == letter_substring;    
}

pub fn validate_cif(cif: String) -> bool {
    let re = Regex::new(r"^([ABCDEFGHJKLMNPQRSUVW])(\d{7})([0-9A-J])$").unwrap();

    if !re.is_match(cif.as_str()) {
        return false;
    }

    true
}

