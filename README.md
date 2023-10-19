# Brownie PDF Cracker

Brownie PDF Cracker is a command-line tool for cracking password-protected PDF files. It provides a flexible and efficient way to recover passwords for PDF documents, whether you've forgotten the password to your own PDF or need to perform a security assessment.

![Brownie PDF Cracker](banner.png)

## Table of Contents

- [About](#about)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## About

Brownie PDF Cracker is a Rust-based tool designed to help you regain access to password-protected PDF files. Whether you've lost your PDF password or need to test the security of your PDF documents, this tool provides the capability to efficiently crack PDF passwords.

## Features

- Password cracking for PDF files.
- Numeric and wordlist-based password generation.
- Yaniv's magic password cracking mode.
- Progress tracking with a customizable progress bar.
- Command-line interface for easy usage.

## Getting Started

Follow these instructions to get Brownie PDF Cracker up and running on your system.

### Prerequisites

To run Brownie PDF Cracker, you'll need the following:

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation
Clone the repository to your local machine:

```bash
$ git clone https://github.com/yourusername/brownie-pdf-cracker.git
$ cd brownie-pdf-cracker
$ cargo build --release
```
### Usage
Brownie PDF Cracker can be used to recover passwords from PDF files. It offers various modes of operation:

### Numeric Passwords
```
$ ./brownie-pdf-cracker --pdf path/to/encrypted.pdf --is-numeric --smallest-numeric-length 4 --largest-numeric-length 6
```

### Wordlist Passwords
Test passwords from a wordlist file.
```
$ ./brownie-pdf-cracker --pdf path/to/encrypted.pdf --wordlist path/to/wordlist.txt
```

### Yaniv's Magic Mode
Activate Yaniv's magic password cracking mode. This mode prompts you to enter the password length and then generates and tests passwords of that length.
```
$ ./brownie-pdf-cracker --pdf path/to/encrypted.pdf --yaniv-magic
```

### For more information and additional options, use the --help flag:
```
$ ./brownie-pdf-cracker --help
```

### Contributing
Contributions to Brownie PDF Cracker are welcome! Whether you want to report a bug, suggest a new feature, or contribute code, please refer to the Contribution Guidelines for details on how to get involved.

### License
This project is licensed under the MIT License.

## How to use
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
