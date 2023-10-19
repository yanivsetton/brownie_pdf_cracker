use rayon::prelude::*;
use super::args::Args;
use std::io::{BufRead, BufReader};

/// Implement the logic for generating the password list here.
pub fn generate_password_list(args: &Args) -> Vec<String> {
    if args.is_numeric {
        let mut password_list: Vec<String> = Vec::new();
        for length in args.smallest_numeric_length..=args.largest_numeric_length {
            password_list.append(
                &mut (0..1000_usize.pow(length as u32))
                    .into_par_iter()
                    .map(|password| format!("{:0width$}", password, width = length))
                    .collect(),
            );
        }
        password_list
    } else if let Some(wordlist) = &args.wordlist {
        let file = std::fs::File::open(wordlist).expect("Unable to open wordlist file");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| line.expect("Error reading line from wordlist"))
            .collect()
    } else {
        // Default to empty wordlist if no method specified
        Vec::new()
    }
}