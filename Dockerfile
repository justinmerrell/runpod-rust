# Use the official Ubuntu image as a base
FROM ubuntu:latest

# Install necessary dependencies (if any). For example, if you have dynamic dependencies on certain libraries, install them here
RUN apt-get update && apt-get install -y \
    # Example libraries (add/remove as needed):
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from your host to the Docker container
COPY runpod_binary /app/my_rust_app

# Give execution permissions to the binary
RUN chmod +x /app/my_rust_app

# Command to run when the Docker container starts
CMD ["/app/my_rust_app"]
