use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref FIB_CACHE: HashMap<u32, u64> = HashMap::new();
}

pub fn fatorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        fatorial(n - 1) * n
    }
}

pub fn fibonnaci(n: u32) -> u64 {
    if n == 2 || n == 1 {
        1
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

pub fn fibonnaci_mem() -> impl FnMut(u32) -> u64 {
    let mut cache = HashMap::new();

    move |n| {
        *cache.entry(n).or_insert_with(|| {
            if n == 2 || n == 1 {
                1
            } else {
                fibonnaci(n - 1) + fibonnaci(n - 2)
            }
        })
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
    #[test]
    fn test_fibonnaci_mem() {
        assert_eq!(fibonnaci_mem()(1), 1);
        assert_eq!(fibonnaci_mem()(2), 1);
        assert_eq!(fibonnaci_mem()(5), 5);
        assert_eq!(fibonnaci_mem()(10), 55);
    }
}
