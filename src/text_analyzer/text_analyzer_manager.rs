use std::io::stdin;
use crate::text_analyzer::text_analyzer::TextAnalyzer;

pub enum TextAnalyzerMenu {
    AnalyzeNewText,
    GetWordCount,
    Exit,
    InvalidChoice,
}

fn convert_u32_to_text_analyzer_option(num: u32) -> TextAnalyzerMenu {
    match num {
        1 => TextAnalyzerMenu::AnalyzeNewText,
        2 => TextAnalyzerMenu::GetWordCount,
        3 => TextAnalyzerMenu::Exit,
        _ => TextAnalyzerMenu::InvalidChoice,
    }
}

fn get_u32_from_user() -> u32 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}


fn get_text_from_user() -> String {
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).expect("Failed to read line");
    input_text.trim().to_string()
}


pub struct TextAnalyzerManager {
    pub text_analyzer: TextAnalyzer,
}

impl TextAnalyzerManager {
    pub fn new() -> Self {
        Self {
            text_analyzer: TextAnalyzer::new(),
        }
    }

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