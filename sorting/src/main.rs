fn main() {
    println!("Hello, world!");

    let mut arr = [1, 5, 3, 2, 4];

    println!("Selection sortBefore: {:?}", arr);
    selection_sort(&mut arr);

    println!("After: {:?}", arr);

    let mut arr: Vec<i32> = vec![1, 5, 3, 2, 4];

    println!("Before: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Bubble After: {:?}", arr)
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
