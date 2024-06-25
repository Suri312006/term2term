use clap::Parser;
// simple greeter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Name of greetee
    #[arg(short, long)]
    name: String,

    //Number of times
    #[arg(short, long, default_value_t = 1)]
    count: u64,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}", args.name);
    }

}
