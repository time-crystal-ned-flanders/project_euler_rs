fn main() {
    //let n = 13195;
    //factors(n);
    let n = 600851475143;
    
    let fs = factors_to_vec(n);
    println!("{:?}", fs);
    println!("max value is {}", fs.iter().max().unwrap())
}

/*
fn factors_rec(n : u64, d : u64) {
    if n > 1 {
        if n % d == 0 { factors_rec(n / d, d) }
        else { factors_rec(n, d+1)}
    } else {
        1
    }
}
*/

fn factors_to_vec(n : u64) -> Vec<u64> { 
    let mut f = n;
    let mut d = 2;
    let mut fs = Vec::new();
    while f > 1 {
        if f % d == 0 { 
            fs.push(d);
            f = f / d;
        } else { 
            d = d + 1;
        }
    }
    fs
}


fn factors(n: u64) {
    let mut f = n;
    let mut d = 2;
    
    while f > 1 {
        println!("f,d={},{}",f,d);
        if f % d == 0 {
            f = f / d

        } else {
            d = d + 1
        }
    }
}
