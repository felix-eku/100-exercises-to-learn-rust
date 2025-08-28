// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let (v1, v2) = v.split_at(v.len() / 2);
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();
    let handle1 = thread::spawn(move || v1.iter().sum());
    let handle2 = thread::spawn(move || v2.iter().sum());
    let s1: i32 = handle1.join().expect("Thread returns partial sum");
    let s2: i32 = handle2.join().expect("Thread returns partial sum");
    s1 + s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
