use std::ops::Add;

pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

impl<T: Add<Output = T> + Copy> Semigroup for T {
    fn combine(&self, other: &Self) -> Self {
        *self + *other
    }
}


//This doesn't consider overflows at all - and I'm not sure that defining 
//combine as addition is actually proper so will see if this changes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_combine() {
        let a = 5;
        let b = 7;
        assert_eq!(a.combine(&b), 12);
    }

    #[test]
    fn i64_combine() {
        let a = 50000;
        let b = 7453;
        assert_eq!(a.combine(&b), 57453);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_isize_32_combine() {
        // Testing for 32-bit architecture
        let a: isize = 1_000_000;
        let b: isize = 1_000_000;
        assert_eq!(a.combine(&b), 2_000_000);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_isize_64_combine() {
        // Testing for 64-bit architecture
        let a: isize = 4_000_000_000_000;
        let b: isize = 4_000_000_000_000;
        assert_eq!(a.combine(&b), 8_000_000_000_000);
    }
}
