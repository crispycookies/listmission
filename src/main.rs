use regex::Regex;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let regex = Regex::new(r"([\\/.]+)").expect("Invalid regex");
    let splits: Vec<_> = regex.split(&*args[1]).into_iter().collect();

    println!("{}",splits[splits.len()-2])
}
