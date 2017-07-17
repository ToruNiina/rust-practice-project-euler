use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("problem011.dat").unwrap();
    let file = BufReader::new(&f);

    let nums = file.lines()
        .map(|l| l.unwrap()
             .split(' ')
             .map(|c| c.parse::<u64>().unwrap())
             .collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();

    println!("{:?}", nums);

    let win_size = 4;
    let mut greatest_prod = 0;

    for i in nums.iter() {
        for j in i.windows(win_size) {
            let p = j.iter().fold(1, |prod, item| prod * item);
            if p > greatest_prod {greatest_prod = p;}
        }
    }

    for i in 0..nums.len() - win_size {
        for j in 0..nums[i].len() {
            let p = vec![nums[i][j], nums[i+1][j], nums[i+2][j], nums[i+3][j]]
                    .iter().fold(1, |prod, item| prod * item);
            if p > greatest_prod {greatest_prod = p;}
        }
    }

    for i in 0..nums.len() - win_size {
        for j in 0..nums[i].len() - win_size {
            let p = vec![nums[i][j], nums[i+1][j+1], nums[i+2][j+2], nums[i+3][j+3]]
                    .iter().fold(1, |prod, item| prod * item);
            if p > greatest_prod {greatest_prod = p;}
        }
    }

    for i in 0..nums.len() - win_size {
        for j in 3..nums[i].len() {
            let p = vec![nums[i][j], nums[i+1][j-1], nums[i+2][j-2], nums[i+3][j-3]]
                    .iter().fold(1, |prod, item| prod * item);
            if p > greatest_prod {greatest_prod = p;}
        }
    }


    println!("the answer is {}", greatest_prod);
}
