# Use Rust official image as base
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the project
RUN cargo build --release

# Start a new stage and use Ubuntu base image
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# Install libsqlite3-dev and other dependencies
RUN apt-get update && \
    apt-get install -y libsqlite3-dev

# Copy the built binary from the builder stage into the current stage
COPY --from=builder /app/target/release/pastry_crust .

# Expose the port that the application will run on
EXPOSE 8080

# Define the command to run the application
CMD ["./pastry_crust"]

