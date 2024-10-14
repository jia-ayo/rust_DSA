pub fn fib(val: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c: i32;

    for _ in 2..=val {
        c = a + b;
        a = b;
        b = c;
    }

    return b;
}

pub fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a: Vec<char> = a.chars().collect(); //all the characters into the vextors
    let b: Vec<char> = b.chars().collect();

    let (len_a, len_b) = (a.len(), b.len());

    //solution[i][j] iss the length of the LCS (longest common susequeence)
    //between a[0..i-1] and b[0..j-1]
    let mut solution = vec![vec![0; len_b + 1]; len_a + 1];

    for (i, mi) in a.iter().enumerate() {
        for (j, mj) in b.iter().enumerate() {
            //if mi == mj, there is a new common char
            //otherwise, take the best of the two solution
            //at (i-1,j) and (i, j - 1)
            solution[i + 1][j + 1] = if mi == mj {
                solution[i][j] + 1
            } else {
                solution[i][j + 1].max(solution[i + 1][j])
            }
        }
    }
    //reconsitute the solution from the length
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

pub fn max_subarray(array: &[i32]) -> i32 {
    let mut tmp = vec![0; array.len()];
    tmp[0] = array[0];
    let mut result = tmp[0];

    for i in 1..array.len() {
        if tmp[i - 1] > 0 {
            tmp[i] = tmp[i - 1] + array[i]
        } else {
            tmp[i] = array[i]
        }
        result = result.max(tmp[i]);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fib() {
        assert_eq!(fib(9), 34)
    }

    #[test]
    fn test_lcs() {
        //empty test case
        assert_eq!(longest_common_subsequence("", ""), "");
        assert_eq!(longest_common_subsequence("", "abcd"), "");
        assert_eq!(longest_common_subsequence("abcd", ""), "");

        assert_eq!(longest_common_subsequence("abcd", "a"), "a");
        assert_eq!(longest_common_subsequence("abcd", "b"), "b");
        assert_eq!(longest_common_subsequence("rice", "rce"), "rce");

        assert_eq!(longest_common_subsequence("aggtab", "gxtxayb"), "gtab");
    }

    #[test]
    fn test_max_subarray(){
      let array = vec![1, 0, 5, 8]; // 1 + 0 + 5 + 8
      assert_eq!(max_subarray(&array), 14);

      let array2 = vec![-3, -1, -8, -2]; // -1
      assert_eq!(max_subarray(&array2), -1);
      
      let array3 = vec![-4, 3, -2, 5, -3]; // 3 -2 + 5
      assert_eq!(max_subarray(&array3), 6);

    
    }
}
