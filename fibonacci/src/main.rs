use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num: u32 = args[1].trim().parse()
        .expect("Please type a number!");

    fib(num);
    println!("{}", fib_recursive(num));
}

fn fib(mut n: u32) {
    let mut result = 1;
    let mut prev = 1;

    while n > 1 {
      result += prev;
      prev = result - prev;
      n -= 1;
    }

    println!("{}", result);
}

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
