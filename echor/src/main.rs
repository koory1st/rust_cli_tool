use clap::Parser;

// 神奇的注释
/// Simple program to greet a person 1
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    dbg!(&args);

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
