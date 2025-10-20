# Exemples d'utilisation AstraPDF

Guide pratique avec des exemples concrets pour différents cas d'usage.

## 📊 Analyse de base

### Obtenir des informations sur un PDF

```bash
astrapdf info contrat.pdf
```

**Output :**
```
═══════════════════════════════════════
📂 Fichier: contrat.pdf
═══════════════════════════════════════
📄 Nombre de pages: 25
ℹ️  Métadonnées:
  Title: Contrat de prestation
  Author: Cabinet Juridique XYZ
  CreationDate: 2024-01-15
💾 Taille: 342 KB
═══════════════════════════════════════
```

---

## 🔍 Recherche simple

### Chercher un terme dans tout le document

```bash
astrapdf search facture.pdf "montant total"
```

### Recherche insensible à la casse

```bash
astrapdf search document.pdf "important" --ignore-case
```

### Recherche avec contexte

```bash
astrapdf search contrat.pdf "clause résolutoire" --context
```

---

## 📤 Extraction et export

### Extraire toutes les occurrences d'un mot-clé

```bash
# Format texte (par défaut)
astrapdf extract rapport.pdf --keyword "recommandation"

# Export en JSON
astrapdf extract rapport.pdf --keyword "recommandation" --format json --output recommandations.json

# Export en CSV
astrapdf extract rapport.pdf --keyword "recommandation" --format csv --output recommandations.csv
```

### Extraire avec contexte étendu

```bash
# 5 lignes de contexte avant et après
astrapdf extract these.pdf --keyword "méthodologie" --context 5
```

---

## 🎯 Extraction par regex

### Extraire tous les emails

```bash
astrapdf extract cv.pdf \
  --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' \
  --format csv \
  --output emails.csv
```

### Extraire des montants en euros

```bash
# Format: 1000€, 1 000€, 1000 €, etc.
astrapdf extract facture.pdf \
  --regex '\d+[\s,]?\d*\s?€' \
  --format json \
  --output montants.json
```

### Extraire des dates (format français)

```bash
# Format: JJ/MM/AAAA
astrapdf extract planning.pdf \
  --regex '\d{2}/\d{2}/\d{4}' \
  --format csv \
  --output dates.csv
```

### Extraire des numéros de téléphone français

```bash
astrapdf extract annuaire.pdf \
  --regex '0[1-9](?:[\s.-]?\d{2}){4}' \
  --format csv \
  --output telephones.csv
```

### Extraire des références légales

```bash
# Articles de loi (ex: article 12, article L123-45)
astrapdf extract jugement.pdf \
  --regex '[Aa]rticle\s+[A-Z]?\d+(-\d+)?' \
  --context 3
```

---

## 📑 Extraction de pages spécifiques

### Une seule page

```bash
astrapdf extract rapport.pdf --pages "5"
```

### Pages individuelles

```bash
astrapdf extract rapport.pdf --pages "1,5,10,15"
```

### Plages de pages

```bash
# Pages 1 à 10
astrapdf extract rapport.pdf --pages "1-10"

# Plusieurs plages
astrapdf extract rapport.pdf --pages "1-5,10-15,20-25"
```

### Combinaison pages + mot-clé

```bash
astrapdf extract rapport.pdf \
  --pages "10-20" \
  --keyword "conclusion" \
  --format json \
  --output conclusions.json
```

---

## 💼 Cas d'usage professionnels

### Avocat : Extraire toutes les clauses d'un contrat

```bash
astrapdf extract contrat_bail.pdf \
  --regex '[Cc]lause\s+\d+\s*[:-]?' \
  --context 10 \
  --format json \
  --output clauses.json
```

### Comptable : Extraire tous les montants d'une facture

```bash
astrapdf extract facture_janvier.pdf \
  --regex '\d+[,.]?\d*\s?€' \
  --format csv \
  --output montants_facture.csv
```

### RH : Extraire les emails et téléphones de CVs

```bash
# Emails
astrapdf extract cv_candidat.pdf \
  --regex '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' \
  --format txt \
  --output email.txt

# Téléphones
astrapdf extract cv_candidat.pdf \
  --regex '0[1-9](?:[\s.-]?\d{2}){4}' \
  --format txt \
  --output telephone.txt
```

### Chercheur : Extraire toutes les citations bibliographiques

```bash
# Format (Auteur, Année)
astrapdf extract article_scientifique.pdf \
  --regex '\([A-Z][a-z]+(?:\s+et\s+al\.?)?,\s*\d{4}\)' \
  --format csv \
  --output citations.csv
```

