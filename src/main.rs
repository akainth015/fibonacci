use std::io;
use std::io::Write;
use std::fmt::Debug;

fn main() {
    print!("Calculate the (nth) fibonacci number: ");
    io::stdout().flush()
        .expect("Failed to write to standard output");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read from standard input");

    let n: u32 = n.trim().parse()
        .expect("Failed to read from standard input");

    let term = if n == 0 { 0 } else if n == 1 { 1 } else {
        let mut t1 = 0;
        let mut t2 = 1;

        for _ in 1..n {
            t2 = t1 + t2;
            t1 = t2 - t1;
        }

        t2
    };

    let n_string = if n > 10 && n < 20 {
        format!("{}th", n)
    } else if n % 10 == 1 {
        format!("{}st", n)
    } else if n % 10 == 2 {
        format!("{}nd", n)
    } else if n % 10 == 3 {
        format!("{}rd", n)
    } else {
        format!("{}th", n)
    };

    println!("The {} fibonacci number is {}", n_string, term)
}
