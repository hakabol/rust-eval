use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
enum Function{
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
}

#[derive(Debug, PartialEq, Clone)]
enum OperationToken{
    Add,
    Sub,
    Mul,
    Div,
    Expo,
}

#[derive(Debug, PartialEq, Clone)]
enum Token{
    Number(f64),
    Operation(OperationToken),
    Func(Function),
    LParen,
    RParen
}

type Equation = Vec<Token>;

fn to_eq(str_eq: String) -> Equation{
    let mut equation: Equation = vec![];

    let mut iterr = str_eq.chars();

    while let Some(c) = iterr.next(){
        if c.is_whitespace(){
            continue;
        }
        match c{
            '+' => equation.push(Token::Operation(OperationToken::Add)),
            '-' => equation.push(Token::Operation(OperationToken::Sub)),
            '*' => equation.push(Token::Operation(OperationToken::Mul)),
            '/' => equation.push(Token::Operation(OperationToken::Div)),
            '^' => equation.push(Token::Operation(OperationToken::Expo)),
            '(' => equation.push(Token::LParen),
            ')' => equation.push(Token::RParen),
            c => {
                if c.is_numeric(){
                    if let Some(Token::Number(x)) = equation.last(){
                        let num = x*10.0 + c.to_digit(10).unwrap() as f64;
                        equation.pop();
                        equation.push(Token::Number(num));
                    }
                    else{
                        equation.push(Token::Number(c.to_digit(10).unwrap() as f64));
                    }
                }
                else{
                    let mut func = c.to_string() + &iterr.next().unwrap().to_string() ;
                    if func.to_lowercase().as_str() == "ln"{
                        equation.push(Token::Func(Function::Ln))
                    }
                    else{
                        func += &iterr.next().unwrap().to_string();
                        equation.push(match func.to_lowercase().as_str(){
                            "sin" => Ok(Token::Func(Function::Sin)),
                            "cos" => Ok(Token::Func(Function::Cos)),
                            "tan" => Ok(Token::Func(Function::Tan)),
                            "log" => Ok(Token::Func(Function::Log)),
                            _ => {Err(())}
                        }.unwrap());
                    }
                }
            }
        }
    }

    equation
}

fn calculate(mut equ: Equation) -> f64{
    let mut result = 3.0;

    loop{
        let index = equ.iter().rposition(|x| *x == Token::LParen);

        if let Some(ind) = index{
            let index = equ[ind..].iter().position(|x| *x == Token::RParen).unwrap() + ind;
            let num = no_cal(equ[(ind + 1)..index].to_vec());

            let mut new_equ: Equation = equ[..ind].to_vec();
            new_equ.push(Token::Number(num));
            new_equ.append(&mut equ[(index + 1)..].to_vec());

            equ = new_equ;

        }
        else{
            result = no_cal(equ);
            break;
        }
    }
    result
}

fn no_cal(mut equ: Equation) -> f64{
    loop{
        if equ.len() == 1{
            break;
        }

        let pri = vec![
            Token::Func(Function::Sin),
            Token::Func(Function::Cos),
            Token::Func(Function::Tan),
            Token::Func(Function::Log),
            Token::Func(Function::Ln),
            Token::Operation(OperationToken::Expo),
            Token::Operation(OperationToken::Div),
            Token::Operation(OperationToken::Mul),
            Token::Operation(OperationToken::Sub),
            Token::Operation(OperationToken::Add),
        ];

        let pri_token = pri.iter().find(|x| equ.contains(*x));

        if pri_token.is_none(){
            break;
        }

        let pri_token = pri_token.unwrap();

        let ind = equ.iter().position(|x| x == pri_token).unwrap();

        match pri_token{
            Token::Operation(OperationToken::Expo) => {
                let mut start: i32 = ind as i32 - 2;
                if start <=0 {
                    start = 0;
                }
                let mut new_equ: Equation = equ[..(start as usize)].to_vec();
                let num1  = match equ[ind - 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                let num2  = match equ[ind + 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                new_equ.push(Token::Number(num1.powf(num2)));
                new_equ.append(&mut equ[(ind + 2)..].to_vec());

                equ = new_equ;

            }
            Token::Operation(OperationToken::Mul) => {
                let mut start: i32 = ind as i32 - 2;
                if start <=0{
                    start = 0;
                }
                let mut new_equ: Equation = equ[..(start as usize)].to_vec();
                let num1  = match equ[ind - 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                let num2  = match equ[ind + 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                new_equ.push(Token::Number(num1 * num2));
                new_equ.append(&mut equ[(ind + 2)..].to_vec());

                equ = new_equ;

            }
             Token::Operation(OperationToken::Div) => {
                let mut start = ind as i32 - 2;
                if start <= 0{
                    start = 0;
                }
                let mut new_equ: Equation = equ[..(start as usize)].to_vec();
                let num1  = match equ[ind - 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                let num2  = match equ[ind + 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                new_equ.push(Token::Number(num1 / num2));
                new_equ.append(&mut equ[(ind + 2)..].to_vec());

                equ = new_equ;

            }
             Token::Operation(OperationToken::Sub) => {
                let mut start = ind as i32 - 2;
                if start <= 0{
                    start = 0;
                }
                let mut new_equ: Equation = equ[..(start as usize)].to_vec();
                let num1  = match equ[ind - 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                let num2  = match equ[ind + 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                new_equ.push(Token::Number(num1 - num2));
                new_equ.append(&mut equ[(ind + 2)..].to_vec());

                equ = new_equ;

            }
             Token::Operation(OperationToken::Add) => {
                let mut start: i32 = ind as i32 - 2;
                if start <= 0{
                    start = 0;
                }
                let mut new_equ: Equation = equ[..(start as usize)].to_vec();
                let num1  = match equ[ind - 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                let num2  = match equ[ind + 1]{
                    Token::Number(x) => x,
                    _ => unreachable!("uh how did u do this")
                };
                new_equ.push(Token::Number(num1 + num2));
                new_equ.append(&mut equ[(ind + 2)..].to_vec());

                equ = new_equ;

            }
            Token::Func(Function::Sin) => {
                let x = match equ.remove(ind + 1){
                    Token::Number(y) => y.sin(),
                    _ => 0.0
                };
                equ.remove(ind);
                equ.insert(ind, Token::Number(x));
            }
            Token::Func(Function::Cos) => {
                let x = match equ.remove(ind + 1){
                    Token::Number(y) => y.cos(),
                    _ => 0.0
                };
                equ.remove(ind);
                equ.insert(ind, Token::Number(x));
            }
            Token::Func(Function::Tan) => {
                let x = match equ.remove(ind + 1){
                    Token::Number(y) => y.tan(),
                    _ => 0.0
                };
                equ.remove(ind);
                equ.insert(ind, Token::Number(x));
            }
            Token::Func(Function::Log) => {
                let x = match equ.remove(ind + 1){
                    Token::Number(y) => y.log10(),
                    _ => 0.0
                };
                equ.remove(ind);
                equ.insert(ind, Token::Number(x));
            }
            Token::Func(Function::Ln) => {
                let x = match equ.remove(ind + 1){
                    Token::Number(y) => y.ln(),
                    _ => 0.0
                };
                equ.remove(ind);
                equ.insert(ind, Token::Number(x));
            }

            _ => {}
        }
    }

    let num = match equ[0]{
        Token::Number(x) => x,
        _ => 0 as f64
    };

    num
}

pub fn eval(inp: String) -> f64{
    let equ = to_eq(inp);
    let ans = calculate(equ);

    ans
}
