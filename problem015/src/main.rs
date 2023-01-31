/*

fn lattice_paths(n: u64,k: u64) -> u64 {
    //println!("n,k = {},{}", n, k);
    if n == 0 && k == 0 {
        0
    } else if n==0 && k > 0 {
        1 + lattice_paths(0, k-1)
    } else if n>0 && k==0 {
        1 + lattice_paths(n-1,0)
    } else {
    2 + lattice_paths(n-1, k) + lattice_paths(n, k-1)
    }
}

*/

fn fact(n: u128, nl: u128) -> u128 {
    //println!("{},{}", n, nl);
    if n<=nl {
        1
    } else {
        n*fact(n-1, nl)
    }
}


fn lattice_paths(n: u128, k: u128) -> u128 {
    // total length of path is n+k
    // choose(n+k, n) = (n+k)!/n!/k!
    // choose(n,k) = n!/k!/(n-k)!
    // fact(n+k)/fact(n)/fact(k) = n*n-1*...*(n-k) / n!
    fact(n+k,k)/fact(n,0)
}


fn main() {
    let (n,k) = (20,20);
    
    //println!("{}", fact(6,1))
    
    println!("{}", lattice_paths(n, k));
}
