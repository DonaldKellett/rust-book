use std::io;

const OS_SHOULD_READ_INPUT: &str = "The operating system should be able to read user input";

fn main() {
    let stdin = io::stdin();
    let mut is_to_celsius = String::new();
    let is_to_celsius: bool = loop {
        println!("Are you converting from Fahrenheit to Celsius? (true/false)");
        stdin
            .read_line(&mut is_to_celsius)
            .expect(OS_SHOULD_READ_INPUT);
        match is_to_celsius.trim().parse() {
            Ok(answer) => break answer,
            Err(_) => {
                println!("Invalid response - expected true or false");
                is_to_celsius.clear();
                continue;
            }
        }
    };
    if is_to_celsius {
        let mut f = String::new();
        let f = loop {
            println!("Please enter a temperature in Fahrenheit:");
            stdin.read_line(&mut f).expect(OS_SHOULD_READ_INPUT);
            match f.trim().parse() {
                Ok(temp) => break temp,
                Err(_) => {
                    println!("Incorrect response - floating point value expected");
                    f.clear();
                    continue;
                }
            }
        };
        println!(
            "{f} degrees Fahrenheit is {} degrees Celsius",
            to_celsius(f)
        );
    } else {
        let mut c = String::new();
        let c = loop {
            println!("Please enter a temperature in Celsius:");
            stdin.read_line(&mut c).expect(OS_SHOULD_READ_INPUT);
            match c.trim().parse() {
                Ok(temp) => break temp,
                Err(_) => {
                    println!("Incorrect response - floating point value expected");
                    c.clear();
                    continue;
                }
            }
        };
        println!(
            "{c} degrees Celsius is {} degrees Fahrenheit",
            from_celsius(c)
        );
    }
}

fn to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn from_celsius(c: f64) -> f64 {
    c * 1.8 + 32.0
}
