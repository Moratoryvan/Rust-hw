pub mod buffer;
pub mod compare;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let vec_i32: Vec<i32> = vec![1, 2, 3, 4, 5];
        let vec_i64: Vec<i64> = vec![-1, -2, -3, -4, -5];
        let vec_f32: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let vec_f64: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let vec_none: Vec<i32> = vec![];
        assert_eq!(buffer::Buffer::new(vec_i32).sum(), Some(15));
        assert_eq!(buffer::Buffer::new(vec_i64).sum(), Some(-15));
        assert_eq!(buffer::Buffer::new(vec_f32).sum(), Some(16.5));
        assert_eq!(buffer::Buffer::new(vec_f64).sum(), Some(15.0));
        assert_eq!(buffer::Buffer::new(vec_none).sum(), None);
    }

    #[test]
    fn test_compare() {
        assert!(compare::compare_string("abc", "ABC"));
        assert!(!compare::compare_string("abc", "abc"));
        assert!(!compare::compare_string("", "ABC"));
        assert!(compare::compare_string("Moratoryvan", "Darkholm"));
        assert!(!compare::compare_string("zxc ", "zzc"));
    }

    #[test]
    fn test_iterc() {
        let list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        let new_list: Vec<char> = list
            .into_iter()
            .map(|x: char| (x as u8 + 1) as char)
            .collect();
        assert_eq!(new_list, vec!['b', 'c', 'd', 'e', 'f']);
    }
}

fn main() {}
