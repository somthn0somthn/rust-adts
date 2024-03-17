use std::ops::Add;

use crate::plug::{Concrete, forall_t, Wrapper};
use crate::classes::Monoid;

#[derive(Clone, Debug)]
pub struct SumMonoid<T: Add<Output = T> + Default + Clone> {
    pub value: T,
}

impl<T: Add<Output = T> + Default + Clone> SumMonoid<T> {
    pub fn new(value: T) -> Self {
        SumMonoid { value }
    }

    // Example of an add method that combines two Sum values
    pub fn add(&self, other: &Self) -> Self {
        SumMonoid {
            value: self.value.clone() + other.value.clone(),
        }
    }
}

impl<A: Add<Output = A> + Clone + Default> Monoid for SumMonoid<A> {
    fn mempty() -> Self {
        SumMonoid{
            value: Default::default(),
        }
    }
    fn mappend(a: Self, b: Self) -> Self {
        SumMonoid {
            value: a.value + b.value
        }
    }
}

//TODO i think this would have a better home somewhere else, e.g. classes.rs
impl<A: Monoid> Monoid for Concrete<Wrapper<forall_t>, A> {
    fn mempty() -> Self {
        Concrete::of(
            Wrapper {
                value: A::mempty()
            }
        )
    }
    fn mappend(a: Self, b: Self) -> Self {
        Concrete::of(
            Wrapper {
                value: Monoid::mappend(a.unwrap.value, b.unwrap.value)
            }
        )
    }
}