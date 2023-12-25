use rand::Rng;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

const WAIT: Duration = Duration::from_millis(100); // 0.1 sec

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let letter: char = rng.gen_range('a'..='z');

        print!("{}", letter);
        io::stdout().flush().unwrap();
        thread::sleep(WAIT);
    }
}
