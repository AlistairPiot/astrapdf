#!/bin/bash

# Script de test pour AstraPDF
# Usage: ./test_app.sh [chemin_vers_pdf]

set -e

ASTRAPDF="./target/release/astrapdf"
TEST_PDF="${1:-}"

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                                                                  ║"
echo "║              🧪 TESTS ASTRAPDF v0.1.0                           ║"
echo "║                                                                  ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

# Vérifier que le binaire existe
if [ ! -f "$ASTRAPDF" ]; then
    echo "❌ Binaire non trouvé: $ASTRAPDF"
    echo "   Compilez d'abord avec: make release"
    exit 1
fi

echo "✅ Binaire trouvé: $ASTRAPDF"
echo ""

# Test 1: Version
echo "═══ Test 1: Version ═══"
$ASTRAPDF --version
echo ""

# Test 2: Help
echo "═══ Test 2: Help général ═══"
$ASTRAPDF --help | head -10
echo "..."
echo ""

# Test 3: Help commande extract
echo "═══ Test 3: Help extract ═══"
$ASTRAPDF extract --help | head -15
echo "..."
echo ""

# Tests avec PDF (si fourni)
if [ -n "$TEST_PDF" ]; then
    if [ ! -f "$TEST_PDF" ]; then
        echo "❌ Fichier PDF non trouvé: $TEST_PDF"
        exit 1
    fi

    echo "📄 Test avec PDF: $TEST_PDF"
    echo ""

    # Test 4: Info
    echo "═══ Test 4: Info PDF ═══"
    $ASTRAPDF info "$TEST_PDF" || echo "⚠️  Erreur lors de l'analyse du PDF"
    echo ""

    # Test 5: Pages (sans contenu)
    echo "═══ Test 5: Liste des pages ═══"
    $ASTRAPDF pages "$TEST_PDF" | head -20 || echo "⚠️  Erreur lors du listing des pages"
    echo "..."
    echo ""

    # Test 6: Search (terme commun)
    echo "═══ Test 6: Recherche simple ═══"
    echo "Recherche du mot 'the' (ou 'le')..."
    $ASTRAPDF search "$TEST_PDF" "the" 2>/dev/null | head -20 || \
    $ASTRAPDF search "$TEST_PDF" "le" 2>/dev/null | head -20 || \
    echo "⚠️  Pas de résultats ou erreur"
    echo "..."
    echo ""

    # Test 7: Extract avec export JSON
    echo "═══ Test 7: Extraction et export JSON ═══"
    TMP_JSON="/tmp/astrapdf_test_$$.json"
    $ASTRAPDF extract "$TEST_PDF" --keyword "the" --format json --output "$TMP_JSON" 2>/dev/null || \
    $ASTRAPDF extract "$TEST_PDF" --keyword "le" --format json --output "$TMP_JSON" 2>/dev/null || \
    echo "⚠️  Pas de résultats"

    if [ -f "$TMP_JSON" ]; then
        echo "✅ Fichier JSON créé: $TMP_JSON"
        echo "Aperçu:"
        head -20 "$TMP_JSON" 2>/dev/null || echo "Fichier vide ou erreur"
        rm -f "$TMP_JSON"
    fi
    echo ""

    # Test 8: Extract avec regex (emails)
    echo "═══ Test 8: Extraction regex (emails) ═══"
    $ASTRAPDF extract "$TEST_PDF" \
        --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' \
        2>/dev/null | head -20 || echo "⚠️  Pas d'emails trouvés ou erreur"
    echo ""

else
    echo "⚠️  Aucun PDF fourni pour les tests avancés"
    echo ""
    echo "Usage: $0 chemin/vers/fichier.pdf"
    echo ""
    echo "Tests de base terminés avec succès ! ✅"
fi

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                                                                  ║"
echo "║                  ✅ TESTS TERMINÉS !                            ║"
echo "║                                                                  ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""
echo "Pour tester avec un PDF:"
echo "  $0 votre_fichier.pdf"
echo ""
