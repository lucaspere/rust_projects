# Day 6 Extended: Hash Tables - 25-02-2024

### Exercise 1 - Explain Hash Tables

#### Hash Table
Data Structure that mapped a value to a specific key. Because your great scalability O(1) to read, insert and delete, it is used in caches systems and applications that requires uniquely data count, for example a voting system.
##### Hash Function
A Hash Function is responsible to convert a given numeric or alphanumeric key input and retrieves a corresponded value for it. Normally, the value returned is used as index to locate the **slot** or **bucket**. There are many hash functions algorithms with different types [list of hash functions](https://en.wikipedia.org/wiki/List_of_hash_functions).

##### Slot or Bucket
The slot is a space in the memory used to save and retrieve value. Bucket is a data structure like array or list that is used to save values with the same mapped key.

##### Collision Resolution
When a hash function return a value for a key for a slot that has already mapped, occurs a hash collision and the hash function must resolve it. There are two types of Collision Resolution: Linear Probing and Open Addressing

###### Separate Chaining
The idea is to save the collided keys to a list where we push the new entry to that list. For example, we could create a list where the data node is a tuple, like ("apple", 0.55), and points to another node data like ("avocado", 1.55).

### Hash Table vs Array vs List
Hash table is excellent for guarantee uniquely values for a specific key and has constant time scalability O(1) to insert, read and delete in average case, in the worst case is O(n) where the bucket is a list or array, and you have to flipping in list to find the value. Arrays also has the O(1) to read and delete, but not to insert, and you have to know the exact position in the array you value is. Trees is cool for searching, but it has O(log n) scalability, so the hash table is faster.

###### Open Addressing
handling collisions by saving all elements in the table itself. So the size of the table must be greater than or equal to the total number of the keys. This entire procedure is based upon probing. The linear probing the hash table is searched sequentially that starts from the original location. If the location is occupied, then it checks for the upfront location: *rehash(key) = (n+1)%table-size*. The Quadratic Probing look for the i<sup>2</sup>th slot in the i<sup>th</sup> iteration. We always start from the original hash location. If only the location is occupied then we check the other slots. The Double Hashing increments for the probing sequence are computed by using another hash function. We use another hash function hash2(x) and look for the i*hash2(x) slot in the i<sup>th</sup> rotation. 

### Exercise 2 - Collision Resolution
- Explain the concept
  - Collision Resolution is the technique useful to determine what to do when a hash function returns a collided key, as our example: what to do when we want to save the price of "apple" and "avocado" to our alphabetic hash table? There are different strategies for it:
- Discuss the trade-offs between these techniques.
  - The Separate Chaining is simple and efficient when we not the exact size of the hash table. So we can add more elements without worrying about frequently of keys inserted or deleted. However, as long the list grows the performance of hash table become O(n) for operation. The Open Addressing requires more computation resources because it must calculate the hashing, and it can overhead the memory reallocate when the size of table is full and must open new spaces to fill the new keys. However, the Open Addressing technique is great for cache performance as every is stored in the same table, so it's perfect to frequencies and computation use cases.

### Exercise 3: Hash Functions
- What makes a good hash function? What qualities should it have?
  - A good hash function is when it minimizes the collision of keys and can allocate the new key fast without very heavily computation.
- 3
  - The rust HashMap use the SipHash algorithm


## Rust Implementation
### Exercise 4, 5 and 6:
```rs
use std::{array, collections::LinkedList};

pub struct AHash {
    table: [(u32, LinkedList<(String, i32)>); 27],
}

impl AHash {
    fn new() -> Self {
        Self {
            table: array::from_fn::<(u32, LinkedList<(String, i32)>), 27, _>(|i| {
                (i as u32, LinkedList::new())
            }),
        }
    }
    fn hash_function(&self, key: &str) -> Option<u32> {
        let first_char = key.chars().next().and_then(|r| Some((r as u32) - 97));

        first_char
    }

    pub fn insert(&mut self, key: String, value: i32) -> Option<i32> {
        self.hash_function(key.as_str()).map(|pos: u32| {
            let (_, list) = &mut self.table[pos as usize];
            list.push_back((key, value));
            value
        })
    }

    pub fn search(&self, key: String) -> Option<i32> {
        self.hash_function(key.as_str()).and_then(|pos| {
            let (_, list) = &self.table[pos as usize];
            let result = list
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(name, value)| *value);

            result
        })
    }
}

```

### Exercise 7: Count Word Frequencies

```rs
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
```

### Exercise 8: Find Duplicates

```rs
fn determine_array_has_duplicate<T: PartialEq + Eq + Hash>(vec: Vec<T>) -> bool {
    let mut set = HashSet::with_capacity(vec.len());
    vec.iter().any(|v| !set.insert(v))
}
```

### Exercise 9: Implement a Cache
I know that I can use the Linear Probing to solve this problem, but I do not know why you suggested using the doubly linked list.