#[macro_use]
extern crate itertools;
#[macro_use]
extern crate rchr;

use itertools::Product;
use itertools::Itertools;
use std::clone::Clone;

use rchr::handle;
use rchr::chr::store::Store;
use rchr::chr::constraints::Constraint;

fn main() {
    let mut store = Store::new();
    store.push(Constraint::new(0));
    store.push(Constraint::new(0));
    store.push(Constraint::new(1));
    store.push(Constraint::new(0));
    store.push(Constraint::new(2));
    store.push(Constraint::new(2));
    let a = chr_iter!(store, 0);
    let b = chr_iter!(store, 1);
    let c = chr_iter!(store, 2);
    let it = head_iter![a.iter(), b.iter(), c.iter()];
    for elt in it {
        println!("ELT: {:?}", elt);
    }
}
