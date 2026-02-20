#![deny(clippy::all)]

use clap::Parser;
use colored::*;
use rand::RngExt;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::Write;
use std::iter::FromIterator;
use std::time::Instant;

type Count = i32;
type Needles = HashSet<String>;
type Progress = HashMap<String, Count>;

const REPORT_EVERY: Count = 100;

/// Babel search: find words in a random stream of letters
#[derive(Parser)]
#[command(name = "babel-search")]
#[command(about = "A Library of Babel search implementation")]
struct Args {
    /// Path to a dictionary file containing needles (one per line)
    #[arg(long)]
    dictionary: Option<String>,
    
    /// Words to search for (alternative to dictionary file)
    needles: Vec<String>,
}

/// Helper function to read search needles from command-line arguments or dictionary file.
fn gather_needles(args: &Args) -> Result<Needles, std::io::Error> {
    let mut needles: Vec<String> = Vec::new();
    
    // If a dictionary file is provided, read from it
    if let Some(dict_path) = &args.dictionary {
        let contents = fs::read_to_string(dict_path)?;
        needles.extend(contents.lines().map(|line| line.to_string()));
    }
    
    // Also include command-line needles for backward compatibility
    needles.extend(args.needles.iter().cloned());
    
    // (1) Convert to lowercase.
    let mut needles: Vec<String> = needles.into_iter().map(|s| s.to_lowercase()).collect();

    // (2) Remove whitespace.
    needles
        .iter_mut()
        .for_each(|s| s.retain(|c| !c.is_whitespace()));

    // (3) Filter out empty strings and convert to a set to deduplicate.
    Ok(Needles::from_iter(needles.into_iter().filter(|s| !s.is_empty())))
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
    let args = Args::parse();
    
    // What we're searching for and how much progress we've made.
    let needles = match gather_needles(&args) {
        Ok(needles) => {
            if needles.is_empty() {
                eprintln!("Error: No needles provided. Use either --dictionary or provide words as arguments.");
                std::process::exit(1);
            }
            needles
        }
        Err(e) => {
            eprintln!("Error reading dictionary file: {}", e);
            std::process::exit(1);
        }
    };
    
    let mut found: Progress = HashMap::new();
    let mut partials: Progress = HashMap::new();

    // Set up for the search and run it until we've found everything:
    let mut rng = rand::rng();
    let mut search = "".to_string();
    let start = Instant::now();
    while found.len() < needles.len() {
        // Pull a new letter at random from [a, z].  Assume it's not a match for
        // our current search.
        let letter: char = rng.random_range('a'..='z');
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
