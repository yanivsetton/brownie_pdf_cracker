use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use super::args::Args;
use super::pdf_crack::try_password;

fn generate_and_test_passwords(
    charset: &[u8],
    length: usize,
    pdf_bytes: &[u8],
    found: Arc<(AtomicBool, Mutex<Option<String>>)>,
    attempts: Arc<AtomicUsize>,
    progress: &ProgressBar,
    start_time: Instant,
) {
    let total_combinations = charset.len().pow(length as u32);

    (0..total_combinations).into_par_iter().for_each_with(found.clone(), |found, index| {
        if found.0.load(Ordering::Relaxed) {
            return;
        }

        let mut combination = Vec::with_capacity(length);
        let mut current_index = index;
        for _ in 0..length {
            combination.push(charset[current_index % charset.len()]);
            current_index /= charset.len();
        }
        let password: String = combination.iter().rev().map(|&c| c as char).collect();

        if try_password(pdf_bytes, &password) {
            found.0.store(true, Ordering::Relaxed);
            let mut found_password = found.1.lock().unwrap();
            *found_password = Some(password.clone());
            println!("Password successfully found: {}", password);
            return;
        }

        let current_attempt = attempts.fetch_add(1, Ordering::SeqCst) + 1;
        // Update the progress bar every 100,000 attempts
        if current_attempt % 100_000 == 0 {
            let elapsed = start_time.elapsed().as_secs_f32();
            let attempts_per_sec = current_attempt as f32 / elapsed;
            progress.set_position(current_attempt as u64);
            progress.set_message(format!("Attempts/sec: {:.2}", attempts_per_sec));
        }
    });
}


pub fn yaniv_magic_crack(_args: &Args, pdf_bytes: &[u8], progress: &ProgressBar, password_length: usize) {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let found = Arc::new((AtomicBool::new(false), Mutex::new(None)));
    let attempts = Arc::new(AtomicUsize::new(0));
    let total_passwords = charset.len().pow(password_length as u32);
    progress.set_length(total_passwords as u64);
    let start_time = Instant::now();

    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) {msg}")
        .progress_chars("#>-");
    progress.set_style(style);

    generate_and_test_passwords(charset, password_length, pdf_bytes, found.clone(), attempts.clone(), progress, start_time);

    let was_found = found.0.load(Ordering::Relaxed);
    if was_found {
        let found_password = found.1.lock().unwrap();
        if let Some(password) = &*found_password {
            println!("Password successfully found: {}", password);
            progress.finish_with_message("Password found. Check output for details.");
        } else {
            progress.finish_with_message("Password found but not set correctly.");
        }
    } else {
        progress.finish_with_message("Password not found.");
    }
}
