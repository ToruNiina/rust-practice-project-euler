extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn main() {
    let mut data = [0,1,2,3,4,5,6,7,8,9];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }
    permutations.sort();

    println!("the answer is {:?}", permutations[999999]);

}
