const INIMEMSIZE: usize = 64;

fn main() {
    // Declare registers, claim memory space
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut z: usize = 0;
    let mut ip: usize = 0;
    let mut mem: Vec<usize> = vec![0; INIMEMSIZE];

    // Load program code
    let ins: Vec<char> = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap().chars().collect();
}
