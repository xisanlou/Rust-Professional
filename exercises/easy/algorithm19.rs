/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n < 2 {
        return n;
    }

    let q: [[i32; 2]; 2] = [[1, 1], [1, 0]];
    let res = power(q, n-1);

    res[0][0]
}

fn power(a: [[i32; 2]; 2], n: i32) -> [[i32; 2]; 2] {
    let mut ret: [[i32; 2]; 2] = [[1, 0], [0, 1]];
    let mut a = a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret = multiply_matrix(ret, a);
        }
        n >>= 1;
        a = multiply_matrix(a, a);
    }
    
    ret
}

fn multiply_matrix(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result: [[i32; 2]; 2] = [[0, 0], [0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
