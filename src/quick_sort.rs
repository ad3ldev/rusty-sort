pub fn serial_quick_sort(arr: &mut [u64]) {
    if arr.len() == 0 {
        return;
    }
    let length = arr.len();
    quick_sort_helper(arr, 0, length - 1);
}

fn quick_sort_helper(arr: &mut [u64], left: usize, right: usize) {
    let mut i = left;
    let mut j = right;
    let pivot = arr[(left + right) / 2];
    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i <= j {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i += 1;
            j -= 1;
        }
    }
    if left < j {
        quick_sort_helper(arr, left, j);
    }
    if i < right {
        quick_sort_helper(arr, i, right);
    }
}

// pub fn parallel_quick_sort(arr: &mut [u64]) -> &mut [u64]{
//     return arr;
// }
