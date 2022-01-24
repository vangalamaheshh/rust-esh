use std::cmp::Ord;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let size = arr.len();
    for i in 0 .. size {
        for j in 0 .. size -1 -i {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

pub fn insertion_sort<T>(arr: &mut [T]) where T: PartialOrd + Copy {
    for i in 1 .. arr.len() {
        let mut j = i;
        while j > 0 && arr[j-1] > arr[j] {
            arr.swap(j-1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut i = vec![10,2,1,3,5];
        bubble_sort(&mut i);
        assert_eq!(i, vec![1,2,3,5,10]);
    }

    #[test]
    fn insertion_sort_test() {
        let mut i = vec![10,2,1,3,5];
        insertion_sort(&mut i);
        assert_eq!(i, vec![1,2,3,5,10]);
    }
}
