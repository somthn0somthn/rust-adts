use std::cell::RefCell;

pub trait SemigroupDef<T1, TE> {
    fn combine_def(&self, b: &T1) -> TE;
}

pub trait Semigroup<T1, TE> {
    fn combine(&self, b: &T1) -> TE;
}

impl<T, T1, TE> Semigroup<T1, TE> for T
where
    T: SemigroupDef<T1, TE>,
{
    fn combine(&self, b: &T1) -> TE {

        T::combine_def(self, b)
    }
}

impl<T1: ToString> SemigroupDef<T1, String> for char {
    fn combine_def(&self, other: &T1) -> String {  // Removed 'a: &char', replaced with &self
        format!("{}{}", self.to_string(), other.to_string())
    }
}

impl<T1: ToString> SemigroupDef<T1, String> for String {
    fn combine_def(&self, other: &T1) -> String {  
    format!("{}{}", self, other.to_string())
    }
}

impl<T1: ToString> SemigroupDef<T1, String> for &str {
    fn combine_def(&self, other: &T1) -> String {  
    format!("{}{}", self, other.to_string())
    }
}

impl<T1: Clone> SemigroupDef<&[T1], Vec<T1>> for &[T1] {
    fn combine_def(&self, other: &&[T1]) -> Vec<T1> {
        let mut result = Vec::with_capacity(self.len() + other.len());
        result.extend_from_slice(self);
        result.extend_from_slice(other);
        result
    }
} 

impl<T1: Clone> SemigroupDef<Vec<T1>, Vec<T1>> for Vec<T1> {
    fn combine_def(&self, other: &Vec<T1>) -> Vec<T1> {
        let mut result = Vec::with_capacity(self.len() + other.len());
        result.extend_from_slice(self);
        result.extend_from_slice(other);
        result
    }
}

impl<T: SemigroupDef<T, TE>, U: SemigroupDef<U,UE>, TE, UE> SemigroupDef<(T,U), (TE,UE)> for (T, U) {
    fn combine_def(&self, other: &(T, U)) -> (TE, UE) {
        let a = self.0.combine_def(&other.0);
        let b = self.1.combine_def(&other.1);
        (a, b)
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

impl<T1: Clone + SemigroupDef<T1, TE>, TE> SemigroupDef<Box<T1>, Box<TE>> for Box<T1> {
    fn combine_def(&self, other: &Box<T1>) -> Box<TE> {
        Box::new((**self).combine_def(&**other))
    }
}

impl<T1: Clone + SemigroupDef<T1, TE>, TE> SemigroupDef<RefCell<T1>, RefCell<TE>> for RefCell<T1> {
    fn combine_def(&self, other: &RefCell<T1>) -> RefCell<TE> {
        let a = self.borrow().clone();
        let b = other.borrow().clone();
        let result = a.combine(&b);
        RefCell::new(result)
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn always_pass() {
        let a: char = 'a';
        assert_eq!(a, 'a');
    }

    #[test]
    fn combine_char_and_string() {
        let a: char = 'a';
        let b: String = "".to_string();
        assert_eq!(a.combine(&b), "a".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn combine_string_and_char() {
        let a: String = "a".to_string();
        let b: char = 'b';
        assert_eq!(a.combine(&b), "ab".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn combine_slice_and_char() {
        let a: &str = "a";
        let b: char = 'b';
        assert_eq!(a.combine(&b), "ab".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn combine_char_and_slice() {
        let a: char = 'a';
        let b: &str = "b";
        assert_eq!(a.combine(&b), "ab".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn combine_string_and_slice() {
        let a: String = String::from("a");
        let b: &str = "b";
        assert_eq!(a.combine(&b), "ab".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn combine_slice_and_string() {
        let a: &str = "a";
        let b: String = "b".to_string();
        assert_eq!(a.combine(&b), "ab".to_string());  // Notice how I changed &b to b
    }

    #[test]
    fn test_combine_array_uint() {
        let a: &[u8] = &[1, 2, 3];
        let b: &[u8] = &[4, 5, 6, 7];
        let result: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_array_slice() {
        let a: &[&str] = &["a", "b", "c"][..];
        let b: &[&str] = &["d", "e", "f", "g"][..];
        let result: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g"];
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_vec_uint() {
        let a: Vec<u8> = vec![1, 2, 3];
        let b: Vec<u8> = vec![4, 5, 6, 7];
        let result: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_tuple_str() {
        let a: (&str, &str) = ("Hello, ", "Goodbye, ");
        let b: (&str, &str) = ("world!", "world!");
        let result: (String, String) = (
            String::from("Hello, world!"),
            String::from("Goodbye, world!"),
        );
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_tuple_array() {
        let a: (&[u32], &[u32]) = (&[1,2,3], &[7,6,5,4]);
        let b: (&[u32], &[u32]) = (&[4,5,6,7], &[3, 2, 1]);
        let result: (Vec<u32>, Vec<u32>) = (
            vec![1,2,3,4,5,6,7],
            vec![7,6,5,4,3,2,1]
        );
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_box_vec_i8() {
        let a: Box<Vec<i8>> = Box::new(vec![1, 2, 3]);
        let b: Box<Vec<i8>> = Box::new(vec![4, 5, 6, 7]);
        let result: Box<Vec<i8>> = Box::new(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_box_slice() {
        let a: Box<&str> = Box::new("Hello, ");
        let b: Box<&str> = Box::new("world!");
        let result: Box<String> = Box::new("Hello, world!".to_string());
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_refcell_tuple_str() {
        let a: RefCell<(&str, &str)> = RefCell::new(("cat ", "dog "));
        let b: RefCell<(&str, &str)> = RefCell::new(("hamster", "pig"));
        let result: RefCell<(String, String)> = RefCell::new(("cat hamster".to_string(), "dog pig".to_string()));
        assert_eq!(a.combine(&b), result)
    }

    #[test]
    fn test_combine_refcell_string() {
        let a: RefCell<String> = RefCell::new("Hello, ".to_string());
        let b: RefCell<String> = RefCell::new("world!".to_string());
        let result: RefCell<String> = RefCell::new("Hello, world!".to_string());
        assert_eq!(a.combine(&b), result)
    }

}
