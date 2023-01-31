use num_bigint::{BigUint,ToBigUint};

fn main() {
    let a = 2_u64.to_biguint().unwrap();
    let b = 28433_u64.to_biguint().unwrap();
    let e = 7830457_u64.to_biguint().unwrap();
    let m = 10000000000_u64.to_biguint().unwrap();

    let p = (b * a.modpow(&e, &m)) % m;
    

    println!("{:?}", p+1_u8.to_biguint().unwrap());

}
