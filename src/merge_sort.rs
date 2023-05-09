fn serial_merge(arr: &mut [f64], start: usize, mid: usize, end: usize) {
    let len1 = mid - start + 1;
    let len2 = end - mid;
    let left = arr[start..mid + 1].to_vec();
    let right = arr[mid + 1..end + 1].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = start;
    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < len1 {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < len2 {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
fn serial_merge_sort_helper(arr: &mut [f64], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = (left + right) / 2;
    serial_merge_sort_helper(arr, left, mid);
    serial_merge_sort_helper(arr, mid + 1, right);
    serial_merge(arr, left, mid, right);
}
pub fn serial_merge_sort(arr: &mut [f64]) {
    if arr.len() == 0 {
        return;
    }
    let len = arr.len();
    serial_merge_sort_helper(arr, 0, len - 1);
}

fn parallel_merge(arr: &mut [f64], start: usize, mid: usize, end: usize) {}
pub fn parallel_merge_sort(arr: &mut [f64], left: usize, right: usize) {}
