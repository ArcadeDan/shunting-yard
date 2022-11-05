use std::{io, str::Chars, collections::VecDeque};

enum TSet {
    INT, 
    WORD, 
    OPERATOR(Operators), 
    WSPACE,
    FLOAT
}

enum Operators {
    ADD, SUB, MUL, DIV
}


struct Token {
    start_pos: usize,
    end_pos: usize,
    t_type: TSet,
}



impl Token {
    fn new(start_pos: usize, end_pos: usize, t_type: TSet) -> Self {
        Self { start_pos, end_pos, t_type }
    }
}

fn is_operator(string_iter: &str, pos: usize) -> bool {
    match string_iter.chars().nth(pos).unwrap() {
        '+' => return true,
        '-' => return true,
        '*' => return true,
        '/' => return true,
         _  => return false
    }
    
}

fn guard_integer(c: char) -> bool {
    return c.is_numeric() || [',', '_'].contains(&c)
}

fn tokenize_num(prev: Option<&mut Token>) -> Option<TSet> {
    match prev {
        None => Some(TSet::INT),
        Some(last) => {
            if let TSet::INT = last.t_type {
                last.end_pos += 1;
                return None; 
            }
        return Some(TSet::INT);
        }
    }
}

fn tokenize(mut accumulator: Vec<Token>, i: usize, c: char) -> Vec<Token> {
    
    let token = match c {
        ' ' | '\n' => None,
        '+' => Some(TSet::OPERATOR(Operators::ADD)),
        '-' => Some(TSet::OPERATOR(Operators::SUB)),
        '*' => Some(TSet::OPERATOR(Operators::MUL)),
        '/' => Some(TSet::OPERATOR(Operators::DIV)),
        _ if guard_integer(c) => tokenize_num(accumulator.last_mut()),
        x => panic!("unknown token[{}:{}] '{}'", i, i, x)
    };
    
    if let Some(TSet) = token {
        accumulator.push(Token::new(i, i + 1, TSet))
    }
   
    accumulator
}



fn main() -> io::Result<()> {
    use std::{io::{stdin, stdout, Write,} };
    
    let mut buffer = String::new();
    let mut operator_stack = Vec::<Token>::new();
    let mut output_queue = VecDeque::<Token>::new();

    loop {
        stdout().write(b"> ")?;
        stdout().flush()?;
        stdin().read_line(&mut buffer);

        let tokens: Vec<Token> = buffer.chars()
            .enumerate()
            .fold(Vec::new(), |acc, (i, c)| tokenize(acc, i, c));

        for i in tokens {
            println!("Start: {} End: {}", i.start_pos, i.end_pos );
        }

        let mut resultvec = Vec::new();


        if buffer.trim().eq("!q") {break}
        buffer.clear();
        


    }


    Ok(())
    
}

