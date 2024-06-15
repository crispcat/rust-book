use std::io;

pub fn stdin_read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read stdin");
    line
}

