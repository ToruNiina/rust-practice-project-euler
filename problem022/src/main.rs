use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn calc_value(s: &String) -> u64 {
    s.chars().map(|c| (c as u64) - 64).sum::<u64>()
}

fn main() {
    let file = File::open("p022_names.txt").unwrap();
    let buf  = BufReader::new(&file);
    let mut names = buf.split(b',')
                   .map(|w| String::from_utf8(w.unwrap()).unwrap().trim_matches('"').to_string())
                   .collect::<Vec<String>>();
    names.sort();
    println!("938th = {}, value = {}, score = {}",
             names[937], calc_value(&names[937]),
             938 * calc_value(&names[937]));

    let mut answer = 0;
    for (i, n) in names.iter().enumerate() {
        answer += ((i+1) as u64) * calc_value(n);
    }
    println!("the answer is {}", answer);
}
