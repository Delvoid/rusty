use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn b_heaps() {
    let mut heap = BinaryHeap::new();

    // We can add elements to our heap.
    heap.push(3);
    heap.push(5);
    heap.push(1);

    println!("heap {:?}", heap);
    println!("heap peek {:?}", heap.peek());

    // We can iterate over the elements in arbitrary order.
    for x in heap.iter() {
        println!("{}", x);
    }

    // We can also use `while let` to pop off one element at a time,
    // until the heap is empty.
    while let Some(x) = heap.pop() {
        println!("{}", x);
    }

    println!("reverse heap");
    reverse_heap();
}

fn reverse_heap() {
    let mut heap = BinaryHeap::new();

    // We can add elements to our heap.
    heap.push(Reverse(3));
    heap.push(Reverse(5));
    heap.push(Reverse(1));

    // We can iterate over the elements in arbitrary order.

    println!("arbitrary order");
    for x in heap.iter() {
        println!("{}", x.0);
    }

    // We can also use `while let` to pop off one element at a time,
    // until the heap is empty.
    println!("while let pop");
    while let Some(x) = heap.pop() {
        println!("{}", x.0);
    }
}
