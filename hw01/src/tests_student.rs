#![cfg(test)]

use problem1::{sum, dedup, filter};
////use problem2::mat_mult;
////use problem3::sieve;
////use problem4::{hanoi, Peg};

//
// Problem 1
//

// Part 1

#[test]
fn test_sum_large() {
    let array = [10; 100];
    assert_eq!(sum(&array), 1000);
}

// Part 2

#[test]
fn test_dedup_large() {
    let vs = vec![10; 100];
    assert_eq!(dedup(&vs), vec![10]);
}

// Part 3

fn positive(x: i32) -> bool {
    x > 0
}

#[test]
fn test_filter_small() {
    let vs = vec![-2, -1, 0, 1, 2];
    assert_eq!(filter(&vs, &positive), vec![1, 2]);
}