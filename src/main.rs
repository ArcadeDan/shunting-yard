use std::{io, collections::VecDeque, borrow::{Borrow, BorrowMut}};

mod Token;
use Token::tokenizer::*;

use crate::Token::tokenizer;


fn main() -> io::Result<()> {
    use std::{io::{stdin, stdout, Write,},
            };
    let mut _stringbuffer         = String::new();
    let mut _operator_stack = Vec::<token_t>::new();
    let mut _output_queue = VecDeque::<token_t>::new();

    loop {
        stdout().write(b"> ")?;
        stdout().flush()?;
        stdin().read_line(&mut _stringbuffer)?;
        
        
        // BEGIN  Tokenizer section
        let tokenarray = tokenize(_stringbuffer.clone().trim().to_string());
        for mut _i in tokenarray {
            set_operator(&mut _i);
            let _t = _i._token.clone().unwrap(); 
            match _t {
                tokenizer::Token::Symbol(Operator) =>  _operator_stack.push(_i),
                tokenizer::Token::Character => _output_queue.push_front(_i),
                _ => {}
            }
        }
        // END   Tokenizer section
        
        // BEGIN  program func section
        
        let mut _final = Vec::<char>::new();

            // first insert output queue's elements
            while _output_queue.back() != None {
                _final.push(_output_queue.back().unwrap()._data);
                _output_queue.pop_back();
            }

            while _operator_stack.first() != None {
                _final.push(_operator_stack.first().unwrap()._data);
                _operator_stack.pop();
            }
        
        print!("> "); 
        for _f in _final {
            print!("{}", _f);
        }
        print!("\n");
            
        // END    program func buffer
        if _stringbuffer.trim().eq("!q") {break} // eq("!q\n") {break}
        _stringbuffer.clear();
    }
    Ok(())
}

