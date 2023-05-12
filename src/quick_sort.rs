// fn serial_quick_sort_helper_old(arr: &mut [u64], left: usize, right: usize) {
//     let mut i = left;
//     let mut j = right;
//     let pivot = arr[(left + right) / 2];
//     while i <= j {
//         while arr[i] < pivot {
//             i += 1;
//         }
//         while arr[j] > pivot {
//             j -= 1;
//         }
//         if i <= j {
//             let temp = arr[i];
//             arr[i] = arr[j];
//             arr[j] = temp;
//             i += 1;
//             j -= 1;
//         }
//     }
//     if left < j {
//         serial_quick_sort_helper(arr, left, j);
//     }
//     if i < right {
//         serial_quick_sort_helper(arr, i, right);
//     }
// }

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
