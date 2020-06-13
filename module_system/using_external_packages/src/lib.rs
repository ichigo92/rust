use rand::Rng;

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", _secret_number);
}
