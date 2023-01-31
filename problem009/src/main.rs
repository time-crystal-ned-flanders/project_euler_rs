use itertools::Itertools;

// kpart(n, k) = (1..j) x kpart(n-j, k-1)


fn k_partitions(n:u128, k:u128) -> Vec<Vec<u128>> {
    if k == 1 {
        return vec![vec![n]]
    }
    let mut kp = Vec::<Vec<u128>>::new();
    for j in 1..n {
        for kpsub in k_partitions(n-j, k-1) {
            let mut b = vec![j];
            b.extend(kpsub);
            kp.push(b);
        }
    }
    kp
}

fn is_pythagorean_triple(trip: Vec<u128>) -> bool {
    let (a,b,c) = trip.iter().next_tuple().unwrap();
    a*a + b*b == c*c
}



fn main() {
    let N = 1000;
    let kp = k_partitions(1000, 3);
    for kpi in kp.iter() {
        //let (a,b,c) = kpi.iter().next_tuple().unwrap();
        if is_pythagorean_triple(kpi.to_vec()) { 
            println!("found one: {:?}", kpi);
        }
    }



    //println!("{:?}", kp);

}
