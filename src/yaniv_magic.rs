// In yaniv_magic.rs

use indicatif::ProgressBar;
use rand::Rng;
use std::borrow::Cow;
use super::args::Args;
use super::pdf_crack::try_password;

pub fn generate_random_password(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

pub fn yaniv_magic_crack(args: &Args, pdf_bytes: &[u8], progress: &ProgressBar, password_length: usize) {
    let total_passwords = 62_usize.pow(password_length as u32);
    progress.set_length(total_passwords as u64);

    for attempt in 0..total_passwords {
        let password = generate_random_password(password_length);
        let message: Cow<'static, str> = Cow::Owned(format!("Attempting: {}", password));

        if try_password(pdf_bytes, &password) {
            println!("Found password: {}", password);
            return;
        }

        progress.set_message(message);
        progress.set_position(attempt as u64);
    }

    if args.yaniv_magic {
        println!("Password not found for a length of {} characters.", password_length);
    } else {
        println!("Password not found for length: {}", password_length);
    }
}