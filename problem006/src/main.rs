extern crate num_bigint;
//extern crate num_traits;
extern crate num_iter;

use num_bigint::BigInt;
//use num_traits::pow;
use num_iter::range_inclusive;

fn main() {
    println!("diff(10) = {}", diff_sqs_ssq(&BigInt::from(10)));
    println!("diff(100) = {}", diff_sqs_ssq(&BigInt::from(100)));
}

pub fn diff_sqs_ssq(n: &BigInt) -> BigInt {
    let sum_of_squares : BigInt = range_inclusive(BigInt::from(1), n.clone()).into_iter()
        .map(|x| &x*&x)
        .sum();
    let sum : BigInt = range_inclusive(BigInt::from(1), n.clone()).into_iter()
        .sum();
    let square_of_sum = &sum * &sum;
    square_of_sum - sum_of_squares
}



