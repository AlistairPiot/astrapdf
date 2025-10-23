# 🗺️ Roadmap AstraPDF

Plan de développement détaillé pour transformer AstraPDF en produit commercial.

---

## ✅ Version 0.1.0 - MVP (ACTUELLE)

**Status**: Terminé
**Date**: Octobre 2025

### Fonctionnalités
- [x] CLI de base avec clap
- [x] Ouverture et parsing de PDFs (lopdf + pdf-extract)
- [x] Commande `info` : métadonnées et statistiques
- [x] Commande `pages` : listing avec prévisualisation
- [x] Commande `search` : recherche simple avec contexte
- [x] Commande `extract` : extraction par mot-clé
- [x] Commande `extract` : extraction par regex
- [x] Export multi-format : TXT, JSON, CSV
- [x] Interface colorée (colored)
- [x] Gestion d'erreurs (thiserror, anyhow)
- [x] Documentation complète (README, EXAMPLES, CONTRIBUTING)

### Limitations connues (✅ RÉSOLUES en v0.2.0)
- ✅ ~~`extract_text_from_page()` retourne tout le texte (pas page par page)~~ → **RÉSOLU**: Extraction page par page optimisée avec lopdf + parsing content streams
- ✅ ~~Batch processing non implémenté~~ → **RÉSOLU**: Module batch.rs avec rayon (multi-threading)
- ✅ ~~Pas de tests unitaires~~ → **RÉSOLU**: 25 tests d'intégration (100% pass rate)
- ✅ ~~Performance non optimisée pour gros PDFs (>100MB)~~ → **RÉSOLU**: Optimisations multi-threading + extraction page par page

---

## ✅ Version 0.2.0 - Stabilité et Performance

**Status**: ✅ Terminé
**Date de release**: 23 Octobre 2025
**Objectif**: Rendre le produit production-ready ✓

### ✅ Priorité HAUTE (Toutes complétées)

#### 1. ✅ Extraction page par page optimisée
**Problème**: Actuellement on lit tout le PDF à chaque fois
**Solution**:
```rust
// Utiliser lopdf pour extraire vraiment page par page
fn extract_text_from_page(&self, page_num: u32) -> Result<String> {
    let page_id = self.document.get_pages().get(&page_num).unwrap();
    let content = self.document.get_page_content(*page_id)?;
    // Parser le content stream...
}
```
**Bénéfices**:
- Réduction mémoire pour gros PDFs
- Extraction plus rapide des pages spécifiques
- Support de PDFs >500 pages

**Résultat**: ✅ Implémenté avec succès
- Extraction via lopdf + parsing content streams
- Fallback automatique vers pdf-extract
- Réduction mémoire significative

---

#### 2. ✅ Batch processing fonctionnel
**Implémentation**:
```rust
// src/batch.rs (nouveau module)
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn process_batch(
    files: Vec<PathBuf>,
    extractor: impl Fn(&Path) -> Result<ExtractionResult>,
) -> Vec<Result<ExtractionResult>> {
    let pb = ProgressBar::new(files.len() as u64);

    files.par_iter()
        .map(|file| {
            let result = extractor(file);
            pb.inc(1);
            result
        })
        .collect()
}
```

**Features**:
- Multi-threading avec rayon
- Progress bar avec indicatif
- Gestion d'erreurs par fichier (ne pas tout arrêter si 1 PDF échoue)
- Export consolidé (1 fichier JSON avec tous les résultats)

**Résultat**: ✅ Implémenté avec succès
- Module src/batch.rs (300+ lignes)
- Multi-threading avec rayon
- Progress bars temps réel
- Export consolidé JSON/CSV/TXT

---

#### 3. ✅ Tests unitaires et intégration
**Coverage cible**: ✅ Excellent (25 tests)

Tests à créer:
```rust
// tests/pdf_tests.rs
#[test]
fn test_pdf_info() { }

#[test]
fn test_keyword_extraction() { }

#[test]
fn test_regex_extraction() { }

#[test]
fn test_page_range_parsing() { }

#[test]
fn test_export_json() { }

#[test]
fn test_export_csv() { }
```

