// this is insertion sort implementation
// it is a stable sort

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}
