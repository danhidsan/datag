use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
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
        #[arg(long)]
        nif: String,

        #[arg(long)]
        nie: String,

        #[arg(long)]
        cif: String,
    },
}

fn main() {
    let args = Args::parse();
    println!("Hello world!");
}
