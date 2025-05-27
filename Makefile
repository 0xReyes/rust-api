# Makefile for Order Book Aggregator API

# Variables
IMAGE_NAME := rust-api
CONTAINER_NAME := rust-api-container
PORT := 8080

.PHONY: all build run clean docker-build docker-run docker-clean

all: build

# Build the Rust application locally
build:
	cargo build

# Run the application locally
run: build
	cargo run

# Clean the local build artifacts
clean:
	cargo clean

# Build the Docker image
docker-build:
	docker build -t $(IMAGE_NAME) .

# Run the Docker container
docker-run:
	docker run --rm -p $(PORT):8080 --name $(CONTAINER_NAME) $(IMAGE_NAME)

# Clean up Docker container and image
docker-clean:
	docker rm -f $(CONTAINER_NAME) || true
	docker rmi $(IMAGE_NAME) || true
