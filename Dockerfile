# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy your project files into the container
COPY . .

# Build your Rust project
RUN cargo build --release

# Copy the entry point script
COPY entrypoint.sh .

# Make the entry point script executable
RUN chmod +x entrypoint.sh

# Make port 5000 available to the world outside this container
EXPOSE 5000

# Set the entry point script to run when the container starts
ENTRYPOINT ["./entrypoint.sh"]