use std::thread;

fn serial_merge(arr: &mut [u64], start: usize, mid: usize, end: usize) {
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


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


fn serial_merge_sort_helper(arr: &mut [u64], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = (left + right) / 2;
    serial_merge_sort_helper(arr, left, mid);
    serial_merge_sort_helper(arr, mid + 1, right);
    serial_merge(arr, left, mid, right);
}
pub fn serial_merge_sort(arr: &mut [u64]) {
    if arr.len() == 0 {
        return;
    }
    let len = arr.len();
    serial_merge_sort_helper(arr, 0, len - 1);
}

fn parallel_merge_sort_helper(
    arr: &mut [u64],
    chunk_size: &mut usize,
    len: usize,
    threads_num: usize,
) {
    if *chunk_size >= len {
        return;
    }
    let mut start = 0;
    let mut end = *chunk_size;
    let mut children = vec![];
    for _ in 0..threads_num {
        if start > end {
            break;
        }
        let mut chunk = arr[start..end].to_vec();
        children.push(thread::spawn(move || {
            chunk.sort();
            chunk
        }));
        start += *chunk_size;
        end += *chunk_size;
        if end > len {
            end = len;
        }
    }
    start = 0;
    end = *chunk_size;
    for child in children {
        let chunk = child.join().unwrap();
        arr[start..end].copy_from_slice(&chunk[..]);
        start += *chunk_size;
        end += *chunk_size;
        if end > len {
            end = len;
        }
    }
    *chunk_size *= 2;
    parallel_merge_sort_helper(arr, chunk_size, len, threads_num);
}

pub fn parallel_merge_sort(arr: &mut [u64], threads_num: usize) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    if len <= threads_num {
        serial_merge_sort(arr)
    }
    let mut chunk_size = (len as f64 / threads_num as f64).ceil() as usize;
    parallel_merge_sort_helper(arr, &mut chunk_size, len, threads_num);
    serial_merge(arr, 0, chunk_size / 2 - 1, len - 1);
}
