// This is a bubble sort algorithm implementation in Rust

fn bubble_sort <T: Ord>(arr: &mut [T]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
}
