use rand::{self, Rng};

fn calculate_nif_letter(nif_number: &i32) -> char {
    let letters = "TRWAGMYFPDXBNJZSQVHLCKE".to_string();
    let letter = letters.chars().nth(nif_number % 23).unwrap();
    
    letter
}

pub fn generate_nif() {
    let x: i32 = 10;
    let range_start = x.pow(7);
    let range_end = x.pow(8) - 1;

    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(range_start..range_end);
    let nif_letter = calculate_nif_letter(&rand_number);

    println!("{}{}", rand_number, nif_letter);
}

pub fn generate_nie() {
    println!("NIE created");
}

pub fn generate_cif() {
    println!("CIF created");
}

