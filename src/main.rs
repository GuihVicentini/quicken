use clap::Parser;
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

fn main() {
    let args = Cli::parse();
    generate_uuid(args.amount);
}
