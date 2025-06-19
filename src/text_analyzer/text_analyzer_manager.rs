use crate::text_analyzer::text_analyzer::TextAnalyzer;
use std::{convert::TryFrom, io::stdin};

/// Enum representing the options available in the [TextAnalyzer]
pub enum TextAnalyzerMenu {
    AnalyzeNewText,
    GetWordCount,
    Exit,
    InvalidChoice,
}

impl TryFrom<u32> for TextAnalyzerMenu {
    type Error = ();

    /// Converts a u32 number to a [TextAnalyzerMenu] enum variant
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TextAnalyzerMenu::AnalyzeNewText),
            2 => Ok(TextAnalyzerMenu::GetWordCount),
            3 => Ok(TextAnalyzerMenu::Exit),
            _ => Err(()),
        }
    }
}

/// Reads a u32 number from the user input
fn get_u32_from_user() -> Result<u32, Box<dyn std::error::Error>> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let number = input.trim().parse::<u32>()?;

    Ok(number)
}

/// Reads a string from the user input
fn get_text_from_user() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// [TextAnalyzerManager] struct that manages the [TextAnalyzer] operations
/// It contains a [TextAnalyzer] instance and methods to interact with it
pub struct TextAnalyzerManager {
    pub text_analyzer: TextAnalyzer,
}

impl TextAnalyzerManager {
    /// Constructor for the [TextAnalyzerManager] struct
    /// Initializes a new [TextAnalyzerManager] with an empty [TextAnalyzer]
    pub fn new() -> Self {
        Self {
            text_analyzer: TextAnalyzer::new(),
        }
    }

    /// This function runs the [TextAnalyzer] menu, allowing the user to analyze text and get word counts
    pub fn run(mut self) {
        loop {
            println!("Text Analyzer Menu:");
            println!("1. Add new text to analyze");
            println!("2. Get Word Count");
            println!("3. Exit");

            let choice = match get_u32_from_user() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error: {e}");
                    continue;
                }
            };
            let choice = match TextAnalyzerMenu::try_from(choice) {
                Ok(menu) => menu,
                Err(_) => TextAnalyzerMenu::InvalidChoice,
            };
            match choice {
                TextAnalyzerMenu::AnalyzeNewText => {
                    println!("Enter text to analyze:");
                    let text = match get_text_from_user() {
                        Ok(text) => text,
                        Err(e) => {
                            println!("Error reading text: {e}");
                            continue;
                        }
                    };
                    self.text_analyzer.analyze_new_text(text);
                }
                TextAnalyzerMenu::GetWordCount => {
                    println!("Enter word to count:");
                    let word = match get_text_from_user() {
                        Ok(w) => w,
                        Err(e) => {
                            println!("Error reading word: {e}");
                            continue;
                        }
                    };
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
