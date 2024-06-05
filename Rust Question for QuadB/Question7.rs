// Implement a function that returns the kth smallest element in a given array.
use std::cmp::Ordering;

fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    if k == 0 || k > arr.len() {
        panic!("k is out of the bounds of the array");
    }
    
    fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;

        for j in low..high {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }

    fn quickselect(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
        if low == high {
            return arr[low];
        }
        
        let pivot_index = partition(arr, low, high);
        
        match pivot_index.cmp(&k) {
            Ordering::Equal => arr[pivot_index],
            Ordering::Less => quickselect(arr, pivot_index + 1, high, k),
            Ordering::Greater => quickselect(arr, low, pivot_index - 1, k),
        }
    }

    quickselect(arr, 0, arr.len() - 1, k - 1)
}

fn main() {
    let mut arr = [3, 2, 1, 5, 4];
    let k = 3;

    let result = kth_smallest(&mut arr, k);
    println!("The {}-th smallest element is {}", k, result);
}
