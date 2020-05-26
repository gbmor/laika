PREFIX?=/usr/local
_INSTDIR=$(PREFIX)
BINDIR?=$(_INSTDIR)/bin

laika: src/main.rs Cargo.toml Cargo.lock
	@printf "%s\n\n" "Building laika. This will take a few minutes."
	cargo build --release
	@printf "\n%s\n" "...Done!"

.PHONY: clean
clean:
	cargo clean

.PHONY: update
update:
	@printf "\n%s\n\n" "Updating from upstream repository..."
	git pull --rebase origin master
	@printf "\n%s\n" "...Done!"

.PHONY: install
install:
	@printf "\n%s\n\n" "Installing laika..."
	@printf "%s\n" "Creating user/group..."
	addgroup laika
	adduser -home /var/gemini --system laika

	@printf "\n%s\n\n" "Creating directories..."
	mkdir -p /var/gemini

	@printf "\n%s\n" "Copying files..."
	install -m755 target/release/laika $(BINDIR)/laika
	
	@printf "\n%s\n" "Setting ownership..."
	chown -R laika:laika /var/gemini
	
	@printf "\n%s\n" "...Done!"

.PHONY: uninstall
uninstall:
	@printf "%s\n\n" "Uninstalling laika..."
	rm -f /usr/local/bin/laika
	rm -f /var/log/laika.log
	userdel laika
	groupdel laika

	@printf "\n%s\n" "...Done!"
