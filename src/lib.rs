use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use pyo3::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcaxhDictionary {
    prefixes: HashMap<String, String>,
    suffixes: HashMap<String, String>,
    vocabulary: HashMap<String, String>,
}

lazy_static! {
    static ref DICTIONARY: ArcaxhDictionary = {
        let mut prefixes = HashMap::new();
        prefixes.insert("ar".to_string(), "arcaxh/civilization related".to_string());
        prefixes.insert("khe".to_string(), "state of being".to_string());
        prefixes.insert("vel".to_string(), "important person/role".to_string());
        // Add more prefixes...

        let mut suffixes = HashMap::new();
        suffixes.insert("in".to_string(), "person".to_string());
        suffixes.insert("ar".to_string(), "object/concept".to_string());
        suffixes.insert("ith".to_string(), "state/quality".to_string());
        // Add more suffixes...

        let mut vocabulary = HashMap::new();
        vocabulary.insert("arkashir".to_string(), "the arcaxh people".to_string());
        vocabulary.insert("zorakhion".to_string(), "symbiotic microorganism".to_string());
        vocabulary.insert("velkharn".to_string(), "capital city".to_string());
        // Add more vocabulary...

        ArcaxhDictionary {
            prefixes,
            suffixes,
            vocabulary,
        }
    };
}

#[pyclass]
pub struct ArcaxhTranslator;

#[pymethods]
impl ArcaxhTranslator {
    #[new]
    pub fn new() -> Self {
        ArcaxhTranslator
    }

    #[pyo3(name = "translate_to_english")]
    pub fn translate_to_english_py(&self, arcaxh_text: &str) -> PyResult<String> {
        Ok(self.translate_to_english(arcaxh_text))
    }

    #[pyo3(name = "analyze_word")]
    pub fn analyze_word_py(&self, word: &str) -> PyResult<String> {
        Ok(self.analyze_word(word))
    }
}

impl ArcaxhTranslator {
    fn translate_to_english(&self, arcaxh_text: &str) -> String {
        let words: Vec<&str> = arcaxh_text.split_whitespace().collect();
        let mut translation = Vec::new();

        for word in words {
            if let Some(meaning) = DICTIONARY.vocabulary.get(word.to_lowercase().as_str()) {
                translation.push(meaning.clone());
            } else {
                // Attempt to break down compound words
                let analyzed = self.analyze_word(word);
                translation.push(analyzed);
            }
        }

        translation.join(" ")
    }

    fn analyze_word(&self, word: &str) -> String {
        let word_lower = word.to_lowercase();
        let mut components = Vec::new();

        // Check prefixes
        for (prefix, meaning) in &DICTIONARY.prefixes {
            if word_lower.starts_with(prefix) {
                components.push(format!("({} [{}])", prefix, meaning));
                break;
            }
        }

        // Check suffixes
        for (suffix, meaning) in &DICTIONARY.suffixes {
            if word_lower.ends_with(suffix) {
                components.push(format!("({} [{}])", suffix, meaning));
                break;
            }
        }

        if components.is_empty() {
            word.to_string()
        } else {
            format!("{}: {}", word, components.join("-"))
        }
    }
}

#[pymodule]
fn arcaxh_translator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ArcaxhTranslator>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let translator = ArcaxhTranslator::new();
        assert_eq!(
            translator.translate_to_english("arkashir"),
            "the arcaxh people"
        );
    }

    #[test]
    fn test_word_analysis() {
        let translator = ArcaxhTranslator::new();
        let analysis = translator.analyze_word("velkharn");
        assert!(analysis.contains("vel"));
    }
}
