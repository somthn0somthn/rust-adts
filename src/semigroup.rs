//use std::borrow::Cow;
use std::cell::RefCell;

//should lifetimes be introduced
/* pub trait Semigroup<'a> {
    type Output;
    fn append(&'a self, other: &'a Self) -> Self::Output;
}
 */

pub trait Semigroup {
    type Output;
    fn append(&self, other: &Self) -> Self::Output;
}

impl Semigroup for char {
    type Output = String;

    fn append(&self, other: &Self) -> Self::Output {
        let mut result = self.to_string();
        result.push_str(&other.to_string());
        result
    }
}

impl Semigroup for &str {
    type Output = String;

    fn append(&self, other: &Self) -> Self::Output {
        let mut result = String::with_capacity(self.len() + other.len());
        result.push_str(self);
        result.push_str(other);
        result
    }
}

//using push_str in place of format!
impl Semigroup for String {
    type Output = String;

    fn append(&self, other: &Self) -> Self::Output {
        let mut result = self.clone();
        result.push_str(other);
        result
    }
}

//Does this Cow implementation make sense - perhaps make another branch and play
//around with an implementation
/* use std::borrow::Cow;

pub trait Semigroup<'a> {
    type Output;
    fn append(&'a self, other: &'a Self) -> Self::Output;
}

impl<'a> Semigroup<'a> for &'a str {
    type Output = Cow<'a, str>;

    fn append(&'a self, other: &'a Self) -> Self::Output {
        if self.is_empty() {
            Cow::Borrowed(other)
        } else if other.is_empty() {
            Cow::Borrowed(self)
        } else {
            let mut result = String::with_capacity(self.len() + other.len());
            result.push_str(self);
            result.push_str(other);
            Cow::Owned(result)
        }
    }
}
 */

impl<T> Semigroup for &[T]
where
    T: Clone,
{
    type Output = Vec<T>;

    fn append(&self, other: &Self) -> Self::Output {
        let mut result = Vec::with_capacity(self.len() + other.len());
        result.extend_from_slice(self);
        result.extend_from_slice(other);
        result
    }
}

impl<T> Semigroup for Vec<T>
where
    T: Clone,
{
    type Output = Vec<T>;

    fn append(&self, other: &Self) -> Self::Output {
        let mut result = self.clone();
        result.extend(other.clone());
        result
    }
}

impl<T, U, X, Y> Semigroup for (T, U)
where
    T: Semigroup<Output = X>,
    U: Semigroup<Output = Y>,
{
    type Output = (X, Y);

    fn append(&self, other: &Self) -> Self::Output {
        let a = self.0.append(&other.0);
        let b = self.1.append(&other.1);
        (a, b)
    }
}

//which pointer types make sense, Box, Cow, Rc?

impl<T> Semigroup for Box<T>
where
    T: Semigroup<Output = T> + Clone,
{
    type Output = Box<T>;

    fn append(&self, other: &Self) -> Self::Output {
        Box::new((**self).append(&**other))
    }
}

impl<T> Semigroup for RefCell<T>
where
    T: Semigroup<Output = T> + Clone,
{
    type Output = RefCell<T>;

    fn append(&self, other: &Self) -> Self::Output {
        let a = self.borrow().clone();
        let b = other.borrow().clone();
        let result = a.append(&b);
        RefCell::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_char() {
        let a: char = 'a';
        let b: char = 'b';
        let answer = String::from("ab");
        assert_eq!(a.append(&b), answer);
    }

    #[test]
    fn test_append_string() {
        let a: String = String::from("Hello, ");
        let b: String = String::from("world!");
        let answer = String::from("Hello, world!");
        assert_eq!(a.append(&b), answer);
    }

    #[test]
    fn test_append_slice() {
        let a: &str = "Hello, ";
        let b: &str = "world!";
        let answer: String = String::from("Hello, world!");
        assert_eq!(a.append(&b), answer);
    }

    #[test]
    fn test_append_array_uint() {
        let a: &[u8] = &[1, 2, 3];
        let b: &[u8] = &[4, 5, 6, 7];
        let result: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.append(&b), result)
    }

    #[test]
    fn test_append_array_slice() {
        let a: &[&str] = &["a", "b", "c"][..];
        let b: &[&str] = &["d", "e", "f", "g"][..];
        let result: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g"];
        assert_eq!(a.append(&b), result)
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
    fn test_append_tuple_str() {
        let a: (&str, &str) = ("Hello, ", "Goodbye, ");
        let b: (&str, &str) = ("world!", "world!");
        let result: (String, String) = (
            String::from("Hello, world!"),
            String::from("Goodbye, world!"),
        );
        assert_eq!(a.append(&b), result)
    }

    #[test]
    fn test_append_box_vec_i8() {
        let a: Box<Vec<i8>> = Box::new(vec![1, 2, 3]);
        let b: Box<Vec<i8>> = Box::new(vec![4, 5, 6, 7]);
        let result: Box<Vec<i8>> = Box::new(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(a.append(&b), result)
    }

    #[test]
    fn test_append_refcell_vec_str() {
        let a: RefCell<Vec<&str>> = RefCell::new(vec!["cat", "dog"]);
        let b: RefCell<Vec<&str>> = RefCell::new(vec!["hamster", "pig", "cow"]);
        let result: RefCell<Vec<&str>> = RefCell::new(vec!["cat", "dog", "hamster", "pig", "cow"]);
        assert_eq!(a.append(&b), result)
    }
}
