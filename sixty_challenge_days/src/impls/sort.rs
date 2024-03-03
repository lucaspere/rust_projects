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
pub fn insertion_sort_by<T: PartialOrd + Copy>(data: &mut [T]) {
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

#[cfg(test)]
mod test {
    use crate::impls::sort::insertion_sort;

    #[test]
    fn should_sort_data() {
        let mut data = [34i32, 25, 51, 1, 4, 5, 99, 105];
        insertion_sort(&mut data);
        println!("{:?}", data);
    }
}

// [1,2,3, null, 42];
