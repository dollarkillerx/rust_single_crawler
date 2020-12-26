install:
	cargo install cross

build:
	make build-linux
	make build-win

go_grpc:
	@echo 'Build GRPC'
	protoc -I proto/ proto/*.proto --go_out=plugins=grpc:proto/.

build-linux:
	@echo 'Building for Linux'
	cross build --release --target=x86_64-unknown-linux-musl

build-win:
	@echo 'Building for Windows'
	cross build --bin main --release --target=x86_64-pc-windows-gnu