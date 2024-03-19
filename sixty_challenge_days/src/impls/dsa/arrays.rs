fn rotate_3_way_reverses<T>(slice: &mut [T], mid: usize) {
    if mid < slice.len() {
        let (start, end) = slice.split_at_mut(mid);
        start.reverse();
        end.reverse();
        slice.reverse();
    }
}

fn rotate_original_element_storage<T: Copy>(slice: &mut [T], mid: usize) {
    if mid < slice.len() {
        let len = slice.len();
        for _ in 0..mid {
            let original_element = slice[0];
            slice.copy_within(1.., 0);
            slice[len - 1] = original_element;
        }
    }
}

fn rotate_by_juggling<T: Copy>(slice: &mut [T], mid: usize) {
    let sets = gcd(slice.len(), mid);
    let mut i = 0;

    while i < sets {
        let mut start = i;
        let original_element = slice[i];
        loop {
            let cycle = (start + mid) % slice.len();
            if cycle == i {
                break;
            } else {
                slice[start] = slice[cycle];
                start = cycle;
            }
        }
        slice[start] = original_element;
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

pub fn rotate_slice<T: Rotate>(slice: &mut [T], mid: usize) {
    if slice.len() < 250 {
        rotate_3_way_reverses(slice, mid)
    } else {
        rotate_by_juggling(slice, mid)
    }
}

pub fn insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    let outer_loop = 0..slice.len();
    for i in outer_loop {
        let inner_loop = (1 + i)..slice.len();
        for j in inner_loop {
            if slice[i] < slice[j] {
                slice.swap(i, j);
                break;
            }
        }
    }
}

pub fn insertion_sort_optimization<T: PartialOrd + Copy>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let key = slice[i];
        let mut j = i;

        while j > 0 && key < slice[j - 1] {
            slice[i] = slice[j - 1];
            j -= 1;
        }
        slice[j] = key
    }
}

pub trait Rotate: Clone + Copy {}

#[cfg(test)]
mod tests {
    use crate::impls::dsa::arrays::*;

    #[test]
    fn should_rotate_by_inplace() {
        let mut array = [1, 2, 3, 4, 5, 6];

        rotate_3_way_reverses(&mut array, 2);
        assert_eq!(array, [3, 4, 5, 6, 1, 2]);
    }

    #[test]
    fn should_rotate_by_original_element_storage() {
        let mut array = [1, 2, 3, 4, 5, 6];

        rotate_original_element_storage(&mut array, 2);
        assert_eq!(array, [3, 4, 5, 6, 1, 2])
    }

    #[test]
    fn should_rotate_by_juggling() {
        let mut array = [1, 2, 3, 4, 5];

        rotate_by_juggling(&mut array, 2);
        assert_eq!(array, [3, 4, 5, 1, 2])
    }

    #[test]
    fn should_sort_data() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 99, 105];
        // [25, 34]
        // [25, 34, 51]
        insertion_sort_optimization(&mut data);
        println!("{:?}", data);
    }
}
