#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::mat_mult;
use problem3::sieve;
use problem4::{hanoi, Peg};

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

//
// Problem 2
//

#[test]
#[should_panic]
fn test_mat_mult_panic() {
    let mat1 = vec![vec![0.;3]; 3];
    let mat2 = vec![vec![5.;3]; 2];
    mat_mult(&mat1, &mat2);
}

//
// Problem 3
//

#[test]
fn test_sieve_empty() {
    assert_eq!(Vec::<u32>::new(), sieve(2));
}

//
// Problem 4
//

#[test]
fn test_hanoi_2_disks() {
    let result = hanoi(2, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::B), (Peg::A, Peg::C), (Peg::B, Peg::C)], result);
    assert_eq!(3, result.len());
}
