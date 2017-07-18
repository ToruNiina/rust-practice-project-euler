use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let file = File::open("problem018.dat").unwrap();
    let buf  = BufReader::new(&file);
    let nums: Vec<Vec<u64>> = buf.lines()
        .map(|l| l.unwrap().split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();
    println!("{:?}", nums);

    let mut results: Vec<Vec<u64>> = Vec::new();
    results.push(nums.first().unwrap().clone());

    for (i, line) in nums.iter().enumerate() {
        if i == 0 {continue;}
        let line_end = line.len() - 1;
        let mut ln = line.clone();

        for (j, n) in ln.iter_mut().enumerate() {
            if j == 0 {
                *n += *results[i-1].first().unwrap();
            } else if j == line_end {
                *n += *results[i-1].last().unwrap();
            } else {
                *n += cmp::max(results[i-1][j-1], results[i-1][j]);
            }
        }
        println!("{:?}", ln);
        results.push(ln);
    }
    println!("{}", results.last().unwrap().iter().max().unwrap());
}
