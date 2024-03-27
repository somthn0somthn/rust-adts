use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Foldable, Functor, Applicative, Monad};

impl<A, E> Unplug for Result<A, E> {
    type F = Result<forall_t, E>;
    type A = A;

}

impl<A, B, E> Plug<A> for Result<B, E> {
    type result_t = Result<A, E>;
}

impl<A, E> Functor for Concrete<Result<forall_t, E>, A> {
    fn map<F, B>(f: F, s: Self) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
    {
        let mapped_result = match s.unwrap {
            Ok(value) => Ok(f.clone()(value)),
            Err(err) => Err(err),
        };

        Concrete::of(mapped_result)
    }
}

impl<A: Clone, E> Applicative for Concrete<Result<forall_t, E>, A> {
    fn pure(a: A) -> Self {
        Concrete::of(Ok(a))
    }
    fn app<B, F>(fs: <Self as Plug<F>>::result_t, s: Self) -> <Self as Plug<B>>::result_t
    where
        F: FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
        {
            let res = Functor::map(|x| Functor::map(|f| f.clone()(x.clone()), fs.clone()), s);
            let app_result = match res.unwrap {
                Ok(value) => match value.unwrap {
                    Ok(value2) => Ok(value2),
                    Err(err) => Err(err),
                }
                Err(err) => Err(err),
            };
            Concrete::of(app_result)
        }
}