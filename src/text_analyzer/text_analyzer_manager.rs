use crate::text_analyzer::text_analyzer::TextAnalyzer;
use std::io::stdin;

/// Enum representing the options available in the Text Analyzer menu
pub enum TextAnalyzerMenu {
    AnalyzeNewText,
    GetWordCount,
    Exit,
    InvalidChoice,
}

/// Converts a u32 number to a TextAnalyzerMenu enum variant
fn convert_u32_to_text_analyzer_option(num: u32) -> TextAnalyzerMenu {
    match num {
        1 => TextAnalyzerMenu::AnalyzeNewText,
        2 => TextAnalyzerMenu::GetWordCount,
        3 => TextAnalyzerMenu::Exit,
        _ => TextAnalyzerMenu::InvalidChoice,
    }
}

/// Reads a u32 number from the user input
fn get_u32_from_user() -> u32 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

/// Reads a string from the user input
fn get_text_from_user() -> String {
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    input_text.trim().to_string()
}

/// TextAnalyzerManager struct that manages the TextAnalyzer operations
/// It contains a TextAnalyzer instance and methods to interact with it
pub struct TextAnalyzerManager {
    pub text_analyzer: TextAnalyzer,
}

impl TextAnalyzerManager {
    /// Constructor for the TextAnalyzerManager struct
    /// Initializes a new TextAnalyzerManager with an empty TextAnalyzer
    pub fn new() -> Self {
        Self {
            text_analyzer: TextAnalyzer::new(),
        }
    }

    /// This function runs the Text Analyzer menu, allowing the user to analyze text and get word counts
    pub fn run(&mut self) {
        loop {
            println!("Text Analyzer Menu:");
            println!("1. Add new text to analyze");
            println!("2. Get Word Count");
            println!("3. Exit");

            let choice = get_u32_from_user();
            match convert_u32_to_text_analyzer_option(choice) {
                TextAnalyzerMenu::AnalyzeNewText => {
                    println!("Enter text to analyze:");
                    let text = get_text_from_user();
                    self.text_analyzer.analyze_new_text(text);
                }
                TextAnalyzerMenu::GetWordCount => {
                    println!("Enter word to count:");
                    let word = get_text_from_user();
                    let count = self.text_analyzer.get_word_counter(&word);
                    println!("The word '{}' appears {} times.", word, count);
                }
                TextAnalyzerMenu::Exit => {
                    println!("~Goodbye!~");
                    break;
                }
                TextAnalyzerMenu::InvalidChoice => {
                    println!("Invalid choice, please try again.");
                }
            }
        }
    }
}
