# Golang code generation based on proto_message.proto
protoc --go_out=../go-single --go_opt=paths=source_relative --go-grpc_out=../go-single --go-grpc_opt=paths=source_relative proto_message.proto
# Rust code generation based on proto_message.proto
protoc --rust_out=../rust-rdkafka-single/src proto_message.proto
