use std::{
    fmt::{Display, Formatter},
    io::{self, Write},
};

#[derive(Debug)]
enum Error {
    ToF64ParseError(String),
    ExtraParenthesis(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ToF64ParseError(text_portion) => {
                write!(
                    f,
                    "não foi possível transformar o trecho do input ({}) em um número f64",
                    text_portion
                )
            }
            Error::ExtraParenthesis(text_portion) => {
                write!(
                    f,
                    "síntaxe incorreta no trecho '{}', parênteses há mais do que o necessário",
                    text_portion
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
}

#[test]
fn opeartion_should_operate_with_correctly_for_add() {
    let op = Operation::Add(5.0);
    assert_eq!(op.operate_with(&3.0), 8.0);
}

#[test]
fn opeartion_should_operate_with_correctly_for_subtract() {
    let op = Operation::Subtract(5.0);
    assert_eq!(op.operate_with(&3.0), -2.0);
}

#[test]
fn opeartion_should_operate_with_correctly_for_multiply() {
    let op = Operation::Multiply(5.0);
    assert_eq!(op.operate_with(&3.0), 15.0);
}

#[test]
fn opeartion_should_operate_with_correctly_for_divide() {
    let op = Operation::Divide(5.0);
    assert_eq!(op.operate_with(&3.0), 3.0 / 5.0);
}

impl Operation {
    fn operate_with(&self, other: &f64) -> f64 {
        match self {
            Operation::Add(num) => num + other,
            Operation::Subtract(num) => other - num,
            Operation::Multiply(num) => num * other,
            Operation::Divide(num) => other / num,
        }
    }

    fn is_multiply_or_divide(&self) -> bool {
        match self {
            Operation::Multiply(_) => true,
            Operation::Divide(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Dispõe a informação necessária que definem uma expressão e organizada de tal forma que possa
/// ser facilmente calculada.
///
/// # Exemplo
///
/// A representação da seguinte expressão "`9 + 2 - (5 + 3) * 2`" usando esta struct seria dada pelo
/// seguinte código:
/// ```rust
/// let parenthesis_expression = Expression {
///     operations: vec![
///         Opeartion::AddNumber(5.0),
///         Opeartion::AddNumber(3.0),
///     ],
/// }
///
/// let expression = Expression {
///     operations: vec![
///         Operation::Add(9.0),
///         Opeartion::Add(2.0),
///         Operation::Subtract(parenthesis_expression.evaluate()),
///         Opeartion::Multiply(2.0),
///     ],
/// }
/// ```
#[derive(Debug, Default)]
struct Expression {
    operations: Vec<Operation>, // a ordem dos valores desse Vec IMPORTA
}

impl Expression {
    fn empty() -> Self {
        Expression {
            operations: Vec::default(),
        }
    }

    fn push_op<T>(&mut self, kind: &OperationKind, val: T)
    where
        T: Into<f64>,
    {
        let num = val.into();
        match kind {
            OperationKind::Add => {
                self.operations.push(Operation::Add(num));
            }
            OperationKind::Subtract => {
                self.operations.push(Operation::Subtract(num));
            }
            OperationKind::Multiply => {
                self.operations.push(Operation::Multiply(num));
            }
            OperationKind::Divide => {
                self.operations.push(Operation::Divide(num));
            }
        };
    }

    fn new(text: &str) -> Result<Self, Error> {
        let pure_text = text.trim().replace(" ", "");

        let mut expression = Self::empty();

        let mut current_determined_operation = OperationKind::Add;
        let mut accumulated_text = String::new();
        let mut inside_parenthesis = false;

        let chars: Vec<&str> = pure_text.split("").collect();
        let chars_len = chars.len();

        for (i, char) in chars.into_iter().enumerate() {
            match char {
                "+" | "-" | "*" | "/" => {
                    if !inside_parenthesis {
                        if accumulated_text.len() > 0 {
                            if let Ok(num) = accumulated_text.parse::<f64>() {
                                expression.push_op(&current_determined_operation, num);
                            } else {
                                return Err(Error::ToF64ParseError(accumulated_text));
                            }
                        }
                    }
                }
                "(" => {
                    if inside_parenthesis {
                        return Err(Error::ExtraParenthesis(accumulated_text));
                    }
                    inside_parenthesis = true;
                }
                ")" => {
                    if inside_parenthesis {
                        inside_parenthesis = false;
                        let parenthesis_expression = Expression::new(&accumulated_text)?;
                        expression.push_op(
                            &current_determined_operation,
                            parenthesis_expression.evaluate(),
                        );
                        accumulated_text = String::new();
                    } else {
                        return Err(Error::ExtraParenthesis(accumulated_text));
                    }
                }
                _ => {}
            };

            if !inside_parenthesis {
                match char {
                    "+" => {
                        current_determined_operation = OperationKind::Add;
                        accumulated_text = String::new();
                    }
                    "-" => {
                        current_determined_operation = OperationKind::Subtract;
                        accumulated_text = String::new();
                    }
                    "*" => {
                        current_determined_operation = OperationKind::Multiply;
                        accumulated_text = String::new();
                    }
                    "/" => {
                        current_determined_operation = OperationKind::Divide;
                        accumulated_text = String::new();
                    }
                    "(" | ")" => {}
                    char => {
                        accumulated_text.push_str(char);
                    }
                };
            } else {
                if char != "(" && char != ")" {
                    accumulated_text.push_str(char);
                }
            }

            if i + 1 == chars_len && accumulated_text.len() > 0 {
                if let Ok(num) = accumulated_text.parse::<f64>() {
                    expression.push_op(&current_determined_operation, num);
                } else {
                    return Err(Error::ToF64ParseError(accumulated_text));
                }
            }
        }

        Ok(expression)
    }

    fn evaluate(&self) -> f64 {
        let mut result = 0.0;

        let mut temp_operations = self.operations.clone();
        let mut i = 0;
        while i < temp_operations.len() {
            let operation = &temp_operations[i];

            if i + 1 < temp_operations.len() {
                let next_operation = &temp_operations[i + 1];
                if next_operation.is_multiply_or_divide() {
                    match operation {
                        Operation::Add(num) => {
                            temp_operations[i] = Operation::Add(next_operation.operate_with(&num));
                        }
                        Operation::Subtract(num) => {
                            temp_operations[i] =
                                Operation::Subtract(next_operation.operate_with(&num));
                        }
                        _ => {} //cannot happen
                    }
                    temp_operations.remove(i + 1);
                } else {
                    match operation {
                        Operation::Add(num) => {
                            result += num;
                        }
                        Operation::Subtract(num) => {
                            result -= num;
                        }
                        _ => {} // the other possible operations will already been taken into account
                    }
                    i += 1;
                }
            } else {
                match operation {
                    Operation::Add(num) => {
                        result += num;
                    }
                    Operation::Subtract(num) => {
                        result -= num;
                    }
                    _ => {} // the other possible operations will already been taken into account
                }
                i += 1;
            }
        }

        result
    }
}

#[test]
fn expression_should_be_created_with_simple_strs_correctly() {
    // 3 + 5
    let expression = Expression::new("3 + 5").expect("falha na criação da Expression: [3 + 5]");
    assert_eq!(expression.evaluate(), 8.0);

    let other_expression = Expression::new("3+5").expect("falha na criação da Expression: [3+5]");
    assert_eq!(other_expression.evaluate(), 8.0);
}

#[test]
fn expression_should_be_created_correclty() {
    // 3 + (3 + 5) * 6 + 4 - 3 / 2
    let expression_str = "3 + (3 + 5) * 6 + 4 - 3 / 2";
    let expression = Expression::new(expression_str)
        .expect("falha na criação da Expression [3 + (3 + 5) * 6 + 4 - 3 / 2]");
    assert_eq!(
        expression.evaluate(),
        3.0 + (3.0 + 5.0) * 6.0 + 4.0 - 3.0 / 2.0
    );
}

#[test]
fn expression_should_be_evaluated_correctly() {
    // 4 + 5 + 9 + 3 * 2 / 3
    let expression = Expression {
        operations: vec![
            Operation::Add(4.0),
            Operation::Add(5.0),
            Operation::Add(9.0),
            Operation::Add(3.0),
            Operation::Multiply(2.0),
            Operation::Divide(3.0),
        ],
    };

    assert_eq!(expression.evaluate(), 4.0 + 5.0 + 9.0 + 3.0 * 2.0 / 3.0);
}

fn app() {
    let mut out_handle = io::stdout();
    out_handle
        .write_all(b"> ")
        .expect("não foi possível escrever '> ' no terminal");
    out_handle
        .flush()
        .expect("não foi possível forçar escrita no terminal");

    let mut expression_string = String::new();
    io::stdin()
        .read_line(&mut expression_string)
        .expect("não foi possível ler input pelo terminal");
    expression_string = expression_string.trim().to_lowercase().to_string();

    if expression_string == "clear" {
        print!("\x1B[2J\x1B[1;1H");
    } else if expression_string == "exit" {
        std::process::exit(0);
    } else {
        let expression = Expression::new(expression_string.as_str()).expect(
            format!(
                "não foi possível compreender a expressão escrita [{}]",
                expression_string
            )
            .as_str(),
        );

        let calculation_result = expression.evaluate();

        out_handle
            .write_all(format!("{}\n", calculation_result).as_bytes())
            .expect("não foi possível escrever resultado no terminal");
        out_handle
            .flush()
            .expect("não foi possível forçar escrita no terminal");
    }
}

fn main() {
    loop {
        app()
    }
}
