use super::base_operator::Operator;

pub struct Sum;

impl Operator for Sum {
    fn get_name() -> String {
        String::from("SUM")
    }

    fn executor(arguments: Option<Vec<f64>>) -> f64 {
        let arguments = arguments.unwrap();

        arguments.iter().sum()
    }
}
