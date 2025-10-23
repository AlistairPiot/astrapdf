use astrapdf::batch::BatchProcessor;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod batch_tests {
    use super::*;

    fn test_pdf_path() -> &'static str {
        "test_pdfs/facture.pdf"
    }

    #[test]
    fn test_batch_processor_from_single_file() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths);

        assert!(processor.is_ok(), "Should create BatchProcessor from valid PDF");
    }

    #[test]
    fn test_batch_processor_from_invalid_file() {
        let paths = vec![PathBuf::from("nonexistent.pdf")];
        let processor = BatchProcessor::from_paths(paths);

        // Devrait réussir même si le fichier n'existe pas (sera filtré)
        assert!(processor.is_ok(), "Should create processor even with invalid path");
    }

    #[test]
    fn test_batch_processor_from_directory() {
        let test_dir = Path::new("test_pdfs");

        if !test_dir.exists() {
            eprintln!("⚠️  Skipping test: test directory not found");
            return;
        }

        let paths = vec![PathBuf::from("test_pdfs")];
        let processor = BatchProcessor::from_paths(paths);

        assert!(processor.is_ok(), "Should create processor from directory");
    }

    #[test]
    fn test_batch_process_single_file() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        // Traiter avec un keyword
        let summary = processor.process(Some("Payo365"), None, 2);

        assert_eq!(summary.total_files, 1, "Should process 1 file");
        assert_eq!(summary.successful, 1, "Should succeed on valid PDF");
        assert_eq!(summary.failed, 0, "Should have no failures");
    }

    #[test]
    fn test_batch_process_with_regex() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        // Regex pour emails
        let regex = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";
        let summary = processor.process(None, Some(regex), 2);

        assert_eq!(summary.total_files, 1);
        assert_eq!(summary.successful, 1);
    }

    #[test]
    fn test_batch_process_multiple_files() {
        let path1 = Path::new(test_pdf_path());
        let path2 = Path::new("test_pdfs/doc1.pdf");

        if !path1.exists() {
            eprintln!("⚠️  Skipping test: test PDFs not found");
            return;
        }

        let mut paths = vec![PathBuf::from(test_pdf_path())];
        if path2.exists() {
            paths.push(PathBuf::from("test_pdfs/doc1.pdf"));
        }

        let processor = BatchProcessor::from_paths(paths.clone())
            .expect("Failed to create processor");

        let summary = processor.process(Some("de"), None, 2);

        assert_eq!(summary.total_files, paths.len());
        assert!(summary.successful >= 1, "Should have at least one success");
    }

    #[test]
    fn test_batch_summary_structure() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        let summary = processor.process(Some("test"), None, 0);

        // Vérifier la structure du résumé
        assert_eq!(summary.total_files, 1);
        assert_eq!(summary.results.len(), 1);

        let result = &summary.results[0];
        assert!(result.file_path.to_string_lossy().contains("facture.pdf"));
        assert_eq!(result.success, true);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_batch_with_no_matches() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        // Chercher quelque chose qui n'existe pas
        let summary = processor.process(Some("INEXISTANT_12345"), None, 2);

        assert_eq!(summary.total_files, 1);
        assert_eq!(summary.successful, 1); // Devrait réussir même sans matches
        assert_eq!(summary.failed, 0);
    }

    #[test]
    fn test_batch_export_json() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        let summary = processor.process(Some("Payo365"), None, 2);

        let output_dir = Path::new("test_pdfs/batch_test_output");
        std::fs::create_dir_all(output_dir).ok();

        // Test export JSON
        let export_result = BatchProcessor::export_summary(
            &summary,
            astrapdf::export::ExportFormat::Json,
            output_dir
        );

        assert!(export_result.is_ok(), "JSON export should succeed");

        let json_file = output_dir.join("batch_results.json");
        assert!(json_file.exists(), "JSON file should be created");

        // Nettoyer
        let _ = std::fs::remove_dir_all(output_dir);
    }

    #[test]
    fn test_batch_export_csv() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let paths = vec![PathBuf::from(test_pdf_path())];
        let processor = BatchProcessor::from_paths(paths).expect("Failed to create processor");

        let summary = processor.process(Some("test"), None, 2);

        let output_dir = Path::new("test_pdfs/batch_test_output_csv");
        std::fs::create_dir_all(output_dir).ok();

        // Test export CSV
        let export_result = BatchProcessor::export_summary(
            &summary,
            astrapdf::export::ExportFormat::Csv,
            output_dir
        );

        assert!(export_result.is_ok(), "CSV export should succeed");

        let csv_file = output_dir.join("batch_results.csv");
        assert!(csv_file.exists(), "CSV file should be created");

        // Nettoyer
        let _ = std::fs::remove_dir_all(output_dir);
    }
}
