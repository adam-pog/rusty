use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
use std::fs;
use std::string;

fn main() {
//     panic!("crash and burn");
//
//    let v = vec![1,2,3];
//    v[99];

    let f = File::open("hello.txt");
//    let f = File::open("hello.txt").unwrap();
//    let f = File::open("hello.txt").expect('like unwrap but with custom message);

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("tried to create file but there was a problem: {:?}", e)
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        }
    };
}

// alternatively
//let f = File::open("hello.txt").map_err(|error| {
//    if error.kind() == ErrorKind::NotFound {
//        File::create("hello.txt").unwrap_or_else(|error| {
//            panic!("Tried to create file but there was a problem: {:?}", error);
//        })
//    } else {
//        panic!("There was a problem opening the file: {:?}", error);
//    }
//});

// propogating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// better version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new():
//     f.read_to_string(&mut s)?;
// }

// even better version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new():

//     File::open("hello.txt")?.to_read_to_string(&mut s)?;
//     Ok(s)
// }

// super even better version
// fn read_username_from_file() -> result<string, io::error> {
//     fs::read_to_string("hello.txt")
// }
