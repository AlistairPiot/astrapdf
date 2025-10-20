use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;
use colored::Colorize;

use crate::pdf::PdfAnalyzer;
use crate::export::{ExportFormat, Exporter};

#[derive(Parser)]
#[command(name = "astrapdf")]
#[command(author = "AstraPDF Team")]
#[command(version = "0.1.0")]
#[command(about = "⇒ Transformez vos PDF en informations exploitables", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Affiche les informations de base d'un PDF (nombre de pages, métadonnées)
    Info {
        /// Chemin vers le fichier PDF
        #[arg(value_name = "FILE")]
        pdf_path: PathBuf,
    },

    /// Liste toutes les pages du PDF avec leur contenu
    Pages {
        /// Chemin vers le fichier PDF
        #[arg(value_name = "FILE")]
        pdf_path: PathBuf,

        /// Afficher le contenu textuel de chaque page
        #[arg(short, long)]
        content: bool,
    },

    /// Extrait du texte selon des critères (mot-clé, regex, section)
    Extract {
        /// Chemin vers le fichier PDF
        #[arg(value_name = "FILE")]
        pdf_path: PathBuf,

        /// Mot-clé à rechercher
        #[arg(short, long)]
        keyword: Option<String>,

        /// Pattern regex à rechercher
        #[arg(short, long)]
        regex: Option<String>,

        /// Numéros de pages spécifiques (ex: 1,3,5-10)
        #[arg(short, long)]
        pages: Option<String>,

        /// Nombre de lignes de contexte autour du match
        #[arg(short = 'C', long, default_value = "2")]
        context: usize,

        /// Format d'export (txt, json, csv)
        #[arg(short = 'f', long, value_enum, default_value = "txt")]
        format: ExportFormat,

        /// Fichier de sortie (par défaut: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Traite plusieurs PDFs en batch
    Batch {
        /// Répertoire contenant les PDFs ou liste de fichiers
        #[arg(value_name = "PATHS")]
        paths: Vec<PathBuf>,

        /// Mot-clé à rechercher
        #[arg(short, long)]
        keyword: Option<String>,

        /// Pattern regex à rechercher
        #[arg(short, long)]
        regex: Option<String>,

        /// Format d'export
        #[arg(short = 'f', long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Répertoire de sortie
        #[arg(short, long)]
        output_dir: PathBuf,
    },

    /// Recherche avancée dans un PDF
    Search {
        /// Chemin vers le fichier PDF
        #[arg(value_name = "FILE")]
        pdf_path: PathBuf,

        /// Terme à rechercher
        #[arg(value_name = "QUERY")]
        query: String,

        /// Recherche insensible à la casse
        #[arg(short, long)]
        ignore_case: bool,

        /// Afficher le contexte autour des résultats
        #[arg(short, long)]
        context: bool,
    },
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Info { pdf_path } => {
                println!("{}", "📄 Analyse du PDF...".cyan().bold());
                let analyzer = PdfAnalyzer::new(pdf_path)?;
                analyzer.display_info()?;
            }

            Commands::Pages { pdf_path, content } => {
                println!("{}", "📑 Listing des pages...".cyan().bold());
                let analyzer = PdfAnalyzer::new(pdf_path)?;
                analyzer.list_pages(*content)?;
            }

            Commands::Extract {
                pdf_path,
                keyword,
                regex,
                pages,
                context,
                format,
                output,
            } => {
                println!("{}", "🔍 Extraction en cours...".cyan().bold());
                let analyzer = PdfAnalyzer::new(pdf_path)?;
                let results = analyzer.extract(keyword, regex, pages, *context)?;

                let exporter = Exporter::new(*format);
                exporter.export(&results, output.as_deref())?;
            }

            Commands::Batch {
                paths: _,
                keyword: _,
                regex: _,
                format: _,
                output_dir: _,
            } => {
                println!("{}", "🚀 Traitement batch en cours...".cyan().bold());
                // TODO: Implémenter le traitement batch
                println!("⚠️  Fonctionnalité en cours de développement");
            }

            Commands::Search {
                pdf_path,
                query,
                ignore_case,
                context,
            } => {
                println!("{}", "🔎 Recherche en cours...".cyan().bold());
                let analyzer = PdfAnalyzer::new(pdf_path)?;
                analyzer.search(query, *ignore_case, *context)?;
            }
        }

        Ok(())
    }
}
