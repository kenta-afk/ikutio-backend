.PHONY: all setup clean

# Go commands
GO := go
PROTOC := protoc
GOPATH := $(shell $(GO) env GOPATH)
GOBIN := $(GOPATH)/bin
CARGO_BIN := $(HOME)/.cargo/bin

# Add GOBIN and CARGO_BIN to PATH
export PATH := $(GOBIN):$(CARGO_BIN):$(PATH)

# Directories
PROTO_DIR := proto
BUILD_DIR := build
PROTO_BUILD_DIR := $(BUILD_DIR)
BFF_SERVICES_DIR := ../bff/src/services

# Proto files
PROTO_FILES := $(wildcard $(PROTO_DIR)/*.proto)

# Default target - does everything
all: setup $(PROTO_BUILD_DIR)
	@echo "Setting up and building all protobuf files..."
	@echo "Building Go protobuf server files..."
	mkdir -p $(PROTO_BUILD_DIR)
	PATH="$(GOBIN):$$PATH" $(PROTOC) --proto_path=$(PROTO_DIR) \
		--go_out=. \
		--go_opt=paths=source_relative \
		--go_opt=Mprofile.proto=./build \
		--go-grpc_out=. \
		--go-grpc_opt=paths=source_relative \
		--go-grpc_opt=Mprofile.proto=./build \
		--go-grpc_opt=require_unimplemented_servers=false \
		profile.proto
	@mv profile.pb.go $(PROTO_BUILD_DIR)/ || true
	@mv profile_grpc.pb.go $(PROTO_BUILD_DIR)/ || true
	@echo "Go protobuf server files built successfully in $(PROTO_BUILD_DIR)"
	@echo "Building Rust protobuf client files for BFF..."
	mkdir -p $(BFF_SERVICES_DIR)
	PATH="$(CARGO_BIN):$(GOBIN):$$PATH" $(PROTOC) --proto_path=$(PROTO_DIR) \
		--prost_out=$(BFF_SERVICES_DIR) \
		--prost_opt=compile_well_known_types \
		--tonic_out=$(BFF_SERVICES_DIR) \
		--tonic_opt=no_include \
		--tonic_opt=no_server \
		profile.proto
	@echo "Cleaning up unnecessary files..."
	@rm -rf $(BFF_SERVICES_DIR)/profile/
	@echo "Rust protobuf client files built successfully in $(BFF_SERVICES_DIR)"
	@echo "All protobuf files generated successfully!"

# Setup necessary tools and dependencies
setup:
	@echo "Setting up Go modules and tools..."
	$(GO) mod tidy
	$(GO) install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	$(GO) install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
	@echo "Installing Rust protoc plugins..."
	cargo install protoc-gen-prost || echo "protoc-gen-prost already installed or failed to install"
	cargo install protoc-gen-tonic || echo "protoc-gen-tonic already installed or failed to install"

# Create build directory
$(PROTO_BUILD_DIR):
	mkdir -p $(PROTO_BUILD_DIR)

# Clean all build artifacts
clean:
	@echo "Cleaning all build artifacts..."
	rm -rf $(BUILD_DIR)
	rm -f $(BFF_SERVICES_DIR)/profile.rs
	rm -f $(BFF_SERVICES_DIR)/profile_grpc.rs
	rm -rf $(BFF_SERVICES_DIR)/profile/
	$(GO) clean
	@echo "Clean complete"
