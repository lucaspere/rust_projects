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

pub fn bubble_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 1..len - i {
            if slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
                swapped = true
            }
        }

        if !swapped {
            break;
        }
    }
}

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
    let mut aux = Vec::with_capacity(left.len() * 2);
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
            aux.extend_from_slice(&&left[ileft..left.len()])
        }
    }

    aux
}
#[cfg(test)]
mod test {
    use crate::impls::dsa::sort::{
        bubble_sort, insertion_sort, insertion_sort_optimization, merge_sort, quick_sort_hoare,
        quick_sort_with_custom_part, selection_sort,
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

    #[test]
    fn test_bubble_sort() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        bubble_sort(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }

    #[test]
    fn test_selection_sort() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        selection_sort(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }
    #[test]
    fn test_merge_sort() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 106, 105];
        merge_sort(&mut data);
        assert_eq!(data, [1, 4, 5, 25, 34, 51, 105, 106]);
    }
}
