use astrapdf::pdf::{ExtractionResult, MatchResult};
use astrapdf::export::{Exporter, ExportFormat};
use std::path::Path;
use std::fs;

#[cfg(test)]
mod export_tests {
    use super::*;

    // Helper: créer des résultats de test
    fn create_test_results() -> Vec<ExtractionResult> {
        vec![
            ExtractionResult {
                page_number: 1,
                content: "This is test content with email test@example.com".to_string(),
                matches: vec![
                    MatchResult {
                        text: "test content".to_string(),
                        context_before: "This is".to_string(),
                        context_after: "with email".to_string(),
                        page: 1,
                        line_number: 1,
                    }
                ],
            },
            ExtractionResult {
                page_number: 2,
                content: "Second page content".to_string(),
                matches: vec![],
            },
        ]
    }

    #[test]
    fn test_export_to_json() {
        let results = create_test_results();
        let exporter = Exporter::new(ExportFormat::Json);

        let output_path = Path::new("test_pdfs/test_export.json");

        // Export
        let export_result = exporter.export(&results, Some(output_path));
        assert!(export_result.is_ok(), "JSON export should succeed");

        // Vérifier que le fichier existe
        assert!(output_path.exists(), "JSON file should be created");

        // Lire et parser le JSON
        let content = fs::read_to_string(output_path).expect("Should read JSON file");
        let parsed: serde_json::Value = serde_json::from_str(&content)
            .expect("Should parse JSON");

        // Vérifier la structure
        assert!(parsed["total_pages"].is_number());
        assert!(parsed["total_matches"].is_number());
        assert!(parsed["results"].is_array());

        assert_eq!(parsed["total_pages"].as_u64().unwrap(), 2);
        assert_eq!(parsed["total_matches"].as_u64().unwrap(), 1);

        // Nettoyer
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_export_to_csv() {
        let results = create_test_results();
        let exporter = Exporter::new(ExportFormat::Csv);

        let output_path = Path::new("test_pdfs/test_export.csv");

        // Export
        let export_result = exporter.export(&results, Some(output_path));
        assert!(export_result.is_ok(), "CSV export should succeed");

        // Vérifier que le fichier existe
        assert!(output_path.exists(), "CSV file should be created");

        // Lire le CSV
        let content = fs::read_to_string(output_path).expect("Should read CSV file");
        let lines: Vec<&str> = content.lines().collect();

        // Vérifier headers
        assert!(lines[0].contains("page_number"), "Should have page_number header");
        assert!(lines[0].contains("matched_text"), "Should have matched_text header");

        // Vérifier qu'on a au moins 2 lignes (header + data)
        assert!(lines.len() >= 2, "Should have header and at least one data row");

        // Nettoyer
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_export_to_txt() {
        let results = create_test_results();
        let exporter = Exporter::new(ExportFormat::Txt);

        let output_path = Path::new("test_pdfs/test_export.txt");

        // Export
        let export_result = exporter.export(&results, Some(output_path));
        assert!(export_result.is_ok(), "TXT export should succeed");

        // Vérifier que le fichier existe
        assert!(output_path.exists(), "TXT file should be created");

        // Lire le contenu
        let content = fs::read_to_string(output_path).expect("Should read TXT file");

        // Vérifier qu'on a du contenu formaté
        assert!(content.contains("ASTRAPDF"), "Should have header");
        assert!(content.contains("PAGE 1"), "Should have page 1");
        assert!(content.contains("Statistiques"), "Should have statistics");

        // Nettoyer
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_export_to_stdout() {
        let results = create_test_results();
        let exporter = Exporter::new(ExportFormat::Json);

        // Export vers stdout (None = stdout)
        let export_result = exporter.export(&results, None);

        // Devrait réussir (même si on ne peut pas capturer stdout facilement dans les tests)
        assert!(export_result.is_ok(), "Export to stdout should succeed");
    }

    #[test]
    fn test_export_empty_results() {
        let results: Vec<ExtractionResult> = vec![];
        let exporter = Exporter::new(ExportFormat::Json);

        let output_path = Path::new("test_pdfs/test_empty.json");

        // Export
        let export_result = exporter.export(&results, Some(output_path));
        assert!(export_result.is_ok(), "Should export empty results");

        // Vérifier le contenu
        let content = fs::read_to_string(output_path).expect("Should read file");
        let parsed: serde_json::Value = serde_json::from_str(&content)
            .expect("Should parse JSON");

        assert_eq!(parsed["total_pages"].as_u64().unwrap(), 0);
        assert_eq!(parsed["total_matches"].as_u64().unwrap(), 0);

        // Nettoyer
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_export_format_variants() {
        // Tester que tous les variants existent
        let _txt = ExportFormat::Txt;
        let _json = ExportFormat::Json;
        let _csv = ExportFormat::Csv;

        // Test de création d'exporters
        let _exporter1 = Exporter::new(ExportFormat::Txt);
        let _exporter2 = Exporter::new(ExportFormat::Json);
        let _exporter3 = Exporter::new(ExportFormat::Csv);
    }

    #[test]
    fn test_export_with_matches() {
        let results = vec![
            ExtractionResult {
                page_number: 1,
                content: "Content with multiple matches".to_string(),
                matches: vec![
                    MatchResult {
                        text: "match1".to_string(),
                        context_before: "before1".to_string(),
                        context_after: "after1".to_string(),
                        page: 1,
                        line_number: 1,
                    },
                    MatchResult {
                        text: "match2".to_string(),
                        context_before: "before2".to_string(),
                        context_after: "after2".to_string(),
                        page: 1,
                        line_number: 2,
                    },
                ],
            },
        ];

        let exporter = Exporter::new(ExportFormat::Json);
        let output_path = Path::new("test_pdfs/test_matches.json");

        let export_result = exporter.export(&results, Some(output_path));
        assert!(export_result.is_ok(), "Export with multiple matches should succeed");

        // Vérifier qu'on a bien 2 matches
        let content = fs::read_to_string(output_path).expect("Should read file");
        let parsed: serde_json::Value = serde_json::from_str(&content).expect("Should parse JSON");

        assert_eq!(parsed["total_matches"].as_u64().unwrap(), 2);

        // Nettoyer
        let _ = fs::remove_file(output_path);
    }
}
