# 🎨 Guide de l'Interface Graphique - AstraPDF GUI

AstraPDF v0.3.0 propose maintenant une interface graphique intuitive pour faciliter l'analyse de PDFs.

## 🚀 Lancement de l'interface GUI

### Méthode 1: Lancement direct (recommandé)

```bash
# Double-clic sur le binaire ou lancer sans arguments
./target/release/astrapdf
```

### Méthode 2: Avec l'option --gui

```bash
./target/release/astrapdf --gui
```

### Méthode 3: Avec cargo

```bash
cargo run --release
```

---

## 📖 Utilisation de l'interface

### 1️⃣ Charger un PDF

1. Cliquez sur le bouton **"📁 Ouvrir PDF"** dans le panneau de gauche
2. Sélectionnez votre fichier PDF
3. Les informations du PDF s'affichent automatiquement :
   - Nom du fichier
   - Nombre de pages
   - Taille du fichier
   - Métadonnées (titre, auteur, etc.)

### 2️⃣ Choisir le mode d'extraction

Quatre modes disponibles :

#### **Mot-clé**
- Recherche un mot ou une phrase exacte
- Exemple : "contrat", "article 12", "confidentialité"

#### **Regex**
- Recherche par expression régulière
- Exemples :
  - Emails : `\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b`
  - Montants : `\d+[,\.]?\d*\s?€`
  - Dates : `\d{2}/\d{2}/\d{4}`

#### **Pages**
- Extraction de pages spécifiques
- Format : `1-5,10,15-20` (plages et pages individuelles)

#### **Tout**
- Extrait tout le contenu du PDF

### 3️⃣ Configurer le contexte

Utilisez le slider **"Lignes de contexte"** pour définir combien de lignes avant/après chaque résultat doivent être affichées (0-10).

### 4️⃣ Lancer l'extraction

Cliquez sur **"▶️ Extraire"** pour lancer l'analyse.

### 5️⃣ Visualiser les résultats

Les résultats s'affichent dans le panneau central :
- Organisés par page
- Nombre de correspondances trouvées
- Texte trouvé surligné en jaune
- Contexte avant/après en gris

### 6️⃣ Exporter les résultats

1. Choisissez le format d'export :
   - **JSON** : Format structuré pour intégrations
   - **CSV** : Compatible Excel/Google Sheets
   - **TXT** : Texte simple lisible

2. Cliquez sur **"💾 Exporter les résultats"**

3. Choisissez l'emplacement et le nom du fichier

---

## 🎯 Cas d'usage

### Extraire tous les emails d'un contrat

1. Ouvrir le PDF
2. Mode : **Regex**
3. Pattern : `\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b`
4. Contexte : 2 lignes
5. Extraire
6. Exporter en JSON

### Rechercher une clause spécifique

1. Ouvrir le PDF
2. Mode : **Mot-clé**
3. Keyword : "clause résolutoire"
4. Contexte : 5 lignes (pour voir le contexte complet)
5. Extraire
6. Exporter en TXT

### Extraire les 3 premières pages

1. Ouvrir le PDF
2. Mode : **Pages**
3. Pages : "1-3"
4. Extraire
5. Exporter en CSV

---

## 💡 Astuces

### Performance
- Pour les gros PDFs (>100 pages), l'extraction peut prendre quelques secondes
- Le mode **Pages** est le plus rapide pour des extractions ciblées

### Regex Templates (à venir)
- Bouton **"📋 Templates"** permettra de sélectionner des patterns prédéfinis

### Raccourcis clavier
- `Ctrl+O` : Ouvrir un PDF (à venir)
- `Ctrl+E` : Lancer extraction (à venir)
- `Ctrl+S` : Exporter (à venir)

---

## 🐛 Dépannage

### L'interface ne se lance pas

```bash
# Vérifier les dépendances système
# Linux: installer libgtk-3-dev si nécessaire
sudo apt-get install libgtk-3-dev

# Compiler en mode debug pour voir les erreurs
cargo run
```

### Erreur "PDF non valide"

- Vérifiez que le fichier est bien un PDF
- Certains PDFs protégés ou corrompus peuvent échouer
- Utilisez le mode CLI pour plus de détails :
  ```bash
  ./target/release/astrapdf info votre-fichier.pdf
  ```

### Les résultats sont vides

- Vérifiez l'orthographe du mot-clé
- Pour regex : testez d'abord avec un pattern simple
- Utilisez le mode **Tout** pour voir si le PDF contient du texte

---

## 🆚 CLI vs GUI : Quand utiliser quoi ?

### Utilisez la **GUI** si :
- ✅ Vous préférez une interface visuelle
- ✅ Vous testez différents patterns
- ✅ Vous voulez voir les résultats immédiatement
- ✅ Vous traitez 1-2 PDFs à la fois

### Utilisez le **CLI** si :
- ✅ Vous automatisez des traitements
- ✅ Vous traitez de nombreux PDFs (batch processing)
- ✅ Vous intégrez dans des scripts
- ✅ Vous utilisez SSH/serveur distant

---

## 🔄 Retour au mode CLI

Pour utiliser les commandes CLI, ajoutez des arguments :

```bash
# Mode CLI
./target/release/astrapdf info document.pdf
./target/release/astrapdf extract document.pdf --keyword "test"
./target/release/astrapdf batch *.pdf --keyword "important"
```

---

## 📸 Captures d'écran

*(À venir dans les prochaines versions)*

---

## 🔮 Fonctionnalités futures

### v0.3.1
- [ ] Drag & drop de fichiers PDF
- [ ] Templates regex prédéfinis
- [ ] Historique des recherches
- [ ] Mode sombre

### v0.4.0
- [ ] Prévisualisation PDF intégrée
- [ ] Highlighting des résultats dans le PDF
- [ ] Support multi-onglets (plusieurs PDFs)
- [ ] Batch processing via GUI

---

**Interface créée avec ❤️ en Rust + egui** | **⇒ Transformez vos PDF en informations exploitables**
