use std::io::Read;

const INIMEMSIZE: usize = 7;

// This code is uglified to comply with
// APCSP algo requirements. Annoying...

static mut x: usize = 0;
static mut y: usize = 0;
static mut z: usize = 0;
static mut ip: usize = 0;
static mut mp: usize = 0;
static mut lp: usize = 0;
// We let the caller declare memory space

unsafe fn msc_run(ins: &Vec<char>, mem: &mut Vec<usize>) -> Result<(), &'static str> {
    let ilen: usize = ins.len();
    // hacky workaround
    let mut stdin: std::io::Stdin = std::io::stdin();
    let mut buf: [u8; 1] = [0; 1];
    while ip != ilen {
        match ins[ip] {
            'x' => x = mem[mp],
            'y' => y = mem[mp],
            'z' => z = mem[mp],
            'a' => mem[mp] = x,
            'b' => mem[mp] = y,
            'c' => mem[mp] = z,
            '+' => mem[mp] += 1,
            '-' => mem[mp] -= 1,
            'p' => mem[mp] = ip,
            'j' => ip = mem[mp],
            '*' => mem[mp] <<= 1,
            '/' => mem[mp] >>= 1,
            '.' => {
                mp += 1;
                if mem.len() >= mp {
                    mem.push(0usize);
                }
            }
            ',' => {
                if mp == 0 {
                    return Err("mindscrew: out of bounds error");
                }
                mp -= 1;
            }
            '{' => lp = ip,
            '}' => {
                if mem[mp] != 0 {
                    ip = lp;
                } 
	    },
            'r' => {
                stdin.read(&mut buf[..]);
                mem[mp] = buf[0] as usize;
            }
            'w' => {
                char_handle((mem[mp] as u8) as char);
            }
            // anything else is simply treated as whitespace
            _ => {},
        };
        ip += 1;
    }
    Ok(())
}

fn char_handle(c: char) {
    match c {
        '\u{0A}' => println!(),
        '\u{0D}' => panic!("mindscrew: cr/crlf not supported"),
        _ => print!("{}", c)
    }
}

fn main() {
    // in future releases, would pull the package name
    // and version from Cargo - but that doesn't fly for
    // APs and APCSP work due to single-file restrictions
    println!("mindscrew v0.1");
    
    // Claim memory space
    let mut mem: Vec<usize> = vec![0; INIMEMSIZE];
    
    let arg: String = std::env::args().nth(1).unwrap();
    println!("mindscrew: loading msc file {}", arg);
    // Load program code
    let ins: Vec<char> = std::fs::read_to_string(arg)
        .unwrap()
        .chars()
        .collect();
    unsafe {
        match msc_run(&ins, &mut mem) {
            Ok(()) => {},
            Err(e) => print!("{}", e),
        };
    }
    println!("{:?}", mem);
}
