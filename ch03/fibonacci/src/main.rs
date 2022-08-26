use std::io;

fn main() {
    println!("Let's compute the nth Fibonacci number!");
    let stdin = io::stdin();
    let mut n = String::new();
    let n: u64 = loop {
        println!("Enter a natural number n:");
        stdin
            .read_line(&mut n)
            .expect("The operating system should be able to read user input");
        match n.trim().parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("Invalid input - expected a natural number");
                n.clear();
                continue;
            }
        }
    };
    println!("The {n}th Fibonacci number is {}", fibonacci(n));
}

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
