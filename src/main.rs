mod text_analyzer;

use crate::text_analyzer::text_analyzer_manager::TextAnalyzerManager;

fn main() {
    let tam = TextAnalyzerManager::new();
    tam.run();
}
