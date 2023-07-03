fn main() {
    println!("Hello, world!");

    let mut arr = [1, 5, 3, 2, 4];

    println!("Selection sortBefore: {:?}", arr);
    selection_sort(&mut arr);

    println!("After: {:?}", arr);

    let mut arr: Vec<i32> = vec![1, 5, 3, 2, 4];

    println!("Before: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Bubble After: {:?}", arr);

    let mut arr: Vec<i32> = vec![10, 5, 3, 2, 4, 6, 9, 7, 8, 1];
    merge_sort(&mut arr);

    println!("Merge After: {:?}", arr);

    let mut arr: Vec<i32> = vec![10, 5, 3, 2, 4, 6, 9, 7, 8, 1];
    quick_sort(&mut arr);
    println!("Quick After: {:?}", arr);
}

fn selection_sort(arr: &mut [i32]) -> Vec<i32> {
    let len = arr.len();

    for i in 0..len {
        let mut min = i;

        for j in (i + 1)..len {
            if arr[j] < arr[min] {
                min = j;
            }
        }

        if min != i {
            arr.swap(i, min);
        }
    }
    arr.to_vec()
}

fn bubble_sort(arr: &mut [i32]) -> Vec<i32> {
    let len = arr.len();
    let mut sorted: bool;

    for i in 0..len {
        sorted = true;
        for j in (i + 1)..len {
            if arr[j] < arr[i] {
                arr.swap(i, j);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    arr.to_vec()
}

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    let len = arr.len();

    if len > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]); // left
        merge_sort(&mut arr[mid..]); // right

        merge(arr, mid)
    }

    arr.to_vec()
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}
