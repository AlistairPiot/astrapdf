use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstraPdfError {
    #[error("Erreur d'ouverture du fichier PDF: {0}")]
    PdfOpenError(String),

    #[allow(dead_code)]
    #[error("Erreur d'extraction du texte: {0}")]
    ExtractionError(String),

    #[allow(dead_code)]
    #[error("Erreur d'export: {0}")]
    ExportError(String),

    #[error("Pattern regex invalide: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Erreur I/O: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Erreur de parsing PDF: {0}")]
    LopdfError(#[from] lopdf::Error),
}
