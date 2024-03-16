use std::ops::Add;

use crate::plug::{Concrete, Unplug, Plug, forall_t, Wrapper};
use crate::classes::{Monoid};

#[derive(Clone, Debug)]
pub struct Sum<T: Add<Output = T> + Default + Clone> {
    pub value: T,
}

impl<T: Add<Output = T> + Default + Clone> Sum<T> {
    pub fn new(value: T) -> Self {
        Sum { value }
    }

    // Example of an add method that combines two Sum values
    pub fn add(&self, other: &Self) -> Self {
        Sum {
            value: self.value.clone() + other.value.clone(),
        }
    }
}

impl<A: Add<Output = A> + Clone + Default> Monoid for Sum<A> {
    fn mempty() -> Self {
        Sum{
            value: Default::default(),
        }
    }
    fn mappend(a: Self, b: Self) -> Self {
        Sum {
            value: a.value + b.value
        }
    }
}

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