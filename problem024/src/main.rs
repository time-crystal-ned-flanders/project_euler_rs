use itertools::Itertools;

fn main() {
    let perms : Vec<_>= (0..=9).permutations(10).skip(999999).next().unwrap();
    println!("{perms:?}");
}
