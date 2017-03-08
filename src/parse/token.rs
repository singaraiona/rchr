use regex::Regex;
use std::str::FromStr;
use parse::error::Error;
use std::str::pattern::{Pattern, ReverseSearcher};

lazy_static! {
    // Tokens
    static ref TOKENS: Vec<Regex> = vec![r"^(true|false)",                         // 0 - BOOL
                                         r"^-?(0w|0N|\d+\.\d*|\d*\.?\d)",          // 2 - NUMBER
                                         r"^[+\x2D*%!&|<>=~,^#_$?@.]{1,2}",        // 6 - VERB 
                                         r"^[a-z][a-z\d]*",                        // 3 - NAME
                                         r",",                                     // 4 - COMMA     
                                         r"^@",                                    // 4 - PREFIX 
                                         r"^\\",                                   // 4 - REPLACE
                                         r"^<?=+>",                                // 5 - SUFFIX 
                                         r"^\|",                                   // 6 - GUARD
                                         r"^`([a-zA-Z0-9.]*)?",                    // 4 - SYMBOL 
                                         r"^\x22(\\.|[^\x5C\x22])*\x22",           // 5 - STRING
                                         r"^;",                                    // 10- SEMI
                                         r"^\(",                                   // 16- OPEN_P
                                         r"^\)",                                   // 19- CLOSE_P 
                                         r"^\\[a-z]+",                             // 21- SYSTEM                                         
                                         r"^\\\\",]                                // 22- QUIT
                                         .iter().map(|x| Regex::new(x).unwrap()).collect();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Token {
    Bool,
    Number,
    Verb,
    Name,
    Comma,
    Prefix,
    Replace,
    Suffix,
    Guard,
    Symbol,
    String,
    Semi,
    OpenP,
    CloseP,
    System,
    Quit,
    Nil,
}

impl Token {
    #[inline]
    fn re(&self) -> &Regex {
        TOKENS.get(*self as usize).expect("Invalid token.")
    }

    pub fn find(&self, s: &str) -> Option<Raw> {
        match self.re().find(s) {
            Some(m) => Some(Raw(s[m.start()..m.end()].to_string())),
            None => None,
        }
    }

    pub fn is_match(&self, s: &str) -> bool {
        self.re().is_match(s)
    }
}

#[derive(Debug)]
pub struct Raw(String);

impl Raw {
    pub fn parse<T: FromStr>(&self) -> Result<T, Error> {
        match self.0.parse::<T>() {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::ParseError(format!("Can not parse type from `{}", self.0))),
        }
    }

    pub fn trim_right_matches<'a, P>(&'a self, pat: P) -> Self
        where P: Pattern<'a>,
              P::Searcher: ReverseSearcher<'a>
    {
        let s = self.0.trim_right_matches(pat);
        Raw(s.to_string())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}