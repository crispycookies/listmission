use std::fs::File;
use std::io::Write;
use regex::Regex;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let regex = Regex::new(r"([\\/.]+)").expect("Invalid regex");
    let splits: Vec<_> = regex.split(&*args[1]).into_iter().collect();


    let mut file = File::create("name").unwrap();
    file.write_all(splits[splits.len() - 2].as_ref()).unwrap();
}
