use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Functor, Applicative, Monad, Foldable, Traversable};

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

impl<A: Clone> Foldable for Concrete<Vec<forall_t>, A> {
    fn foldr<G>(g: G, a: <Self as Unplug>::A, s: Self) -> <Self as Unplug>::A
    where
        G: FnMut(<Self as Unplug>::A, <Self as Unplug>::A) -> <Self as Unplug>::A + Clone,
    {
        let answer = s.unwrap.into_iter().rev().fold(a, |acc, x| g.clone()(acc, x));
        answer
    }
    fn foldMap<G, F>(g: G, s:Self) -> F
    where
        G:FnMut(<Self as Unplug>::A) -> F + Clone,
        F: Monoid,
    {
        let answer = s.unwrap.into_iter().map(g).fold(F::mempty(), |acc, x| F::mappend(acc, x));
        answer
    }
}

impl<B: Clone> Traversable for Concrete<Vec<forall_t>, Option<B>> {
    type Output = Concrete< Option<forall_t>, Vec<B>>;

    fn sequence(self) -> Self::Output {
        let maybe_vec_b: Option<Vec<B>> = self.unwrap.into_iter().collect(); // Use Rust's built-in collect behavior for Options

        Concrete::of(maybe_vec_b)
}
}

impl<B: Clone, E: Default> Traversable for Concrete<Vec<forall_t>, Result<B,E>> {
    type Output = Concrete< Result<forall_t, E>, Vec<B>>;

    fn sequence(self) -> Self::Output {
        let maybe_vec_b: Result<Vec<B>, E> = self.unwrap.into_iter().collect(); // Use Rust's built-in collect behavior for Options

        Concrete::of(maybe_vec_b)
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
