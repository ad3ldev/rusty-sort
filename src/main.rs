#![allow(unused)]
mod bubble_sort;
mod merge_sort;
mod quick_sort;

use crate::bubble_sort::*;
use crate::merge_sort::*;
// use crate::quick_sort::*;
use rand::prelude::*;
use std::time::Instant;

fn check_if_equal(control: &mut [u64], tested: &mut [u64]) -> bool {
    let len = control.len();
    let len_test = tested.len();
    if len != len_test {
        return false;
    }
    for i in 0..len {
        if control[i] != tested[i] {
            return false;
        }
    }
    return true;
}

fn running_samples(
    size: usize,
    processors: usize,
    parallel_sort: fn(&mut [u64], usize),
    serial_sort: fn(&mut [u64]),
) {
    let mut arr: Vec<u64> = vec![0; size];
    rand::thread_rng().fill(&mut arr[..]);
    let mut control = arr.clone();
    let mut parallel = arr.clone();

    // Control
    let mut start = Instant::now();
    control.sort();
    let mut end = Instant::now();
    println!("std::sort:\t{:?}, 1", end.duration_since(start));

    // Serial
    start = Instant::now();
    serial_sort(&mut arr);
    end = Instant::now();
    println!("Serialized:\t{:?}, 1", end.duration_since(start));

    // Parallel
    start = Instant::now();
    parallel_sort(&mut parallel, processors);
    end = Instant::now();
    println!("Parallel:\t{:?}, {}", end.duration_since(start), processors);

    if !check_if_equal(&mut control, &mut arr) {
        panic!("NOT EQUAL SERIAL");
    }
    if !(check_if_equal(&mut control, &mut parallel)) {
        panic!("NOT EQUAL PARALLEL");
    }
}

fn main() {
    // let processors: usize = available_parallelism().unwrap().get();
    let threads = vec![2, 4, 8, 16, 32];
    let size = 100_000_000;
    for thread in threads {
        println!("Sizes: {}\tThreads: {}\n", size, thread);
        running_samples(size, thread, parallel_merge_sort, serial_merge_sort);
        println!("------------------------------------\n")
    }
}
