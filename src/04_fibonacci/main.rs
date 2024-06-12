use std::io;

fn main() {

    println!("\nHi! I can tell you n-th fibonacci number.\n");

    loop {

        println!("And n is:\n");

        let n: u8 = loop {
            match stdin_read_line().trim().parse() {
                Ok(f) => match f {
                    // check possible overflow
                    0..=93 => break f,
                    _      => println!("\nI cannot see so far...\n")
                },
                Err(_) => { println!("\nEnter a small positive integer number:\n"); }
            }
        };

        println!("\n{}-th fibonacci is: {}\n", n, nth_fibonacci(n));
    }
}

fn nth_fibonacci(n: u8) -> u64 {

    let mut a1: u64 = 1;
    let mut a2: u64 = 0;
    let mut a3: u64 = 0;

    for _ in 0..n {
        a3 = a1 + a2;
        a1 = a2;
        a2 = a3;
    }

    a3
}

fn stdin_read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read stdin");
    line
}