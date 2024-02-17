use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Warren", version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub pdf: String,

    #[arg(short, long)]
    pub wordlist: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub is_numeric: bool,

    #[arg(short, long, default_value_t = 1)]
    pub smallest_numeric_length: usize,

    #[arg(short, long, default_value_t = 8)]
    pub largest_numeric_length: usize,

    // Add this field to enable yaniv_magic
    #[arg(short, long)]
    pub yaniv_magic: bool,

    // New field for specifying password length directly
    #[arg(short = 'n', long = "password-length")]
    pub password_length: Option<usize>,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}