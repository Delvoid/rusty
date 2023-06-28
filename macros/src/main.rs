macro_rules! gcd {
    ($a: expr, $b: expr) => {{
        let mut m = $b;
        let mut n = $a;

        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }};
}
fn main() {
    let test = gcd!(14, 17);
    println!("{}", test);
}
