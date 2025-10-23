#!/bin/bash

# Script de test pour AstraPDF CLI
# Usage: ./test_cli.sh

set -e

echo "=========================================="
echo "🧪 Script de test AstraPDF CLI"
echo "=========================================="
echo ""

# Couleurs
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. Compiler en mode release
echo -e "${BLUE}📦 Étape 1: Compilation en mode release...${NC}"
cargo build --release
echo ""

# 2. Vérifier la version
echo -e "${BLUE}📌 Étape 2: Vérification de la version${NC}"
./target/release/astrapdf --version
echo ""

# 3. Afficher l'aide
echo -e "${BLUE}ℹ️  Étape 3: Affichage de l'aide${NC}"
./target/release/astrapdf --help
echo ""

# 4. Vérifier les PDFs de test
echo -e "${BLUE}📂 Étape 4: Vérification des PDFs de test${NC}"
if [ -d "test_pdfs" ]; then
    echo "✅ Dossier test_pdfs/ trouvé"
    ls -lh test_pdfs/*.pdf 2>/dev/null || echo "⚠️  Aucun PDF trouvé dans test_pdfs/"
else
    echo "⚠️  Dossier test_pdfs/ non trouvé"
    echo "Créez un dossier test_pdfs/ et ajoutez-y des PDFs pour tester"
fi
echo ""

# 5. Tester la commande info
if [ -f "test_pdfs/facture.pdf" ]; then
    echo -e "${BLUE}📊 Étape 5: Test de la commande 'info'${NC}"
    ./target/release/astrapdf info test_pdfs/facture.pdf
    echo ""
fi

# 6. Tester la commande pages
if [ -f "test_pdfs/facture.pdf" ]; then
    echo -e "${BLUE}📄 Étape 6: Test de la commande 'pages'${NC}"
    ./target/release/astrapdf pages test_pdfs/facture.pdf
    echo ""
fi

# 7. Tester la commande search
if [ -f "test_pdfs/facture.pdf" ]; then
    echo -e "${BLUE}🔍 Étape 7: Test de la commande 'search'${NC}"
    ./target/release/astrapdf search test_pdfs/facture.pdf "Payo365" || echo "⚠️  Mot-clé 'Payo365' non trouvé"
    echo ""
fi

# 8. Tester la commande extract
if [ -f "test_pdfs/facture.pdf" ]; then
    echo -e "${BLUE}📤 Étape 8: Test de la commande 'extract'${NC}"
    ./target/release/astrapdf extract test_pdfs/facture.pdf --keyword "Payo365" || echo "⚠️  Extraction avec mot-clé 'Payo365' a échoué"
    echo ""
fi

# 9. Tester le batch processing
if [ -d "test_pdfs" ] && [ $(ls test_pdfs/*.pdf 2>/dev/null | wc -l) -gt 0 ]; then
    echo -e "${BLUE}⚡ Étape 9: Test du batch processing${NC}"
    mkdir -p test_output
    ./target/release/astrapdf batch test_pdfs/*.pdf --keyword "Payo365" --format json --output-dir test_output || echo "⚠️  Batch processing a échoué"
    echo ""

    if [ -f "test_output/batch_results.json" ]; then
        echo "✅ Résultat batch créé: test_output/batch_results.json"
        echo "Contenu:"
        cat test_output/batch_results.json | head -20
    fi
fi

echo ""
echo "=========================================="
echo -e "${GREEN}✅ Tests terminés!${NC}"
echo "=========================================="
echo ""
echo "Pour tester manuellement, utilisez:"
echo -e "${YELLOW}  ./target/release/astrapdf --help${NC}"
echo -e "${YELLOW}  ./target/release/astrapdf info test_pdfs/facture.pdf${NC}"
echo -e "${YELLOW}  ./target/release/astrapdf extract test_pdfs/facture.pdf --keyword 'votre-mot'${NC}"
echo ""
