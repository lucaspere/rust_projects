# Day 8 - Part 1: Insertion Sort

## Introduction
Insertion Sort is a stable In-place linear algorithm that sorting by putting a value in your correct position for each iteraction. For each value, it analyzes the value with the subsequents values, switting the position with correspondict value. For that reason, the time complexity is O(n^2) on the worst and average cases. On the best case, it has O(n) time Complexity. Since is In-place, the space complexity is constant O(1), because it not uses some extra spacing to sort the data, it only uses a reference of values.


## Complexity Analysis
### Time Complexity
* Best-case: O(n) when the data is almotly sorted;
* Average-case: O(n^2) when the data is almotly sorted;
* Worse-case: O(n^2) when the data is reversed;

### Space Complexity
As it is In-place, so O(n)

## Illustration

## Implementation
### Rust
```rs
pub fn insertion_sort<T: Copy + PartialOrd>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let key = slice[i]
        let mut j = i

        while j > 0 && key < slice[j - 1] {
            slice[i] = slice[j - 1]
            j -= 1;
        }

        slice[j] = key
    }
}
```

### Javascript

```js
function insertion_sort(arr) {
    for(let i = 0; i < arr.length; i++) {

        for(let j = 1; j < arr.length; j++) {
            if (arr[i] > arr[j]) {
                [arr[i], arr[j]] = [arr[j], arr[i]]
                break;
            }
        }
    }
}
```

## References
https://bheisler.github.io/criterion.rs/book/criterion_rs.html
https://engineering.deptagency.com/benchmarking-rust-code-using-criterion-rs
https://www.geeksforgeeks.org/insertion-sort/