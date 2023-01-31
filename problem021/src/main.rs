fn d(n:u64) -> u64 {
    let mut k = 2;
    let mut sum = 1;
    while k*k <= n {
        if n % k == 0 {
            sum += k + n/k;
        }
        k += 1;
    }
    sum
}

fn main() {
    //println!("{}",d(220));
    //println!("{}",d(284));
    let di : Vec<_> = (1..10000).filter(|x| d(*x) != *x && d(d(*x))==*x).collect();
    //let amisum : Vec<usize> = (2..10000).filter(|i| di[di[*i]] == *i).collect();

    println!("amis = {:?}", di);
    let mut amisum = 0;
    for dii in di {
        println!("{} {}", dii, d(dii));
        amisum += dii;
    }
    println!("amisum = {:?}", amisum);
    
}