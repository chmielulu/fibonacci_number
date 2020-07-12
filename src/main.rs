use std::io;
use std::io::Write;

fn fib(n: i32) -> i32 { 
    if n <= 1 { return n; }

    fib(n-1) + fib(n-2) 
} 

fn main() {

    loop {

        print!("Type a number: ");
        io::stdout().flush().unwrap();
    
        let mut number = String::new();
    
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");
    
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Fibonacci number for {} is {}.", number, fib(number));
        break;

    }
    
}
