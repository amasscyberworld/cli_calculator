use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    operation: String,
    #[arg(short, long)]
    first: f64,
    #[arg(short, long)]
    second: f64,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.operation.as_str() {
        "add" => cli.first + cli.second,
        "subtract" => cli.first - cli.second,
        "multiply" => cli.first * cli.second,
        "divide" => {
            if cli.second == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            cli.first / cli.second
        },
        _ => {
            println!("Error: Unknown operation. Use 'add', 'subtract', 'multiply', or 'divide'.");
            return;
        }
    };

    println!("Result: {}", result);
}
