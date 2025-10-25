use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use lopdf::Document;
use colored::Colorize;
use regex::Regex;
use serde::Serialize;

use crate::error::AstraPdfError;

pub struct PdfAnalyzer {
    path: PathBuf,
    document: Document,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtractionResult {
    pub page_number: u32,
    pub content: String,
    pub matches: Vec<MatchResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchResult {
    pub text: String,
    pub context_before: String,
    pub context_after: String,
    #[allow(dead_code)]
    pub page: u32,
    pub line_number: usize,
}

impl PdfAnalyzer {
    pub fn new(path: &Path) -> Result<Self> {
        let document = Document::load(path)
            .map_err(|e| AstraPdfError::PdfOpenError(e.to_string()))?;

        Ok(Self {
            path: path.to_path_buf(),
            document,
        })
    }


    /// Retourne le nombre de pages
    pub fn get_page_count(&self) -> u32 {
        self.document.get_pages().len() as u32
    }

    /// Retourne les métadonnées du PDF
    pub fn get_metadata(&self) -> Vec<(String, String)> {
        let mut metadata = Vec::new();
        if let Ok(info) = self.document.trailer.get(b"Info") {
            if let Ok(info_dict) = info.as_dict() {
                for (key, value) in info_dict.iter() {
                    let key_str = String::from_utf8_lossy(key).to_string();
                    let value_str = format!("{:?}", value);
                    metadata.push((key_str, value_str));
                }
            }
        }
        metadata
    }
    pub fn display_info(&self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".bright_blue());
        println!("{} {}", "📂 Fichier:".bold(), self.path.display());
        println!("{}", "═══════════════════════════════════════".bright_blue());

        // Nombre de pages
        let page_count = self.document.get_pages().len();
        println!("{} {}", "📄 Nombre de pages:".bold(), page_count.to_string().green());

        // Métadonnées
        if let Ok(info) = self.document.trailer.get(b"Info") {
            println!("\n{}", "ℹ️  Métadonnées:".bold());
            if let Ok(info_dict) = info.as_dict() {
                for (key, value) in info_dict.iter() {
                    let key_str = String::from_utf8_lossy(key);
                    let value_str = format!("{:?}", value);
                    println!("  {} {}", format!("{}:", key_str).cyan(), value_str);
                }
            }
        }

        // Taille du fichier
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            let size_kb = metadata.len() / 1024;
            println!("{} {} KB", "💾 Taille:".bold(), size_kb.to_string().yellow());
        }

        println!("{}\n", "═══════════════════════════════════════".bright_blue());

        Ok(())
    }

    pub fn list_pages(&self, show_content: bool) -> Result<()> {
        let pages = self.document.get_pages();

        println!("\n{} {}", "Total:".bold(), pages.len().to_string().green());
        println!("{}", "─────────────────────────────────────".bright_black());

        for (page_num, _page_id) in pages.iter() {
            println!("\n{} {}", "📄 Page".cyan().bold(), page_num.to_string().yellow());

            if show_content {
                match self.extract_text_from_page(*page_num) {
                    Ok(text) => {
                        if !text.is_empty() {
                            println!("{}", "Contenu:".bold());
                            // Limiter à 300 caractères pour l'aperçu
                            let preview = if text.len() > 300 {
                                format!("{}...", &text[..300])
                            } else {
                                text.clone()
                            };
                            println!("{}", preview.bright_black());
                        } else {
                            println!("{}", "(page vide ou sans texte extractible)".bright_black().italic());
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "⚠️  Erreur:".red(), e);
                    }
                }
            }

            println!("{}", "─────────────────────────────────────".bright_black());
        }

        Ok(())
    }

    pub fn extract(
        &self,
        keyword: &Option<String>,
        regex_pattern: &Option<String>,
        pages_spec: &Option<String>,
        context_lines: usize,
        ignore_case: bool,
    ) -> Result<Vec<ExtractionResult>> {
        let pages = self.get_pages_to_process(pages_spec)?;
        let mut results = Vec::new();

        for page_num in pages {
            let text = self.extract_text_from_page(page_num)?;
            let mut matches = Vec::new();

            if let Some(kw) = keyword {
                matches.extend(self.find_keyword_matches(&text, kw, page_num, context_lines, ignore_case)?);
            }

            if let Some(pattern) = regex_pattern {
                matches.extend(self.find_regex_matches(&text, pattern, page_num, context_lines, ignore_case)?);
            }

            if !matches.is_empty() || (keyword.is_none() && regex_pattern.is_none()) {
                results.push(ExtractionResult {
                    page_number: page_num,
                    content: text,
                    matches,
                });
            }
        }

        Ok(results)
    }

    pub fn search(&self, query: &str, ignore_case: bool, show_context: bool) -> Result<()> {
        let pages = self.document.get_pages();
        let mut total_matches = 0;

        println!("\n{} \"{}\"", "🔍 Recherche de:".bold(), query.yellow());
        println!("{}", "═══════════════════════════════════════".bright_blue());

        for (page_num, _) in pages.iter() {
            let text = self.extract_text_from_page(*page_num)?;
            let lines: Vec<&str> = text.lines().collect();

            for (line_idx, line) in lines.iter().enumerate() {
                let matches = if ignore_case {
                    line.to_lowercase().contains(&query.to_lowercase())
                } else {
                    line.contains(query)
                };

                if matches {
                    total_matches += 1;
                    println!("\n{} {} {} {}",
                        "📄".cyan(),
                        format!("Page {}", page_num).yellow().bold(),
                        "ligne".bright_black(),
                        (line_idx + 1).to_string().cyan()
                    );

                    if show_context && line_idx > 0 {
                        println!("  {}", lines[line_idx - 1].bright_black());
                    }

                    // Highlight le match
                    let highlighted = if ignore_case {
                        line.to_string()
                    } else {
                        line.replace(query, &format!("{}", query.on_yellow().black()))
                    };
                    println!("  {}", highlighted.white().bold());

                    if show_context && line_idx < lines.len() - 1 {
                        println!("  {}", lines[line_idx + 1].bright_black());
                    }
                }
            }
        }

        println!("\n{}", "═══════════════════════════════════════".bright_blue());
        println!("{} {}", "✨ Total correspondances:".bold(), total_matches.to_string().green().bold());
        println!();

        Ok(())
    }

    fn extract_text_from_page(&self, page_num: u32) -> Result<String> {
        // Essayer d'extraire la page spécifique avec lopdf
        if let Some(&page_id) = self.document.get_pages().get(&page_num) {
            // Extraire le contenu de la page spécifique
            match self.extract_page_content(page_id) {
                Ok(text) if !text.is_empty() => return Ok(text),
                _ => {} // Fallback ci-dessous
            }
        }

        // Fallback: extraire tout le PDF (comme avant)
        // Utilisé si lopdf ne peut pas extraire la page ou si elle est vide
        let text = pdf_extract::extract_text_from_mem(&std::fs::read(&self.path)?)
            .context("Erreur lors de l'extraction du texte")?;

        Ok(text)
    }

    fn extract_page_content(&self, page_id: (u32, u16)) -> Result<String> {
        // Récupérer le contenu de la page
        let content = self.document.get_page_content(page_id)?;

        // Extraire le texte du content stream
        let mut text = String::new();
        let content_str = String::from_utf8_lossy(&content);

        // Parser basique des commandes Tj (show text) dans le content stream
        // Format: (texte) Tj ou [(texte)] TJ
        for line in content_str.lines() {
            if let Some(extracted) = self.extract_text_from_content_line(line) {
                text.push_str(&extracted);
                text.push(' ');
            }
        }

        Ok(text.trim().to_string())
    }

    fn extract_text_from_content_line(&self, line: &str) -> Option<String> {
        // Chercher les patterns (text) Tj
        if line.contains("Tj") || line.contains("TJ") {
            // Extraction simplifiée du texte entre parenthèses
            if let Some(start) = line.find('(') {
                if let Some(end) = line.rfind(')') {
                    if end > start {
                        let text = &line[start + 1..end];
                        // Nettoyer les caractères d'échappement PDF
                        return Some(text.replace("\\(", "(")
                                       .replace("\\)", ")")
                                       .replace("\\\\", "\\"));
                    }
                }
            }
        }
        None
    }

    fn get_pages_to_process(&self, pages_spec: &Option<String>) -> Result<Vec<u32>> {
        if let Some(spec) = pages_spec {
            self.parse_page_spec(spec)
        } else {
            Ok(self.document.get_pages().keys().copied().collect())
        }
    }

    fn parse_page_spec(&self, spec: &str) -> Result<Vec<u32>> {
        let mut pages = Vec::new();

        for part in spec.split(',') {
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() == 2 {
                    let start: u32 = range[0].trim().parse()?;
                    let end: u32 = range[1].trim().parse()?;
                    pages.extend(start..=end);
                }
            } else {
                pages.push(part.trim().parse()?);
            }
        }

        Ok(pages)
    }

