use itertools::Itertools;

// given choose(n,k) wall position convert to int partition
fn wall_to_part(n : u64, walls : Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    let mut residue = n;
    let mut wls = walls.clone();
    wls.reverse();

    while let Some(wi) = wls.pop() {
        let tk = wi - (n - residue);
        result.push(tk);
        residue = residue - tk;
    }
    if residue > 0 {
        result.push(residue);
    }

    result.sort();
    result
}


fn k_partitions(n: u64, k: usize) -> Vec<Vec<u64>> {
    let walls = (1..=n-1).combinations(k-1).collect::<Vec<_>>();

    let parts = walls.into_iter()
        .map(|w| wall_to_part(n, w))
        .collect();

    parts
}


// product-sum

// for k start with 1..1


fn main() {
    let n = 11;
    let k = 10000;
    
    for i in (1..=k).map(|_| 1..=n).multi_cartesian_product() {
        let s : u64 = i.iter().sum();
        let p : u64 = i.iter().product();
        println!("{} =? {}", s, p);
        if s == p {
            break;
        }

    }

}