**Résultat**: ✅ Implémenté avec succès
- 25 tests d'intégration (100% pass rate)
- 3 fichiers: pdf_tests, export_tests, batch_tests
- Tests avec PDFs réels
- Edge cases validés

---

#### 4. ✅ Optimisation performance
**Actions**:
- Profiling avec `cargo flamegraph`
- Lazy loading du contenu PDF
- Cache des pages fréquemment accédées
- Optimisation des regex (compilation unique)

**Résultat**: ✅ Optimisations implémentées
- Extraction page par page (réduit RAM)
- Multi-threading avec rayon
- Performance: <100ms petits PDFs, ~1s batch
- Tests performance validés

**Benchmarks atteints**:
- Petits PDFs (6KB): <100ms
- PDFs moyens (500KB): <1s
- Batch 2 PDFs: ~1s avec progress bar

---

### Priorité MOYENNE

#### 5. Amélioration des exports
- [ ] Export Markdown avec formatage
- [ ] Export HTML avec highlighting
- [ ] Export Excel (xlsx) avec crate `rust_xlsxwriter`
- [ ] Templates d'export personnalisables

**Estimation**: 2 jours

---

#### 6. Configuration file support
```toml
# .astrapdf.toml
[defaults]
output_format = "json"
context_lines = 3

[regex_templates]
email = '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b'
phone_fr = '0[1-9](?:[\s.-]?\d{2}){4}'
date_fr = '\d{2}/\d{2}/\d{4}'

[export]
indent = 2
pretty = true
```

**Estimation**: 1-2 jours

---

#### 7. Regex templates prédéfinis
```bash
astrapdf extract doc.pdf --template email
astrapdf extract doc.pdf --template phone-fr
astrapdf extract doc.pdf --template date-iso
```

**Estimation**: 1 jour

---

### Priorité BASSE

#### 8. Amélioration CLI UX
- [ ] Auto-complétion shell (bash, zsh, fish)
- [ ] Alias de commandes (ex: `search` → `s`)
- [ ] Mode interactif pour extractions complexes
- [ ] Dry-run mode pour voir ce qui serait extrait

**Estimation**: 2-3 jours

---

## 🎨 Version 0.3.0 - Interface Graphique

**Status**: Planification préliminaire
**Date cible**: Mars 2026
**Objectif**: Élargir l'audience aux non-techniciens

### GUI avec egui

```rust
// src/gui.rs
use eframe::egui;

struct AstraPdfApp {
    pdf_path: String,
    keyword: String,
    results: Vec<ExtractionResult>,
}

impl eframe::App for AstraPdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AstraPDF");

            ui.horizontal(|ui| {
                ui.label("PDF:");
                ui.text_edit_singleline(&mut self.pdf_path);
                if ui.button("Browse").clicked() {
                    // File picker
                }
            });

            // ... reste de l'interface
        });
    }
}
```

### Features GUI
- [ ] Drag & drop de PDFs
- [ ] Prévisualisation du PDF
- [ ] Highlighting des résultats de recherche
- [ ] Export visuel (sélection format, destination)
- [ ] Historique des recherches
- [ ] Templates de regex avec preview

**Estimation**: 10-15 jours

---

### OCR Support

**Dépendances**:
```toml
tesseract-rs = "0.1"
image = "0.24"
```

**Fonctionnalités**:
- [ ] Détection automatique PDF scanné vs natif
- [ ] OCR avec Tesseract
- [ ] Support multi-langue (FR, EN, ES, DE)
- [ ] Pré-processing d'image (deskew, denoise)

**Estimation**: 5-7 jours

---

### Extraction de tables

**Approche**:
- Détecter les structures tabulaires
- Parser en grille ligne/colonne
- Export CSV structuré

**Libraries**:
- `pdf-extract` (baseline)
- Algo custom pour détecter alignements

**Estimation**: 7-10 jours

---

## 🚀 Version 1.0.0 - Production Ready

**Date cible**: Septembre 2026

