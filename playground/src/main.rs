fn main() {
    let nums = vec![1, 3, 5, 7, 9];
    let times_10: Vec<_> = nums.iter().map(|n| n * 10).collect();
    println!("{:?}", times_10)
}
