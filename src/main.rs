use colored::*;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;
use std::time::Instant;

type Count = i32;
type Needles = Vec<String>;
type Progress = HashMap<String, Count>;

const REPORT_EVERY: Count = 100;

/// Helper function to gather our needles: our command-line arguments, converted
/// to lowercase and stripped of whitespace.
fn gather_needles() -> Needles {
    let mut needles: Vec<String> = env::args().skip(1).map(|s| s.to_lowercase()).collect();
    needles
        .iter_mut()
        .for_each(|s| s.retain(|c| !c.is_whitespace()));
    needles
}

/// Helper function to report our progress along the way and at the end.
fn report(&start: &Instant, found: &Progress, partials: &Progress) {
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
    // What we're searching for and how much progress we've made.
    let needles = gather_needles();
    let mut found: Progress = HashMap::new();
    let mut partials: Progress = HashMap::new();

    // Set up for the search and run it until we've found everything:
    let mut rng = rand::thread_rng();
    let mut search = "".to_string();
    let start = Instant::now();
    while found.len() < needles.len() {
        // Pull a new letter at random from [a, z].  Assume it's not a match for
        // our current search.
        let letter: char = rng.gen_range('a'..='z');
        let mut highlight = false;

        // The partial sequence we're searching for: at least the current
        // character, appended to however far we've gotten so far.
        let partial = search.to_owned() + &letter.to_string();

        // For every needle we're searching for:
        for needle in &needles {
            // Do we have part of it?
            if needle.starts_with(&partial) {
                // Do we have the whole thing?
                if *needle == partial {
                    found
                        .entry(needle.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    search = "".to_string()
                } else {
                    // We get credit towards our progress only if it's more than
                    // the first letter.  ;-)
                    if partial.len() > 1 {
                        let count = partials
                            .entry(partial.clone())
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                        if *count % REPORT_EVERY == 0 {
                            report(&start, &found, &partials);
                        }
                    }
                    search = partial.clone();
                }

                // ...either way it's at least a partial match!
                highlight = true;
                break;

            // Nope; we'll keep going.
            } else {
                highlight = false;
                search = "".to_string();
            }
        }

        // Print the current character: highlighted if it's part of a match;
        // otherwise normal.
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
