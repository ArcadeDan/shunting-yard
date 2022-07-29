#[derive(PartialEq)]
pub struct Token_t {
    pub data: char,
    pub token: Option<Token>, //
}

// tokenize a string
pub fn tokenize(s: String) -> Vec<Token_t> {
    let mut x: Vec<Token_t> = Vec::new();
    for c in s.chars() {
        let mut obj = Token_t{data: c, token: Some(Token::Character)};
        //evaluate the type of token first
        
        x.push(obj);
    }
    x
}
#[derive(Clone, PartialEq)]
pub enum Operator {
    Add, Sub, 
    Mult, Pow, 
    Div
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Character,
    Symbol(Operator)
}


pub fn is_operator(c: &Token_t) -> Token {
    match c.data {
        '+' => return Token::Symbol(Operator::Add),
        '-' => return Token::Symbol(Operator::Sub),
        '*' => return Token::Symbol(Operator::Mult),
        '/' => return Token::Symbol(Operator::Div),
        _ =>   return Token::Character
    }
  
}

pub fn set_operator(c: &mut Token_t) {
    c.token = Some(is_operator(c));
    
}

pub fn get_prec(c: &Token_t) -> u8  {
    let x: u8 = match c.token.as_ref().unwrap() {
        Token::Symbol(Operator::Add)  => 1,
        Token::Symbol(Operator::Sub)  => 1,
        Token::Symbol(Operator::Mult) => 2,
        Token::Symbol(Operator::Pow)  => 2,
        Token::Symbol(Operator::Div)  => 3, 
        _ => 0, // invalid
    };
    x
}

