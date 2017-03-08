use parse::error::Error;
use exec::chr::{self, Kind, CHR};
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

    #[inline]
    fn parse_head(&mut self) -> Result<CHR, Error> {
        let mut vec: Vec<CHR> = Vec::new();
        while self.matches(Token::Comma).is_some() {
            let n = try!(self.parse_noun());
            vec.push(n);
        }
        match vec.len() {
            0 => Ok(CHR::Nil),
            1 => Ok(vec.pop().unwrap()),
            _ => Ok(CHR::List(vec)),
        }
    }

    fn parse_noun(&mut self) -> Result<CHR, Error> {
        if self.at(Token::Name) {
            let n = try!(self.expect(Token::Name));
            let t = try!(n.parse::<String>());
            if self.matches(Token::OpenP).is_some() {
                let v = try!(self.parse_list(Some(Token::CloseP), Token::Comma));
                return Ok(v);
            }
            return Ok(CHR::Name(t));
        }
        Err(Error::ParseError(format!("Unexpected token: `{}", self.text)))
    }

    fn parse_ex(&mut self, node: CHR) -> Result<CHR, Error> {
        // In case when rule name is specified
        if self.matches(Token::Prefix).is_some() {
            let head = try!(self.parse_list(None, Token::Comma));
            println!("HEAD: {:?}", head);
            let mut replace = chr::empty_list();
            if self.matches(Token::Replace).is_some() {
                println!("REPLACE");
                replace = try!(self.parse_list(None, Token::Comma));
            }
            let rt = try!(self.expect(Token::Suffix));
            let kind = try!(rt.parse::<Kind>());
            let body = try!(self.parse_list(None, Token::Comma));
            if self.matches(Token::Guard).is_some() {
                let b = try!(self.parse_list(None, Token::Comma));
                return Ok(chr::rule(kind, node, head, replace, body, b));
            }
            return Ok(chr::rule(kind, node, head, replace, CHR::Nil, body));
        }
        // In case without name and simple head
        if self.at(Token::Suffix) {
            let rt = try!(self.expect(Token::Suffix));
            let kind = try!(rt.parse::<Kind>());
            let body = try!(self.parse_list(None, Token::Comma));
            if self.matches(Token::Guard).is_some() {
                let b = try!(self.parse_list(None, Token::Comma));
                return Ok(chr::rule(kind, CHR::Nil, node, chr::empty_list(), body, b));
            }
            return Ok(chr::rule(kind, CHR::Nil, node, chr::empty_list(), CHR::Nil, body));
        }
        // In case without name and compound head
        if self.at(Token::Comma) {
            let h = try!(self.parse_head());
            return match h {
                CHR::List(mut v) => {
                    v.insert(0, node);
                    Ok(CHR::List(v))
                }
                x => Ok(CHR::List(vec![node, x])),
            };
        }
        // if self.at(Token::Verb) {
        //     let n = try!(self.expect(Token::Name));
        //     let t = try!(n.parse::<String>());
        //     // let
        // }
        Ok(node)
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