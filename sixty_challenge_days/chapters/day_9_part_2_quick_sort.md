# Day 9 - Part 2: Quick Sort 

## Introduction
Insertion Sort is a stable In-place linear algorithm that sorting by putting a value in your correct position for each iteraction. For each value, it analyzes the value with the subsequents values, switting the position with correspondict value. For that reason, the time complexity is O(n^2) on the worst and average cases. On the best case, it has O(n) time Complexity. Since is In-place, the space complexity is constant O(1), because it not uses some extra spacing to sort the data, it only uses a reference of values.

### Divide & Conquer (D&C)
This is a tool to solve problem by **divide** a complex problem in one subproblem until reaches a point that the problem can be solved. With that, is the fase to **conquer** and resolve the problem. Each step of divide must subtract a portion of the problem, so the step 1 and step 2 must not have the same portion of the problem.

### Partition Schema Strategies
#### Lomuto
Is more simplist strategy, but is the slowest one. This algorithm works by assuming the pivot element as the last element. If any other element is given as a pivot element then swap it first with the last element.
```rs
fn lomuto_partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let pivot_index = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}
```
* Time Complexity: O(n^2) 
* Auxiliary Space: O(1)
#### Hoare
works by initializing two indexes that start at two ends, the two indexes move toward each other until an inversion is (A smaller value on the left side and greater value on the right side) found. When an inversion is found, two values are swapped and the process is repeated.
```rs
fn hoare_partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let pivot = get_pivot();
    let mut start = 0;
    let mut end = slice.len() - 1;

    loop {
        while slice[start] < slice[pivot] {
            start += 1;
        }
        while slice[end] > slice[pivot] {
            end -= 1;
        }

        if start >= end {
            break end;
        }

        slice.swap(start, end)
    }
}
```

* Time Complexity: O(N) 
* Auxiliary Space: O(1)

## Complexity Analysis
### Time Complexity
* Best-case: O(n * Log n) When the halves of array are balanced
* Average-case: O(n * Log n) when  
* Worse-case: O(n^2) when the data is sorted or almostly sorted or the pivot cause very unbalanced partitions in each step. There are differents partition strategies to avoid this.

### Space Complexity
O(1) when not considering the call stack memory. In this case is O(n)

## Implementations
```rs
/// Quick sort using middle of three strategy.
pub fn quick_sort_middle_three<T: PartialOrd + Copy>(slice: &mut [T]) {
    let slice_len = slice.len();
    if slice_len > 1 {
        if slice_len == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        } else {
            let pivot = hoare_partition(slice, || {
                (0 + slice_len.div_ceil(2) + slice_len - 1).div_ceil(3)
            });
            let (less, greater) = slice.split_at_mut(pivot);
            quick_sort_hoare(less);
            quick_sort_hoare(greater);
        }
    }
}
```

## References
1. Bhargava, Aditya; Grokking Algorithms 2ed. Chapter 4: Quicksort.
2. Sweigart, Al; The Recursive Book of Recursion 1ed. Chapter 6: Divide-and-Conquer Algorithms.
3. https://www.geeksforgeeks.org/hoares-vs-lomuto-partition-scheme-quicksort/
4. https://www.geeksforgeeks.org/quick-sort/
