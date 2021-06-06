//use std::fs;
use crate::Parser;

pub struct Lexer {
    _file: String,
    _parser: Parser,
}

impl Lexer {
    pub fn new(file: String, parser: Parser) -> Self {
        Self {
            _file: file,
            _parser: parser,
        } 
        
    }

    pub fn print(&self) {
        println!("File: {}", &self._file);
    }

}