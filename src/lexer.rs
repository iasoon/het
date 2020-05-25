#[derive(Debug)]
pub enum Token {
    Whitespace(String),
    Identifier(String),
    Number(String),
    Symbol(char),
}

use Token::*;


#[derive(PartialEq, Eq)]
enum CharType {
    ALPHA,
    NUMERIC,
    WHITESPACE,
    SYMBOLIC,
    OTHER,
}

fn char_type(ch: &char) -> CharType {
    match ch {
        ' ' | '\t' |'\n' => CharType::WHITESPACE,
        'a'..='z'| 'A'..='Z' => CharType::ALPHA,
        '0'..='9' => CharType::NUMERIC,
        '(' | ')' | '{' | '}' | '[' | ']' => CharType::SYMBOLIC,
        '+' | '-' | '*' | '/'  => CharType::SYMBOLIC,
        _ => CharType::OTHER,
    }
}


struct Lexer<'a> {
    head: Option<char>, 
    tail: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(string: &'a str) -> Self {
        let mut chars = string.chars();
        Lexer {
            head: chars.next(),
            tail: chars,
        }
    }

    fn is_eos(&self) -> bool {
        self.head.is_none()
    }

    fn pos(&self) -> Option<char> {
        self.head
    }

    fn advance(&mut self) {
        self.head = self.tail.next();
    }

    fn match_head<P>(&self, pred: P) -> Result<char, String>
        where P: FnOnce(&char) -> bool
    {
        match self.head {
            Some(ch) if pred(&ch) => Ok(ch),
            Some(ch) => Err(format!("unexpected '{}'", ch)),
            None => Err("unexpected EOS".into()),
        }
    }

    fn read_while<P>(&mut self, mut pred: P) -> String
        where P: FnMut(&char) -> bool
    {
        let mut buf = String::new();
        self.read_while_mut(&mut buf, pred);
        return buf;
    }

    fn read_while_mut<P>(&mut self, buf: &mut String, mut pred: P)
        where P: FnMut(&char) -> bool
    {
        loop {
            match self.head {
                Some(ch) if pred(&ch) => {
                    buf.push(ch);
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn read_multiple<P>(&mut self, mut pred: P) -> Result<String, String>
        where P: FnMut(&char) -> bool
    {
        let first_ch = self.match_head(&mut pred)?;
        let mut buf = String::new();
        buf.push(first_ch);
        self.advance();
        self.read_while_mut(&mut buf, pred);
        return Ok(buf);
    }
}

fn lex_identifier<'a>(lexer: &mut Lexer<'a>) -> Result<String, String> {
    lexer.read_multiple(|ch| char_type(ch) == CharType::ALPHA)
}

fn lex_whitespace<'a>(lexer: &mut Lexer<'a>) -> Result<String, String> {
    lexer.read_multiple(|ch| char_type(ch) == CharType::WHITESPACE)
}

fn lex_number<'a>(lexer: &mut Lexer<'a>) -> Result<String, String> {
    lexer.read_multiple(|ch| char_type(ch) == CharType::NUMERIC)
}

fn lex_symbol<'a>(lexer: &mut Lexer<'a>) -> Result<char, String> {
    let ch = lexer.match_head(|ch| char_type(ch) == CharType::SYMBOLIC)?;
    lexer.advance();
    return Ok(ch);
}


fn lex<'a>(lexer: &mut Lexer<'a>) -> Result<Token, String> {
    if let Ok(text) = lex_whitespace(lexer) {
        return Ok(Whitespace(text))
    }
    if let Ok(text) = lex_identifier(lexer) {
        return Ok(Identifier(text))
    }
    if let Ok(text) = lex_number(lexer) {
        return Ok(Number(text))
    }
    if let Ok(text) = lex_symbol(lexer) {
        return Ok(Symbol(text))
    }
    return Err("lexing failed".into())
}

fn lex_all<'a>(lexer: &mut Lexer<'a>) -> Result<Vec<Token>, String> {
    let mut buf = Vec::new();
    while !lexer.is_eos() {
        let token = lex(lexer)?;
        buf.push(token);
    }
    return Ok(buf);
}


pub fn lex_str(string: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(string);
    return lex_all(&mut lexer);
}