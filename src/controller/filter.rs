use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

#[derive(Debug)]
/// # Filter
/// Filter can filter words from a given string using a JSON dictionary
/// It holds a Hashset **insults** which represent words that should be filtered out
///# Example
/// ```
/// let filter = Filter::new("English");
/// let mut is_insult = filter.contains_insult(&"This is Bullshit".to_string());
/// assert_eq!(is_insult, true);
/// is_insult = filter.contains_insult(&"This is nice".to_string(),);
/// assert_eq!(is_insult, false)
/// ```
pub struct Filter {
    pub insults: HashSet<String>
}

impl Filter {
    /// Sets up the Filter with a dictionary.
    /// Currently, there exists a (minimal) englisch dic, and a german dic for insults.
    /// Dictionaries are saved as a JSON in the form
    /// ```
    /// {
    /// "insults" :
    /// [your_insult_1,
    /// [your_insult_2,
    /// ...]
    /// }
    /// ```
    /// # Example
    /// ```
    /// let filter = Filter::new("German");
    /// ```
    pub fn new(language: &str) -> Filter {
        // init filter
        info!("Initiating filter");
        let mut filter = Filter { insults: HashSet::new() };
        // Read JSON to String
        let mut data = String::new();
        // Choose language
        let mut f;
        if language == "English" || language == "Englisch" {
            f = File::open("./src/data/dictionary_en.json").expect("Unable to open file");
        } else if language == "German" || language == "Deutsch" {
            f = File::open("./src/data/dictionary_de.json").expect("Unable to open file");
        } else {
            return filter;
        }
        f.read_to_string(&mut data).expect("Unable to read string");
        //Deserialize JSON and write values into filter
        let serde_val: Value = serde_json::from_str(data.as_str()).unwrap();
        let obj = serde_val.as_object().unwrap();
        for (_key, values) in obj.iter() {
            for value in values.as_array() {
                for val in value {
                    filter.insults.insert(val.as_str().unwrap().to_string().to_uppercase());
                }
            }
        }
        return filter;
    }
    /// Checks passed messages for any insults.
    /// # Example
    /// ```
    /// let mut filter = Filter::new("Deutsch");
    /// let mut is_insult = filter.contains_insult(&"Das ist scheiße".to_string());
    /// assert_eq!(is_insult, true);
    /// ```
    pub fn contains_insult(&self, message: &String) -> bool {
        let mut contains_insult = false;
        // split message into substrings for each word in sentence
        let words = message.split(" ");
        for word in words {
            // look up each part in Hashset if present
            if self.insults.contains(word.to_uppercase().as_str()) {
                // if present, return true
                contains_insult = true
            }
        }
        return contains_insult;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_loading_dic_is_there() {
        let mut filter = Filter::new("English");
        assert_eq!(filter.insults.is_empty(), false);
        filter.insults.clear();
        assert_eq!(filter.insults.is_empty(), true);
        filter = Filter::new("Englisch");
        assert_eq!(filter.insults.is_empty(), false);
        filter.insults.clear();
        assert_eq!(filter.insults.is_empty(), true);
        filter = Filter::new("German");
        assert_eq!(filter.insults.is_empty(), false);
        filter.insults.clear();
        assert_eq!(filter.insults.is_empty(), true);
        filter = Filter::new("Deutsch");
        assert_eq!(filter.insults.is_empty(), false);
    }

    #[test]
    fn test_filter_loading_dic_is_absent() {
        let filter = Filter::new("Aksdjn");
        assert_eq!(filter.insults.is_empty(), true);
    }

    #[test]
    fn test_filter_english() {
        let filter = Filter::new("English");
        let mut is_insult = filter.contains_insult(&"This is Bullshit".to_string());
        assert_eq!(is_insult, true);
        is_insult = filter.contains_insult(&"This is nice".to_string(),);
        assert_eq!(is_insult, false)
    }

    #[test]
    fn test_filter_german() {
        let filter = Filter::new("Deutsch");
        let mut is_insult = filter.contains_insult(&"This is Scheiße".to_string());
        assert_eq!(is_insult, true);
        is_insult = filter.contains_insult(&"This is nice".to_string());
        assert_eq!(is_insult, false)
    }

    #[test]
    fn test_filter_is_case_insensitive() {
        let mut filter = Filter::new("Deutsch");
        let mut is_insult = filter.contains_insult(&"This is scheiße".to_string());
        assert_eq!(is_insult, true);
        filter = Filter::new("English");
        is_insult = filter.contains_insult(&"This is bulLshiT".to_string());
        assert_eq!(is_insult, true);
    }
}

