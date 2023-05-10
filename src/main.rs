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
use std::time::Instant;

fn main() {
    let logical_processors: usize = available_parallelism().unwrap().get();
    const CAPACITY: usize = 1_000_000;
    let mut arr: [u64; CAPACITY] = [0; CAPACITY];
    rand::thread_rng().fill(&mut arr[..]);

    // let mut arr = [76, 921, 32, 0, 237, 24, 132, 40, 1248, 21394, 4, 14];

    let start = Instant::now();
    // serial_bubble_sort(&mut arr);
    // parallel_bubble_sort(&mut arr);
    // serial_quick_sort(&mut arr);
    // serial_merge_sort(&mut arr);
    // serial_radix_sort(&mut arr);
    // serial_merge_sort(&mut arr);
    parallel_merge_sort(&mut arr, 8);
    let end = Instant::now();

    // for i in arr.iter()
    //     println!("{}", i);
    // }
    println!("Time elapsed: {:?}", end.duration_since(start));
}
