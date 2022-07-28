#[derive(PartialEq)]
pub struct token_t {
    pub _data: char,
    pub _token: Option<Token>, //
}

// tokenize a string
pub fn tokenize(s: String) -> Vec<token_t> {
    let mut x: Vec<token_t> = Vec::new();
    for c in s.chars() {
        let mut obj = token_t{_data: c, _token: Some(Token::Character)};
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


pub fn is_operator(c: &token_t) -> Token {
    match c._data {
        '+' => return Token::Symbol(Operator::Add),
        '-' => return Token::Symbol(Operator::Sub),
        '*' => return Token::Symbol(Operator::Mult),
        '/' => return Token::Symbol(Operator::Div),
        _ =>   return Token::Character
    }
  
}

pub fn set_operator(c: &mut token_t) {
    c._token = Some(is_operator(c));
    
}

pub fn get_prec(c: &token_t) -> u8  {
    let x: u8 = match c._token.as_ref().unwrap() {
        Token::Symbol(Operator::Add)  => 1,
        Token::Symbol(Operator::Sub)  => 1,
        Token::Symbol(Operator::Mult) => 2,
        Token::Symbol(Operator::Pow)  => 2,
        Token::Symbol(Operator::Div)  => 3, 
        _ => 0, // invalid
    };
    x
}

