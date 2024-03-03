use std::{array, collections::LinkedList};

pub struct AHash {
    table: [(u32, LinkedList<(String, i32)>); 27],
}

impl AHash {
    fn new() -> Self {
        Self {
            table: array::from_fn::<(u32, LinkedList<(String, i32)>), 27, _>(|i| {
                let tuple = (i as u32, LinkedList::new());

                tuple
            }),
        }
    }
    fn hash_function(&self, key: &str) -> Option<u32> {
        let first_char = key.chars().next().and_then(|r| Some((r as u32) - 97));

        first_char
    }

    pub fn insert(&mut self, key: String, value: i32) -> Option<i32> {
        let idx = self.hash_function(key.as_str()).map(|pos: u32| {
            let (_, list) = &mut self.table[pos as usize];
            list.push_back((key, value));
            value
        });

        idx
    }

    pub fn search(&self, key: String) -> Option<i32> {
        let idx = self.hash_function(key.as_str()).and_then(|pos| {
            let (_, list) = &self.table[pos as usize];
            let result = list
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| *value);

            result
        });

        idx
    }
}

#[cfg(test)]
mod tests {

    use crate::impls::hashes::a_hash::AHash;

    #[test]
    fn should_implemented_a_hash() {
        let mut a_hash = AHash::new();
        a_hash.insert("apple".to_string(), 5);
        a_hash.insert("banana".to_string(), 8);
        a_hash.insert("avocado".to_string(), 7);

        assert_eq!(a_hash.search("banana".to_string()), Some(8));
        assert_eq!(a_hash.search("avocado".to_string()), Some(7));
        assert_eq!(a_hash.search("apple".to_string()), Some(5));
        assert_eq!(a_hash.search("abacaxi".to_string()), None);
    }
}
