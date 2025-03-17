use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Operation to perform: add, subtract, multiply, divide
    #[arg(short = 'o', long)]
    operation: String,

    /// Operands for the operation
    #[arg(short = 'p', long, num_args = 1..)]
    operands: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();

    if cli.operands.is_empty() {
        println!("Error: No operands provided!");
        return;
    }

    let result = match cli.operation.as_str() {
        "add" => cli.operands.iter().sum(),
        "subtract" => cli.operands.iter().skip(1).fold(cli.operands[0], |acc, &x| acc - x),
        "multiply" => cli.operands.iter().product(),
        "divide" => {
            if cli.operands[1..].contains(&0.0) {
                println!("Error: Division by zero!");
                return;
            }
            cli.operands.iter().skip(1).fold(cli.operands[0], |acc, &x| acc / x)
        },
        _ => {
            println!("Unsupported operation: {}", cli.operation);
            return;
        }
    };

    println!("Result: {}", result);
}
