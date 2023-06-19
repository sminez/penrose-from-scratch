all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@cargo build --release


.PHONY: build-debug
build-debug:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in debug mode..."
	@cargo build

.PHONY: install-helpers
install-helpers:
	@echo ":: Installing ./bin..."
	@mkdir -p /usr/local/bin
	@cp -R bin/. /usr/local/bin
	@ls bin | xargs -I {} chmod 755 /usr/local/bin/{}
	@echo ":: Installing ./scripts..."
	@cp -r scripts /usr/local
	@ls scripts | xargs -I {} chmod 755 /usr/local/scripts/{}
	@echo ":: Copying over xsession file..."
	@cp penrose-from-scratch.desktop /usr/share/xsessions/

.PHONY: install-penrose-release
install-penrose-release:
	@echo ":: Installing release build of penrose-from-scratch..."
	@mkdir -p /usr/local/bin
	@cp -f target/release/penrose-from-scratch /usr/local/bin
	@chmod 755 /usr/local/bin/penrose-from-scratch

.PHONY: install-penrose-debug
install-penrose-debug:
	@echo ":: Installing debug build of penrose-from-scratch..."
	@strip target/debug/penrose-from-scratch
	@mkdir -p /usr/local/bin
	@cp -f target/debug/penrose-from-scratch /usr/local/bin
	@chmod 755 /usr/local/bin/penrose-from-scratch

.PHONY: install
install: install-penrose-release install-helpers
	@echo ":: Done"
	
.PHONY: install-debug
install-debug: install-penrose-debug install-helpers
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@ls bin | xargs -I {} rm -f /usr/local/bin/{}
	@rm -f /usr/local/bin/penrose-from-scratch
	@echo ":: Removing scripts..."
	@ls scripts | xargs -I {} rm -f /usr/local/scripts/{}
	@echo ":: Done"

.PHONY: update-penrose
update-penrose:
	@echo "Updating to latest version of penrose from GitHub..."
	cargo update -p penrose
