fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(0..=10);
    println!("{}", x * 5);
}
