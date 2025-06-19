use std::collections::HashMap;

/// [TextAnalyzer] strucdt that can recieve text and analyze it
/// It counts the number of occurrences of each word in the text
pub struct TextAnalyzer {
    analyzed_words: HashMap<String, u32>,
}

impl TextAnalyzer {
    /// Constructor for the [TextAnalyzer] struct
    /// Initializes a new [TextAnalyzer] with an empty [HashMap] for analyzed words
    pub fn new() -> Self {
        Self {
            analyzed_words: HashMap::new(),
        }
    }

    /// This function analyzes new text by counting the occurrences of each word and storing them in the analyzed_words [HashMap]
    pub fn analyze_new_text(&mut self, new_text: String) {
        for word in new_text.split_whitespace() {
            let current_word_count = self.analyzed_words.entry(word.to_string()).or_insert(0);
            *current_word_count += 1;
        }
    }

    /// This function retrieves the count of a specific word from the analyzed_words [HashMap]
    /// If the word is not found, it returns 0
    pub fn get_word_counter(&self, word: &String) -> u32 {
        *self.analyzed_words.get(word).unwrap_or(&0)
    }
}
