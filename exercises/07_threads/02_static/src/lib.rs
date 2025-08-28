use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let (slice_left, slice_right) = slice.split_at(slice.len() / 2);
    let handle_left = thread::spawn(|| slice_left.iter().sum::<i32>());
    let handle_right = thread::spawn(|| slice_right.iter().sum::<i32>());
    let s1 = handle_left.join().unwrap();
    let s2 = handle_right.join().unwrap();
    s1 + s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
