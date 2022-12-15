use std::io;

fn fib(n: u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    println!("0th number: 0");
    for i in 1..n {
        match i % 2 == 0 {
            true => {
                a = a + b;
                println!("{i}th number: {a}");
            },
            false => {
                b = a + b;
                println!("{i}th number: {b}");
            },
        }
    }  
}

fn main() {
    println!("Give how many fib to print");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Could not read line");

    let num: u32 = input
        .trim()
        .parse()
        .expect("Could not parse input into int");

    fib(num);
}
