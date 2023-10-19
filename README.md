# brownie_pdf_cracker
This repo include smart pdf cracker

## Installation
```
### git clone https://github.com/yanivsetton/brownie_pdf_cracker.git
### cd brownie_pdf_cracker
### cargo build --release 
### ./target/release/pdf_cracker -p <PDF_path_file> -y
### ./target/release/pdf_cracker -p <PDF_path_file> -i
```

```bash
Usage: crackify [OPTIONS] --pdf <PDF>

Options:
  -p, --pdf <PDF>
          Path to a password protected PDF file
  -y, --yaniv-magic specify password length and let the magic begin
  -w, --wordlist <WORDLIST>
          Path to a wordlist. Each word should be on a new line
  -i, --is-numeric
          Whether or not the password is entirely numeric
  -s, --smallest-numeric-length <SMALLEST_NUMERIC_LENGTH>
          Minimum length of numeric password [default: 1]
  -l, --largest-numeric-length <LARGEST_NUMERIC_LENGTH>
          Maximum length of numeric password [default: 8]
  -h, --help
          Print help
  -V, --version
          Print version
```





This project was cloned from git@github.com:WarrenHood/crackify.git
as a base code
