use std::path::{Path, PathBuf};
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Serialize;

use crate::pdf::{PdfAnalyzer, ExtractionResult};
use crate::export::{Exporter, ExportFormat};

#[derive(Debug, Serialize)]
pub struct BatchResult {
    pub file_path: PathBuf,
    pub success: bool,
    pub error: Option<String>,
    pub results: Option<Vec<ExtractionResult>>,
}

#[derive(Debug, Serialize)]
pub struct BatchSummary {
    pub total_files: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<BatchResult>,
}

pub struct BatchProcessor {
    files: Vec<PathBuf>,
}

impl BatchProcessor {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files }
    }

    pub fn from_paths(paths: Vec<PathBuf>) -> Result<Self> {
        let mut files = Vec::new();

        for path in paths {
            if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                    files.push(path);
                }
            } else if path.is_dir() {
                // Lister tous les PDFs dans le répertoire
                Self::collect_pdfs_from_dir(&path, &mut files)?;
            }
        }

        Ok(Self { files })
    }

    fn collect_pdfs_from_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    pub fn process(
        &self,
        keyword: Option<&str>,
        regex: Option<&str>,
        context_lines: usize,
    ) -> BatchSummary {
        println!("\n{}", "🚀 Traitement batch".cyan().bold());
        println!("{} {} fichiers à traiter\n", "📊".cyan(), self.files.len().to_string().yellow());

        // Progress bar
        let pb = ProgressBar::new(self.files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-")
        );

        // Traitement parallèle avec rayon
        let results: Vec<BatchResult> = self.files
            .par_iter()
            .map(|file_path| {
                let result = Self::process_single_file(
                    file_path,
                    keyword,
                    regex,
                    context_lines,
                    false, // ignore_case
                );
                pb.inc(1);
                result
            })
            .collect();

        pb.finish_with_message("✅ Traitement terminé");

        // Calculer les statistiques
        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.len() - successful;

        BatchSummary {
            total_files: results.len(),
            successful,
            failed,
            results,
        }
    }

    fn process_single_file(
        file_path: &Path,
        keyword: Option<&str>,
        regex: Option<&str>,
        context_lines: usize,
        ignore_case: bool,
    ) -> BatchResult {
        match PdfAnalyzer::new(file_path) {
            Ok(analyzer) => {
                match analyzer.extract(
                    &keyword.map(String::from),
                    &regex.map(String::from),
                    &None, // Toutes les pages
                    context_lines,
                    false, // ignore_case par défaut
                ) {
                    Ok(results) => BatchResult {
                        file_path: file_path.to_path_buf(),
                        success: true,
                        error: None,
                        results: Some(results),
                    },
                    Err(e) => BatchResult {
                        file_path: file_path.to_path_buf(),
                        success: false,
                        error: Some(e.to_string()),
                        results: None,
                    },
                }
            }
            Err(e) => BatchResult {
                file_path: file_path.to_path_buf(),
                success: false,
                error: Some(e.to_string()),
                results: None,
            },
        }
    }

    pub fn export_summary(
        summary: &BatchSummary,
        format: ExportFormat,
        output_dir: &Path,
    ) -> Result<()> {
        // Créer le répertoire de sortie si nécessaire
        std::fs::create_dir_all(output_dir)?;

        match format {
            ExportFormat::Json => {
                let output_file = output_dir.join("batch_results.json");
                let json = serde_json::to_string_pretty(summary)?;
                std::fs::write(&output_file, json)?;
                println!("{} {}", "✅ Résultats exportés:".green().bold(), output_file.display());
            }
            ExportFormat::Csv => {
                let output_file = output_dir.join("batch_results.csv");
                let mut wtr = csv::Writer::from_path(&output_file)?;

                // Headers
                wtr.write_record(&["file_path", "success", "error", "total_matches"])?;

                // Data
                for result in &summary.results {
                    let total_matches = result
                        .results
                        .as_ref()
                        .map(|r| r.iter().map(|res| res.matches.len()).sum::<usize>())
                        .unwrap_or(0);

                    wtr.write_record(&[
                        result.file_path.to_string_lossy().to_string(),
                        result.success.to_string(),
                        result.error.as_deref().unwrap_or("").to_string(),
                        total_matches.to_string(),
                    ])?;
                }

                wtr.flush()?;
                println!("{} {}", "✅ Résultats exportés:".green().bold(), output_file.display());
            }
            ExportFormat::Txt => {
                let output_file = output_dir.join("batch_results.txt");
                let mut output = String::new();

                output.push_str("═══════════════════════════════════════\n");
                output.push_str("  ASTRAPDF - RÉSULTATS BATCH\n");
                output.push_str("═══════════════════════════════════════\n\n");

                output.push_str(&format!("📊 Statistiques:\n"));
                output.push_str(&format!("   - Total fichiers: {}\n", summary.total_files));
                output.push_str(&format!("   - Réussis: {}\n", summary.successful));
                output.push_str(&format!("   - Échoués: {}\n\n", summary.failed));

                for result in &summary.results {
                    output.push_str(&format!("\n📄 {}\n", result.file_path.display()));
                    output.push_str("─────────────────────────────────────\n");

                    if result.success {
                        if let Some(results) = &result.results {
                            let total_matches: usize = results.iter()
                                .map(|r| r.matches.len())
                                .sum();
                            output.push_str(&format!("✅ Succès - {} correspondance(s)\n", total_matches));
                        }
                    } else {
                        output.push_str(&format!("❌ Erreur: {}\n", result.error.as_deref().unwrap_or("Unknown")));
                    }
                }

                std::fs::write(&output_file, output)?;
                println!("{} {}", "✅ Résultats exportés:".green().bold(), output_file.display());
            }
        }

        Ok(())
    }

    pub fn display_summary(summary: &BatchSummary) {
        println!("\n{}", "═══════════════════════════════════════".bright_blue());
        println!("{}", "  📊 RÉSULTATS BATCH".bold());
        println!("{}", "═══════════════════════════════════════".bright_blue());

        println!("\n{}", "Statistiques:".bold());
        println!("  {} Total: {}", "📁".cyan(), summary.total_files.to_string().yellow());
        println!("  {} Réussis: {}", "✅".green(), summary.successful.to_string().green());
        println!("  {} Échoués: {}", "❌".red(), summary.failed.to_string().red());

        if summary.failed > 0 {
            println!("\n{}", "Fichiers échoués:".red().bold());
            for result in &summary.results {
                if !result.success {
                    println!("  {} {}", "❌".red(), result.file_path.display());
                    if let Some(error) = &result.error {
                        println!("     Erreur: {}", error.bright_black());
                    }
                }
            }
        }

        println!("\n{}", "═══════════════════════════════════════".bright_blue());
    }
}
