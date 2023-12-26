use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
struct Cli {
    #[arg(index = 1, default_value = "uuid")]
    generator: String,

    #[arg(short, long, default_value_t = 1)]
    amount: u32,
}

fn generate_uuid(amount: u32) {
    for _n in 0..amount {
        println!("{}", Uuid::new_v4());
    }
}

fn generate_password(amount: u32) {
    for _n in 0..amount {
        println!("{}", generate_random_string())
    }
}

fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();

    let letter: char = rng.gen_range('a'..='z');
    let number: char = rng.gen_range('0'..='9');
    let special_char: char = rng.gen_range('!'..='~');
    let remaining_chars: String = (0..10).map(|_| rng.gen_range('!'..='~')).collect();
    let mut chars: Vec<char> = vec![letter, number, special_char];

    chars.extend(remaining_chars.chars());
    chars.shuffle(&mut rng);

    let result: String = chars.into_iter().collect();
    result
}

fn generate(generator: String, amount: u32) {
    match generator.to_lowercase().as_str() {
        "uuid" => generate_uuid(amount),
        "password" => generate_password(amount),
        _ => println!("Unknown generator"),
    }
}

fn main() {
    let args = Cli::parse();
    generate(args.generator, args.amount);
}
