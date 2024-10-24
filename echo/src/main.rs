use clap::Parser;

#[derive(Parser)]
#[command(name = "echo")]
#[command(about = "A simple CLI tool that prints the given argument")]
struct Args {
    argument: String,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    println!("You entered: {}", args.argument);
}
