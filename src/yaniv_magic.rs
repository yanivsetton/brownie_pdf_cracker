use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use super::args::Args;
use super::pdf_crack::try_password;

fn generate_random_password(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn yaniv_magic_crack(args: &Args, pdf_bytes: &[u8], progress: &ProgressBar, password_length: usize) {
    let found = Arc::new((AtomicBool::new(false), Mutex::new(None))); // Hold both the flag and the password
    let attempts = Arc::new(AtomicUsize::new(0));
    let total_passwords = 62_usize.pow(password_length as u32);
    progress.set_length(total_passwords as u64);

    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) Attempting: {msg}")
        .progress_chars("#>-");
    progress.set_style(style);

    (0..total_passwords).into_par_iter().for_each_with(found.clone(), |found, _| {
        if found.0.load(Ordering::Relaxed) {
            return; // If password is found, exit early
        }

        let password = generate_random_password(password_length);
        if try_password(pdf_bytes, &password) {
            found.0.store(true, Ordering::Relaxed);
            let mut found_password = found.1.lock().unwrap();
            *found_password = Some(password.clone());
            println!("Password successfully found: {}", password);
            // Correctly access `args.pdf` here
            let pdf_file_name = args.pdf.trim_end_matches(".pdf");
            let output_file_name = format!("{}_password.txt", pdf_file_name);
    
            // Attempt to write the found password to the file
            match std::fs::write(&output_file_name, password.as_bytes()) {
                Ok(_) => println!("Password successfully found and written to {}", output_file_name),
                Err(e) => println!("Failed to write password to file: {}", e),
            }
    
            return; // Exit the loop early
        }
    

        let current_attempt = attempts.fetch_add(1, Ordering::SeqCst);
        progress.set_position(current_attempt as u64 + 1); // Correctly update the progress position
    });

    // After the parallel processing, check if the password was found and print it
    let was_found = found.0.load(Ordering::Relaxed);
    if was_found {
        let found_password = found.1.lock().unwrap();
        if found_password.is_some() {
            // The password print statement has been moved to the point of discovery above.
            progress.finish_with_message("Password found. Check above for the password.");
        } else {
            println!("Password found but not set correctly.");
            progress.finish_with_message("Password found but not set correctly.");
        }
    } else {
        println!("Password not found.");
        progress.finish_with_message("Password not found.");
    }
}
