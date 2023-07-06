pub fn fib(val: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c: i32;

    for _ in 2..=val {
        c = a + b;
        a = b;
        b = c
    }

    b
}

pub fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let (len_a, len_b) = (a.len(), b.len());

    let mut solution = vec![vec![0; len_b + 1]; len_a + 1];

    for (i, mi) in a.iter().enumerate() {
        for (j, mj) in b.iter().enumerate() {
            // if mi == mj, there is anew common char
            // otherwise, take the best of the two solutions
            // at (i-1,j) and (i,j-i)

            solution[i + 1][j + 1] = if mi == mj {
                solution[i][j] + 1
            } else {
                solution[i][j + 1].max(solution[i + 1][j])
            }
        }
    }

    let mut result: Vec<char> = Vec::new();
    let (mut i, mut j) = (len_a, len_b);

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if solution[i - 1][j] > solution[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result.iter().collect()
}

pub fn maximum_subarray(arr: &[i32]) -> i32 {
    let mut tmp = vec![0; arr.len()];
    tmp[0] = arr[0];
    let mut result = tmp[0];

    for i in 1..arr.len() {
        if tmp[i - 1] > 0 {
            tmp[i] = tmp[i - 1] + arr[i];
        } else {
            tmp[i] = arr[i];
        }

        result = result.max(tmp[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(9), 34);
    }

    #[test]
    fn test_lcs() {
        assert_eq!(longest_common_subsequence("", ""), "");
        assert_eq!(longest_common_subsequence("", "acdb"), "");
        assert_eq!(longest_common_subsequence("asfg", ""), "");

        assert_eq!(longest_common_subsequence("abcd", "a"), "a");
        assert_eq!(longest_common_subsequence("adbc", "b"), "b");
        assert_eq!(longest_common_subsequence("qwerty", "rty"), "rty");

        assert_eq!(longest_common_subsequence("abcdgh", "aedfhr"), "adh");
        assert_eq!(longest_common_subsequence("aggtab", "gxtxayb"), "gtab");
    }

    #[test]
    fn test_max_subarray() {
        assert_eq!(maximum_subarray(&[1, 0, 5, 8]), 14);
        assert_eq!(maximum_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
