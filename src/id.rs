use rand::{self, Rng};

const NIF_RAND_START: i32 = 10000000;
const NIF_RAND_END: i32 = 100000000;

const NIE_RAND_START: i32 = 1000000;
const NIE_RAND_END: i32 = 10000000;

fn calculate_letter(number: &i32) -> char {
    let letters = "TRWAGMYFPDXBNJZSQVHLCKE".to_string();
    let letter = letters.chars().nth((number % 23).try_into().unwrap()).unwrap();
    
    letter
}

pub fn generate_nif() {
    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(NIF_RAND_START..NIF_RAND_END);
    let nif_letter = calculate_letter(&rand_number);

    println!("{}{}", rand_number, nif_letter);
}

pub fn generate_nie() {
    let mut rng = rand::thread_rng();
    let initial_random = rng.gen_range(0..3);

    let nie_rand_number = rng.gen_range(NIE_RAND_START..NIE_RAND_END);

    let complete_number: i32 = format!("{}{}", initial_random, nie_rand_number).parse().unwrap();

    let end_letter = calculate_letter(&complete_number);

    if initial_random == 0 {
        println!("X{}{}", nie_rand_number, end_letter);
    } else if initial_random == 1 {
        println!("Y{}{}", nie_rand_number, end_letter);
    } else if initial_random == 2 {
        println!("Z{}{}", nie_rand_number, end_letter);
    }
}

pub fn generate_cif() {
    println!("CIF created");
}

