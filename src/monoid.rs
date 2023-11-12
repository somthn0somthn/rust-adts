use crate::semigroup::{Semigroup, SemigroupDef};

pub trait MonoidDef<T1, TE>: SemigroupDef<T1, TE> {
    fn mempty_def() -> TE;
    fn mappend_def(&self, other: &T1) -> TE;
}

pub trait Monoid<T1, TE>: Semigroup<T1, TE> {
    //type Output;
    fn mempty() -> TE;
    fn mappend(&self, other: &T1) -> TE;
}

impl<T, T1, TE> Monoid<T1, TE> for T
where
    T: MonoidDef<T1, TE>,
{
    fn mempty() -> TE {
        T::mempty_def()
    }

    fn mappend(&self, other: &T1) -> TE {
        T::mappend_def(self, other)
    }
}

impl<T1: ToString> MonoidDef<T1, String> for char {
    fn mempty_def() -> String {
        String::new()
    }

    fn mappend_def(&self, other: &T1) -> String {
        format!("{}{}", self.to_string(), other.to_string())
    }
}

impl<T1: ToString> MonoidDef<T1, String> for String {
    fn mempty_def() -> String {
        String::new()
    }

    fn mappend_def(&self, other: &T1) -> String {
        format!("{}{}", self, other.to_string())
    }
}

impl<T1: ToString> MonoidDef<T1, String> for &str {
    fn mempty_def() -> String {
        String::new()
    }

    fn mappend_def(&self, other: &T1) -> String {
        format!("{}{}", self.to_string(), other.to_string())
    }
}

/* To Implement

impl<T1: Clone> MonoidDef<Vec<T1>, Vec<T1>> for Vec<T1>

impl<T: MonoidDef<T, TE>, U: MonoidDef<U,UE>, TE, UE> MonoidDef<(T,U), (TE,UE)> for (T, U)

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

impl<T1: Clone + MonoidDef<T1, TE>, TE> MonoidDef<Box<T1>, Box<TE>> for Box<T1>

impl<T1: Clone + MonoidDef<T1, TE>, TE> MonoidDef<RefCell<T1>, RefCell<TE>> for RefCell<T1>

 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_pass() {
        let a = String::from("hello");
        let b = String::from("hello");

        assert_eq!(a, b);
    }

    #[test]
    fn test_char_mempty() {
        let b = <char as Monoid<char, String>>::mempty();
        let c = String::from("");

        assert_eq!(b, c);
    }

    #[test]
    fn test_slice_mempty() {
        let b = <&str as Monoid<&str, String>>::mempty();
        let c = String::from("");

        assert_eq!(b, c);
    }

    #[test]
    fn test_string_mempty() {
        let b = <String as Monoid<String, String>>::mempty();
        let c = String::from("");

        assert_eq!(b, c);
    }

    #[test]
    fn test_mempty_rite_identity() {
        let a = String::from("Hello");
        let b = <String as Monoid<String, String>>::mempty();

        assert_eq!(a.mappend(&b), a)
    }

    #[test]
    fn test_mempty_left_identity() {
        let a = <String as Monoid<String, String>>::mempty();
        let b = "Hello";

        assert_eq!(a.mappend(&b), b.to_string())
    }
}
