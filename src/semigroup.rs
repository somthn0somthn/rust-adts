pub trait Semigroup {
    fn append(&self, other: &Self) -> Self;
}

//using push_str in place of format!
impl Semigroup for String {
    fn append(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.push_str(other);
        result
    }
}

//str slices will involve significant work

impl<T> Semigroup for Vec<T>
where
    T: Clone,
{
    fn append(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.extend(other.clone());
        result
    }
}

//which pointer types make sense, Box, Cow, Rc?

impl<T> Semigroup for Box<T> where 
T: Semigroup + Clone 
{
    fn append(&self, other: &Self) -> Self {
        Box::new((**self).append(&**other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_string() {
        let a = String::from("Hello, ");
        let b = String::from("world!");
        let answer = String::from("Hello, world!");
        assert_eq!(a.append(&b), answer);
    }

    #[test]
    fn test_append_vec_uint() {
        let a: Vec<u8> = vec![1, 2, 3];
        let b: Vec<u8> = vec![4, 5, 6, 7];
        let result: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.append(&b), result)
    }

    #[test]
    fn test_append_vec_slice() {
        let a: Vec<&str> = vec!["a", "b", "c"];
        let b: Vec<&str> = vec!["d", "e", "f", "g"];
        let result: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g"];
        assert_eq!(a.append(&b), result)
    }

    #[test]
    fn test_append_box_vec_i8() {
        let a:Box<Vec<i8>> = Box::new(vec![1, 2, 3]);
        let b:Box<Vec<i8>> = Box::new(vec![4, 5, 6, 7]);
        let result: Box<Vec<i8>> = Box::new(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(a.append(&b), result)
    }
}
