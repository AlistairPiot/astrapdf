# 🚀 Guide de Publication sur GitHub

Ce guide explique comment publier AstraPDF sur GitHub.

---

## ✅ Pré-requis

- [x] Git initialisé ✓
- [x] Commit initial créé ✓
- [x] Documentation complète ✓
- [x] Templates GitHub créés ✓
- [ ] Compte GitHub actif
- [ ] Repository GitHub créé

---

## 📋 Étape 1 : Créer le Repository GitHub

### Via l'interface web GitHub

1. **Aller sur GitHub** : https://github.com
2. **Cliquer sur "New repository"** (bouton vert en haut à droite)
3. **Remplir les informations** :
   - **Repository name** : `astrapdf`
   - **Description** : `⇒ Transformez vos PDF en informations exploitables - CLI Rust pour analyser et extraire des données de PDFs`
   - **Visibility** : `Public` (pour open-source)
   - **NE PAS** cocher "Initialize with README" (on a déjà un README)
   - **NE PAS** ajouter .gitignore (on en a déjà un)
   - **License** : Sélectionner "MIT License" ou laisser vide (on a déjà LICENSE)
4. **Cliquer sur "Create repository"**

---

## 📋 Étape 2 : Connecter le Repository Local

Une fois le repository créé, GitHub affiche des instructions. Utiliser la section **"push an existing repository"**.

### Commandes à exécuter

```bash
# Ajouter le remote GitHub
git remote add origin https://github.com/VOTRE_USERNAME/astrapdf.git

# Vérifier que le remote est bien ajouté
git remote -v

# Pousser le code vers GitHub
git push -u origin master
```

**Remplacez `VOTRE_USERNAME`** par votre nom d'utilisateur GitHub.

### Alternative avec SSH (recommandé)

Si vous avez configuré SSH :

```bash
git remote add origin git@github.com:VOTRE_USERNAME/astrapdf.git
git push -u origin master
```

---

## 📋 Étape 3 : Configurer le Repository

### 3.1 Ajouter des Topics

Sur GitHub, aller dans **Settings** → **General** → **Topics** et ajouter :

```
rust, cli, pdf, pdf-parser, pdf-extraction, command-line-tool,
rust-cli, pdf-tools, data-extraction, regex, json-export, csv-export
```

### 3.2 Configurer la Description

Dans **Settings** → **General** → **Description** :

```
⇒ Transformez vos PDF en informations exploitables - CLI Rust pour analyser et extraire des données de PDFs
```

### 3.3 Ajouter un Website (optionnel pour maintenant)

Dans **Settings** → **General** → **Website** :

```
https://astrapdf.com
```

(À configurer plus tard quand le site sera prêt)

### 3.4 Activer GitHub Actions

GitHub Actions devrait être automatiquement activé. Vérifier dans :
- **Actions** → Les workflows devraient être visibles

### 3.5 Activer Issues

- **Settings** → **General** → **Features**
- Cocher ✓ **Issues**
- Cocher ✓ **Projects** (optionnel)
- Cocher ✓ **Discussions** (pour la communauté)

---

## 📋 Étape 4 : Créer une Release

### 4.1 Créer un Tag

```bash
# Créer un tag pour v0.1.0
git tag -a v0.1.0 -m "Release v0.1.0 - Initial public release"

# Pousser le tag vers GitHub
git push origin v0.1.0
```

### 4.2 Créer la Release sur GitHub

1. **Aller sur** : https://github.com/VOTRE_USERNAME/astrapdf/releases
2. **Cliquer sur** "Create a new release"
3. **Remplir** :
   - **Tag** : `v0.1.0` (sélectionner le tag créé)
   - **Release title** : `v0.1.0 - Initial Release`
   - **Description** : Copier depuis CHANGELOG.md

```markdown
## 🎉 First Public Release of AstraPDF!

**AstraPDF** est un outil CLI puissant en Rust pour transformer vos PDF en informations exploitables.

### ✨ Fonctionnalités

- 📊 Analyse PDF (métadonnées, statistiques)
- 🔍 Extraction par mot-clé et regex
- 📤 Export multi-format (TXT, JSON, CSV)
- 🎨 Interface colorée et user-friendly
- 📑 Sélection de pages (ranges)
- 🔎 Recherche avancée avec contexte

### 🚀 Installation

```bash
# Cloner et compiler
git clone https://github.com/VOTRE_USERNAME/astrapdf.git
cd astrapdf
cargo build --release

# Ou utiliser le script
./install.sh
```

### 📚 Documentation

- [README](README.md) - Guide complet
- [QUICKSTART](QUICKSTART.md) - Démarrage rapide
- [EXAMPLES](EXAMPLES.md) - Exemples pratiques

### 🐛 Limitations connues

- Extraction lit tout le PDF (pas page par page)
- Batch processing non implémenté
- Pas de tests unitaires encore
- Performance non optimisée pour très gros PDFs

Voir le [CHANGELOG](CHANGELOG.md) pour détails complets.

---

**Merci d'avoir essayé AstraPDF ! 🚀**

Feedback, bugs, et feature requests : [Issues](https://github.com/VOTRE_USERNAME/astrapdf/issues)
```

