const OPERATORS: [&str; 5] = ["+", "-", "*", "/", "^"];

#[allow(dead_code)]
/// Parses Mathematical expressions from strings, converts them to their postfix notation and
/// evaluates them
pub struct Parser {
    expr: String,
    postfix: String,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]

/// The various Error types of Parser;
/// InvalidExpression - Occurs if evaluating the expressions finds a unknown operator, unlikely to
/// happen,
/// UnclosedParentheses, if a open parenthese is left on the stack, throws an error,
/// InvalidOperator, Occurs if parsing the expression into postfix finds an unknown operator,
/// DivisionByZero, Occurs if evaluating the expression causes a divide by zero error,
pub enum Error {
    InvalidExpression,
    UnclosedParentheses,
    InvalidOperator,
    DivisionByZero,
}

#[allow(dead_code)]
impl Parser {
    /// Returns a new Parser, or an Error if an invalid expression occurs
    pub fn new(expr: String) -> Result<Self, Error> {
        Ok(Self {
            expr: expr.clone(),
            postfix: {
                let t = Self::shunting_yard(expr);
                match t {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                }
            },
        })
    }
    
    /// returns the expression in infix notation
    pub fn get_expr(&self) -> &String {
        &self.expr
    }
    
    /// returns the expression in postfix notation
    pub fn get_postfix(&self) -> &String {
        &self.postfix
    }

    /// evaluates the expression, returning the result on success, otherwise returning an error
    pub fn eval(&self) -> Result<f32, Error> {
        let mut stack: Vec<f32> = vec![];

        for token in self.get_postfix().split_whitespace() {
            if let Ok(t) = token.parse::<f32>() {
                stack.push(t);
            }
            else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match token {
                    "+" => {
                        stack.push(a + b);
                    },
                    "-" => {
                        stack.push(a - b);
                    },
                    "*" => {
                        stack.push(a * b);
                    },
                    "/" => {
                        if b == 0.0 {
                            return Err(Error::DivisionByZero);
                        }
                        stack.push(a / b);
                    },
                    "^" => {
                        stack.push(a.powf(b as f32));
                    },
                    _ => return Err(Error::InvalidExpression),
                };
            }
        }
        Ok(stack[0])
    }

    // 1 + 2 * 3 / 4 ^ 5
    //  1 2 3 * 4 5 ^ / + 
    /// parses infix to postfix, returning the postfix expression on success, otherwise throws an
    /// error
    fn shunting_yard(expr: String) -> Result<String, self::Error>  {
        let mut output_stack: Vec<String> = vec![];
        let mut operator_stack: Vec<String> = vec![];

        for token in expr.split_whitespace() {
            if let Ok(_) = token.parse::<f32>() {
                output_stack.push(token.to_string());
            }
            else if token == "(" {
                operator_stack.push(token.to_string());
            }
            else if token == ")" {
                while !operator_stack.is_empty() && operator_stack[operator_stack.len() - 1] != "(" {
                    output_stack.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop();
            }
            else if OPERATORS.contains(&token) {
                operator_stack.push(token.to_string());
            }
            else {
                while !operator_stack.is_empty() {
                    let stack_precendence = match Self::get_precedence(operator_stack[operator_stack.len() - 1].to_string()) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    let token_precendence = match Self::get_precedence(token.to_string()) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    while stack_precendence < token_precendence {
                        output_stack.push(operator_stack.pop().unwrap());
                    }

                }
                output_stack.push(token.to_string());
            }

        }
        operator_stack.reverse();
        output_stack.append(&mut operator_stack);
        if output_stack.contains(&"(".to_string()) {
            return Err(Error::UnclosedParentheses);
        }

        Ok(output_stack.join(" "))
    }

    /// returns operator precedence, or an error on encountering an unknown operator,
    fn get_precedence(operator: String) -> Result<i32, Error> {

        match operator.as_str() {
            "+" | "-" => Ok(2),
            "*" | "/" => Ok(1),
            "^" => Ok(3),
            "(" | ")" => Ok(0),
            _ => {
                println!("{operator}");
                Err(Error::InvalidOperator)
            }, }
    }

}
