extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn main() {
//     println!("{}", is_pandigital(39, 186));
//     println!("{}", is_pandigital(1, 1));

    let mut nums = ['1','2','3','4','5','6','7','8','9'];
    let mut permutations: Vec<String> = Vec::new();
    loop {
        permutations.push(nums.iter().cloned().collect());
        if !nums.next_permutation() {
            break;
        }
    }
//     println!("{:?}", permutations);

    let mut prods = Vec::new();
    for perm in permutations {
        for i in 1..8 {
            let (lhs, rest) = perm.split_at(i);
            for j in i+1..9 {
                let (rhs, prd) = rest.split_at(j - i);
                if lhs.parse::<u64>().unwrap() * rhs.parse::<u64>().unwrap() == prd.parse::<u64>().unwrap() {
                    prods.push(prd.parse().unwrap());
                }
            }
        }
    }
    prods.sort();
    prods.dedup();

    println!("the answer is {}", prods.iter().sum::<u64>());
}
