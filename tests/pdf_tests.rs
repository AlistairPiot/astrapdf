use astrapdf::pdf::{PdfAnalyzer, ExtractionResult, MatchResult};
use std::path::Path;

#[cfg(test)]
mod pdf_tests {
    use super::*;

    // Test helper: chemin vers le PDF de test
    fn test_pdf_path() -> &'static str {
        "test_pdfs/facture.pdf"
    }

    #[test]
    fn test_pdf_analyzer_new() {
        let path = Path::new(test_pdf_path());

        // Ignorer le test si le PDF n'existe pas
        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found at {}", test_pdf_path());
            return;
        }

        let result = PdfAnalyzer::new(path);
        assert!(result.is_ok(), "PdfAnalyzer should be created successfully");
    }

    #[test]
    fn test_pdf_analyzer_new_invalid_path() {
        let path = Path::new("nonexistent.pdf");
        let result = PdfAnalyzer::new(path);
        assert!(result.is_err(), "PdfAnalyzer should fail on invalid path");
    }

    #[test]
    fn test_extract_with_keyword() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let analyzer = PdfAnalyzer::new(path).expect("Failed to create analyzer");

        let keyword = Some("Payo365".to_string());
        let results = analyzer.extract(&keyword, &None, &None, 2);

        assert!(results.is_ok(), "Extract should succeed");
        let results = results.unwrap();

        // Vérifier qu'on a des résultats
        assert!(!results.is_empty(), "Should have at least one result page");

        // Vérifier qu'on a trouvé le mot-clé
        let total_matches: usize = results.iter()
            .map(|r| r.matches.len())
            .sum();
        assert!(total_matches > 0, "Should find at least one match for 'Payo365'");
    }

    #[test]
    fn test_extract_with_regex() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let analyzer = PdfAnalyzer::new(path).expect("Failed to create analyzer");

        // Regex pour emails
        let regex = Some(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string());
        let results = analyzer.extract(&None, &regex, &None, 2);

        assert!(results.is_ok(), "Extract with regex should succeed");
        let results = results.unwrap();

        // On devrait avoir au moins un email dans la facture
        let total_matches: usize = results.iter()
            .map(|r| r.matches.len())
            .sum();
        assert!(total_matches > 0, "Should find at least one email");
    }

    #[test]
    fn test_extract_no_matches() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let analyzer = PdfAnalyzer::new(path).expect("Failed to create analyzer");

        // Chercher quelque chose qui n'existe pas
        let keyword = Some("INEXISTANT_KEYWORD_12345".to_string());
        let results = analyzer.extract(&keyword, &None, &None, 2);

        assert!(results.is_ok(), "Extract should succeed even with no matches");
        let results = results.unwrap();

        // Vérifier qu'il n'y a aucun match
        let total_matches: usize = results.iter()
            .map(|r| r.matches.len())
            .sum();
        assert_eq!(total_matches, 0, "Should have zero matches");
    }

    #[test]
    fn test_page_range_parsing() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let analyzer = PdfAnalyzer::new(path).expect("Failed to create analyzer");

        // Extraire page 1 uniquement
        let pages = Some("1".to_string());
        let results = analyzer.extract(&None, &None, &pages, 0);

        assert!(results.is_ok(), "Extract with page spec should succeed");
        let results = results.unwrap();

        // Vérifier qu'on a bien 1 page
        assert_eq!(results.len(), 1, "Should extract exactly 1 page");
        assert_eq!(results[0].page_number, 1, "Should be page 1");
    }

    #[test]
    fn test_extraction_result_structure() {
        // Test de la structure ExtractionResult
        let result = ExtractionResult {
            page_number: 1,
            content: "Test content".to_string(),
            matches: vec![
                MatchResult {
                    text: "match text".to_string(),
                    context_before: "before".to_string(),
                    context_after: "after".to_string(),
                    page: 1,
                    line_number: 1,
                }
            ],
        };

        assert_eq!(result.page_number, 1);
        assert_eq!(result.content, "Test content");
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].text, "match text");
    }

    #[test]
    fn test_invalid_regex() {
        let path = Path::new(test_pdf_path());

        if !path.exists() {
            eprintln!("⚠️  Skipping test: test PDF not found");
            return;
        }

        let analyzer = PdfAnalyzer::new(path).expect("Failed to create analyzer");

        // Regex invalide
        let regex = Some("[invalid regex(".to_string());
        let results = analyzer.extract(&None, &regex, &None, 2);

        // Devrait échouer avec une regex invalide
        assert!(results.is_err(), "Should fail with invalid regex");
    }
}
