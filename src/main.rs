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

fn parallel_running(parallel: &mut [u64], processors: usize, parallel_sort: fn(&mut [u64], usize)) {
    // Parallel
    let start = Instant::now();
    parallel_sort(parallel, processors);
    let end = Instant::now();
    println!("Parallel:\t{:?}, {}", end.duration_since(start), processors);
}

fn serial_running(control: &mut [u64], arr: &mut [u64], serial_sort: fn(&mut [u64])) {
    // Control
    let mut start = Instant::now();
    control.sort();
    let mut end = Instant::now();
    println!("std::sort:\t{:?}, 1", end.duration_since(start));

    // Serial
    start = Instant::now();
    serial_sort(arr);
    end = Instant::now();
    println!("Serialized\t{:?}, 1", end.duration_since(start));
}

fn main() {
    // let processors: usize = availa  ble_parallelism().unwrap().get();
    let threads = vec![2, 4, 8, 16, 32];
    let size = 500_000_000;

    let mut arr: Vec<u64> = vec![0; size];
    rand::thread_rng().fill(&mut arr[..]);
    let mut control = arr.clone();
    let mut parallel = arr.clone();

    serial_running(&mut control, &mut arr, serial_merge_sort);
    println!("###############################################");
    for thread in threads {
        println!("Sizes: {}\tThreads: {}\n", size, thread);
        parallel_running(&mut parallel, thread, parallel_merge_sort);
        println!("------------------------------------\n")
    }
    if !check_if_equal(&mut control, &mut arr) {
        panic!("NOT EQUAL SERIAL");
    }
    if !check_if_equal(&mut control, &mut parallel) {
        panic!("NOT EQUAL SERIAL");
    }
}
