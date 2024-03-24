# In-Place Algorithm: Rotate an Array
## Introduction
Rotations in the array is defined as the process of rearranging the elements in an array by shifting each element to a new position. This is mostly done by rotating the elements of the array clockwise or counterclockwise.

## Complexity Analysis
### Time Complexity
* Reversal algorithm: O(n)
* Juggling Algorithm: O(n);
* Rotate one by one: O(n*d), d equal the quantity to rotate.

### Space Complexity
As it is In-place, so O(1)

## Illustration

## Implementations 

```rs
/// [Reversal algorithm](https://www.geeksforgeeks.org/program-for-array-rotation-continued-reversal-algorithm/)
/// Strategy that rotate a array by split it into two subarrays and reverse each one,
/// then brings together and do the final reverse.
pub fn rotate_3_way_reverses<T>(slice: &mut [T], mid: usize) {
    if mid < slice.len() {
        let (start, end) = slice.split_at_mut(mid);
        start.reverse();
        end.reverse();
        slice.reverse();
    }
}

/// [Juggling Algorithm(https://www.geeksforgeeks.org/array-rotation/)
/// Rotates by loop sets using a GCD (Greatest Common Divisor) to determined the amount of sets to use
pub fn rotate_left<T: Copy>(slice: &mut [T], mid: usize) {
    if mid < slice.len() {
        let len = slice.len();

        for _ in 0..mid {
            let original_element = slice[0];
            slice.copy_within(1.., 0);
            slice[len - 1] = original_element;
        }
    }
}

/// [Rotate one by one](https://www.geeksforgeeks.org/array-rotation/)
/// Rotates by loop sets using a GCD (Greatest Common Divisor) to determined the amount of sets to use
pub fn rotate_by_juggling<T: Copy>(slice: &mut [T], mid: usize) {
    let cycles = gcd(slice.len(), mid);
    let mut i = 0;

    while i < cycles {
        let mut start = i;
        let original_elem = slice[i];
        loop {
            let key = (start + mid) % slice.len();
            if key == i {
                break;
            } else {
                slice[start] = slice[key];
                start = key;
            }
        }
        slice[start] = original_elem;
        i += 1;
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut x = a;
    let mut y = b;

    while y != 0 {
        let original_element = y;
        y = x % y;
        x = original_element;
    }

    x
}
```

## References
https://www.youtube.com/watch?v=72Zelk_XCy8&list=PLWb0cWXA-TbCD9xkgaDicM3G6Ch6jfhnO
https://www.geeksforgeeks.org/complete-guide-on-array-rotations