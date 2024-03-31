use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Foldable, Functor, Applicative, Monad, Traversable};


impl<A> Unplug for Option<A> {
    type F = Option<forall_t>;
    type A = A;
}

impl<A, B> Plug<A> for Option<B> {
    type result_t = Option<A>;
}

impl<A: Clone + Monoid> Monoid for Concrete<Option<forall_t>, A> {
    fn mempty() -> Self {
        Concrete::of(None)
    }
    fn mappend(a: Self, b: Self) -> Self {
        let res = match (a.unwrap, b.unwrap) {
            (Some(x), Some(y)) => Some(Monoid::mappend(x, y)),
            (None, Some(y)) => Some(y),
            (Some(x), None) => Some(x),
            (None, None) => None,
        };
        Concrete::of(res)
    }
}

impl<A: Clone> Foldable for Concrete<Option<forall_t>, A> {
    fn foldr<G>(g: G, a: <Self as Unplug>::A, s: Self) -> <Self as Unplug>::A
    where
        G: FnMut(<Self as Unplug>::A, <Self as Unplug>::A) -> <Self as Unplug>::A + Clone,
    {
        let res = match s.unwrap {
            Some(value) => g.clone()(value, a),
            None => a,
        };
        res
    }

    fn foldMap<G, F>(g: G, s:Self) -> F
    where
        G:FnMut(<Self as Unplug>::A) -> F + Clone,
        F:Monoid,
    {
        let res = match s.unwrap {
            Some(value) => g.clone()(value),
            None => Monoid::mempty(),
        };
        res
    }
}

impl<B: Clone> Traversable for Concrete<Option<forall_t>, Vec<B>> {
    type Output = Concrete<Vec<forall_t>, Option<B>>;

    fn sequence(self) -> Self::Output {
        let res = match self.unwrap {
            Some(vec) => vec.into_iter().map(|x| Some(x)).collect(),
            None => vec![],
        };
        Concrete::of(res)
    }

}

impl<B: Clone, E: Default> Traversable for Concrete<Option<forall_t>, Result<B, E>> {
    type Output = Concrete<Result<forall_t, E>, Option<B>>;

    fn sequence(self) -> Self::Output {
        let res = match self.unwrap {
            Some(res) => Functor::map(|x| Some(x), Concrete::of(res)),
            None => Concrete::of(Err(E::default())),
        };
        res
    }
}

impl<A> Functor for Concrete<Option<forall_t>, A> {
    fn map<F, B>(f: F, s: Self) -> <Self as Plug<B>>::result_t
    where 
        F: FnMut(<Self as Unplug>::A) -> B + Clone,
    {
        let mapped_option = match s.unwrap {
            Some(value) => Some(f.clone()(value)),
            None => None,
        };

        Concrete::of(mapped_option)
    }
}

impl<A: Clone> Applicative for Concrete<Option<forall_t>, A> {
    fn pure(a: A) -> Self {
        Concrete::of(Some(a))
    }
    fn app<B, F>(fs: <Self as Plug<F>>::result_t, s: Self) -> <Self as Plug<B>>::result_t
    where  
        F: FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
        {
            let res = Functor::map(|x| Functor::map(|f| f.clone()(x.clone()), fs.clone()), s);
            let app_option = match res.unwrap {
                Some(value) => match value.unwrap {
                    Some(value2) => Some(value2), 
                    None => None,
                }
                None => None,
            };
            Concrete::of(app_option)
        }
}

impl<A: Clone> Monad for Concrete<Option<forall_t>, A> {
    fn returns(a: A) -> Self {
        Concrete::of(Some(a))
    }
    fn bind<G, B>(g: G, s: Self) -> <Self as Plug<B>>::result_t
    where
        G: FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone,
    {
        let mon_option = match s.unwrap {
            Some(value) => g.clone()(value),
            None => Concrete::of(None),
        };
        mon_option
    }
}
