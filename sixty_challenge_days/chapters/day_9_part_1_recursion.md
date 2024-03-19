# Day 9 - Part 1: Recursion

## Recursion
It is a technique to solve problems by calling a function itself till reach a point that it knows how to solve. For example, a reverse string can be solved by calling a function (f) that if the string's length (n) is 0 or 1 return it, otherwise return the f(str[1..]) + str[0].

## Use cases
Recursion is perfect for tree-like algorithms because its flow data naturally. Tree-like algorithms normally needs to perform the same procedure until reach the point to be solved. Mostly fastest algorihtms and data structure uses recursive: quicksort, mergesort, tree transverse, btree.

## How it works
To allow recursion, the compiler must save each function call context in a *call stack*. The call stack save the context of the function and push again to the top of stack. When the function reaches the final point, the compiler pops the function and return it to the function caller. Since it will be pushing function calls to the stack until the function return a value, it can cause infinite loop if the function does not have a stop flag.

### Illustration of reverse_str_rec stack call push and unwind
```
*
**
***
****
*****
******
*******
********
*********
**********
*********
********
*******
******
*****
****
***
**
*
```

### Memoization
It is a technique of remembering the return values from a function for the specific arguments supplied to it by caching the previously calculated results. Memoization makes a trade-off to save on execution time by increasing memory usage.

### Top-Down Dynamic Programming
It's a computer technique to breaking a large problem into **overlapping** subproblems. So Dynamic programming uses recursion with repeated recursive cases. Fibonacci is an algorithm with overlapping case, but mergesort and quicksort is not. There is a constraint to use dynamic programming: the function must be pure. Pure function is a concept in functional programming that a function have to produce the same value with the same argument. For example, ``fibonnaci(7)`` always produce ``13``, so it is pure. However, if we have a function ``days_after_from_now(2)`` that uses the time right now plus the days in arguments, it will produce different results even with the same argument, so it's not determinist.

## Performance
Because the call stack context saving, recursion is not very performative than the iterator technique. It has to save each context for each function call, so it has memory overhead. There are techniques to make the recursion more performative like TOC (Tail Optimization Call), Memoization Recursion and others. With Memoization, we can use LRU cache to save function call returns and remove the oldest result if the cache becomes full.


```js
let count = ''
function reverse_str_rec(str) {
    if(str.length <= 1) {
        return str
    } else {
        const [head, ...tail] = str
        count += '*'
        console.log(count)
        const r = reverse_str_rec(tail) + head
        count = count.slice(0, count.length - 1)
        console.log(count)
        return 
    }
}
function fatorial(n) {
    if (n === 1 || n === 0) {
        return 1
    } else {
        return fatorial(n - 1) * n
    }
}
function fibonacci(nth) {
    if (nth === 1 || nth === 2) {
        return 1
    } else {
        return fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}

const dp = new Map
function fibonacci_bottom_up(nth) {
    if (nth === 1 || nth === 2) {
        return 1
    } else {
        let r1 = dp.get(nth - 1) || fibonacci(nth - 1)
        let r2 = dp.get(nth - 2) || fibonacci(nth - 2)

        const sum = r1 + r2;
        dp.set(nth, sum)
        return sum
    }
}

```
```rs

fn fatorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        fatorial(n - 1) * n
    }
}

fn fibonnaci(n: u32) -> u64 {
    if n == 2 || n == 1 {
        1
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

fn fibonnaci_bottom_up(n: u32) -> u64 {
    if n == 2 || n == 1 {
        1
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fatorial() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(5), 120);
        assert_eq!(fatorial(10), 3628800);
    }

    #[test]
    fn test_fibonnaci() {
        assert_eq!(fibonnaci(1), 1);
        assert_eq!(fibonnaci(2), 1);
        assert_eq!(fibonnaci(5), 5);
        assert_eq!(fibonnaci(10), 55);
    }
}
```

## References
1. Bhargava, Aditya; Grokking Algorithms 2ed. Chapter 3: Recursion.
2. Sweigart, Al; The Recursive Book of Recursion 1ed. Chapter 7: Memoization and Dynamic.
3. https://www.geeksforgeeks.org/introduction-to-recursion-data-structure-and-algorithm-tutorials/?ref=lbp
4. https://www.youtube.com/watch?v=oBt53YbR9Kk