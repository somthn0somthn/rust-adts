use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Functor, Applicative, Monad, Foldable};

impl<A, B> Plug<A> for Vec<B> {
    type result_t = Vec<A>;
}

impl<A> Unplug for Vec<A> {
    type F = Vec<forall_t>;
    type A = A;
}

impl<A:Clone> Monoid for Concrete<Vec<forall_t>, A> {
    fn mempty() -> Self {
        Concrete::of(vec![])
    }
    fn mappend(a:Self, b:Self) -> Self {
        let res: Vec<A> = [a.unwrap, b.unwrap].concat();
        Concrete::of(res)
    }
}

impl<A> Functor for Concrete<Vec<forall_t>, A> {
    fn map<F, B>(f: F, s: Self) -> <Self as Plug<B>>::result_t
    where
        F: FnMut(<Self as Unplug>::A) -> B,
    {
        Concrete::of(s.unwrap.into_iter().map(f).collect())
    }
}

impl<A:Clone> Applicative for Concrete<Vec<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(vec![a])
    }
    fn app<B, F>(
        fs:<Self as Plug<F>>::result_t,
        s:Self
    ) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
    {
        let flat:Vec<B> =
            Functor::map(|x|
                Functor::map(|f|
                    f.clone()(x.clone()),
                fs.clone()),
            s)
            .unwrap
            .into_iter()
            .map(|x|x.unwrap)
            .flatten()
            .collect();
        Concrete::of(flat)
    }
}

impl<A:Clone> Monad for Concrete<Vec<forall_t>, A> {
    fn returns(a:A) -> Self {
        Concrete::of(vec![a])
    }
    fn bind<G, B>(g:G, s:Self) -> <Self as Plug<B>>::result_t
    where
        G:FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
        {
            let res:Vec<B> =
                s.unwrap
                .into_iter()
                .map(|x|g.clone()(x).unwrap)
                .flatten()
                .collect();
            Concrete::of(res)
        }
}

impl<A:Clone> Foldable for Concrete<Vec<forall_t>, A> {
    fn foldr<G>(g:G, a:<Self as Unplug>::A, s:Self) -> <Self as Unplug>::A
    where
        G: FnMut(<Self as Unplug>::A, <Self as Unplug>::A) -> <Self as Unplug>::A + Clone
        {
            let answer =
                s.unwrap
                .into_iter()
                .fold(a, |acc, x| g.clone()(acc, x));
            answer
        }
}
