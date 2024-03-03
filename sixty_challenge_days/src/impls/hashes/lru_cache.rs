use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

/// A simple [Least Recently Used Cache](https://www.geeksforgeeks.org/lru-cache-implementation/?ref=header_search)
pub struct LRUCache<K: Eq + PartialEq + Hash + Clone, V> {
    doubly_list: LinkedList<K>,
    map: HashMap<K, V>,
}

impl<K: Eq + PartialEq + Hash + Clone, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            doubly_list: LinkedList::new(),
            map: HashMap::with_capacity(capacity),
        }
    }
    pub fn get(&mut self, key: K) -> Option<&V> {
        self.map.get(&key).and_then(|v| {
            let head = self.doubly_list.front_mut().unwrap();
            *head = key;

            Some(v)
        })
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(v) = self.map.get_mut(&key) {
            *v = value;

            self.doubly_list.front_mut().and_then(|front| {
                *front = key;
                Some(())
            });
        } else {
            if self.map.len() >= self.map.capacity() {
                self.eviction();
            }

            self.map.insert(key.clone(), value);

            self.doubly_list.push_front(key);
        }
    }

    fn eviction(&mut self) {
        if let Some(tail) = self.doubly_list.pop_back() {
            self.map.remove(&tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::hashes::lru_cache::LRUCache;

    #[test]
    fn should_evict_lru() {
        let mut lru = LRUCache::new(0);

        lru.put("Bard".to_string(), 2);
        lru.put("Teste".to_string(), 2);
        lru.put("Gemini".to_string(), 2);

        lru.get("Teste".to_string());
        lru.put("Lucas".to_string(), 5);
        lru.get("Teste".to_string());
        lru.get("Lucas".to_string());

        assert_eq!(lru.doubly_list.back(), Some(&"Teste".to_string()));
    }
}
