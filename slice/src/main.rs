use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num: u32 = args[2].trim().parse()
        .expect("Please type a number!");

    println!("{}", print_nth_word(&args[1], num));
}

fn print_nth_word(s: &String, num: u32) -> &str {
    let mut spaces = 0;
    let mut last_i = 0;

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            spaces += 1;

            if spaces == num {
                return &s[last_i..i];
            }

            last_i = i;
        }
    }

    if spaces == num - 1{
        &s[last_i..]
    } else {
        &s[..]
    }
}
