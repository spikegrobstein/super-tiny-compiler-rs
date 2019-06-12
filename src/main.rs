fn main() {
    let tokens = tokenizer("(add 2 (subtract 4 2))");
    println!("{:?}", tokens);

    let ast = parser(tokens);

    println!("");
    println!("{:?}", ast);
}

#[derive(Debug, PartialEq)]
enum Token <'a> {
    Paren(char),
    Name(&'a str),
    Integer(&'a str),
    Char(&'a str),
    Whitespace,
}

#[derive(Debug)]
enum Ast {
    Program(Vec<Ast>),
    IntegerLiteral(i64),
    StringLiteral(String),
    CallExpression { name: String, params: Vec<Ast> },
}

fn tokenizer(data: &str) -> Vec<Token> {
    let mut bytes = data.chars().enumerate().peekable();

    let mut tokens: Vec<Token> = vec![];

    while let Some((index, current)) = bytes.next() {
        let token: Token =
            if current == '(' || current == ')' {
                Token::Paren(current) 
            } else if current == '"' {
                // it's an open quote read a string
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if current == &'"' {
                        break;
                    }
                    counter += 1;
                    bytes.next();
                }

                Token::Char(&data[index..counter])
            } else if current.is_whitespace() {
                Token::Whitespace
            } else if current.is_digit(10) {
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if ! current.is_digit(10) {
                        break;
                    }

                    counter += 1;
                    bytes.next();
                }

                Token::Integer(&data[index..counter])
            } else if current.is_alphabetic() {
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if ! current.is_alphabetic() {
                        break;
                    }

                    counter += 1;
                    bytes.next();
                }

                Token::Name(&data[index..counter])
            } else {
                panic!("Unexpected character: {}", current);
            };

        if token != Token::Whitespace {
            tokens.push(token);
        }
    }

    tokens
}

fn walk<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Ast
where
    I: std::iter::Iterator<Item = &'a Token<'static>>
{
    let token = tokens.next().unwrap();

    match token {
        Token::Integer(value) => {
            Ast::IntegerLiteral(i64::from_str_radix(value, 10).unwrap())
        },
        Token::Char(value) => {
            Ast::StringLiteral(value.to_string())
        }
        Token::Paren('(') => {
            let token = tokens.next().unwrap(); // skip the ( and get the next token, which shoulld be a name

            // save the call expression name as a legit String
            let call_expression_name = match token {
                Token::Name(name) => name.to_string(),
                token => panic!("Expected Token::Name, but got {:?}", token),
            };

            // let token = tokens.next().unwrap();

            let mut params: Vec<Ast> = Vec::new();

            loop {
                if let Token::Paren(')') = tokens.peek().unwrap() {
                    // bump the token and break
                    let _token = tokens.next().unwrap();
                    break;
                }
                params.push(walk(tokens)); 
            }

            Ast::CallExpression {
                name: call_expression_name,
                params,
            }
        },
        _ => panic!("oh no")
    }
}

fn parser(tokens: Vec<Token<'static>>) -> Ast {
    let mut tokens = tokens.iter().peekable();

    let mut body: Vec<Ast> = Vec::new();

    loop {
        if tokens.peek().is_none() {
            break;
        }

        body.push(walk(&mut tokens));
    }

    Ast::Program(body)
}
