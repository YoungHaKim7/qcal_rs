use super::{
    converter::Converter, evaluator::Evaluator, formatter::Formatter, lexer::Lexer, parser::Parser,
};

pub struct Engine {
    evaluator: Evaluator,
    last: Option<f64>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            evaluator: Evaluator::new(),
            last: None,
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<String, String> {
        if input.contains("to unicode") || input.contains("to uni") {
            return Ok(Converter::unicode(input));
        }

        let mut input = input.to_string();

        if let Some(last) = self.last {
            input = input.replace("res", &last.to_string());
        }

        let input = self.preprocess(&input);

        let tokens = Lexer::tokenize(&input)?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let val = self.evaluator.eval(&ast);
        self.last = Some(val);

        Ok(Formatter::full(val))
    }

    fn preprocess(&self, input: &str) -> String {
        let mut s = input.to_string();

        while let Some(pos) = s.find("0x") {
            let hex: String = s[pos + 2..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();

            if !hex.is_empty() {
                let val = i64::from_str_radix(&hex, 16).unwrap();
                s.replace_range(pos..pos + 2 + hex.len(), &val.to_string());
            } else {
                break;
            }
        }

        while let Some(pos) = s.find("0b") {
            let bin: String = s[pos + 2..]
                .chars()
                .take_while(|c| *c == '0' || *c == '1')
                .collect();

            if !bin.is_empty() {
                let val = i64::from_str_radix(&bin, 2).unwrap();
                s.replace_range(pos..pos + 2 + bin.len(), &val.to_string());
            } else {
                break;
            }
        }

        while let Some(pos) = s.find("0o") {
            let oct: String = s[pos + 2..]
                .chars()
                .take_while(|c| *c >= '0' && *c <= '7')
                .collect();

            if !oct.is_empty() {
                let val = i64::from_str_radix(&oct, 8).unwrap();
                s.replace_range(pos..pos + 2 + oct.len(), &val.to_string());
            } else {
                break;
            }
        }

        s
    }
}
