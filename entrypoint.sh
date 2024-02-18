#!/bin/bash

# Example script that runs the Rust application with a specified PDF file
# This script could be enhanced to automatically select a PDF file or use an argument

PDF_FILE=$1
PASSWORD_LENGTH=$2

if [ -z "$PDF_FILE" ]; then
    echo "No PDF file specified. Usage: docker run <image> <pdf_file_name>"
    exit 1
fi

./target/release/brownie_pdf_cracker --pdf "/app/documents/$PDF_FILE" -y --password-length "$PASSWORD_LENGTH"
