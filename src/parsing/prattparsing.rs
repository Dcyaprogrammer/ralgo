#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

enum Expression {
    Atom(char),
    Operation(char, Vec<Expression>),
}

impl Expression {
    fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }
}

struct Lexer{
    tokens: Vec<Token>,
}

impl Lexer{
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it|!it.is_ascii_whitespace())
            .map(|c| match c{
                '0'..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' => Token::Atom(c),
                _ => Token::Op(c),
            }).collect::<Vec<_>>();
        // reverse so we can pop
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
} 

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        _ => panic!("Unknown operator: {}", op),
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(')') => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };
        lexer.next();
        let (lhs_bp, rhs_bp) = infix_binding_power(op);
        if rhs_bp < min_bp {
            break;
        }
        let rhs = parse_expression(lexer, rhs_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}



