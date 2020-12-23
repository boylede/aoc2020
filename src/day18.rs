use crate::PartResult;
use std::collections::HashMap;
use std::io::{BufRead, Cursor, Read, Seek};


#[derive(Debug, Clone)]
enum ParseState {
    Free,
    Left(Box<Expression>),
    Op(Box<Expression>, Operation),
    Right(Box<Expression>),
    Parentheses(Box<ParseState>),
    BubbleParen(Box<Expression>),
    RightParen(Box<Expression>, Operation, Box<ParseState>),
}

impl ParseState {
    fn new() -> ParseState {
        ParseState::Free
    }
    fn push(self, c: char) -> ParseState {
        use ParseState::*;
        match self {
            Free => {
                if c == '(' {
                    Parentheses(Box::new(ParseState::Free))
                } else {
                    let num = c.to_string().parse::<i64>().unwrap();
                    Left(Box::new(Expression::Factor(Factor::Number(num))))
                }           
            },
            Left(e) => {
                let op = match c {
                    '+' => Operation::Addition,
                    '*' => Operation::Multiplication,
                    _ => panic!("found unexpected operator"),
                };
                Op(e, op)
            },
            Op(e,o) => {
                match c {
                    '(' => {
                        RightParen(e, o, Box::new(Free))
                    }
                    _ => {
                        let num = c.to_string().parse::<i64>().unwrap();
                        let r = Factor::Number(num);
                        match o {
                            Operation::Addition => Right(Box::new(Expression::Addition(Factor::Expression(e), r))),
                            Operation::Multiplication => Right(Box::new(Expression::Multiplication(Factor::Expression(e), r))),
                        }
                    }
                }
            },
            Right(e) => {
                match c {
                    '+' => {
                        Op(e, Operation::Addition)
                    }
                    '*' => {
                        Op(e, Operation::Multiplication)
                    }
                    ')' => BubbleParen(e),
                    _ => panic!("unexpected input {}", c),
                }
            },
            Parentheses(p) => {
                let next = p.push(c);
                match next {
                    BubbleParen(ee) => {
                        Left(ee)
                    },
                    _ => Parentheses(Box::new(next)),
                }
            },
            BubbleParen(_) => {
                // bubbleparens are stripped prior to this loop
                unreachable!()
            }
            RightParen(e, o, p) => {
                let next = p.push(c);
                match next {
                    BubbleParen(ee) => {
                        let expression = match o {
                            Operation::Addition => Expression::Addition(Factor::Expression(e), Factor::Expression(ee)),
                            Operation::Multiplication => Expression::Multiplication(Factor::Expression(e), Factor::Expression(ee)),
                        };
                        Right(Box::new(expression))
                    },
                    _ => RightParen(e, o, Box::new(next)),
                }
            }
        }
    }
    fn express(self) -> Expression {
        use ParseState::*;
        match self {
            Right(e) => *e,
            Parentheses(p) => p.express(),
            _ => panic!("unexpected end of line"),
        }
    }
}


/*
 BNF
expression ::= addition | factor
addition ::= multiplication '+' multiplication
multiplication ::= factor '*' factor | factor
factor     ::= number | '(' expression ')'
*/

fn consume_operation(stream: &mut Walk) -> Result<char, ()> {
    let maybe_operation = stream.next().ok_or(())?;
    if maybe_operation == '+' {
        Ok('+')
    } else if maybe_operation == '*' {
        Ok('*')
    } else {
        stream.backward();
        Err(())
    }
}

// fn consume_left(first: char, stream: &mut Walk) -> Result<Expression, ()> {
//     // let left = consume_factor(stream)?;
//     let left = first;
//     if let Ok(op) = consume_operation(stream) {
//         let right = consume_factor(stream)?;
//         let expr = match op {
//             '+' => Expression::Addition(left, right),
//             '*' => Expression::Multiplication(left, right),
//             _ => panic!("???"),
//         };
//         Ok(expr)
//     } else {
//         Ok(Expression::Factor(left))
//     }
// }

