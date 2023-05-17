![Language](https://img.shields.io/badge/language-Rust%20-brown.svg)
![License](https://img.shields.io/badge/License-MIT%20-red.svg)


                d8888b. db    db .d8888. d888888b db    db      .d8888.  .d88b.  d8888b. d888888b 
                88  `8D 88    88 88'  YP `~~88~~' `8b  d8'      88'  YP .8P  Y8. 88  `8D `~~88~~' 
                88oobY' 88    88 `8bo.      88     `8bd8'       `8bo.   88    88 88oobY'    88    
                88`8b   88    88   `Y8b.    88       88           `Y8b. 88    88 88`8b      88    
                88 `88. 88b  d88 db   8D    88       88         db   8D `8b  d8' 88 `88.    88    
                88   YD ~Y8888P' `8888Y'    YP       YP         `8888Y'  `Y88P'  88   YD    YP    
                                                                                  
                                                                                  


## Problem Statement
Rust is a multi-paradigm, high-level, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—ensuring that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent concurrent data races, its "borrow checker" tracks the object lifetime of all references in a program during compilation. Rust is popularized for systems programming but also has high-level features including some functional programming constructs.
With Rust native support for concurrency and parallelism, and performance, it was decided to implement a few algorithms to show and analyze the performance of Rust in a serial and a parallel implementation.

## Sorting Algorithms
-   Bubble Sort
-   Merge Sort

### Bubble Sort
![image](https://github.com/ad3ldev/rusty-sort/assets/58489322/142e72a8-3325-45db-887d-33c926fdaf83)

-----------------------------------------------------------------------------------------------------------------------------------

### Merge Sort
![image](https://github.com/ad3ldev/rusty-sort/assets/58489322/1a5cc60a-c3d8-4bec-8a59-14a78dee5629)

## Conculsion
With the divide and conquer strategy that was used on the previous algorithms.
We can devise a standard algorithm to make any sorting algorithm parallel:
1. Divide the array into chunks, where `chunk = array size / number of processors`.
2. Use the algorithm to sort each chunk in parallel.
3. Take each two chunks and merge them in parallel.
4. Continue until one chunk is left.
5. Return the array sorted.
