use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::Result;
use colored::Colorize;
use serde::Serialize;

use crate::pdf::ExtractionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ExportFormat {
    Txt,
    Json,
    Csv,
}

pub struct Exporter {
    format: ExportFormat,
}

#[derive(Serialize)]
struct JsonOutput {
    total_pages: usize,
    total_matches: usize,
    results: Vec<JsonResult>,
}

#[derive(Serialize)]
struct JsonResult {
    page_number: u32,
    content: String,
    matches: Vec<JsonMatch>,
}

#[derive(Serialize)]
struct JsonMatch {
    text: String,
    line_number: usize,
    context_before: String,
    context_after: String,
}

impl Exporter {
    pub fn new(format: ExportFormat) -> Self {
        Self { format }
    }

    pub fn export(&self, results: &[ExtractionResult], output_path: Option<&Path>) -> Result<()> {
        let output = match self.format {
            ExportFormat::Txt => self.to_text(results)?,
            ExportFormat::Json => self.to_json(results)?,
            ExportFormat::Csv => self.to_csv(results)?,
        };

        if let Some(path) = output_path {
            let mut file = File::create(path)?;
            file.write_all(output.as_bytes())?;
            println!("{} {}", "✅ Export réussi vers:".green().bold(), path.display());
        } else {
            println!("\n{}", "═══════════════════════════════════════".bright_blue());
            println!("{}", output);
            println!("{}", "═══════════════════════════════════════".bright_blue());
        }

        Ok(())
    }

    fn to_text(&self, results: &[ExtractionResult]) -> Result<String> {
        let mut output = String::new();

        output.push_str("═══════════════════════════════════════\n");
        output.push_str(&format!("  ASTRAPDF - RÉSULTATS D'EXTRACTION\n"));
        output.push_str("═══════════════════════════════════════\n\n");

        let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
        output.push_str(&format!("📊 Statistiques:\n"));
        output.push_str(&format!("   - Pages analysées: {}\n", results.len()));
        output.push_str(&format!("   - Correspondances trouvées: {}\n\n", total_matches));

        for result in results {
            output.push_str(&format!("\n📄 PAGE {}\n", result.page_number));
            output.push_str("─────────────────────────────────────\n");

            if result.matches.is_empty() {
                output.push_str("\nContenu complet:\n");
                output.push_str(&result.content);
                output.push_str("\n");
            } else {
                output.push_str(&format!("\n✨ {} correspondance(s) trouvée(s)\n\n", result.matches.len()));

                for (idx, m) in result.matches.iter().enumerate() {
                    output.push_str(&format!("Match {} (ligne {}):\n", idx + 1, m.line_number));

                    if !m.context_before.is_empty() {
                        output.push_str("  [contexte avant]\n");
                        for line in m.context_before.lines() {
                            output.push_str(&format!("  {}\n", line));
                        }
                    }

                    output.push_str(&format!("  >>> {}\n", m.text));

                    if !m.context_after.is_empty() {
                        output.push_str("  [contexte après]\n");
                        for line in m.context_after.lines() {
                            output.push_str(&format!("  {}\n", line));
                        }
                    }

                    output.push_str("\n");
                }
            }
        }

        Ok(output)
    }

    fn to_json(&self, results: &[ExtractionResult]) -> Result<String> {
        let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();

        let json_results: Vec<JsonResult> = results
            .iter()
            .map(|r| JsonResult {
                page_number: r.page_number,
                content: r.content.clone(),
                matches: r
                    .matches
                    .iter()
                    .map(|m| JsonMatch {
                        text: m.text.clone(),
                        line_number: m.line_number,
                        context_before: m.context_before.clone(),
                        context_after: m.context_after.clone(),
                    })
                    .collect(),
            })
            .collect();

        let output = JsonOutput {
            total_pages: results.len(),
            total_matches,
            results: json_results,
        };

        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn to_csv(&self, results: &[ExtractionResult]) -> Result<String> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Headers
        wtr.write_record(&[
            "page_number",
            "line_number",
            "matched_text",
            "context_before",
            "context_after",
        ])?;

        // Data
        for result in results {
            if result.matches.is_empty() {
                // Si pas de matches, on écrit juste le numéro de page
                wtr.write_record(&[
                    result.page_number.to_string(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ])?;
            } else {
                for m in &result.matches {
                    wtr.write_record(&[
                        result.page_number.to_string(),
                        m.line_number.to_string(),
                        m.text.clone(),
                        m.context_before.clone(),
                        m.context_after.clone(),
                    ])?;
                }
            }
        }

        let data = wtr.into_inner()?;
        Ok(String::from_utf8(data)?)
    }
}
