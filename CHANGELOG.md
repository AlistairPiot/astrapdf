# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [Unreleased]

### À venir
- Drag & drop dans la GUI
- Templates regex prédéfinis avec preview
- Prévisualisation PDF intégrée
- Configuration file support (.astrapdf.toml)
- Support OCR pour PDFs scannés

## [0.3.0] - 2025-10-23

### 🎨 Interface Graphique

#### Ajouté
- **Interface graphique complète avec egui**
  - Mode dual : GUI ou CLI selon les arguments
  - Sélection de fichiers avec dialogue natif (rfd)
  - Affichage des informations PDF (métadonnées, pages, taille)
  - 4 modes d'extraction : Mot-clé, Regex, Pages, Tout
  - Configuration du contexte avec slider (0-10 lignes)
  - Visualisation des résultats avec highlighting
  - Export intégré vers JSON/CSV/TXT
  - Barre de status en temps réel
- **Nouveau module gui.rs** (450+ lignes)
  - AstraPdfApp avec state management
  - Intégration complète avec PdfAnalyzer
  - Interface responsive et intuitive
- **Documentation GUI**
  - GUI_GUIDE.md : Guide complet d'utilisation
  - Exemples de cas d'usage
  - Troubleshooting et FAQ
- **Helper methods dans PdfAnalyzer**
  - get_page_count() : Retourne le nombre de pages
  - get_metadata() : Extrait les métadonnées PDF

#### Modifié
- main.rs supporte maintenant dual mode (CLI + GUI)
- Lancement sans arguments démarre la GUI
- ExportFormat implémente PartialEq pour egui
- lib.rs expose le module gui

#### Technique
- Ajout dépendances : eframe 0.29, egui 0.29, rfd 0.15
- Binaire : ~6.2 MB (légère augmentation pour GUI)
- ~1450 lignes de code (+450)
- Compatible Linux, macOS, Windows

#### Améliorations UX
- Mode GUI pour utilisateurs non-techniciens
- Dialogues de fichiers natifs
- Résultats surlignés pour meilleure lisibilité
- Messages de status clairs

## [0.2.0] - 2025-10-20

### ✨ Stabilité et Performance

#### Ajouté
- **Extraction page par page optimisée**
  - Utilisation de lopdf pour extraction ciblée des pages
  - Parsing des content streams PDF
  - Fallback automatique vers pdf-extract si échec
  - Réduction significative de l'utilisation mémoire pour gros PDFs
- **Batch processing fonctionnel**
  - Multi-threading avec rayon pour traitement parallèle
  - Progress bar en temps réel avec indicatif
  - Gestion d'erreurs par fichier (un échec n'arrête pas le lot)
  - Support répertoires et fichiers individuels
  - Export consolidé (JSON, CSV, TXT)
- **Module batch.rs** (300+ lignes)
  - BatchProcessor avec from_paths()
  - BatchResult et BatchSummary structures
  - Statistiques détaillées (succès/échecs)
- **Serialization support**
  - ExtractionResult et MatchResult sont maintenant Serialize
  - Compatible avec serde_json et csv

#### Modifié
- extract_text_from_page() maintenant extrait vraiment page par page
- Commande batch pleinement implémentée (n'est plus TODO)
- Amélioration gestion d'erreurs dans extraction

#### Technique
- Ajout dépendance rayon 1.10 pour parallélisation
- Architecture modulaire maintenue
- ~1000 lignes de code (+300)

#### Performance
- Traitement parallèle des PDFs en batch
- Extraction page par page réduit RAM
- Smart caching et fallback

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
