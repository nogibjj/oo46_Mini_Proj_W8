use std::time::Instant;
use std::fs::File;
use std::io::Write;
use rand::Rng;

fn bubble_sort(array: &mut Vec<i32>) {
    let n = array.len();

    for i in 0..n {
        let mut is_sorted = true;

        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                is_sorted = false;
            }
        }

        if is_sorted {
            break;
        }
    }
}

fn main() {
    let array_len = 10000;
    let mut array: Vec<i32> = (0..array_len).map(|_| rand::thread_rng().gen_range(0..1000)).collect();

    let start_time = Instant::now();

    // Estimate memory usage
    let element_size = std::mem::size_of::<i32>();
    let total_memory_usage = element_size * array_len;

    bubble_sort(&mut array);

    let end_time = Instant::now();

    let execution_time = end_time - start_time;

    // Create a results file
    let mut file = File::create("rust_results.txt").expect("Failed to create file");

    // Write the results to the file
    writeln!(file, "Bubble Sort implementation in Rust took: {:?} seconds to execute", execution_time)
        .expect("Failed to write to file");
    writeln!(file, "Estimated memory usage: {} bytes", total_memory_usage)
        .expect("Failed to write to file");

    println!("Bubble Sort implementation in Rust took: {:?} seconds to execute", execution_time);
    println!("Estimated memory usage: {} bytes", total_memory_usage);
}
