# 🧪 Guide de Test - AstraPDF

Guide rapide pour tester l'outil CLI en local.

## 🚀 Option 1: Script automatique (recommandé)

```bash
# Lancer tous les tests automatiquement
./test_cli.sh
```

Ce script va:
- ✅ Compiler en mode release
- ✅ Vérifier la version
- ✅ Tester toutes les commandes
- ✅ Créer des exports de test

---

## 🔧 Option 2: Tests manuels

### Étape 1: Compiler

```bash
# Compiler en mode release (optimisé)
cargo build --release

# Ou en mode debug (plus rapide à compiler)
cargo build
```

### Étape 2: Tester les commandes de base

```bash
# Vérifier la version
./target/release/astrapdf --version

# Afficher l'aide
./target/release/astrapdf --help

# Aide pour une commande spécifique
./target/release/astrapdf extract --help
```

### Étape 3: Tester avec vos PDFs

```bash
# Info sur un PDF
./target/release/astrapdf info test_pdfs/facture.pdf

# Lister les pages
./target/release/astrapdf pages test_pdfs/facture.pdf

# Rechercher un mot
./target/release/astrapdf search test_pdfs/facture.pdf "votre-mot"

# Extraire avec mot-clé
./target/release/astrapdf extract test_pdfs/facture.pdf --keyword "important"

# Extraire avec regex (emails)
./target/release/astrapdf extract test_pdfs/facture.pdf --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b'

# Batch processing
./target/release/astrapdf batch test_pdfs/*.pdf --keyword "test" --format json --output-dir results/
```

---

## 📦 Option 3: Utiliser cargo run (mode dev)

```bash
# Plus lent mais pratique pour le développement
cargo run -- info test_pdfs/facture.pdf
cargo run -- extract test_pdfs/facture.pdf --keyword "test"
cargo run -- batch test_pdfs/*.pdf --keyword "important"
```

---

## 🔨 Option 4: Installer globalement

```bash
# Installer dans /usr/local/bin
sudo cp target/release/astrapdf /usr/local/bin/

# Tester
astrapdf --version
astrapdf info test_pdfs/facture.pdf
```

**Pour désinstaller:**
```bash
sudo rm /usr/local/bin/astrapdf
```

---

## 🎯 Exemples de tests rapides

### Test 1: Extraire des emails

```bash
./target/release/astrapdf extract test_pdfs/facture.pdf \
  --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' \
  --format json \
  --output emails.json
```

### Test 2: Extraire des montants en euros

```bash
./target/release/astrapdf extract test_pdfs/facture.pdf \
  --regex '\d+[,\.]?\d*\s?€' \
  --format csv \
  --output montants.csv
```

### Test 3: Batch avec plusieurs PDFs

```bash
./target/release/astrapdf batch \
  test_pdfs/*.pdf \
  --keyword "Payo365" \
  --format json \
  --output-dir batch_results/
```

### Test 4: Extraire des pages spécifiques

```bash
./target/release/astrapdf extract test_pdfs/doc1.pdf \
  --pages "1-3,5" \
  --keyword "important" \
  --format txt \
  --output extract.txt
```

---

## 🐛 Debugging

### Voir les logs détaillés

```bash
# Avec cargo run
RUST_LOG=debug cargo run -- info test_pdfs/facture.pdf

# Avec le binaire
RUST_LOG=debug ./target/release/astrapdf info test_pdfs/facture.pdf
```

### Vérifier les dépendances

```bash
cargo tree
```

### Lancer les tests unitaires

```bash
# Tous les tests
cargo test

# Tests spécifiques
cargo test pdf_tests
cargo test batch_tests
cargo test export_tests

# Avec output détaillé
cargo test -- --nocapture
```

---

## 📂 Structure des fichiers de test

```
astrapdf/
├── test_pdfs/           # Vos PDFs de test
│   ├── facture.pdf
│   └── doc1.pdf
├── results/             # Exports générés
│   ├── batch_results.json
│   └── extractions.csv
└── target/release/      # Binaire compilé
    └── astrapdf
```

---

## ✅ Checklist de test

Avant de considérer une version comme stable:

- [ ] Compilation sans warnings
- [ ] `--version` affiche la bonne version
- [ ] `--help` fonctionne
- [ ] `info` affiche les métadonnées
- [ ] `pages` liste correctement les pages
- [ ] `search` trouve les occurrences
- [ ] `extract --keyword` fonctionne
- [ ] `extract --regex` fonctionne
- [ ] `extract --pages` fonctionne
- [ ] Export JSON valide
- [ ] Export CSV valide
- [ ] Export TXT valide
- [ ] Batch processing avec progress bar
- [ ] Batch export consolidé
- [ ] Gestion d'erreurs (PDF inexistant, etc.)
- [ ] Tests unitaires passent (100%)

---

## 🚀 Performances

Pour tester les performances:

```bash
# Mesurer le temps d'exécution
time ./target/release/astrapdf batch test_pdfs/*.pdf --keyword "test"

# Avec hyperfine (plus précis)
hyperfine './target/release/astrapdf info test_pdfs/facture.pdf'
```

---

**Bon test!** 🎉
