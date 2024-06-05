// Given a sorted array of integers, implement a function that returns the median of the array.
fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        panic!("The array is empty!");
    }
    
    if len % 2 == 0 {
        let mid1 = len / 2 - 1;
        let mid2 = len / 2;
        (arr[mid1] as f64 + arr[mid2] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    println!("The median of {:?} is {}", arr1, find_median(&arr1));
}
