pub fn serial_bubble_sort(arr: &mut [f64]){
    let n: usize = arr.len();
    for i in 0..n-1 {
        for j in 0..n-i-1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

// pub fn parallel_bubble_sort(arr: &mut [f64]) -> &mut [f64]{
//     return arr;
// }