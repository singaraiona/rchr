use std::str;
use parse::error::Error as ParseError;

#[derive(Debug)]
pub enum Kind {
    Simplification,
    Propagation,
    Simpagation,
}

impl str::FromStr for Kind {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..] {
            "<=>" => Ok(Kind::Simplification),
            "==>" => Ok(Kind::Propagation),
            x => Err(ParseError::ParseError(format!("Invalid rule kind: {}", x))),
        }
    }
}

#[derive(Debug)]
pub enum CHR {
    Rule {
        kind: Kind,
        name: Option<String>,
        head: (Vec<CHR>, Vec<CHR>),
        guard: Vec<CHR>,
        body: Vec<CHR>,
    },
    Name(String),
    List(Vec<CHR>),
    Nil,
}

pub fn rule(k: Kind, n: CHR, h: CHR, r: CHR, g: CHR, b: CHR) -> CHR {
    let name = match n {
        CHR::Name(s) => Some(s),
        _ => None,
    };
    let head = match h {
        CHR::List(v) => v,
        x => vec![x],
    };
    let replace = match r {
        CHR::List(v) => v,
        x => vec![x],
    };
    let guard = match g {
        CHR::List(v) => v,
        x => vec![x],
    };
    let body = match b {
        CHR::List(v) => v,
        x => vec![x],
    };
    CHR::Rule {
        kind: k,
        name: name,
        head: (head, replace),
        guard: guard,
        body: body,
    }
}

pub fn empty_list() -> CHR {
    let v: Vec<CHR> = Vec::new();
    CHR::List(v)
}

pub fn list(v: Vec<CHR>) -> CHR {
    CHR::List(v)
}
