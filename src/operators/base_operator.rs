pub trait Operator {
    fn get_name() -> String;
    fn executor(arguments: Option<Vec<f64>>) -> f64;
}
