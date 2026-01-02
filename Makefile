APP_NAME = gorkitale
VERSION = 1.0.0

.PHONY: all clean release release-linux release-windows

all: release

clean:
	cargo clean
	rm -rf releases

release: release-linux

release-linux:
	@echo "Building for Linux..."
	cargo build --release
	mkdir -p releases/linux/$(APP_NAME)
	cp target/release/$(APP_NAME) releases/linux/$(APP_NAME)/
	cp -r assets releases/linux/$(APP_NAME)/
	cd releases/linux && tar -czf ../$(APP_NAME)-linux-$(VERSION).tar.gz $(APP_NAME)
	@echo "Linux release created at releases/$(APP_NAME)-linux-$(VERSION).tar.gz"

release-windows:
	@echo "Building for Windows (Requires mingw-w64 and rust target x86_64-pc-windows-gnu)..."
	cargo build --release --target x86_64-pc-windows-gnu
	mkdir -p releases/windows/$(APP_NAME)
	cp target/x86_64-pc-windows-gnu/release/$(APP_NAME).exe releases/windows/$(APP_NAME)/
	cp -r assets releases/windows/$(APP_NAME)/
	cd releases/windows && zip -r ../$(APP_NAME)-windows-$(VERSION).zip $(APP_NAME)
	@echo "Windows release created at releases/$(APP_NAME)-windows-$(VERSION).zip"
