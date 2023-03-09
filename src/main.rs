use clap::Parser;

mod id;

use id::{generate_nif, generate_nie, generate_cif, validate_nif, validate_nie, validate_cif};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct DatagArgs {
    #[command(subcommand)]
    data_type: DataType,
}

#[derive(clap::Subcommand)]
enum DataType {
    Id {
        #[command(subcommand)]
        action: IdAction,
    },
    Vehicle {
        #[arg(long)]
        plate: bool,

        #[arg(long)]
        tourist_plate: bool,

        #[arg(long)]
        motorcycle_plate: bool,

        #[arg(long)]
        historic_plate: bool,

        #[arg(long)]
        trailer_plate: bool,

        #[arg(long)]
        special_plate: bool,
    },
    GeneratePassword {
        #[arg(long, short)]
        characters: bool,

        #[arg(long, short)]
        uppercase_characters: bool,

        #[arg(long, short)]
        strange_characters: bool,

        char_number: i32,
    },
}

#[derive(clap::Subcommand)]
enum IdAction {
    Generate {
        #[arg(long)]
        nif: bool,

        #[arg(long)]
        nie: bool,

        #[arg(long)]
        cif: bool,
    },
    Validate {

        string_to_validate: String,

        #[arg(long, required(false))]
        nif: bool,

        #[arg(long, required(false))]
        nie: bool,

        #[arg(long, required(false))]
        cif: bool,
    },
}


fn main() {
    let args = DatagArgs::parse();
    match args.data_type {
        DataType::Id { action } => {
            match action {
               IdAction::Generate { nif, nie, cif } => {
                    if nif {
                        println!("{}", generate_nif());
                    } else if nie {
                        println!("{}", generate_nie());
                    } else if cif {
                        println!("{}", generate_cif());
                    } else {
                        println!("error: you must select id type")
                    }
               }
               IdAction::Validate { nif, nie, cif, string_to_validate} => {

                    if nif {
                        if validate_nif(string_to_validate) {
                            println!("Valid!!");
                        } else {
                            println!("Not valid!!");
                        }
                    } else if nie {
                        if validate_nie(string_to_validate) {
                            println!("Valid!!");
                        } else {
                            println!("Not valid!!")
                        }
                    } else if cif {
                        println!("{}", validate_cif(string_to_validate));
                        println!("cif")
                    } else {
                        println!("error: select validation type")
                    }

               } 
            }
        }
        DataType::Vehicle { plate, tourist_plate, motorcycle_plate, historic_plate, trailer_plate, special_plate } => {
            if plate {
                println!("plate created")
            } else if tourist_plate {
                println!("tourist plate created")
            } else if motorcycle_plate {
                println!("motorcycle plate")
            } else if historic_plate {
                println!("historic plate created")
            } else if trailer_plate {
                println!("trailer plate created")
            } else if special_plate {
                println!("special plate created")
            } else {
                println!("error: you must select vehicle type")
            }
        }
        DataType::GeneratePassword { characters, uppercase_characters, strange_characters, char_number } => {
            if characters {
                println!("password contains chars")
            }
            if uppercase_characters {
                println!("password contains uppercase chars")
            }
            if strange_characters {
                println!("password contains strange characters")
            }
            println!("char number {}", char_number)
        }
    }
}
