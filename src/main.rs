use std::{collections::VecDeque, io, ops::Deref, str::Chars};

#[derive(Debug, Copy, Clone, PartialEq)]
enum TSet {
    INT,
    OPERATOR(Operators),
    FLOAT,
    OPARAM,
    CPARAM,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Operators {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    MOD
}
#[derive(PartialEq, Debug)]
enum Assoc {
    LEFT,
    RIGHT,
    NONE,
}

impl TSet {
    fn getPrec(&self) -> Option<usize> {
        match &self {
            Self::OPERATOR(Operators::POW) => return Some(4),
            Self::OPERATOR(Operators::MUL) => return Some(3),
            Self::OPERATOR(Operators::DIV) => return Some(3),
            Self::OPERATOR(Operators::ADD) => return Some(2),
            Self::OPERATOR(Operators::SUB) => return Some(2),
            _ => None,
        }
    }
    fn getAssoc(&self) -> Option<Assoc> {
        match &self {
            Self::OPERATOR(Operators::POW) => return Some(Assoc::RIGHT),
            Self::OPERATOR(Operators::MUL) => return Some(Assoc::LEFT),
            Self::OPERATOR(Operators::DIV) => return Some(Assoc::LEFT),
            Self::OPERATOR(Operators::ADD) => return Some(Assoc::LEFT),
            Self::OPERATOR(Operators::SUB) => return Some(Assoc::LEFT),
            _ => None,
        }
    }

    fn is_operator(&self) -> bool {
        match self {
            Self::INT | Self::OPARAM | Self::CPARAM => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Token {
    start_pos: usize,
    end_pos: usize,
    t_type: TSet,
}

impl Token {
    fn new(start_pos: usize, end_pos: usize, t_type: TSet) -> Self {
        Self {
            start_pos,
            end_pos,
            t_type,
        }
    }
}

fn guard_integer(c: char) -> bool {
    return c.is_numeric() || ['.', '_'].contains(&c);
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
        '*' => Some(TSet::OPERATOR(Operators::MUL)),
        '/' => Some(TSet::OPERATOR(Operators::DIV)),
        '^' => Some(TSet::OPERATOR(Operators::POW)),
        '(' => Some(TSet::OPARAM),
        ')' => Some(TSet::CPARAM),
        '-' => match accumulator.last() {
            None => Some(TSet::INT),
            // if a previous token exists
            Some(x) => match x.t_type {
                TSet::INT | TSet::CPARAM => Some(TSet::OPERATOR(Operators::SUB)),
                _ => Some(TSet::INT),
            },
        },
        _ if guard_integer(c) => tokenize_num(accumulator.last_mut()),
        x => panic!("unknown token[{}:{}] '{}'", i, i, x),
    };
    if let Some(TSet) = token {
        accumulator.push(Token::new(i, i + 1, TSet))
    }
    accumulator
}
// shunting yard function | parses into postfix
fn parse(tokens: & [Token]) -> Vec<Token> {
    let mut operator_stack = Vec::<Token>::new();
    let mut output_queue = Vec::<Token>::new();
    
    for toke in tokens {
        match toke.t_type {
            // condition:
            // when operator o2 other than the left parenthesis
            // at the top of the operator stack and (o2 has greater precedence
            // than o1 or they have same precedence and o1 is left-assoc)
            //
            o1 if o1.is_operator() => {
                while let Some(o2) = operator_stack.last() {
                    let op = {
                        let o2 = o2.to_owned().t_type;
                        let po1 = o1.getPrec();
                        let po2 = o2.getPrec();
                        if po2 > po1 {
                            operator_stack.pop().unwrap()
                        } else if po2 == po1 && o1.getAssoc() == Some(Assoc::LEFT) {
                            operator_stack.pop().unwrap()
                        } else {
                            break;
                        }
                    };
                    output_queue.push(op);
                }
                operator_stack.push(*toke);
            }
            TSet::INT => output_queue.push(*toke),
            TSet::OPARAM => operator_stack.push(*toke),
            TSet::CPARAM => {
                while let Some(op) = operator_stack.last() {
                    if op.t_type == TSet::OPARAM 
                    { break }
                    output_queue.push(operator_stack.pop().unwrap());
                } //operator_stack.push_front(toke.clone()),
                operator_stack.pop()
                    .expect("Expected '(' in operator stack");  
            }
            _ => unreachable!(),
        }
    }
    /*
    for toke in tokens.iter() {
        print!("[{:?}] ", toke.t_type);
    }
    print!("\n");

    for op in operator_stack.iter() {
        print!("[{:?}]", op.t_type)
    }
    print!("\n");
    for out in output_queue.iter() {
        print!("[{:?}]", out.t_type)
    }
    print!("\n");
    */
    //let mut resultvec = Vec<>::new();
    output_queue.extend(operator_stack.drain(..).rev());
    return output_queue.clone()
}

fn to_int(data: &str, toke: &Token) -> f32 {
    data[toke.start_pos..toke.end_pos]
        .chars()
        .filter(|&c| c != '_')
        .collect::<String>()
        .parse::<f32>()
        .expect("Cannot parse integer")
}

fn evaluate(data: &str, postfix: VecDeque<Token>) -> f32 {
    let mut stack = Vec::new();
    for token in postfix.iter() {
        if let TSet::INT = token.t_type {
            stack.push(to_int(data, token));
            continue;
        }
        let rhs = stack.pop();
        let lhs = stack.pop();

        match (lhs, rhs) {
            (Some(a), Some(b)) => {
                let result = match token.t_type {
                    TSet::OPERATOR(Operators::ADD) => a + b,
                    TSet::OPERATOR(Operators::SUB) => a - b,
                    TSet::OPERATOR(Operators::DIV) => a / b,
                    TSet::OPERATOR(Operators::MUL) => a * b,
                    TSet::OPERATOR(Operators::POW) => a.powf(b),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            (None, Some(b)) => return b,
            (None, None) | (Some(_), None) => unreachable!(),
        }
    }
    stack.pop().expect("no integers left")
}

fn main() -> io::Result<()> {
    use std::io::{stdin, stdout, Write};
    let mut buffer = String::new();
    //let mut operator_stack = VecDeque::<Token>::new();
    //let mut output_queue = VecDeque::<Token>::new();

    loop {
        stdout().write(b"> ")?;
        stdout().flush()?;
        stdin().read_line(&mut buffer);

        if buffer.trim().eq("!q") {
            break;
        }
        
        // token creation
        let tokens: Vec<Token> = buffer
            .chars()
            .enumerate()
            .fold(Vec::new(), |acc, (i, c)| tokenize(acc, i, c));

        let postfix = parse(&tokens);
        for out in postfix.iter() {
            println!("{:?}", out);
        }

        let result = evaluate(&buffer, (postfix.into()));
        println!("{}", result);

        
        
        buffer.clear();
        //output_queue.clear();
        //operator_stack.clear();
    }
    Ok(())
}