### Analyste : Extraire les sections "Risques" d'un rapport

```bash
astrapdf extract rapport_annuel.pdf \
  --keyword "risque" \
  --context 8 \
  --format json \
  --output risques.json
```

### Juriste : Trouver toutes les références au Code Civil

```bash
astrapdf extract jugement.pdf \
  --regex 'articles?\s+\d+(?:-\d+)?\s+(?:du\s+)?[Cc]ode\s+[Cc]ivil' \
  --context 5
```

---

## 🔄 Workflows avancés

### Pipeline de traitement

```bash
# 1. Extraire les montants
astrapdf extract facture.pdf \
  --regex '\d+[,.]?\d*\s?€' \
  --format json \
  --output montants.json

# 2. Traiter avec jq ou Python
cat montants.json | jq '.results[].matches[].text'

# 3. Analyser les données
python analyze_montants.py montants.json
```

### Combiner plusieurs extractions

```bash
# Script bash pour extraire plusieurs types de données
#!/bin/bash

PDF="document.pdf"

# Emails
astrapdf extract "$PDF" --regex '\b[\w._%+-]+@[\w.-]+\.[A-Z|a-z]{2,}\b' \
  --format csv --output emails.csv

# Téléphones
astrapdf extract "$PDF" --regex '0[1-9](?:[\s.-]?\d{2}){4}' \
  --format csv --output phones.csv

# URLs
astrapdf extract "$PDF" --regex 'https?://[^\s]+' \
  --format csv --output urls.csv

echo "✅ Extraction terminée !"
```

### Recherche multi-termes

```bash
# Chercher plusieurs termes dans un même document
for term in "clause" "article" "annexe"; do
  echo "Recherche: $term"
  astrapdf search contrat.pdf "$term" --context
  echo "---"
done
```

---

## 📊 Analyse de documents volumineux

### Extraire uniquement la table des matières

```bash
# Généralement dans les premières pages
astrapdf extract livre.pdf --pages "1-5" --format txt --output toc.txt
```

### Extraire par chapitres

```bash
# Chapitre 1 (pages 10-25)
astrapdf extract manuel.pdf --pages "10-25" --format txt --output chapitre1.txt

# Chapitre 2 (pages 26-45)
astrapdf extract manuel.pdf --pages "26-45" --format txt --output chapitre2.txt
```

### Chercher dans des sections spécifiques

```bash
# Chercher uniquement dans les annexes (pages 50-100)
astrapdf extract rapport.pdf \
  --pages "50-100" \
  --keyword "référence" \
  --format json \
  --output annexes_references.json
```

---

## 🎓 Tips & Astuces

### Tester une regex avant extraction

```bash
# Rechercher d'abord pour valider le pattern
astrapdf search test.pdf 'votre_pattern_regex'

# Une fois validé, extraire
astrapdf extract test.pdf --regex 'votre_pattern_regex' --format json
```

### Limiter le bruit dans les résultats

```bash
# Utiliser un contexte minimal
astrapdf extract document.pdf --keyword "term" --context 0

# Ou exporter en CSV pour filtrer ensuite
astrapdf extract document.pdf --keyword "term" --format csv | grep -v "bruit"
```

### Combiner avec d'autres outils Unix

```bash
# Compter les occurrences
astrapdf search document.pdf "important" | grep -c "Page"

# Extraire et trier
astrapdf extract facture.pdf --regex '\d+€' --format txt | sort -n

# Pipeline complet
astrapdf extract rapport.pdf --keyword "erreur" --format json \
  | jq '.total_matches' \
  | echo "Nombre d'erreurs trouvées: $(cat)"
```

---

## 🚫 Ce qu'AstraPDF ne peut pas (encore) faire

- ❌ OCR de PDFs scannés (prévu v0.3)
- ❌ Extraction de tableaux structurés (prévu v0.2)
- ❌ Édition de PDFs
- ❌ Fusion/séparation de PDFs
- ❌ Conversion depuis d'autres formats vers PDF

---

## 🆘 Aide et support

Si vous rencontrez des problèmes :

```bash
# Aide générale
astrapdf --help

# Aide sur une commande spécifique
astrapdf extract --help
astrapdf search --help
```

Pour des bugs ou suggestions : [GitHub Issues](https://github.com/votre-compte/astrapdf/issues)

---

**Plus d'exemples ?** Consultez la [documentation complète](README.md) ou contribuez avec vos propres cas d'usage !
