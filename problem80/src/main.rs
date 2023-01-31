use rug::*;





fn main() {
    let n = 99;

    let mut total = 0;

    for k in 1..=n {
        //println!("k = {k}");
        let mut kf = Float::with_val(350, k);
        kf.sqrt_mut();
        let kfd = kf.to_string_radix(10, None);

        let digits : Vec<_> = kfd.chars().take(101).flat_map(|c| c.to_digit(10)).collect();
        let dlen = digits.len();
        let digit_sum : u32 = digits.into_iter().sum();


        if digit_sum*digit_sum - k == 0 {
            continue;
        } else {
            println!("sqrt({k}) - {digit_sum} - {dlen}");
            total += digit_sum;
        }        
    }
    
    println!("total = {total}");
   
}
