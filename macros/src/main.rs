use art_macros::debug_print;
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

macro_rules! average {
    ($(,)*) => {
        0.0
    };
    ($($val: expr), + $(,)*) => {{
        let count = 0usize $(+ {let _ = stringify!($val); 1})*;
        let sum = 0.0 $(+ $val as f64)*;

        sum / count as f64
    }}
}

macro_rules! op {
    ($a: expr, $b: expr, $op: expr) => {
        match $op {
            1 => $a + $b,
            2 => $a - $b,
            3 => $a * $b,
            4 => $a / $b,
            5 => $a % $b,
            _ => -1,
        }
    };
}

#[debug_print]
fn main() {
    let test = gcd!(14, 17);
    println!("{}", test);
    println!("{}", average!(1.0, 2.0, 3.0));
    println!("{}", average!(1, 2, 3, 4, 5));

    println!("{}", op!(5, 2, 1));
    println!("{}", op!(5, 2, 2));
    println!("{}", op!(5, 2, 3));
    println!("{}", op!(5, 2, 4));
    println!("{}", op!(5, 2, 5));
    println!("{}", op!(5, 2, 6));
}
