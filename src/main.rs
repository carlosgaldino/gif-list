extern crate serde;
extern crate regex;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;
use serde::json::{self, Value};
use regex::Regex;

fn main() {
    let mut file    = File::open(Path::new(&env::home_dir().unwrap()).join(".gifs")).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content);

    let data: Value = json::from_str(&content).unwrap();
    let obj         = data.as_object().unwrap();
    let search      = match env::args().skip(1).next() {
        Some(term)  => term,
        None        => String::from(".*"),
    };

    let regex   = Regex::new(&search).unwrap();
    let entries = obj.iter().filter(|&(k, _)| regex.is_match(k));
    let items   = entries.fold(String::new(), |acc, (k, v)| acc + &format!("<item arg ={:?} uid ='{}'>\n<title>{}</title>\n<subtitle>{:?}</subtitle>\n</item>", v, k, k, v));

    println!("<?xml version='1.0'?>\n<items>\n{}</items>", items);
}
