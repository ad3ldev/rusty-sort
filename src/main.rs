#![allow(unused)]
mod bubble_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;

use crate::bubble_sort::parallel_bubble_sort;
use crate::bubble_sort::parallel_bubble_sort;
use crate::bubble_sort::serial_bubble_sort;
use crate::merge_sort::serial_merge_sort;
use crate::quick_sort::serial_quick_sort;
use crate::radix_sort::serial_radix_sort;
use rand::prelude::*;
use std::time::Instant;

fn gen_rand_arr<const SIZE: usize>(rng: &mut ThreadRng) -> [u64; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(0..SIZE as u64);
    }
    arr
}

fn main() {
    let mut rng = thread_rng();
    // let mut arr: [u64; 1_000_000] = gen_rand_arr(&mut rng);
    let mut arr = [813, 123, 9, 0, 241, 420];
    let start = Instant::now();

    // serial_bubble_sort(&mut arr);
    // parallel_bubble_sort(&mut arr);
    // serial_quick_sort(&mut arr);
    // serial_merge_sort(&mut arr);
    // serial_radix_sort(&mut arr);
    // serial_merge_sort(&mut arr);

    for i in 0..arr.len() {
        print!("{}  ", arr[i]);
    }
    println!();

    let end = Instant::now();
    // for i in arr.iter() {
    //     println!("{}", i);
    // }
    println!("Time elapsed: {:?}", end.duration_since(start));
}
