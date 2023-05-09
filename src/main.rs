mod bubble_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;

use crate::bubble_sort::serial_bubble_sort;
use crate::merge_sort::serial_merge_sort;
use crate::quick_sort::serial_quick_sort;

fn main() {
    let mut arr = [34.0, 147.0, 20.0, 3.0, 89.6];
    let len_arr = arr.len();

    // serial_bubble_sort(&mut arr);
    serial_quick_sort(&mut arr);
    // serial_merge_sort(&mut arr);

    for element in arr.iter() {
        println!("{}", element)
    }
}
