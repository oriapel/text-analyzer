mod text_analyzer;

use crate::text_analyzer::text_analyzer_manager::TextAnalyzerManager;

fn main() {
    let mut tam = TextAnalyzerManager::new();
    tam.run();
}
