fn quick_sort <T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr.len() / 2;
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        while arr[left] < arr[pivot] {
            left += 1;
        }
        while arr[right] > arr[pivot] {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot..]);
}

fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}
