use indicatif::{ProgressBar, ProgressStyle};
use std::fs::read;
use env_logger::Builder;
use log::info;
use std::io;

mod args;
mod pdf_crack;
mod password_list;
mod yaniv_magic;

fn print_banner() {
    let banner = r#"
   / \__
  (    @\____
  /         O
 /   (_____/ 
/_____/   U 

   Brownie PDF Cracker

  👅 It's Treat Time! 🐶
"#;
    println!("{}", banner);
}

fn main() {
    Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    print_banner();

    let args = args::Args::parse_args();
    let pdf_bytes = read(&args.pdf).expect("Unable to read PDF");
    let password_list = password_list::generate_password_list(&args);

    let password_length: usize;

    if args.yaniv_magic {
        println!("Enter the password length:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        password_length = input.trim().parse().expect("Invalid input");
    } else {
        password_length = args.smallest_numeric_length;
    }

    let progress = ProgressBar::new(62_usize.pow(password_length as u32) as u64);
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:45.cyan/blue} {pos:>7}/{len:7} ({eta}) Attempting: {msg}")
        .progress_chars("#>-");
    progress.set_style(style);

    if args.yaniv_magic {
        yaniv_magic::yaniv_magic_crack(&args, &pdf_bytes, &progress, password_length);
    } else {
        pdf_crack::crack_pdf(&args, &pdf_bytes, &password_list, &progress);
    }

    progress.finish();
    info!("Done");
}