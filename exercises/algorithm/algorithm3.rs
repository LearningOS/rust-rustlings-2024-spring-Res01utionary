/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::clone::Clone;
use std::cmp::PartialOrd;
fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    if array.len() <= 1{return}
    heapify(array);
    for end in (1..array.len()).rev() {
        array.swap(0, end);
        down_heap(array, 0, end - 1);
    }
}

fn down_heap<T: PartialOrd + Clone>(array: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child + 1 <= end && array[child] < array[child + 1] {
            child += 1;
        }
        if array[root] < array[child] {
            array.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

fn heapify<T: PartialOrd + Clone>(array: &mut [T]) {
    let len = array.len();
    let start = (len - 2) / 2;
    for i in (0..=start).rev() {
        down_heap(array, i, len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
