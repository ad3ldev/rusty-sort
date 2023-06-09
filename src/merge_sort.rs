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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parallel_merge_helper(left: &mut [u64], right: &mut [u64]) -> Vec<u64> {
    let len1 = left.len();
    let len2 = right.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut merged = vec![0; len1 + len2];
    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            merged[k] = left[i];
            i += 1;
        } else {
            merged[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < len1 {
        merged[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < len2 {
        merged[k] = right[j];
        j += 1;
        k += 1;
    }
    merged
}
pub fn parallel_merge(arr: &mut [u64], chunk_size: &mut usize, len: usize, threads_num: usize) {
    if *chunk_size > len {
        return;
    }
    let mut start = 0;
    let mut end = *chunk_size;
    let mut children = vec![];
    for _ in 0..threads_num {
        let mut left = arr[start..end].to_vec();
        start += *chunk_size;
        end += *chunk_size;
        if end > len {
            end = len;
        }
        if start > end {
            break;
        }
        let mut right = arr[start..end].to_vec();
        children.push(thread::spawn(move || {
            let merged = parallel_merge_helper(&mut left, &mut right);
            merged
        }));
        start += *chunk_size;
        end += *chunk_size;
        if end > len {
            end = len;
        }
        if start > end {
            break;
        }
    }
    start = 0;
    end = 0;
    for child in children {
        let chunk = child.join().unwrap();
        end += chunk.len();
        arr[start..end].copy_from_slice(&chunk[..]);
        start = end;
    }
    *chunk_size *= 2;
    parallel_merge(arr, chunk_size, len, threads_num)
}
fn parallel_merge_sort_helper(
    arr: &mut [u64],
    chunk_size: &mut usize,
    len: usize,
    threads_num: usize,
) {
    let mut start = 0;
    let mut end = if *chunk_size > len { len } else { *chunk_size };
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
    parallel_merge(arr, chunk_size, len, threads_num);
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
}
