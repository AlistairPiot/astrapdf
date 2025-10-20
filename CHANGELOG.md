# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [Unreleased]

### À venir
- Traitement batch avec multi-threading
- Extraction page par page optimisée
- Tests unitaires complets
- Templates regex prédéfinis

## [0.1.0] - 2025-10-20

### 🎉 Release initiale

#### Ajouté
- CLI complet avec 5 commandes principales
  - `info` : Affiche les métadonnées d'un PDF
  - `pages` : Liste les pages avec prévisualisation optionnelle
  - `search` : Recherche avancée dans le PDF
  - `extract` : Extraction ciblée par mot-clé ou regex
  - `batch` : Structure pour traitement batch (TODO)
- Extraction par mot-clé avec contexte configurable
- Extraction par regex pour patterns avancés
  - Emails, téléphones, dates, montants, etc.
- Sélection de pages spécifiques
  - Support des ranges (ex: 1-5,10,15-20)
- Export multi-format
  - TXT : Format texte lisible
  - JSON : Format structuré pour traitement programmatique
  - CSV : Format tableur pour Excel/Google Sheets
- Interface terminal colorée et user-friendly
- Gestion d'erreurs robuste avec messages clairs
- Documentation complète
  - README avec guide complet
  - QUICKSTART pour démarrage rapide
  - EXAMPLES avec cas d'usage pratiques
  - CONTRIBUTING pour les contributeurs
  - ROADMAP technique détaillée
  - PROJECT_SUMMARY avec business plan
- Scripts d'installation
  - install.sh pour installation automatique
  - Makefile avec commandes pratiques
- Templates GitHub
  - Issues templates (bug, feature)
  - Pull request template
  - CI/CD workflow (GitHub Actions)

#### Technique
- Langage : Rust 2021 edition
- Architecture modulaire (5 modules)
- ~700 lignes de code
- 11 dépendances principales
- Binaire optimisé : 5.4 MB
- Support Linux, macOS, Windows

#### Limitations connues
- Extraction lit tout le PDF (pas page par page)
- Batch processing non implémenté
- Pas de tests unitaires
- Performance non optimisée pour gros PDFs (>100 pages)
- Pas de support OCR (PDFs scannés)

---

## Légende

- `Ajouté` : Nouvelles fonctionnalités
- `Modifié` : Changements de fonctionnalités existantes
- `Déprécié` : Fonctionnalités qui seront supprimées
- `Supprimé` : Fonctionnalités supprimées
- `Corrigé` : Corrections de bugs
- `Sécurité` : Correctifs de sécurité

[Unreleased]: https://github.com/votre-compte/astrapdf/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/votre-compte/astrapdf/releases/tag/v0.1.0
