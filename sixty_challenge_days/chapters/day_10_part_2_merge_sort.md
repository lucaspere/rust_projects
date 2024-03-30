# Day 10 - Part 1: Merge Sort

## Introduction
Merge Sort is an efficient D&C algorithm that basically split the list in sublists until reaches the basic case (normally the array is length 1 or 2) and then returns merging it.
## Complexity Analysis
### Time Complexity
The worst scenary is when the list is already sorted. The complexity is the same, but will perform unecessary sorted operations.
* Best-case: O(n * log n),
* Average-case: O(n * log n),
* Worse-case: O(n * log n).

### Space Complexity
O(N) it's not an in-place algorithm because is necessary to have a temporary list when is merging. So it uses the size of list as memory.

## Implementations

```rs
pub fn merge_sort<T: PartialOrd + Copy>(slices: &mut [T]) {
    if slices.len() > 1 {
        if slices.len() == 2 && slices[0] > slices[1] {
            slices.swap(0, 1)
        } else {
            let len = slices.len();
            let mid = len / 2;
            let (right, left) = slices.split_at_mut(mid);
            merge_sort(right);
            merge_sort(left);

            let aux_array = merge(right, left);
            slices.copy_from_slice(&aux_array.as_slice());
        }
    }
}

fn merge<T: PartialOrd + Copy>(left: &mut [T], right: &mut [T]) -> Vec<T> {
    let mut aux = Vec::with_capacity(left.len() + right.len());
    let mut ileft = 0;
    let mut iright = 0;

    while aux.len() < (left.len() + right.len()) {
        if left[ileft] < right[iright] {
            aux.push(left[ileft]);
            ileft += 1;
        } else {
            aux.push(right[iright]);
            iright += 1;
        }
        if ileft == left.len() {
            aux.extend_from_slice(&right[iright..right.len()])
        } else if iright == right.len() {
            aux.extend_from_slice(&left[ileft..left.len()])
        }
    }

    aux
}

#[test]
fn test_merge_sort() {
    let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
    merge_sort(&mut data);
    assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
}
```

### Benchmark results
#### Inputs
Array with 7500 items randomly inserted.
#### Results
Sort Algorithms Randomly Smaples/insert_sort 
                        time:   [5.4455 µs 5.4859 µs 5.5296 µs]
                        change: [+106.08% +108.99% +111.64%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Sort Algorithms Randomly Smaples/quick_sort
                        time:   [69.291 µs 69.806 µs 70.353 µs]
                        change: [+46.154% +47.115% +48.055%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Sort Algorithms Randomly Smaples/bubble_sort
                        time:   [2.8036 µs 2.8253 µs 2.8511 µs]
                        change: [+48.601% +49.791% +51.068%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
Sort Algorithms Randomly Smaples/selection_sort
                        time:   [14.019 ms 14.116 ms 14.217 ms]
                        change: [+113.56% +115.82% +117.99%] (p = 0.00 < 0.05)
                        Performance has regressed.
Sort Algorithms Randomly Smaples/merge_sort
                        time:   [335.06 µs 337.30 µs 339.71 µs]
                        change: [+68.258% +69.786% +71.295%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

## References
1. https://visualgo.net/en/sorting
2. Sweigart, Al; The Recursive Book of Recursion 1ed. Chapter 6: Divide-and-Conquer Algorithms.
2. https://www.geeksforgeeks.org/merge-sort/
