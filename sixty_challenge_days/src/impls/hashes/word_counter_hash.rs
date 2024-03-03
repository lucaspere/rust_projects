use std::collections::HashMap;

pub struct WordCountTable {
    table: HashMap<String, i64>,
}

impl WordCountTable {
    pub fn count_word(&mut self, word: &String) {
        self.table
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_default();
    }

    pub fn summarize(&self) {
        for word in &self.table {
            println!("Word: {}, Total: {}", word.0, word.1);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::WordCountTable;

    #[test]
    fn should_count_word() {
        let mut word_counter = WordCountTable {
            table: HashMap::new(),
        };

        word_counter.count_word(&"String".to_string());
        word_counter.count_word(&"String".to_string());
        word_counter.count_word(&"String".to_string());
        word_counter.count_word(&"String".to_string());
        word_counter.count_word(&"String".to_string());
        word_counter.count_word(&"STest".to_string());
        word_counter.count_word(&"STest".to_string());
        word_counter.count_word(&"STest".to_string());

        word_counter.summarize();
    }
}
