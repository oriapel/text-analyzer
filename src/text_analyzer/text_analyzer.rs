use std::collections::HashMap;

pub struct TextAnalyzer {
    analyzed_words : HashMap<String, u32>,
}

impl TextAnalyzer {
    pub fn new() -> Self {
        Self { analyzed_words : HashMap::new() }
    }

    pub fn analyze_new_text(&mut self, new_text : String) {
        for word in new_text.split_whitespace() {
            let current_word_count = self.analyzed_words.entry(word.to_string()).or_insert(0);
            *current_word_count += 1;
        }
    }

    pub fn get_word_counter(&self, word : &String) -> u32 {
        self.analyzed_words.get(word).copied().unwrap_or(0)
    }
}
