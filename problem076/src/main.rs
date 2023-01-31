use itertools::Itertools;

fn main() {
    let N = 100;
    let part_sum :u64 = (2..=N).map(|k| p(k,N)).sum();
    println!("parts {N} = {}", part_sum);
}

// computer  p(k,n) by recurrence relation p(k,n) = p(k,n-k) + p(k-1, n-1)
fn p(k:u64,n:u64) -> u64 {
    if k == 1 {
        1
    } else if k > n || n == 0 {
        0
    } else {
        p(k,n-k) + p(k-1,n-1)
    }
}


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


fn count_k_partitions(n: u64, k: usize) -> Vec<Vec<u64>> {
    let walls = (1..=n-1).combinations(k-1).collect::<Vec<_>>();

    let parts = walls.into_iter()
        .map(|w| wall_to_part(n, w))
        .unique()
        .collect();

    parts
}
