#![allow(unused)]
mod bubble_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;

use crate::bubble_sort::*;
use crate::merge_sort::*;
use crate::quick_sort::*;
use rand::prelude::*;
use std::thread::*;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let logical_processors: usize = available_parallelism().unwrap().get();
    const CAPACITY: usize = 100_000_000;
    let mut arr: Vec<u64> = vec![0; CAPACITY];
    rand::thread_rng().fill(&mut arr[..]);
    // let mut arr = [76, 921, 32, 0, 237];
    let mut least: Duration = Duration::MAX;
    let mut least_threads = CAPACITY;
    for i in 2..logical_processors + 1 {
        let mut temp = arr.clone();
        let start = Instant::now();
        parallel_merge_sort(&mut temp, i);
        let end = Instant::now();
        if (end.duration_since(start) < least) {
            least = end.duration_since(start);
            least_threads = i;
        }
        println!(
            "Time elapsed: {:?},\tthreads: {}",
            end.duration_since(start),
            i
        );
    }
    println!("Parallel: {:?}, {}", least, least_threads);
    let mut trial = arr.clone();
    let mut start = Instant::now();
    serial_merge_sort(&mut arr);
    let mut end = Instant::now();
    println!("Serial: {:?}, 1", end.duration_since(start));
    start = Instant::now();
    trial.sort();
    end = Instant::now();
    println!("std: {:?}, 1", end.duration_since(start));
}
