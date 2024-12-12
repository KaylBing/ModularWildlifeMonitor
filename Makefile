# Variables
INSTALL_DIR = /usr/local/bin
SYSTEMD_DIR = /etc/systemd/system
CARGO = $(HOME)/.cargo/bin/cargo
JQ = jq
PYTHON = python3

# Check if the required tools are available
check_deps:
	@if ! command -v $(CARGO) &>/dev/null; then \
		echo "cargo not found. Installing Rust..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s --; \
		echo "Rust installed."; \
	fi
	@if ! command -v $(JQ) &>/dev/null; then \
		echo "jq not found. Installing jq..."; \
		sudo apt-get install -y jq; \
		echo "jq installed."; \
	fi
	@if ! command -v $(PYTHON) &>/dev/null; then \
		echo "python3 not found. Installing Python..."; \
		sudo apt-get install -y python3 python3-pip; \
		echo "Python installed."; \
	fi

# Check Python dependencies
check_python_deps:
	@for dir in libs/*/; do \
		if [ -d "$$dir" ] && [ -f "$$dir/requirements.txt" ]; then \
			echo "Installing Python dependencies for $$dir..."; \
			pip3 install -r $$dir/requirements.txt; \
		else \
			echo "No requirements.txt found in $$dir. Skipping Python dependencies check."; \
		fi; \
	done

# Build Rust project (in each lib directory)
build_rust:
	@for dir in libs/*/; do \
		if [ -d "$$dir" ] && [ -f "$$dir/Cargo.toml" ]; then \
			echo "Building Rust project in $$dir..."; \
			cd $$dir && cargo build --release && cd -; \
		fi; \
	done

# Install Rust binaries
install_rust:
	@for dir in libs/*/; do \
		if [ -d "$$dir" ] && [ -f "$$dir/Cargo.toml" ]; then \
			echo "Installing Rust binaries from $$dir..."; \
			BINARY_NAME=$$(cargo metadata --no-deps --format-version 1 --manifest-path $$dir/Cargo.toml | $(JQ) -r '.packages[0].targets[0].name'); \
			BINARY_PATH="$$dir/target/release/$$BINARY_NAME"; \
			if [ -f "$$BINARY_PATH" ]; then \
				sudo install -Dm755 "$$BINARY_PATH" $(INSTALL_DIR)/$$BINARY_NAME; \
				make add_to_startup PROGRAM_NAME=$$BINARY_NAME; \
			else \
				echo "Warning: Compiled Rust binary not found in $$dir. Skipping..."; \
			fi; \
		fi; \
	done

# Install Java binaries (for Maven or Gradle projects)
install_java:
	@for dir in libs/*/; do \
		if [ -d "$$dir" ] && [ -f "$$dir/pom.xml" ]; then \
			JAR_FILE="$$dir/target/$$basename-1.0.jar"; \
			if [ -f "$$JAR_FILE" ]; then \
				sudo install -Dm644 "$$JAR_FILE" $(INSTALL_DIR)/$$basename-1.0.jar; \
				make add_to_startup PROGRAM_NAME=$$basename-1.0.jar; \
			fi; \
		elif [ -d "$$dir" ] && [ -f "$$dir/build.gradle" ]; then \
			JAR_FILE="$$dir/build/libs/$$basename-1.0.jar"; \
			if [ -f "$$JAR_FILE" ]; then \
				sudo install -Dm644 "$$JAR_FILE" $(INSTALL_DIR)/$$basename-1.0.jar; \
				make add_to_startup PROGRAM_NAME=$$basename-1.0.jar; \
			fi; \
		fi; \
	done

# Add program to system startup using systemd
add_to_startup:
	@if [ "$(PROGRAM_NAME)" ]; then \
		echo "Do you want to add '$(PROGRAM_NAME)' to startup? (y/n)"; \
		read -r add_startup; \
		if [[ "$$add_startup" =~ ^[Yy]$$ ]]; then \
			SERVICE_FILE=$(SYSTEMD_DIR)/$(PROGRAM_NAME).service; \
			echo "Creating systemd service for $(PROGRAM_NAME)..."; \
			sudo bash -c "cat > $$SERVICE_FILE" <<EOF \
[Unit] \
Description=$(PROGRAM_NAME) Service \
After=network.target \
\
[Service] \
ExecStart=$(INSTALL_DIR)/$(PROGRAM_NAME) \
Restart=always \
User=nobody \
Group=nogroup \
WorkingDirectory=$(INSTALL_DIR) \
\
[Install] \
WantedBy=multi-user.target \
EOF \
			sudo systemctl daemon-reload; \
			sudo systemctl enable $(PROGRAM_NAME).service; \
			echo "'$(PROGRAM_NAME)' has been added to startup!"; \
		else \
			echo "Skipping startup configuration for $(PROGRAM_NAME)."; \
		fi; \
	fi

# Uninstall all binaries and remove from startup
uninstall:
	@sudo rm -rf $(INSTALL_DIR)/*
	@echo "All binaries removed from $(INSTALL_DIR)."
	@echo "Removing systemd services..."
	@for service in $(SYSTEMD_DIR)/*.service; do \
		SERVICE_NAME=$$(basename $$service .service); \
		sudo systemctl disable $$SERVICE_NAME.service; \
		sudo rm $$service; \
		echo "Removed $$SERVICE_NAME from startup."; \
	done
	@echo "Uninstallation complete!"

# Main targets

install: check_deps check_python_deps build_rust install_rust install_java
	@echo "Installation complete!"
