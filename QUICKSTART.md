# 🚀 Quick Start - AstraPDF

Guide de démarrage rapide en 5 minutes.

## 🎯 Choix du mode

AstraPDF propose **deux modes** :
- **🖥️ GUI (Interface Graphique)** : Idéal pour usage interactif
- **💻 CLI (Ligne de commande)** : Idéal pour scripts et automatisation

## Installation rapide

```bash
# 1. Compiler le projet
cargo build --release

# 2. (Optionnel) Installer globalement
sudo cp target/release/astrapdf /usr/local/bin/
```

**Ou utilisez le script d'installation :**
```bash
chmod +x install.sh
./install.sh
```

## Premiers pas

### 1️⃣ Vérifier l'installation

```bash
./target/release/astrapdf --version
# Output: astrapdf 0.3.0
```

### 2️⃣ Mode GUI (Nouveau v0.3.0!)

```bash
# Lancer l'interface graphique
./target/release/astrapdf

# L'interface s'ouvre avec :
# - Bouton pour ouvrir un PDF
# - Options d'extraction
# - Visualisation des résultats
# - Export facile
```

📖 **Voir [GUI_GUIDE.md](GUI_GUIDE.md) pour le guide complet GUI**

### 3️⃣ Mode CLI - Afficher l'aide

```bash
./target/release/astrapdf info --help
./target/release/astrapdf extract --help
```

### 4️⃣ Analyser votre premier PDF (CLI)

```bash
# Remplacez 'document.pdf' par votre fichier
astrapdf info document.pdf
```

**Résultat attendu :**
```
═══════════════════════════════════════
📂 Fichier: document.pdf
═══════════════════════════════════════
📄 Nombre de pages: 10
ℹ️  Métadonnées:
  Title: Mon Document
  ...
💾 Taille: 250 KB
═══════════════════════════════════════
```

### 4️⃣ Rechercher dans le PDF

```bash
astrapdf search document.pdf "votre terme"
```

### 5️⃣ Extraire et exporter

```bash
# Extraire en JSON
astrapdf extract document.pdf --keyword "important" --format json --output results.json

# Ouvrir les résultats
cat results.json | jq .
```

## Cas d'usage courants

### 📧 Extraire tous les emails

```bash
astrapdf extract cv.pdf \
  --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' \
  --format csv \
  --output emails.csv
```

### 💰 Extraire les montants

```bash
astrapdf extract facture.pdf \
  --regex '\d+[\s,]?\d*\s?€' \
  --format csv \
  --output montants.csv
```

### 📅 Extraire les dates

```bash
astrapdf extract planning.pdf \
  --regex '\d{2}/\d{2}/\d{4}' \
  --format txt
```

### 📑 Extraire des pages spécifiques

```bash
# Pages 5 à 10 uniquement
astrapdf extract rapport.pdf --pages "5-10" --keyword "conclusion"
```

## Commandes disponibles

| Commande | Description | Exemple |
|----------|-------------|---------|
| `info` | Infos sur le PDF | `astrapdf info doc.pdf` |
| `pages` | Liste les pages | `astrapdf pages doc.pdf --content` |
| `search` | Recherche simple | `astrapdf search doc.pdf "terme"` |
| `extract` | Extraction avancée | `astrapdf extract doc.pdf --keyword "x"` |
| `batch` | Traitement batch | `astrapdf batch *.pdf --keyword "x"` |

## Formats d'export

| Format | Extension | Usage |
|--------|-----------|-------|
| `txt` | `.txt` | Lecture humaine |
| `json` | `.json` | Traitement programmatique |
| `csv` | `.csv` | Analyse dans Excel/Sheets |

**Exemple :**
```bash
# Export JSON
astrapdf extract doc.pdf --keyword "test" --format json --output results.json

# Export CSV
astrapdf extract doc.pdf --keyword "test" --format csv --output results.csv

# Sortie terminal (défaut)
astrapdf extract doc.pdf --keyword "test"
```

## Options utiles

### Contexte autour des résultats

```bash
# 5 lignes avant et après chaque match
astrapdf extract doc.pdf --keyword "clause" --context 5
```

### Recherche insensible à la casse

```bash
astrapdf search doc.pdf "important" --ignore-case
```

### Pages spécifiques

```bash
# Une page
astrapdf extract doc.pdf --pages "5"

# Plusieurs pages
astrapdf extract doc.pdf --pages "1,5,10"

# Plage de pages
astrapdf extract doc.pdf --pages "10-20"

# Combiné
astrapdf extract doc.pdf --pages "1-5,10,15-20"
```

## Workflows pratiques

### Pipeline complet

```bash
# 1. Info du PDF
astrapdf info contrat.pdf

# 2. Chercher un terme
astrapdf search contrat.pdf "clause" --context

# 3. Extraire en JSON
astrapdf extract contrat.pdf --keyword "clause" --format json --output clauses.json

# 4. Analyser avec jq
cat clauses.json | jq '.total_matches'
```

### Traiter plusieurs PDFs

```bash
#!/bin/bash
for pdf in documents/*.pdf; do
  echo "Traitement: $pdf"
  astrapdf extract "$pdf" \
    --keyword "important" \
    --format json \
    --output "results/$(basename $pdf .pdf).json"
done
echo "✅ Terminé !"
```

## Troubleshooting

### Le binaire n'est pas trouvé

```bash
# Utilisez le chemin complet
./target/release/astrapdf --help

# Ou ajoutez au PATH
export PATH="$PATH:$(pwd)/target/release"
```

### PDF ne s'ouvre pas

```bash
# Vérifiez que le fichier existe
ls -la document.pdf

# Vérifiez les permissions
chmod 644 document.pdf

# Testez avec la commande info
astrapdf info document.pdf
```

### Pas de résultats

```bash
# 1. Testez d'abord avec search
astrapdf search doc.pdf "terme"

# 2. Essayez insensible à la casse
astrapdf search doc.pdf "terme" --ignore-case

# 3. Vérifiez le pattern regex
astrapdf search doc.pdf "votre.*pattern"
```

## Ressources

- 📖 **Documentation complète**: [README.md](README.md)
- 💡 **Exemples détaillés**: [EXAMPLES.md](EXAMPLES.md)
- 🤝 **Contribuer**: [CONTRIBUTING.md](CONTRIBUTING.md)
- 🗺️ **Roadmap**: [ROADMAP.md](ROADMAP.md)

## Support

- 🐛 **Bugs**: [GitHub Issues](https://github.com/votre-compte/astrapdf/issues)
- 💬 **Questions**: [GitHub Discussions](https://github.com/votre-compte/astrapdf/discussions)
- 🌐 **Site web**: [astrapdf.com](https://astrapdf.com)

---

**Prêt à transformer vos PDFs en données exploitables ! 🎉**

**Prochaine étape**: Consultez [EXAMPLES.md](EXAMPLES.md) pour des cas d'usage avancés.
