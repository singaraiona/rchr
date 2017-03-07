
#[derive(Debug)]
pub enum Rule {
    Simplification,
    Propagation,
    Simpagation,
}

#[derive(Debug)]
pub enum CHR {
    Rule(Rule),
    Name(String),
    List(Vec<CHR>),
    Nil,
}