use colored::*;
use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::time::Instant;

const REPORT_EVERY: i32 = 100;

fn report(&start: &Instant, found: &HashMap<String, i32>, partials: &HashMap<String, i32>) {
    let now = Instant::now();
    let duration = now - start;
    println!(
        "{}",
        format!(
            "\nFound {:?} in {} sec in {:?}",
            found,
            duration.as_secs(),
            partials
        )
        .bold()
    );
}

fn main() {
    let start = Instant::now();
    let needles: Vec<String> = vec!["foo".into(), "bar".into()];
    let mut found: HashMap<String, i32> = HashMap::new();
    let mut partials: HashMap<String, i32> = HashMap::new();

    let mut rng = rand::thread_rng();
    let mut search = "".to_string();
    while found.len() < needles.len() {
        let letter: char = rng.gen_range('a'..='z');
        let partial = search.to_owned() + &letter.to_string();
        let mut highlight = false;

        for needle in &needles {
            if needle.starts_with(&partial) {
                if partial.len() > 1 {
                    let count = partials
                        .entry(partial.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    if *count % REPORT_EVERY == 0 {
                        report(&start, &found, &partials);
                    }
                }
                highlight = true;

                if *needle == partial {
                    found
                        .entry(needle.clone())
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
    }

    report(&start, &found, &partials);
}
