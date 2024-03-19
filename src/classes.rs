use crate::plug::{Concrete, Unplug, Plug, forall_t, Wrapper};

pub trait Monoid {
    fn mempty() -> Self; //does fn mempty() -> Self make more sense
    fn mappend(a:Self, b:Self) -> Self;
}

//TODO is the Monoid constraint necessary here? Unplug constraint enough?
//TODO does foldr make sense - the returned type has no wrapper - do some testing to see what bx works
pub trait Foldable: Monoid {
    fn foldr<G>(g: G, a: <Self as Unplug>::A, s: Self) -> <Self as Unplug>::A
    where
        Self: Unplug,
        G: FnMut(<Self as Unplug>::A, <Self as Unplug>::A) -> <Self as Unplug>::A + Clone;
    fn foldMap<G, F>(g: G, s: Self) -> F
    where
        Self: Unplug + Foldable,
        G: FnMut(<Self as Unplug>::A) -> F + Clone,
        F: Monoid;
}

//TODO define Traversable

pub trait Functor: Unplug + Plug<<Self as Unplug>::A> {
    fn map<F, B>(f: F, s: Self) -> <Self as Plug<B>>::result_t
    where
        Self: Plug<B>,
        F: FnMut(<Self as Unplug>::A) -> B + Clone;
}

pub trait Applicative: Functor {
    fn pure(s:<Self as Unplug>::A) -> Self;
    fn app<B, F>(
        f:<Self as Plug<F>>::result_t, //M<F>
        s:Self                         //M<A>
    ) -> <Self as Plug<B>>::result_t   //M<B>
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
        Self:Plug<F> + Plug<B> + Unplug,
        <Self as Plug<F>>::result_t:
            Unplug<F=<Self as Unplug>::F,A=F> +
            Plug<F> +
            Clone,
        <Self as Unplug>::F:Plug<F>
    ;
}

pub trait Monad: Applicative {
    fn returns(a:<Self as Unplug>::A) -> Self;
    fn bind<G, B>(g:G, s:Self) -> <Self as Plug<B>>::result_t
    where
        Self:Plug<B>+Plug<G>+Unplug,
        G:FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
        ;
}
