use crate::merge_sort::parallel_merge;

pub fn serial_bubble_sort(arr: &mut [u64]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn parallel_bubble_sort(arr: &mut [u64], threads_num: usize) {
    let n = arr.len();
    let swapped = AtomicBool::new(true);
    while swapped.load(Ordering::Relaxed) {
        swapped.store(false, Ordering::Relaxed);
        let mut chunk_size = (n as f64 / threads_num as f64).ceil() as usize; // ~(array size / number of  on my pc)
        arr.par_chunks_mut(chunk_size).for_each(|chunk| {
            let mut local_swapped = false;
            for j in 0..chunk.len() - 1 {
                if chunk[j] > chunk[j + 1] {
                    chunk.swap(j, j + 1);
                    local_swapped = true;
                }
            }
            if local_swapped {
                swapped.store(true, Ordering::Relaxed);
            }
        });
        parallel_merge(arr, &mut chunk_size, n, threads_num);
    }
}
