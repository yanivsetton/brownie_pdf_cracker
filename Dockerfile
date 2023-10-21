# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy your project files into the container
COPY . .

# Build your Rust project
RUN cargo build --release

WORKDIR /app/target/release

# Define the command to run when the container starts
CMD ["./brownie_pdf_cracker", "--help"]