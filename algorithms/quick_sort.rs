fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let mut i = 0;
    for j in 1..arr.len() {
        if arr[j] < arr[0] {
            i += 1;
            arr.swap(i, j);
        }
    }
    arr.swap(0, i);
    i
}

fn main() {
    let mut arr = [5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    quicksort(&mut arr);
    println!("{:?}", arr);
}
