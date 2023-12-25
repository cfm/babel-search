use colored::*;
use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

const WAIT: Duration = Duration::from_millis(10); // 0.1 sec

fn main() {
    let start = Instant::now();
    let needles: Vec<String> = vec!["foo".into(), "bar".into()];
    let mut found = HashMap::new();

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
                    found
                        .entry(needle)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
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

    let finish = Instant::now();
    let duration = finish - start;
    println!("\nFound {:?} in {} sec", found, duration.as_secs());
}
