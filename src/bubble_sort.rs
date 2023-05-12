use std::thread;

pub fn serial_bubble_sort(arr: &mut [u64]) {
    let n: usize = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn parallel_bubble_sort(arr: &mut [u64]) {
    let n = arr.len();
    let swapped = AtomicBool::new(true);
    while swapped.load(Ordering::Relaxed) {
        swapped.store(false, Ordering::Relaxed);
        let chunk_size = 170; // ~(array size / number of  on my pc)
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
    }
}
