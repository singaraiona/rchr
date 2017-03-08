#[macro_use]
extern crate itertools;
#[macro_use]
extern crate rchr;

use itertools::Product;
use itertools::Itertools;
use std::clone::Clone;

use std::io::{self, Read, Write};
use std::str;
use std::ascii::AsciiExt;

use rchr::handle;
use rchr::exec::store::Store;
use rchr::exec::constraints::Constraint;
use rchr::parse::parser;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn ps1() {
    print!("chr>");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut p = parser::new();
    let mut input = vec![0u8; 256];
    println!("RCHR\\ {}", VERSION);
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        let v = p.parse(&input[..size - 1]);
        match v {
            Ok(n) => {
                println!("------ Parse ------ \n{:#?}", n);
            }
            Err(e) => println!("'{}", format!("{:?}", e).to_ascii_lowercase()),
        }
        ps1();
    }
}
