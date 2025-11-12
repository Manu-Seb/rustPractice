pub enum Sort {
    Insertion(Vec<i32>),
    Quick(Vec<i32>),
}

impl Sort {
    pub fn sort(&mut self) {
        match self {
            Sort::Insertion(arr) => insertion_wrapper(arr),
            Sort::Quick(arr) => quick_wrapper(arr),
        }
    }
}

fn insertion_wrapper(arr: &mut Vec<i32>) {
    let len = arr.len();
    insertion_sort(arr, len as u32);
}

fn quick_wrapper(arr: &mut Vec<i32>) {
    let left = 0;
    let len = arr.len();
    quick_sort(arr, left, len - 1);
}
pub fn insertion_sort(arr: &mut Vec<i32>, n: u32) {
    for i in 0..n as usize {
        for j in 0..n as usize {
            let first = arr.get(i);
            let second = arr.get(j);

            if let Some(f) = first
                && let Some(s) = second
            {
                if s > f {
                    arr.swap(i, j);
                } else {
                    continue;
                }
            }
        }
    }
}

pub fn quick_sort(arr: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let pivot = partition(arr, left, right);

        if pivot > 0 {
            quick_sort(arr, left, pivot - 1);
        }
        quick_sort(arr, pivot + 1, right);
    }
}

fn partition(arr: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut smallest_val = left;
    for i in left..right {
        if arr[i] < pivot {
            arr.swap(i, smallest_val);
            smallest_val += 1;
        }
    }
    arr.swap(smallest_val, right);
    smallest_val
}
