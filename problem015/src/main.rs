fn main() {
    let lattice_size = 21;
    let mut lattice : Vec<Vec<u64>> = Vec::with_capacity(lattice_size);
    lattice.push(vec![1; lattice_size]);

    for i in 1..lattice_size {
        let mut line : Vec<u64> = Vec::with_capacity(lattice_size);

        line.push(lattice[i-1][0]);
        for j in 1..lattice_size {
            let prev = line[j-1];
            line.push(lattice[i-1][j] + prev);
        }
        lattice.push(line);
    }
    println!("the answer is {}", lattice.last().unwrap().last().unwrap());
}
