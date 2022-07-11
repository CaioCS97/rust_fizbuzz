use std::io;

fn main() {
    println!("FizzBuzz test!");

    loop {
        println!("Set a max value.");

        let mut max = String::new();

        io::stdin()
            .read_line(&mut max)
            .expect("Failed to read line");

        let max: u32 = match max.trim().parse() {
            Ok(num) => {
                if num < 1000 {
                    num
                } else {
                    println!("The maximum is 999.");
                    continue;
                }
            }
            Err(_) => {
                println!("Please input only numbers.");
                continue;
            }
        };

        fizz_buzz(max);
        break;
    }
}

fn fizz_buzz(max: u32) {
    for element in 1..=max {
        if element % 15 == 0 {
            println!("FizzBuzz");
        } else if element % 5 == 0 {
            println!("Buzz");
        } else if element % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", element)
        }
    }
}
