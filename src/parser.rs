use peg;

#[derive(Debug, PartialEq)]
pub enum FormulaAst {
    Number(f64),
    Function(String, Vec<FormulaAst>),
}

peg::parser! {
    pub grammar formula() for str {
        rule _ = [' ' | '\n' | '\t']*
        rule number() -> FormulaAst
            = value:$("-"?['0'..='9']+("."['0'..='9']+)?) { FormulaAst::Number(value.parse::<f64>().unwrap()) }

        rule operands() -> Vec<FormulaAst>
            = "(" _ arguments:((number() / operator()) ** (_ "," _)) _ ")" { arguments }

        rule operator() -> FormulaAst
            = name:$(['a'..='z' | 'A'..='Z']+("_"['a'..='z' | 'A'..='Z' | '0'..='9']+)?) args:operands() { FormulaAst::Function(name.to_string(), args) }

        pub rule expression() -> FormulaAst
            = number() / operator()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_number() {
        assert_eq!(formula::expression("1"), Ok(FormulaAst::Number(1.0)));
        assert_eq!(formula::expression("1.5"), Ok(FormulaAst::Number(1.5)));
    }

    #[test]
    fn it_parse_negative_number() {
        assert_eq!(formula::expression("-2"), Ok(FormulaAst::Number(-2.0)))
    }

    #[test]
    fn it_parse_function() {
        assert_eq!(
            formula::expression("test(1.0)"),
            Ok(FormulaAst::Function(
                String::from("test"),
                vec![FormulaAst::Number(1.0)]
            ))
        );
        assert_eq!(
            formula::expression("test(test_1(1))"),
            Ok(FormulaAst::Function(
                String::from("test"),
                vec![FormulaAst::Function(
                    String::from("test_1"),
                    vec![FormulaAst::Number(1.0)]
                )]
            ))
        );
    }
}
