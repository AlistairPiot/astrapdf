.PHONY: help build release test clean install run check fmt clippy docs

# Affiche l'aide
help:
	@echo "📦 AstraPDF - Commandes disponibles:"
	@echo ""
	@echo "  make build      - Compiler en mode debug"
	@echo "  make release    - Compiler en mode release (optimisé)"
	@echo "  make test       - Lancer les tests"
	@echo "  make check      - Vérifier le code (sans compiler)"
	@echo "  make fmt        - Formater le code"
	@echo "  make clippy     - Linter le code"
	@echo "  make clean      - Nettoyer les fichiers de build"
	@echo "  make install    - Installer globalement (nécessite sudo)"
	@echo "  make run        - Compiler et lancer"
	@echo "  make docs       - Générer la documentation"
	@echo ""

# Compiler en mode debug
build:
	@echo "🔨 Compilation en mode debug..."
	cargo build
	@echo "✅ Terminé ! Binaire: target/debug/astrapdf"

# Compiler en mode release
release:
	@echo "🚀 Compilation en mode release (optimisé)..."
	cargo build --release
	@echo "✅ Terminé ! Binaire: target/release/astrapdf"
	@ls -lh target/release/astrapdf

# Lancer les tests
test:
	@echo "🧪 Lancement des tests..."
	cargo test

# Vérifier le code
check:
	@echo "🔍 Vérification du code..."
	cargo check

# Formater le code
fmt:
	@echo "🎨 Formatage du code..."
	cargo fmt
	@echo "✅ Code formaté"

# Linter avec clippy
clippy:
	@echo "📋 Analyse avec clippy..."
	cargo clippy -- -D warnings

# Nettoyer les fichiers de build
clean:
	@echo "🧹 Nettoyage..."
	cargo clean
	@echo "✅ Nettoyé"

# Installer globalement
install: release
	@echo "📍 Installation dans /usr/local/bin (mot de passe requis)..."
	sudo cp target/release/astrapdf /usr/local/bin/
	sudo chmod +x /usr/local/bin/astrapdf
	@echo "✅ Installé ! Testez avec: astrapdf --help"

# Compiler et lancer avec --help
run:
	@echo "▶️  Compilation et lancement..."
	cargo run -- --help

# Générer la documentation
docs:
	@echo "📚 Génération de la documentation..."
	cargo doc --open

# Watch mode (nécessite cargo-watch)
watch:
	@echo "👀 Mode watch activé..."
	cargo watch -x check -x test -x run

# Benchmark (nécessite cargo-criterion)
bench:
	@echo "⚡ Lancement des benchmarks..."
	cargo bench

# Vérifier la taille du binaire
size: release
	@echo "📊 Taille du binaire:"
	@ls -lh target/release/astrapdf
	@echo ""
	@echo "Avec strip:"
	@strip target/release/astrapdf
	@ls -lh target/release/astrapdf

# Tout vérifier avant commit
pre-commit: fmt clippy test
	@echo "✅ Prêt pour commit !"

# Build pour toutes les plateformes (nécessite cross)
build-all:
	@echo "🌍 Build multi-plateforme..."
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target x86_64-pc-windows-gnu
