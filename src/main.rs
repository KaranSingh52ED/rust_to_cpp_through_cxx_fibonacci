use std::io::{self, Write};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-fibSeq/myFibSeqProgram/fibSeq.h");

        fn fibonacci(n: i32) -> Vec<i32>;
    }
}
fn main() {
    let mut n = String::new();
    print!("ENTER ANY VALID NUMBER: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut n)
        .expect("FAILED TO READ INPUT NUMBER   ");
    let n: i32 = n.trim().parse().expect("Enter a valid number");

    let sequence = ffi::fibonacci(n);
    println!("{:?}", sequence);
}
