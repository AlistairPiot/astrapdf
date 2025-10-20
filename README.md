# AstraPDF

**⇒ Transformez vos PDF en informations exploitables**

Un outil CLI puissant et rapide pour analyser, extraire et transformer vos documents PDF en données structurées.

## 🎯 Problème résolu

Les professionnels (juristes, chercheurs, analystes, consultants) traitent souvent des **PDF volumineux et complexes** et ont besoin de :

-   ✅ Extraire certaines sections ou clauses automatiquement
-   ✅ Rechercher des termes précis dans des centaines de pages
-   ✅ Générer des résumés ou rapports structurés
-   ✅ Convertir en formats exploitables (CSV, JSON, TXT)

Les solutions existantes sont souvent lentes, coûteuses, ou mal adaptées à des workflows précis.

## ✨ Fonctionnalités

-   📊 **Analyse PDF** : Affiche métadonnées, nombre de pages, taille du fichier
-   📑 **Liste des pages** : Visualise le contenu de chaque page
-   🔍 **Extraction ciblée** : Extrait par mot-clé, regex, ou numéros de pages
-   🔎 **Recherche avancée** : Trouve des occurrences avec contexte
-   📤 **Export multi-format** : TXT, JSON, CSV
-   ⚡ **Traitement batch** : Analyse plusieurs PDFs simultanément (en développement)
-   🎨 **Interface colorée** : Output terminal agréable et lisible

## 🚀 Installation

### À partir du code source

```bash
# Cloner le dépôt
git clone https://github.com/votre-compte/astrapdf.git
cd astrapdf

# Compiler en mode release
cargo build --release

# L'exécutable sera disponible dans target/release/astrapdf
```

### Ajouter au PATH (optionnel)

```bash
# Linux/macOS
sudo cp target/release/astrapdf /usr/local/bin/

# Ou créer un alias dans votre shell
echo 'alias astrapdf="~/path/to/astrapdf/target/release/astrapdf"' >> ~/.bashrc
```

## 📖 Utilisation

### Afficher les informations d'un PDF

```bash
astrapdf info document.pdf
```

Affiche :

-   Nombre de pages
-   Métadonnées (titre, auteur, date)
-   Taille du fichier

### Lister toutes les pages

```bash
# Liste simple
astrapdf pages document.pdf

# Avec aperçu du contenu
astrapdf pages document.pdf --content
```

### Rechercher dans un PDF

```bash
# Recherche simple
astrapdf search document.pdf "clause résolutoire"

# Recherche avec contexte
astrapdf search document.pdf "article 12" --context

# Recherche insensible à la casse
astrapdf search document.pdf "important" --ignore-case
```

### Extraire avec mot-clé

```bash
# Extraction simple
astrapdf extract document.pdf --keyword "confidentialité"

# Avec contexte (2 lignes avant/après par défaut)
astrapdf extract document.pdf --keyword "clause" --context 3

# Export en JSON
astrapdf extract document.pdf --keyword "article" --format json --output results.json

# Export en CSV
astrapdf extract document.pdf --keyword "montant" --format csv --output data.csv
```

### Extraction par regex

```bash
# Trouver tous les emails
astrapdf extract document.pdf --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b'

# Trouver des montants
astrapdf extract document.pdf --regex '\d+[\s,]?\d*\s?€'

# Trouver des dates
astrapdf extract document.pdf --regex '\d{2}/\d{2}/\d{4}'
```

### Extraction de pages spécifiques

```bash
# Pages individuelles
astrapdf extract document.pdf --pages "1,5,10"

# Plages de pages
astrapdf extract document.pdf --pages "1-5,10-15"

# Combinaison
astrapdf extract document.pdf --pages "1,3-7,12" --keyword "article"
```

### Traitement batch (en développement)

```bash
# Analyser un répertoire
astrapdf batch ./documents/*.pdf --keyword "important" --output-dir ./results

# Plusieurs fichiers spécifiques
astrapdf batch doc1.pdf doc2.pdf doc3.pdf --regex '\d+€' --format json --output-dir ./exports
```

## 🎯 Cas d'usage

### Juristes / Avocats

```bash
# Extraire toutes les clauses d'un contrat
astrapdf extract contrat.pdf --keyword "clause" --format json --output clauses.json

# Trouver toutes les références légales
astrapdf extract jugement.pdf --regex "article\s+\d+" --context 2
```

### Analystes / Consultants

```bash
# Extraire des données chiffrées
astrapdf extract rapport.pdf --regex '\d+[\s,]?\d*\s?€' --format csv --output montants.csv

# Rechercher des mentions spécifiques
astrapdf search audit.pdf "recommandation" --context
```

### Chercheurs

```bash
# Extraire des citations
astrapdf extract these.pdf --regex '\([A-Za-z]+,\s*\d{4}\)'

# Trouver des sections
astrapdf extract article.pdf --keyword "méthodologie" --context 5
```

## 🏗️ Architecture technique

-   **Langage** : Rust (vitesse, sécurité, portabilité)
-   **Parsing PDF** : `lopdf` + `pdf-extract`
-   **CLI** : `clap` v4 avec derive macros
-   **Export** : `serde_json` + `csv`
-   **Interface** : `colored` pour output terminal

## 📊 Roadmap

### Version 0.1.0 (Actuelle)

-   [x] CLI de base
-   [x] Extraction par mot-clé
-   [x] Extraction par regex
-   [x] Export TXT/JSON/CSV
-   [x] Recherche avec contexte

### Version 0.2.0

-   [ ] Traitement batch fonctionnel
-   [ ] Extraction page par page optimisée
-   [ ] Support des tables et annexes
-   [ ] Résumé automatique (algorithme basique)

### Version 0.3.0

-   [ ] Interface GUI légère (egui)
-   [ ] Support OCR pour PDFs scannés
-   [ ] Extraction d'images
-   [ ] Plugins / extensibilité

### Version 1.0.0

-   [ ] Performance optimale
-   [ ] Tests complets
-   [ ] Documentation complète
-   [ ] Distribution multi-plateforme (Windows, macOS, Linux)

## 💰 Modèle économique

### Options envisagées

1. **Licence annuelle** : 170€ / utilisateur
2. **Abonnement SaaS** : 28€/mois
3. **Freemium** : Extraction simple gratuite, fonctionnalités avancées payantes

## 🤝 Contribution

## 📝 Licence

MIT License - voir le fichier LICENSE pour plus de détails

## 🌐 Contact

-   **Website** : [astrapdf.com](https://astrapdf.com) (à venir)
-   **Email** : contact@astrapdf.com
-   **Issues** : [GitHub Issues](https://github.com/votre-compte/astrapdf/issues)

---

**Made with ❤️ in Rust** | **⇒ Transformez vos PDF en informations exploitables**
