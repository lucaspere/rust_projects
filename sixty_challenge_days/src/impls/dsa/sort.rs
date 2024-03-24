use rand::Rng;

pub fn insertion_sort<T: PartialOrd + Copy>(data: &mut [T]) {
    for i in 0..data.len() {
        let inner_loop = (1 + i)..data.len();

        for j in inner_loop {
            if data[i] > data[j] {
                let aux = data[i];
                data[i] = data[j];
                data[j] = aux;
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

pub fn quick_sort_with_custom_part<T, P>(slice: &mut [T], partition_strategy: &P)
where
    P: Fn(usize) -> usize,
    T: PartialOrd,
{
    let slice_len: usize = slice.len();
    if slice_len > 1 {
        if slice_len == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        } else {
            let pivot = lomuto_partition(slice, || partition_strategy(slice_len));
            let (less, greater) = slice.split_at_mut(pivot);
            quick_sort_with_custom_part(less, partition_strategy);
            quick_sort_with_custom_part(greater, partition_strategy);
        }
    }
}
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

pub fn quick_sort_random<T: PartialOrd + Copy>(slice: &mut [T]) {
    let slice_len = slice.len();
    if slice_len > 1 {
        if slice_len == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        } else {
            let pivot = hoare_partition(slice, || rand::thread_rng().gen_range(0..slice_len - 1));
            let (less, greater) = slice.split_at_mut(pivot);
            quick_sort_hoare(less);
            quick_sort_hoare(greater);
        }
    }
}

pub fn quick_sort_hoare<T: PartialOrd + Copy>(slice: &mut [T]) {
    let slice_len = slice.len();
    if slice_len > 1 {
        if slice_len == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        } else {
            let pivot = hoare_partition(slice, || slice_len - 1);
            let (less, greater) = slice.split_at_mut(pivot);
            quick_sort_hoare(less);
            quick_sort_hoare(greater);
        }
    }
}

pub fn quick_sort_lomuto<T: PartialOrd + Copy>(slice: &mut [T]) {
    let slice_len = slice.len();
    if slice_len > 1 {
        if slice_len == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        } else {
            let pivot = lomuto_partition(slice, || slice_len - 1);
            let (less, greater) = slice.split_at_mut(pivot);
            quick_sort_hoare(less);
            quick_sort_hoare(greater);
        }
    }
}

fn lomuto_partition<T, P>(slice: &mut [T], get_pivot: P) -> usize
where
    P: FnOnce() -> usize,
    T: PartialOrd,
{
    let pivot_index = get_pivot();
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

fn hoare_partition<T, P>(slice: &mut [T], get_pivot: P) -> usize
where
    P: FnOnce() -> usize,
    T: PartialOrd,
{
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
#[cfg(test)]
mod test {
    use crate::impls::dsa::sort::{
        insertion_sort, insertion_sort_optimization, quick_sort_hoare, quick_sort_with_custom_part,
    };

    #[test]
    fn test_insertion_sort() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 99, 105];
        insertion_sort(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 99, 105]);
    }

    #[test]
    fn test_insertion_sort_optimization() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 99, 105];
        insertion_sort_optimization(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 99, 105]);
    }

    #[test]
    fn test_quick_sort_with_lomuto_partition() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        quick_sort_hoare(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }

    #[test]
    fn test_quick_sort_with_hoare_partition() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        quick_sort_hoare(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }

    #[test]
    fn test_quick_sort_with_custom_strategy() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        quick_sort_with_custom_part(&mut data, &|len| len - 1);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }
}
