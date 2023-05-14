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

// fn hyper_quick_sort(arr: &mut [u64], low: isize, high: isize, threads_num: usize) {
//     if (low < high) {
//         let p = partition(arr, low, high);
//         if threads_num > 1 {
//             rayon::join(
//                 || hyper_quick_sort(arr, low, p - 1, threads_num / 2),
//                 || hyper_quick_sort(arr, p + 1, high, threads_num - threads_num / 2),
//             );
//         } else {
//             hyper_quick_sort(arr, low, p - 1, threads_num);
//             hyper_quick_sort(arr, p + 1, high, threads_num);
//         }
//     }
// }
pub fn parallel_quick_sort(arr: &mut [u64], threads_num: usize) {
    if arr.len() == 0 {
        return;
    }
    let length = arr.len();
    // hyper_quick_sort(arr, 0, (length - 1) as isize, threads_num);
}