4. **Ajouter les binaires** (optionnel pour v0.1.0) :
   - Compiler pour différentes plateformes
   - Uploader les binaires dans la section "Attach binaries"

5. **Cliquer sur** "Publish release"

---

## 📋 Étape 5 : Personnaliser le README GitHub

Le README s'affiche automatiquement. Vérifier qu'il s'affiche bien et ajouter éventuellement :

### Badges (en haut du README)

Ajouter avant le titre :

```markdown
[![Build Status](https://github.com/VOTRE_USERNAME/astrapdf/workflows/CI/badge.svg)](https://github.com/VOTRE_USERNAME/astrapdf/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Release](https://img.shields.io/github/v/release/VOTRE_USERNAME/astrapdf)](https://github.com/VOTRE_USERNAME/astrapdf/releases)
```

---

## 📋 Étape 6 : Activer GitHub Pages (optionnel)

Pour héberger la documentation :

1. **Settings** → **Pages**
2. **Source** : Deploy from a branch
3. **Branch** : `gh-pages` ou `main` (dossier `/docs`)
4. Créer un dossier `docs/` avec une page HTML simple

---

## 📋 Étape 7 : Promouvoir le Projet

### Sur Reddit

- r/rust
- r/commandline
- r/productivity
- r/programming

**Template post** :

```
[Project] AstraPDF - CLI Rust pour extraire des données de PDFs

Bonjour r/rust !

Je viens de publier AstraPDF, un outil CLI en Rust pour transformer
des PDF en données exploitables (JSON/CSV/TXT).

Features :
- Extraction par mot-clé et regex
- Export multi-format
- Interface colorée
- Sélection de pages
- Open-source (MIT)

GitHub : https://github.com/VOTRE_USERNAME/astrapdf

Feedback bienvenu ! 🚀
```

### Sur Hacker News

Titre : `Show HN: AstraPDF – Extract data from PDFs using Rust CLI`

### Sur Product Hunt

- Créer un post avec screenshots
- Taguer : CLI, Developer Tools, Productivity, Open Source

### Sur Twitter/X

```
🎉 Just released AstraPDF v0.1.0!

A powerful Rust CLI to transform PDFs into actionable data 📄➡️📊

✨ Keyword & regex extraction
📤 JSON/CSV/TXT export
🚀 Fast & open-source

Check it out: https://github.com/VOTRE_USERNAME/astrapdf

#rustlang #opensource #cli
```

---

## 📋 Checklist Complète

### Avant publication
- [x] Code compilé sans erreurs ni warnings
- [x] Documentation complète
- [x] LICENSE ajoutée (MIT)
- [x] .gitignore configuré
- [x] CHANGELOG créé
- [x] Templates GitHub créés
- [x] Commit initial créé

### Publication
- [ ] Repository GitHub créé
- [ ] Code poussé sur GitHub
- [ ] Topics ajoutés
- [ ] Description configurée
- [ ] Issues activées
- [ ] Tag v0.1.0 créé
- [ ] Release v0.1.0 publiée

### Après publication
- [ ] README badges ajoutés
- [ ] Discussions activées
- [ ] Post Reddit (r/rust)
- [ ] Post Hacker News
- [ ] Post Product Hunt
- [ ] Tweet

---

## 🆘 En Cas de Problème

### Erreur : "remote already exists"

```bash
git remote remove origin
git remote add origin https://github.com/VOTRE_USERNAME/astrapdf.git
```

### Erreur : "permission denied"

Utiliser HTTPS au lieu de SSH, ou configurer SSH :

```bash
ssh-keygen -t ed25519 -C "votre_email@example.com"
# Ajouter la clé publique dans GitHub Settings → SSH Keys
```

### Erreur : "failed to push"

```bash
# Forcer le push (ATTENTION : écrase l'historique distant)
git push -u origin master --force

# Ou pull d'abord
git pull origin master --allow-unrelated-histories
git push -u origin master
```

---

## 🎉 Félicitations !

Votre projet est maintenant public sur GitHub !

**Prochaines étapes** :

1. Monitorer les Issues et PRs
2. Répondre aux questions
3. Intégrer les feedbacks
4. Planifier v0.2.0

**Liens utiles** :

- Repository : https://github.com/VOTRE_USERNAME/astrapdf
- Issues : https://github.com/VOTRE_USERNAME/astrapdf/issues
- Actions : https://github.com/VOTRE_USERNAME/astrapdf/actions

---

**Bon lancement ! 🚀**
