use indicatif::ProgressBar;
use super::args::Args;
use std::fs::OpenOptions;
use std::io::Write;

pub fn try_password(pdf_contents: &[u8], password: &str) -> bool {
    pdf::file::FileOptions::cached()
        .password(password.as_bytes())
        .load(pdf_contents)
        .is_ok()
}

pub fn crack_pdf(_args: &Args, pdf_bytes: &[u8], password_list: &[String], progress: &ProgressBar) {
    let total_passwords = password_list.len() as u64;
    progress.set_length(total_passwords);
    for password in password_list {
        if try_password(pdf_bytes, password) {
            println!("Found password: {}", password);
            progress.inc(1);
            // Write to file
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("password.txt")
                .expect("Cannot open password.txt");
            writeln!(file, "{}: {}", _args.pdf, password).expect("Cannot write to password.txt");
            return;
        }
        progress.inc(1);
    }
}