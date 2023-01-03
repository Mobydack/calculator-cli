use crate::parser::FormulaAst;
use crate::operators::{sum,diff};
use crate::operators::base_operator::Operator;

pub fn execute_ast(ast_node: &FormulaAst) -> f64 {
    match ast_node {
        FormulaAst::Number(value) => {
            *value
        },
        FormulaAst::Function(name, arguments) => {
            let operator = match name.as_str() {
                "SUM" => {
                    sum::Sum::executor
                },
                "DIFF" => {
                    diff::Diff::executor
                },
                _ => {
                    panic!("Unexpected operator {:?}", name);
                }
            };

            operator(Some(arguments.iter().map(|argument| {
                execute_ast(argument)
            }).collect()))
        }
    }
}