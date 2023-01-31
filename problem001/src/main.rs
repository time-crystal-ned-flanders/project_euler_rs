fn main() {
    let mut sum = 0;
    for k in 0..1000 {
        if k % 3 == 0 || k % 5 == 0 {
            sum += k;
        }
    }
    println!("answer = {}", sum);
}