    fn find_keyword_matches(
        &self,
        text: &str,
        keyword: &str,
        page_num: u32,
        context_lines: usize,
        ignore_case: bool,
    ) -> Result<Vec<MatchResult>> {
        let lines: Vec<&str> = text.lines().collect();
        let mut matches = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            let matches_keyword = if ignore_case {
                line.to_lowercase().contains(&keyword.to_lowercase())
            } else {
                line.contains(keyword)
            };

            if matches_keyword {
                let context_before = self.get_context(&lines, idx, context_lines, true);
                let context_after = self.get_context(&lines, idx, context_lines, false);

                matches.push(MatchResult {
                    text: line.to_string(),
                    context_before,
                    context_after,
                    page: page_num,
                    line_number: idx + 1,
                });
            }
        }

        Ok(matches)
    }

    fn find_regex_matches(
        &self,
        text: &str,
        pattern: &str,
        page_num: u32,
        context_lines: usize,
        ignore_case: bool,
    ) -> Result<Vec<MatchResult>> {
        let regex = Regex::new(pattern)?;
        let lines: Vec<&str> = text.lines().collect();
        let mut matches = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if regex.is_match(line) {
                let context_before = self.get_context(&lines, idx, context_lines, true);
                let context_after = self.get_context(&lines, idx, context_lines, false);

                matches.push(MatchResult {
                    text: line.to_string(),
                    context_before,
                    context_after,
                    page: page_num,
                    line_number: idx + 1,
                });
            }
        }

        Ok(matches)
    }

    fn get_context(&self, lines: &[&str], idx: usize, context_lines: usize, before: bool) -> String {
        if before {
            let start = idx.saturating_sub(context_lines);
            lines[start..idx].join("\n")
        } else {
            let end = (idx + 1 + context_lines).min(lines.len());
            lines[idx + 1..end].join("\n")
        }
    }
}
