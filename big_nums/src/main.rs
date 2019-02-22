use rug::Integer;
use rug::rand::RandState;

fn main() {
    let a = Integer::from(Integer::u_pow_u(29, 40*80*410));
   // let a = a + Integer::from(2);
    //let b = Integer::from(31);
   // let c = a.to_string_radix(29);
    let mut rand = RandState::new();
//    let bound = a;
    let i = Integer::from(a.random_below_ref(&mut rand));
    let t = Integer::from(a.random_below_ref(&mut rand));
    println!("Hello, {:?}", i);
    println!("Hello, {:?}", t);
}
