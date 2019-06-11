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
                let start_index = index;
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if current == &'"' {
                        break;
                    }
                    counter += 1;
                    bytes.next();
                }

                Token::Char(&data[start_index..counter])
            } else if current.is_whitespace() {
                Token::Whitespace
            } else if current.is_digit(10) {
                let start_index = index;
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if ! current.is_digit(10) {
                        break;
                    }

                    counter += 1;
                    bytes.next();
                }

                Token::Integer(&data[start_index..counter])
            } else if current.is_alphabetic() {
                let start_index = index;
                let mut counter = index + 1;

                while let Some((_index, current)) = bytes.peek() {
                    if ! current.is_alphabetic() {
                        break;
                    }

                    counter += 1;
                    bytes.next();
                }

                Token::Name(&data[start_index..counter])
            } else {
                panic!("Unexpected character: {}", current);
            };

        if token != Token::Whitespace {
            tokens.push(token);
        }
    }

    tokens
}



fn walk(current: usize, tokens: &Vec<Token>) -> Ast {
    let token = &tokens[current];

    match token {
        Token::Integer(value) => {
            let current = current + 1;
            Ast::IntegerLiteral(i64::from_str_radix(value, 10).unwrap())
        },
        Token::Char(value) => {
            let current = current + 1;
            Ast::StringLiteral(value.to_string())
        }
        Token::Paren('(') => {
            let current = current + 1;
            let token = &tokens[current]; // skip the ( and get the next token, which shoulld be a name

            // save the call expression name as a legit String
            let call_expression_name = match token {
                Token::Name(name) => name.to_string(),
                token => panic!("Expected Token::Name, but got {:?}", token),
            };

            let current = current + 1;
            let token = &tokens[current];

            let mut params: Vec<Ast> = Vec::new();

            loop {
               params.push(walk(current, tokens)); 
            }
        },
        _ => panic!("oh no")
    }
}

fn parser(tokens: Vec<Token>) -> Ast {
    Ast::Program(vec![walk(0, &tokens)])
}
