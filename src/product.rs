use std::ops::Mul;

use crate::classes::Monoid;

#[derive(Clone, Debug)]
pub struct ProductMonoid<T: Mul<Output =T> + MultiplicativeIdentity + Clone> {
    pub value: T
}

pub trait MultiplicativeIdentity {
    fn multiplicative_identity() -> Self;
}

//TODO can this be condensed into a macro?
impl MultiplicativeIdentity for i8 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for i16 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for i32 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for i64 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for i128 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for u8 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for u16 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for u32 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for u64 {
    fn multiplicative_identity() -> Self { 1 }
}

impl MultiplicativeIdentity for u128 {
    fn multiplicative_identity() -> Self { 1 }
}

impl<T: Mul<Output = T> + MultiplicativeIdentity + Clone> ProductMonoid<T> {
    pub fn new(value: T) -> Self {
        ProductMonoid { value }
    }

    // Example of an add method that combines two Sum values
    pub fn mul(&self, other: &Self) -> Self {
        ProductMonoid {
            value: self.value.clone() * other.value.clone(),
        }
    }
}

impl<A: Mul<Output = A> + Clone + MultiplicativeIdentity> Monoid for ProductMonoid<A> {
    fn mempty() -> Self {
        ProductMonoid{
            value: MultiplicativeIdentity::multiplicative_identity(),
        }
    }
    fn mappend(a: Self, b: Self) -> Self {
        ProductMonoid {
            value: a.value * b.value
        }
    }
}