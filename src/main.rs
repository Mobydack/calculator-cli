use clap::Parser;

mod executor;
mod operators;
mod parser;

#[derive(Parser)]
#[command(name = "calc")]
#[command(bin_name = "calc")]
#[command(author, version, about, long_about = None)]
struct Cli {
    expression: String,
}

fn main() {
    let cli = Cli::parse();

    println!("expression result {:?}", cli.expression);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn is_correct_result_sum() {
        let ast = parser::formula::expression("SUM(1, 1)").unwrap();

        assert_eq!(executor::execute_ast(&ast), 2.0)
    }

    #[test]
    pub fn is_correct_result_diff() {
        let ast = parser::formula::expression("DIFF(1, 2)").unwrap();

        assert_eq!(executor::execute_ast(&ast), -1.0)
    }
}
