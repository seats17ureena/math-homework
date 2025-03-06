
pub fn generate_random_rust_code() {
let mut rng = thread_rng();
let code = format!("fn main() {{ let x = {}; let y = {}; println!(\"{}\"); }}", rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10));
return code;
}