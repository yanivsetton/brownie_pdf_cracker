use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use super::args::Args;
use super::pdf_crack::try_password;

fn generate_and_test_passwords(
    charset: &[u8],
    length: usize,
    pdf_bytes: &[u8],
    found: Arc<(AtomicBool, Mutex<Option<String>>)>,
    attempts: Arc<AtomicUsize>,
    progress: &ProgressBar,
) {
    // Calculate total number of combinations
    let total_combinations = charset.len().pow(length as u32);

    // Parallelize the process of generating and testing passwords
    (0..total_combinations).into_par_iter().for_each_with(found.clone(), |found, index| {
        if found.0.load(Ordering::Relaxed) {
            return; // If password is found, exit early
        }

        // Generate password based on current index
        let mut combination = Vec::with_capacity(length);
        let mut current_index = index;
        for _ in 0..length {
            combination.push(charset[current_index % charset.len()]);
            current_index /= charset.len();
        }
        let password: String = combination.iter().rev().map(|&c| c as char).collect();

        // Test the generated password
        if try_password(pdf_bytes, &password) {
            found.0.store(true, Ordering::Relaxed);
            let mut found_password = found.1.lock().unwrap();
            *found_password = Some(password.clone());
            println!("Password successfully found: {}", password);
            return; // Exit the loop early
        }

        let current_attempt = attempts.fetch_add(1, Ordering::SeqCst);
        progress.set_position(current_attempt as u64 + 1); // Correctly update the progress position
    });
}

pub fn yaniv_magic_crack(_args: &Args, pdf_bytes: &[u8], progress: &ProgressBar, password_length: usize) {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let found = Arc::new((AtomicBool::new(false), Mutex::new(None)));
    let attempts = Arc::new(AtomicUsize::new(0));
    let total_passwords = charset.len().pow(password_length as u32);
    progress.set_length(total_passwords as u64);

    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) Attempting: {msg}")
        .progress_chars("#>-");
    progress.set_style(style);

    generate_and_test_passwords(charset, password_length, pdf_bytes, found.clone(), attempts.clone(), progress);

    // After parallel processing, check if the password was found
    let was_found = found.0.load(Ordering::Relaxed);
    if was_found {
        let found_password = found.1.lock().unwrap();
        if let Some(password) = &*found_password {
            println!("Password successfully found: {}", password);
            // Directly print the dynamic message and use a static message for finish_with_message
            progress.finish_with_message("Password found. Check output for details.");
        } else {
            println!("Password found but not set correctly.");
            progress.finish_with_message("Password found but not set correctly.");
        }
    } else {
        println!("Password not found.");
        progress.finish_with_message("Password not found.");
    }
}

