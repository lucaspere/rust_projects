# Day 10 - Part 1: Bubble Sort and Selection Sort

## Introduction
Bubble Sort and Selection Sort are the simplest sorting algorithms that are more useful for educational purposes because they are easy to understand and implement but inefficient and slow for real-world use cases. The Bubble sort acts by comparing and switching the adjacent values (``j and arr[j - 1]``) if the criterion passes. Selection Sort, on the other hand, selects the smallest value in the unordered list and swaps it with the current interaction. Both algorithms are in-place space complexity O(1), but the selection sort is more memory efficient because uses fewer memory swapps. The Bubble Sort is stable because it preserves the order when the values are equal and the selection doesn't preserve. Since the best case of Bubble Sort is O(n) when the list is already sorted, it is useful to verify it.

## Bubble Sort
### Complexity Analysis
#### Time Complexity
* Best-case: O(n) when the list is already sorted.
* Average-case: Always O(n^2), due to the nested loops, even when dealing with mostly sorted data.
* Worse-case: Same Average-case.

#### Space Complexity
O(1) when not considering the call stack memory. In this case, is O(n)

## Implementations
```rs
pub fn bubble_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        for j in 1..len - i {
            if slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
    bubble_sort(&mut data);
    assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
}
```
```js
function bubbleSort(arr) {
  const len = arr.length;  
  for (let i = 0; i < len; i++) {
    for (let j = 1; j < len - i; j++) {
       if (arr[j - 1] > arr[j]) {
           [arr[j - 1], arr[j]] = [arr[j], arr[j - 1]];
       }
    }
  }
  return arr; 
}
```

## Selection Sort
### Complexity Analysis
#### Time Complexity
* Best-case: O(n^2).
* Average-case: Always O(n^2), due to the nested loops, even when dealing with mostly sorted data.
* Worse-case: Same Average-case.

#### Space Complexity
O(1) when not considering the call stack memory. In this case, is O(n)

```rs
pub fn selection_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if slice[j] < slice[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            slice.swap(i, min_index);
        } 
    }
}

#[test]
fn test_selection_sort() {
    let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
    selection_sort(&mut data);
    assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
}
```

```js


function selectionSort(arr,  n) { 
    var i, j, min_idx; 
  
    for (i = 0; i < n-1; i++) 
    { 
        min_idx = i; 
        for (j = i + 1; j < n; j++)  {

            if (arr[j] < arr[min_idx]) 
                min_idx = j; 
        }
  
        if (min_idx !== i) {

       [arr[i], arr[min_idx]] = [arr[min_idx], arr[i]]
        }
    } 
} 

```

```rs

pub fn selection_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if slice[j] < slice[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            slice.swap(i, min_index);
        }
    }
}
#[test]
fn test_selection_sort() {
    let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
    selection_sort(&mut data);
    assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);

}
``` 
```js
function selectionSort(arr,  n) { 
    var i, j, min_idx; 
  
    for (i = 0; i < n-1; i++) 
    { 
        min_idx = i; 
        for (j = i + 1; j < n; j++)  {

            if (arr[j] < arr[min_idx]) 
                min_idx = j; 
        }
  
        if (min_idx !== i) {

       [arr[i], arr[min_idx]] = [arr[min_idx], arr[i]]
        }
    } 
} 
```
## References
1. https://visualgo.net/en/sorting
2. https://www.geeksforgeeks.org/selection-sort/?ref=shm
3. https://www.geeksforgeeks.org/bubble-sort/?ref=header_search
