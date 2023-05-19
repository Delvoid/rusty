mod binary_heap;
mod map;
mod set;
mod vectors;

use binary_heap::b_heaps;
use map::mappy;
use set::set_me;
use vectors::vectors;
fn main() {
    println!("Hello, world!");

    vectors();
    b_heaps();
    mappy();
    set_me();
}
