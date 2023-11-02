/* use crate::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    //type Output;
    fn mempty() -> Self::Output;
    fn mappend(&self, other: &Self) -> Self::Output;
}

impl Monoid for String {
    fn mempty() -> Self::Output {
        String::from("")
    }

    fn mappend(&self, other: &Self) -> Self::Output {
        //checks for emptiness not needed
        self.append(other)
    }
}

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
    fn mempty_test() {
        assert_eq!(String::mempty(), String::from(""));
    }

    #[test]
    fn mappend_test() {
        let a = String::from("Hello, ");
        let b = String::from("world!");

        assert_eq!(a.mappend(&b), String::from("Hello, world!"));
    }

    #[test]
    fn mempty_rite_identity_test() {
        let a = String::from("Hello");
        let b = String::mempty();

        assert_eq!(a.mappend(&b), String::from("Hello"));
    }

    #[test]
    fn mempty_left_identity_test() {
        let a = String::mempty();
        let b = String::from("Hello");

        assert_eq!(a.mappend(&b), String::from("Hello"));
    }
}
 */