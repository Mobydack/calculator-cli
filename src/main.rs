use clap::Parser;

#[derive(Parser)]
#[command(name = "calc")]
#[command(bin_name = "calc")]
#[command(author, version, about, long_about = None)]
struct Cli {
    expression: String,
}

fn main() {
    let cli = Cli::parse();

    println!("expression {:?}", cli.expression);
}
