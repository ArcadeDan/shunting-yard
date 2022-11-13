use std::{collections::VecDeque, io, str::Chars, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
enum TSet {
    INT,
    WORD,
    OPERATOR(Operators),
    WSPACE,
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
}
#[derive(PartialEq, Debug)]
enum Assoc {
    LEFT,
    RIGHT,
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

fn is_operator(string_iter: &str, pos: usize) -> bool {
    match string_iter.chars().nth(pos).unwrap() {
        '+' => return true,
        '-' => return true,
        '*' => return true,
        '/' => return true,
        '^' => return true,
        _ => return false,
    }
}

fn guard_integer(c: char) -> bool {
    return c.is_numeric() || [',', '_'].contains(&c);
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
        '^' => Some(TSet::OPERATOR(Operators::POW)),
        '(' => Some(TSet::OPARAM),
        ')' => Some(TSet::CPARAM),
        _ if guard_integer(c) => tokenize_num(accumulator.last_mut()),
        x => panic!("unknown token[{}:{}] '{}'", i, i, x),
    };
    if let Some(TSet) = token {
        accumulator.push(Token::new(i, i + 1, TSet))
    }
    accumulator
}
// shunting yard function | parses into postfix
fn parse(tokens: &[Token]) -> VecDeque<Token> {

    let mut operator_stack = VecDeque::<Token>::new();
    let mut output_queue = VecDeque::<Token>::new();

    for toke in tokens {
        match toke.t_type {
            TSet::OPERATOR(_) => {
                while let Some(op) = operator_stack.front() {
                    if op.t_type == TSet::OPARAM {
                        break;
                    }   // condition:
                    // when operator o2 other than the left parenthesis
                    // at the top of the operator stack and (o2 has greater precedence
                    // than o1 or they have same precedence and o1 is left-assoc)
                        //

                    match (
                        op.t_type.getPrec(),
                        toke.t_type.getPrec(),
                        toke.t_type.getAssoc(),
                    ) {
                        (Some(o2), Some(o1), Some(Assoc::LEFT)) if (o2 >= o1) => {
                            output_queue.push_back(*operator_stack.front().clone().unwrap());
                            operator_stack.pop_front();
                        }
                        _ => {}
                    }
                }
                operator_stack.push_front(toke.clone());
            }
            TSet::INT => output_queue.push_back(toke.clone()),
            TSet::OPARAM => operator_stack.push_front(toke.clone()),
            TSet::CPARAM => {
                while let Some(op) = operator_stack.front() {
                    if op.t_type == TSet::OPARAM {
                        break;
                    }
                    output_queue.push_front(operator_stack.pop_front().unwrap());
                } //operator_stack.push_front(toke.clone()),
                operator_stack.pop_front();
                if let Some(op) = operator_stack.front() {
                    match op.t_type {
                        TSet::OPERATOR(_) => {
                            output_queue.push_front(operator_stack.pop_front().unwrap());
                        }
                        _ => {}
                    }
                }
            }
            _ => unreachable!()
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
    return output_queue
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
            continue
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
                    _ => unreachable!()
                };
                stack.push(result);
            },
            (None, Some(b)) => return b,
            (None, None) | (Some(_), None) => unreachable!()
        }
    }
    stack.pop()
        .expect("no integers left")
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

        // token creation
        let tokens: Vec<Token> = buffer
            .chars()
            .enumerate()
            .fold(Vec::new(), |acc, (i, c)| tokenize(acc, i, c));
        
        let postfix = parse(&tokens);
        for out in postfix.iter() {
            println!("{:?}", out);
        }

        let result = evaluate(&buffer, dbg!(postfix));
        println!("{}", result);


        if buffer.trim().eq("!q") { break; }
        buffer.clear();
        //output_queue.clear();
        //operator_stack.clear();
    }
    Ok(())
}