fn consume_expression(stream: &mut Walk) -> Result<Expression, ()> {
    // let left = consume_left(stream.next().ok_or(())?, stream);
    // if let Ok(op) = consume_operation(stream) {
    //     let right = consume_factor(stream)?;
    //     let expr = match op {
    //         '+' => Expression::Addition(left, right),
    //         '*' => Expression::Multiplication(left, right),
    //         _ => panic!("???"),
    //     };
    //     Ok(expr)
    // } else {
    //     Ok(Expression::Factor(left))
    // }
    let factor_a = consume_factor(stream)?;
    println!("found factor {}", factor_a);
    if let Some(maybe_operation) = stream.next() {
        if maybe_operation == '+' {
            println!("+");
            let factor_b = consume_factor(stream)?;
            // if let Ok(factor_b) = maybe_factor {
            println!("factor b: {}", factor_b);
            Ok(Expression::Addition(factor_b, factor_a))
            // } else {
            //     println!("f1");
            //     Err(())
            // }
        } else if maybe_operation == '*' {
            println!("*");
            let factor_b = consume_factor(stream)?;
            // if let Ok(factor_b) = maybe_factor {
            println!("factor b: {}", factor_b);
            Ok(Expression::Multiplication(factor_b, factor_a))
            // } else {
            //     println!("f2");
            //     Err(())
            // }
        } else {
            println!("consumed {}, didn't want it yet", maybe_operation);
            stream.backward();
            Ok(Expression::Factor(factor_a))
            // println!("f3");
            // Err(())
        }
    } else {
        // stream.backward();
        println!("found EOF");
        Ok(Expression::Factor(factor_a))
    }

}

fn consume_factor(stream: &mut Walk) -> Result<Factor, ()> {
    let maybe_number = stream.next().ok_or(())?;
    if let Ok(num) = maybe_number.to_string().parse::<i64>() {
        Ok(Factor::Number(num))
    } else {
        if maybe_number == '(' {
            println!("found open paren");
            let expr = consume_expression(stream)?;
            let expect_paren = stream.next().ok_or(())?;
            if expect_paren == ')' {
                println!("found close paren");
                Ok(Factor::Expression(Box::new(expr)))
            } else {
                stream.backward();
                stream.backward();
                println!("f4");
                Err(())
            }
        } else {
            stream.backward();
            println!("f5");
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone)]
enum Expression {
    Factor(Factor),
    Addition(Factor, Factor),
    Multiplication(Factor, Factor),
}

#[derive(Debug, Clone)]
enum Factor {
    Number(i64),
    Expression(Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> i64 {
        match self {
            Expression::Factor(f) => f.evaluate(),
            Expression::Addition(e, f) => e.evaluate() + f.evaluate(),
            Expression::Multiplication(e, f) => e.evaluate() * f.evaluate(),
        }
    }
}

impl Factor {
    fn evaluate(&self) -> i64 {
        match self {
            Factor::Number(n) => *n,
            Factor::Expression(e) => e.evaluate(),
        }
    }
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Factor::Number(n) => write!(f, "{}", n),
            Factor::Expression(e) => write!(f, "({})", e),
        }
    }
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Factor(t) => write!(f, "{}", t),
            Expression::Addition(e, t) => write!(f, "{} + {}", e, t),
            Expression::Multiplication(e, t) => write!(f, "{} * {}", e, t),
        }
    }
}

pub fn part1(lines: &Vec<String>) -> PartResult {
    let total: i64 = lines
        .iter()
        // .inspect(|expr| println!("{}", expr))
        // .filter_map(|line| {
            // println!("------------------------------------");
            // consume_expression(&mut Walk::wrap(line)).ok()
        // })
        // .inspect(|expr| println!("{}", expr))
        // .map(|e|e.evaluate())
        // .inspect(|result| println!("{:?}", result))
        // .sum();
        .map(|line| line.chars().filter(|c| *c != ' ').fold(ParseState::Free, |s, c|s.push(c)))
        // .inspect(|expr| println!("{:?}", expr))
        .map(|s|s.express().evaluate())
        .sum();
        // .fold(ParseState::Free, |s, c|)


    Ok(total.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    Ok("".to_string())
}

struct Walk<'a> {
    inner: &'a str,
    index: usize,
}

impl<'a> Walk<'a> {
    fn next(&mut self) -> Option<char> {
        loop {
            let c = self.inner.chars().nth(self.index);
            if let Some(' ') = c {
                self.index += 1;
            } else {
                self.index += 1;
                break c;
            }
        }
        
    }
    fn backward(&mut self) {
        self.index -= 1;
    }
    fn wrap(inner: &'a str) -> Self {
        Walk { inner, index: 0 }
    }
}
