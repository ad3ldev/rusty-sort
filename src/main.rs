#![allow(unused)]
mod bubble_sort;
mod merge_sort;
mod quick_sort;

use crate::bubble_sort::*;
use crate::merge_sort::*;
use crate::quick_sort::*;
use rand::prelude::*;
use std::thread::*;
use std::time::Duration;
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

fn main() {
    let processors: usize = available_parallelism().unwrap().get();
    const CAPACITY: usize = 1_000;
    let mut arr: Vec<u64> = vec![0; CAPACITY];
    rand::thread_rng().fill(&mut arr[..]);

    // Control
    let mut control = arr.clone();
    let mut start = Instant::now();
    control.sort();
    let mut end = Instant::now();
    println!("std::sort:\t{:?}, 1", end.duration_since(start));

    // Parallel
    let mut parallel = arr.clone();
    start = Instant::now();
    // parallel_merge_sort(&mut parallel, processors);
    parallel_bubble_sort(&mut parallel);
    end = Instant::now();
    println!("Parallel:\t{:?}, {}", end.duration_since(start), processors);

    // Serial
    start = Instant::now();
    serial_bubble_sort(&mut arr);
    end = Instant::now();
    println!("Serialized:\t{:?}, 1", end.duration_since(start));

    if !check_if_equal(&mut control, &mut arr) {
        panic!("not equal Serial");
    }
    if !(check_if_equal(&mut control, &mut parallel)) {
        panic!("not equal parallel");
    }
}
