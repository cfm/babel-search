use colored::*;
use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

const WAIT: Duration = Duration::from_millis(10); // 0.1 sec

fn main() {
    let needles: Vec<String> = vec!["foo".into(), "bar".into()];
    let mut found = HashSet::new();

    let mut rng = rand::thread_rng();
    let mut search = "".to_string();
    while found.len() < needles.len() {
        let letter: char = rng.gen_range('a'..='z');
        let partial = search.to_owned() + &letter.to_string();
        let mut highlight = false;

        for needle in &needles {
            if needle.starts_with(&partial) {
                highlight = true;

                if *needle == partial {
                    found.insert(needle);
                    search = "".to_string()
                } else {
                    search = partial.clone();
                }
                break;
            } else {
                highlight = false;
                search = "".to_string();
            }
        }

        print!(
            "{}",
            if highlight {
                letter.to_string().reversed()
            } else {
                letter.to_string().normal()
            }
        );
        io::stdout().flush().unwrap();
        thread::sleep(WAIT);
    }

    println!();
}
