use rayon::prelude::*;

pub fn serial_bubble_sort(arr: &mut [u64]) {
    let n: usize = arr.len();
    for i in 0..n-1 {
        for j in 0..n - i-1 {
            if arr[j] > arr[j+1] {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}


use std::sync::atomic::{AtomicBool, Ordering};

pub fn parallel_bubble_sort(arr: &mut [u64]) {
    let n = arr.len();
    let swapped = AtomicBool::new(true);
    while swapped.load(Ordering::Relaxed) {  // the "Ordering::Relaxed" argument specifies the memory ordering for the atomic operations, and this affects how memory accesses are synchronized between threads
        
        swapped.store(false, Ordering::Relaxed);
        let chunk_size = 166; // 1000/6    (the size of the array/number of  on my pc)
        arr.par_chunks_mut(chunk_size).for_each(|chunk| {
            let mut local_swapped = false;
            for j in 0..chunk.len()-1 {

                if chunk[j] > chunk[j+1] {
                    chunk.swap(j, j+1);
                    local_swapped = true;
                }
                
            }
            if local_swapped {
                swapped.store(true, Ordering::Relaxed);
            }
        });

    }
}
