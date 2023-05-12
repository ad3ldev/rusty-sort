fn partition(arr: &mut [u64], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = ((low) as isize) - 1;
    for j in low..high {
        if (arr[j as usize] < pivot) {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    i += 1;
    arr.swap(i as usize, high as usize);
    return i;
}
fn serial_quick_sort_helper(arr: &mut [u64], low: isize, high: isize) {
    if (low < high) {
        let p = partition(arr, low, high);
        serial_quick_sort_helper(arr, low, p - 1);
        serial_quick_sort_helper(arr, p + 1, high);
    }
}
pub fn serial_quick_sort(arr: &mut [u64]) {
    if arr.len() == 0 {
        return;
    }
    let length = arr.len();
    serial_quick_sort_helper(arr, 0, (length - 1) as isize);
}
