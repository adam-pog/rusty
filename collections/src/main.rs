fn main() {
    let v = vec![1];
    let mut v2 = vec![v];
    v2[0][0] = 2;
    println!("Hello, world! {:?}", v2);

    let mut v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    v.push(5);
    println!("The third element is {}", third);

    // looping over vector
    let v = vec![100, 32, 57];
    for i in &v { println!("{}", *i); }

    // will break since i will not be a reference
    // for i in v { println!("{}", *i); }

    // mutable reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 20;
        println!("{}", i);
    }


    // scalars!
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    //println!("{}", s1); not valid since s1 has been moved
}
