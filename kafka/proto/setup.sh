# Install protobuf compiler
sudo apt-get update
sudo apt-get install -y protobuf-compiler

# Install Rust protobuf plugin
cargo install protobuf-codegen

# Install Go protobuf plugin
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
