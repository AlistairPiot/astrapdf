use eframe::egui;
use std::path::PathBuf;
use crate::pdf::PdfAnalyzer;
use crate::export::{ExportFormat, Exporter};

/// État de l'application GUI
pub struct AstraPdfApp {
    // Fichier PDF sélectionné
    pdf_path: Option<PathBuf>,
    pdf_analyzer: Option<PdfAnalyzer>,

    // Informations PDF
    pdf_info: Option<PdfInfo>,

    // Paramètres d'extraction
    keyword: String,
    regex_pattern: String,
    pages_range: String,
    context_lines: usize,

    // Mode d'extraction
    extraction_mode: ExtractionMode,

    // Résultats
    results: Vec<crate::pdf::ExtractionResult>,
    status_message: String,

    // Export
    export_format: ExportFormat,
    export_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct PdfInfo {
    filename: String,
    num_pages: u32,
    file_size: String,
    metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ExtractionMode {
    Keyword,
    Regex,
    Pages,
    All,
}

impl Default for AstraPdfApp {
    fn default() -> Self {
        Self {
            pdf_path: None,
            pdf_analyzer: None,
            pdf_info: None,
            keyword: String::new(),
            regex_pattern: String::new(),
            pages_range: String::new(),
            context_lines: 2,
            extraction_mode: ExtractionMode::Keyword,
            results: Vec::new(),
            status_message: "Bienvenue sur AstraPDF GUI".to_string(),
            export_format: ExportFormat::Json,
            export_path: None,
        }
    }
}

impl AstraPdfApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Charge un fichier PDF
    fn load_pdf(&mut self, path: PathBuf) {
        self.status_message = format!("Chargement de {}...", path.display());

        match PdfAnalyzer::new(&path) {
            Ok(analyzer) => {
                // Récupérer les infos
                let num_pages = analyzer.get_page_count();
                let metadata_vec = analyzer.get_metadata();

                let file_size = match std::fs::metadata(&path) {
                    Ok(meta) => {
                        let size_kb = meta.len() / 1024;
                        if size_kb > 1024 {
                            format!("{:.1} MB", size_kb as f64 / 1024.0)
                        } else {
                            format!("{} KB", size_kb)
                        }
                    }
                    Err(_) => "Unknown".to_string(),
                };

                self.pdf_info = Some(PdfInfo {
                    filename: path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    num_pages,
                    file_size,
                    metadata: metadata_vec,
                });

                self.pdf_analyzer = Some(analyzer);
                self.pdf_path = Some(path.clone());
                self.status_message = format!("✅ PDF chargé: {} ({} pages)",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    num_pages);
            }
            Err(e) => {
                self.status_message = format!("❌ Erreur: {}", e);
                self.pdf_analyzer = None;
                self.pdf_info = None;
            }
        }
    }

    /// Effectue l'extraction selon le mode sélectionné
    fn perform_extraction(&mut self) {
        if let Some(analyzer) = &self.pdf_analyzer {
            self.status_message = "🔍 Extraction en cours...".to_string();

            let result = match self.extraction_mode {
                ExtractionMode::Keyword if !self.keyword.is_empty() => {
                    analyzer.extract(
                        &Some(self.keyword.clone()),
                        &None,
                        &None,
                        self.context_lines
                    )
                }
                ExtractionMode::Regex if !self.regex_pattern.is_empty() => {
                    analyzer.extract(
                        &None,
                        &Some(self.regex_pattern.clone()),
                        &None,
                        self.context_lines
                    )
                }
                ExtractionMode::Pages if !self.pages_range.is_empty() => {
                    analyzer.extract(
                        &None,
                        &None,
                        &Some(self.pages_range.clone()),
                        self.context_lines
                    )
                }
                ExtractionMode::All => {
                    analyzer.extract(
                        &None,
                        &None,
                        &None,
                        self.context_lines
                    )
                }
                _ => {
                    self.status_message = "⚠️ Veuillez remplir les paramètres d'extraction".to_string();
                    return;
                }
            };

            match result {
                Ok(results) => {
                    let total_matches: usize = results.iter()
                        .map(|r| r.matches.len())
                        .sum();
                    self.results = results;
                    self.status_message = format!("✅ Extraction terminée: {} résultats trouvés", total_matches);
                }
                Err(e) => {
                    self.status_message = format!("❌ Erreur d'extraction: {}", e);
                    self.results.clear();
                }
            }
        } else {
            self.status_message = "⚠️ Veuillez d'abord charger un PDF".to_string();
        }
    }

    /// Parse une chaîne de plages de pages (ex: "1-5,10,15-20")
    fn parse_pages_range(&self) -> Option<Vec<u32>> {
        if self.pages_range.is_empty() {
            return None;
        }

        let mut pages = Vec::new();
        for part in self.pages_range.split(',') {
            let part = part.trim();
            if part.contains('-') {
                // Plage de pages
                let bounds: Vec<&str> = part.split('-').collect();
                if bounds.len() == 2 {
                    if let (Ok(start), Ok(end)) = (bounds[0].parse::<u32>(), bounds[1].parse::<u32>()) {
                        for p in start..=end {
                            pages.push(p);
                        }
                    }
                }
            } else {
                // Page unique
                if let Ok(page) = part.parse::<u32>() {
                    pages.push(page);
                }
            }
        }

        if pages.is_empty() {
            None
        } else {
            Some(pages)
        }
    }

    /// Exporte les résultats
    fn export_results(&mut self) {
        if self.results.is_empty() {
            self.status_message = "⚠️ Aucun résultat à exporter".to_string();
            return;
        }

        // Ouvrir un dialogue de sauvegarde
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name(match self.export_format {
                ExportFormat::Json => "results.json",
                ExportFormat::Csv => "results.csv",
                ExportFormat::Txt => "results.txt",
            })
            .save_file()
        {
            let exporter = Exporter::new(self.export_format);
            match exporter.export(&self.results, Some(&path)) {
                Ok(_) => {
                    self.status_message = format!("✅ Résultats exportés vers: {}", path.display());
                    self.export_path = Some(path);
                }
                Err(e) => {
                    self.status_message = format!("❌ Erreur d'export: {}", e);
                }
            }
        }
    }
}

impl eframe::App for AstraPdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Panel supérieur avec titre et barre de menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📄 AstraPDF");
                ui.separator();
                ui.label("Transformez vos PDF en informations exploitables");
            });
        });

        // Panel inférieur avec status
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
            });
        });

        // Panel gauche - Contrôles
        egui::SidePanel::left("control_panel")
            .default_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Contrôles");
                ui.separator();

                // Section 1: Sélection du fichier
                ui.group(|ui| {
                    ui.label("📂 Fichier PDF");
                    ui.horizontal(|ui| {
                        if ui.button("📁 Ouvrir PDF").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("PDF Files", &["pdf"])
                                .pick_file()
                            {
                                self.load_pdf(path);
                            }
                        }

                        if let Some(path) = &self.pdf_path {
                            ui.label(path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string());
                        }
                    });
                });

                ui.add_space(10.0);

                // Section 2: Paramètres d'extraction
                ui.group(|ui| {
                    ui.label("🔍 Extraction");

                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.extraction_mode, ExtractionMode::Keyword, "Mot-clé");
                        ui.selectable_value(&mut self.extraction_mode, ExtractionMode::Regex, "Regex");
                        ui.selectable_value(&mut self.extraction_mode, ExtractionMode::Pages, "Pages");
                        ui.selectable_value(&mut self.extraction_mode, ExtractionMode::All, "Tout");
                    });

                    ui.separator();

                    match self.extraction_mode {
                        ExtractionMode::Keyword => {
                            ui.label("Mot-clé:");
                            ui.text_edit_singleline(&mut self.keyword);
                        }
                        ExtractionMode::Regex => {
                            ui.label("Pattern regex:");
                            ui.text_edit_singleline(&mut self.regex_pattern);

                            if ui.button("📋 Templates").clicked() {
                                // TODO: Menu avec templates regex prédéfinis
                            }
                        }
                        ExtractionMode::Pages => {
                            ui.label("Pages (ex: 1-5,10,15-20):");
                            ui.text_edit_singleline(&mut self.pages_range);
                        }
                        ExtractionMode::All => {
                            ui.label("Extraction complète du document");
                        }
                    }

                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Lignes de contexte:");
                        ui.add(egui::Slider::new(&mut self.context_lines, 0..=10));
                    });

                    ui.add_space(10.0);

                    if ui.button("▶️ Extraire").clicked() {
                        self.perform_extraction();
                    }
                });

                ui.add_space(10.0);

                // Section 3: Export
                ui.group(|ui| {
                    ui.label("📤 Export");

                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.export_format, ExportFormat::Json, "JSON");
                        ui.selectable_value(&mut self.export_format, ExportFormat::Csv, "CSV");
                        ui.selectable_value(&mut self.export_format, ExportFormat::Txt, "TXT");
                    });

                    ui.add_space(5.0);

                    if ui.button("💾 Exporter les résultats").clicked() {
                        self.export_results();
                    }
                });
            });

        // Panel central - Résultats
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Résultats");
            ui.separator();

            // Afficher les infos du PDF si chargé
            if let Some(info) = &self.pdf_info {
                ui.group(|ui| {
                    ui.heading(&info.filename);
                    ui.horizontal(|ui| {
                        ui.label(format!("📄 Pages: {}", info.num_pages));
                        ui.separator();
                        ui.label(format!("💾 Taille: {}", info.file_size));
                    });

                    if !info.metadata.is_empty() {
                        ui.collapsing("ℹ️ Métadonnées", |ui| {
                            for (key, value) in &info.metadata {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", key));
                                    ui.label(value);
                                });
                            }
                        });
                    }
                });

                ui.separator();
            }

            // Afficher les résultats d'extraction
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.results.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("Aucun résultat. Sélectionnez un PDF et lancez une extraction.");
                    });
                } else {
                    for result in &self.results {
                        ui.group(|ui| {
                            ui.heading(format!("Page {}", result.page_number));

                            if !result.matches.is_empty() {
                                ui.label(format!("🔍 {} correspondance(s) trouvée(s)", result.matches.len()));

                                for (i, m) in result.matches.iter().enumerate() {
                                    ui.collapsing(format!("Match #{}", i + 1), |ui| {
                                        if !m.context_before.is_empty() {
                                            ui.label(egui::RichText::new(&m.context_before).weak());
                                        }

                                        ui.label(egui::RichText::new(&m.text)
                                            .background_color(egui::Color32::from_rgb(255, 255, 150))
                                            .color(egui::Color32::from_rgb(0, 0, 0))
                                            .strong());

                                        if !m.context_after.is_empty() {
                                            ui.label(egui::RichText::new(&m.context_after).weak());
                                        }
                                    });
                                }
                            }
                        });

                        ui.add_space(5.0);
                    }
                }
            });
        });
    }
}

/// Lance l'application GUI
pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AstraPDF",
        options,
        Box::new(|cc| Ok(Box::new(AstraPdfApp::new(cc)))),
    )
}
