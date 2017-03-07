use parse::error::Error;
use exec::chr::{self, Rule, CHR};
use parse::token::{Token, Raw};
use std::str;

pub struct Parser {
    text: String,
}

impl Parser {
    fn begin(&mut self, s: &str) {
        self.text = s.to_string();
    }

    #[inline]
    fn expect(&mut self, t: Token) -> Result<Raw, Error> {
        match t.find(&self.text) {
            Some(x) => {
                self.text = self.text[x.len()..].trim_left().to_string();
                Ok(x)
            }
            None => Err(Error::ParseError(format!("Expected: {:?}", t))),
        }
    }

    #[inline]
    fn at(&self, t: Token) -> bool {
        t.is_match(&self.text)
    }

    #[inline]
    fn matches(&mut self, t: Token) -> Option<Raw> {
        if self.at(t) {
            return match self.expect(t) {
                Ok(x) => Some(x),
                Err(..) => None,
            };
        }
        None
    }

    #[inline]
    fn done(&self) -> bool {
        self.text.is_empty()
    }

    fn parse_noun(&mut self) -> Result<CHR, Error> {
        println!("PARSE_NOUN TEXT: {:?}", self.text);
        if self.at(Token::Name) {
            let n = try!(self.expect(Token::Name));
            let t = try!(n.parse::<String>());
            if self.matches(Token::OpenP).is_some() {
                let v = try!(self.parse_list(Some(Token::CloseP), Token::Comma));
                return Ok(v);
            }
            return Ok(CHR::Name(t));
        }
        Err(Error::ParseError(format!("Unexpected token {}", self.text)))
    }

    fn parse_ex(&mut self, node: CHR) -> Result<CHR, Error> {
        println!("PARSE_EX TEXT: {:?}", self.text);
        if self.matches(Token::NameSep).is_some() {
            return Ok(node);
        }
        if self.matches(Token::RuleSep).is_some() {
            return Ok(CHR::Rule(Rule::Propagation));
        }
        self.parse_list(None, Token::Comma)
    }

    fn parse_list(&mut self, terminal: Option<Token>, sep: Token) -> Result<CHR, Error> {
        let mut vec: Vec<CHR> = Vec::new();
        loop {
            if terminal.is_some() && self.at(terminal.unwrap()) {
                vec.push(CHR::Nil);
                break;
            }
            let n = try!(self.parse_noun());
            match self.parse_ex(n) {
                Ok(a) => vec.push(a),
                Err(e) => return Err(e),
            }
            if self.done() || self.matches(sep).is_none() {
                break;
            }
        }
        if let Some(x) = terminal {
            let _ = try!(self.expect(x));
        };
        match vec.len() {
            0 => Ok(CHR::Nil),
            1 => Ok(vec.pop().unwrap()),
            _ => Ok(CHR::List(vec)),
        }
    }

    pub fn parse(&mut self, b: &[u8]) -> Result<CHR, Error> {
        self.begin(str::from_utf8(b).expect("Invalid input."));
        self.parse_list(None, Token::Semi)
    }
}

pub fn new() -> Parser {
    Parser { text: String::new() }
}