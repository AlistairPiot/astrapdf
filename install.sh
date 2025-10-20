#!/bin/bash

# AstraPDF Installation Script
# Usage: ./install.sh

set -e

echo "🚀 Installation d'AstraPDF..."
echo ""

# Vérifier que Rust est installé
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust n'est pas installé."
    echo "📥 Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo "✅ Rust est installé ($(rustc --version))"
fi

# Compiler en mode release
echo ""
echo "🔨 Compilation d'AstraPDF (cela peut prendre quelques minutes)..."
cargo build --release

# Vérifier que la compilation a réussi
if [ ! -f "target/release/astrapdf" ]; then
    echo "❌ Erreur lors de la compilation"
    exit 1
fi

echo "✅ Compilation réussie"

# Demander où installer
echo ""
echo "📍 Où souhaitez-vous installer AstraPDF ?"
echo "1) /usr/local/bin (recommandé, nécessite sudo)"
echo "2) ~/.local/bin (utilisateur uniquement)"
echo "3) Autre emplacement"
echo "4) Ne pas installer (juste compiler)"

read -p "Choix [1-4]: " choice

case $choice in
    1)
        echo "🔒 Installation dans /usr/local/bin (mot de passe requis)..."
        sudo cp target/release/astrapdf /usr/local/bin/
        sudo chmod +x /usr/local/bin/astrapdf
        echo "✅ AstraPDF installé dans /usr/local/bin/"
        ;;
    2)
        mkdir -p ~/.local/bin
        cp target/release/astrapdf ~/.local/bin/
        chmod +x ~/.local/bin/astrapdf
        echo "✅ AstraPDF installé dans ~/.local/bin/"

        # Vérifier si ~/.local/bin est dans le PATH
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo ""
            echo "⚠️  Ajoutez ~/.local/bin à votre PATH :"
            echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
            echo "   source ~/.bashrc"
        fi
        ;;
    3)
        read -p "Chemin d'installation: " custom_path
        if [ ! -d "$custom_path" ]; then
            mkdir -p "$custom_path"
        fi
        cp target/release/astrapdf "$custom_path/"
        chmod +x "$custom_path/astrapdf"
        echo "✅ AstraPDF installé dans $custom_path/"
        ;;
    4)
        echo "✅ Compilation terminée. L'exécutable est dans target/release/astrapdf"
        ;;
    *)
        echo "❌ Choix invalide"
        exit 1
        ;;
esac

# Test de l'installation
echo ""
echo "🧪 Test de l'installation..."
if command -v astrapdf &> /dev/null; then
    echo "✅ AstraPDF est accessible depuis le terminal"
    astrapdf --version
else
    echo "⚠️  AstraPDF n'est pas dans le PATH. Utilisez le chemin complet ou ajoutez-le au PATH"
fi

echo ""
echo "🎉 Installation terminée !"
echo ""
echo "📖 Pour commencer :"
echo "   astrapdf --help"
echo "   astrapdf info document.pdf"
echo "   astrapdf search document.pdf 'terme à chercher'"
echo ""
echo "📚 Documentation complète : README.md"
echo "🌐 Site web : https://astrapdf.com"
