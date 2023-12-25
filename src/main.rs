use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range('a'..='z');
    println!("{}", letter);
}