### Fonctionnalités complètes
- [ ] Toutes les features v0.1-0.3 stables
- [ ] Performance optimale (benchmarks publiés)
- [ ] Tests coverage >80%
- [ ] Documentation API complète
- [ ] Tutoriels vidéo
- [ ] Support ticket system

### Distribution
- [ ] Binaires pour Windows (x64, ARM)
- [ ] Binaires pour macOS (Intel, Apple Silicon)
- [ ] Binaires pour Linux (x64, ARM, musl)
- [ ] Package managers:
  - [ ] Cargo (`cargo install astrapdf`)
  - [ ] Homebrew (macOS/Linux)
  - [ ] Chocolatey (Windows)
  - [ ] Snap (Linux)
  - [ ] APT/YUM repos

### Licensing & Monétisation
- [ ] Licence commerciale claire
- [ ] Système de licensing (clés de produit)
- [ ] Freemium: gratuit jusqu'à 10 PDFs/jour
- [ ] Pro: €35 licence unique
- [ ] Enterprise: €99/an avec support prioritaire

---

## 🌐 Version 2.0.0 - Cloud & API

**Date cible**: 2027

### API REST
```
POST /api/v1/extract
POST /api/v1/search
POST /api/v1/batch
GET  /api/v1/status/:job_id
```

### SaaS Platform
- [ ] Web app (React/Svelte)
- [ ] Stockage cloud des PDFs
- [ ] Historique des extractions
- [ ] Partage de résultats
- [ ] Webhooks pour intégrations
- [ ] API rate limiting

### Pricing SaaS
- Free: 5 PDFs/mois
- Pro: €10/mois - 100 PDFs/mois
- Business: €50/mois - 1000 PDFs/mois
- Enterprise: Custom

---

## 🤖 Version 3.0.0 - IA & Advanced

**Date cible**: 2028+

### Intégration LLM
- [ ] Résumés automatiques intelligents
- [ ] Q&A sur le contenu du PDF
- [ ] Classification automatique de documents
- [ ] Extraction d'entités nommées (NER)
- [ ] Traduction automatique

### Mobile Apps
- [ ] iOS app (SwiftUI)
- [ ] Android app (Kotlin)
- [ ] Sync cloud

---

## 📊 Métriques de succès

### Technique
- Performance: <5s pour PDF de 100 pages
- Précision extraction: >95%
- Crash rate: <0.1%
- Test coverage: >80%

### Business
- **Mois 1-3**: 50 utilisateurs
- **Mois 4-6**: 200 utilisateurs, €500/mois
- **Mois 7-12**: 1000 utilisateurs, €5000/mois
- **Année 2**: 5000 utilisateurs, €30000/mois

---

## ✅ Bilan v0.2.0 (Complété le 23 Oct 2025)

### Accomplissements

**Sprint 1** ✅ TERMINÉ:
1. ✅ Setup tests infrastructure (tests/ directory)
2. ✅ Implémenter extraction page par page (lopdf + fallback)
3. ✅ Tests unitaires extraction (8 tests PDF)

**Sprint 2** ✅ TERMINÉ:
1. ✅ Batch processing avec rayon (src/batch.rs)
2. ✅ Progress indicators (indicatif)
3. ✅ Tests batch (10 tests)

**Sprint 3** ✅ TERMINÉ:
1. ✅ Optimisation performance (page-by-page, parallel)
2. ✅ Tests validés (25 tests, 100% pass)
3. ✅ Documentation v0.2 (CHANGELOG, TEST_RESULTS)

**Sprint 4** ✅ TERMINÉ:
1. ✅ Release v0.2.0 (tag GitHub créé)
2. ✅ Tests avec PDFs réels (facture, documents)
3. ✅ Prêt pour feedback utilisateurs

### Statistiques Finales v0.2.0

- **Code**: ~1000 lignes (+43%)
- **Modules**: 6 (ajout: batch, lib)
- **Tests**: 25 (100% succès)
- **Performance**: <100ms petits PDFs
- **Qualité**: 0 warnings, 0 erreurs

---

## 🎯 Prochaines actions (Version 0.3.0 & au-delà)

---

**Cette roadmap est vivante et sera mise à jour selon les retours utilisateurs et priorités business.**
