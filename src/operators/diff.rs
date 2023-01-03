use super::base_operator::Operator;

pub struct Diff;

impl Operator for Diff {
    fn get_name() -> String {
        String::from("DIFF")
    }

    fn executor(arguments: Option<Vec<f64>>) -> f64 {
        let arguments = arguments.unwrap();

        arguments.iter().copied().reduce(|acc, e| { acc - e }).unwrap()
    }
}
