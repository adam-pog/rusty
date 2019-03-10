fn main() {
//   // inavlid
//   {
//       let r;                // ---------+-- 'a
//                             //          |
//       {                     //          |
//           let x = 5;        // -+-- 'b  |
//           r = &x;           //  |       |
//       }                     // -+       |
//                             //          |
//       println!("r: {}", r); //          |
//   }                         // --
//
//   // valid
//   {
//       let x = 5;            // ----------+-- 'b
//                             //           |
//       let r = &x;           // --+-- 'a  |
//                             //   |       |
//       println!("r: {}", r); //   |       |
//                             // --+       |
//   }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
