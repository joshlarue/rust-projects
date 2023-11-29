use std::io;

fn main() {
    println!("Enter the nth fibonacci number you'd like to calculate up to!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    println!("\n");
    println!("Calculating results...");
    let mut fib_num = 1;
    let mut last_fib_num = 0;
    let mut tmp_fib_num = 0; // throws warning saying tmp_fib_num is never read. that's the point
    for n in 1..=input {
        if n != input {
            println!("{fib_num}");
            tmp_fib_num = fib_num;
            fib_num += last_fib_num;
            last_fib_num = tmp_fib_num;
        } else {
            println!("Result: {fib_num}");
        }
    }
}
