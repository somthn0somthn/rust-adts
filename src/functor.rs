use crate::monoid::{Monoid, MonoidDef};
use crate::semigroup::{Semigroup, SemigroupDef};
use std::cell::RefCell;
use core::ops::Add;

//You could also go this way with associated types instead:
/* 
pub trait FunctorDef<T1> {
    type Output<TE>;  // Associated type representing the output container

    fn fmap_def<F, TE>(&self, f: F) -> Self::Output<TE>
    where
        F: Fn(&T1) -> TE;
} */

//can you change this to FunctoDef<T1, Vec<TE>>??

pub trait FunctorDef<T1, T2,  TE> {
    fn fmap_def<F>(&self, f: F) -> TE
    where
        F: Fn(&T1) -> T2;
}

pub trait Functor<T1, T2, TE> {
    fn fmap<F>(&self, f: F) -> TE
    where
        F: Fn(&T1) -> T2;
}

impl<T, T1, T2, TE> Functor<T1, T2, TE> for T
where
    T: FunctorDef<T1, T2, TE>,
{
    fn fmap<F>(&self, f: F) -> TE
    where
        F: Fn(&T1) -> T2,
    {
        T::fmap_def(self, f)
    }
}

impl FunctorDef<char, char, String> for String {
    fn fmap_def<F>(&self, f: F) -> String
    where
        F: Fn(&char) -> char,
    {
        self.chars().map(|c| f(&c)).collect()
    }
}

impl FunctorDef<char, char, String> for char {
    fn fmap_def<F>(&self, f: F) -> String
    where
        F: Fn(&char) -> char,
    {
        let result = f(self);
        result.to_string()
    }
}

impl FunctorDef<char, char, String> for &str {
    fn fmap_def<F>(&self, f: F) -> String
    where
        F: Fn(&char) -> char,
    {
        let a = self.to_string();
        a.chars().map(|c| f(&c)).collect()
    }
}

impl<T1, T2> FunctorDef<T1, T2, Vec<T2>> for Vec<T1> {
    fn fmap_def<F>(&self, f: F) -> Vec<T2>
    where
        F: Fn(&T1) -> T2,
    {
        self.iter().map(|x| f(x)).collect()
    }
}

impl<T1: Clone, T2> FunctorDef<T1, T2, Vec<T2>> for &[T1] {
    fn fmap_def<F>(&self, f: F) -> Vec<T2> 
    where
        F: Fn(&T1) -> T2 
    {
        self.iter().map(f).collect()
    }
}

impl<T1, T2, U: Clone> FunctorDef<T1, T2, (U, T2)> for (U, T1) {
    fn fmap_def<F>(&self, f: F) -> (U, T2) 
    where
        F: Fn(&T1) -> T2 
    {
        let val = &self.1;
        (self.0.clone(), f(val))
    }
}

//helpers => move to utils.rs later

fn increment_char(c: char) -> char {
    match c {
        'a'..='y' | 'A'..='Y' => ((c as u8) + 1) as char,
        'z' => 'A',
        'Z' => 'a',
        _ => c,
    }
}

fn char_to_upper(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}

fn char_to_lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

fn add_five<T>(num: T) -> T 
where 
    T: Add<Output = T> + From<u8> + Copy
{
    num + 5.into()
}

 

mod tests {
    use super::*;

    #[test]
    fn always_pass() {
        let a: char = 'a';
        assert_eq!(a, 'a');
    }

    #[test]
    fn fmap_string() {
        let a = "hello".to_string();
        let b = "HELLO".to_string();
        assert_eq!(a.fmap(|c| char_to_upper(*c)), b)
    }

    #[test]
    fn fmap_char() {
        let a = 'A';
        let aa = 'a'.to_string();
        assert_eq!(a.fmap(|c| char_to_lower(*c)), aa)
    }

    #[test]
    fn fmap_string_slice() {
        let a = "Hello, World!";
        let result = "Ifmmp, Xpsme!".to_string();
        assert_eq!(a.fmap(|c| increment_char(*c)), result)
    }

    #[test]
    fn fmap_vec_u8() {
        let vect: Vec<u8> = vec![1,3,5,7];
        let result: Vec<u8> = vec![6, 8, 10, 12];
        assert_eq!(vect.fmap(|x| add_five(*x)), result);
    }

    #[test]
    fn fmap_slice_u8() {
        let vect: &[u8] = &[1,3,5,7];
        let result: Vec<u8> = vec![6, 8, 10, 12];
        assert_eq!(vect.fmap(|x| add_five(*x)), result);
    }

    #[test]
    fn fmap_tuple_u32() {
        let tup: (&str, u32) = ("yo", 18);
        let result: (&str, u32) = ("yo", 23);
        assert_eq!(tup.fmap(|x| add_five(*x)), result);
    }
}