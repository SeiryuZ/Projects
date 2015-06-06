extern crate num;

use num::bigint::ToBigInt;

fn main() {

    let mut current  = 1.to_bigint().unwrap();
    let mut previous = 0.to_bigint().unwrap();

    for x in 0..10100 {

        println!("Iteration {} = {:?}", x, current);

        let proxy = current.clone();
        current = proxy.clone() + previous;
        previous = proxy;
    }
}
