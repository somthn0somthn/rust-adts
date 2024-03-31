use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Foldable, Functor, Applicative, Monad, Traversable};

impl<A, E> Unplug for Result<A, E> {
    type F = Result<forall_t, E>;
    type A = A;

}

impl<A, B, E> Plug<A> for Result<B, E> {
    type result_t = Result<A, E>;
}

impl<A: Clone + Monoid, E: Default> Monoid for Concrete<Result<forall_t, E>, A> {
    fn mempty() -> Self {
        Concrete::of(Err(E::default()))
    }
    fn mappend(a: Self, b: Self) -> Self {
        let res = match (a.unwrap, b.unwrap) {
            (Ok(x), Ok(y)) => Ok(Monoid::mappend(x, y)),
            (Err(_), Ok(y)) => Ok(y),
            (Ok(x), Err(_)) => Ok(x),
            (Err(_), Err(_)) => Err(E::default()),
        };
        Concrete::of(res)
    }
}

impl<A: Clone, E> Foldable for Concrete<Result<forall_t, E>, A> {
    fn foldr<G>(g: G, a: <Self as Unplug>::A, s: Self) -> <Self as Unplug>::A 
    where 
        G: FnMut(<Self as Unplug>::A, <Self as Unplug>::A) -> <Self as Unplug>::A + Clone,
    {
        let res = match s.unwrap {
            Ok(val) => g.clone()(val, a),
            Err(err) => a,
        };
        res
    }

    fn foldMap<G, F>(g: G, s:Self) -> F
    where 
        G:FnMut(<Self as Unplug>::A) -> F + Clone,
        F: Monoid,
    {
        let res = match s.unwrap {
            Ok(val) => g.clone()(val),
            Err(_) => Monoid::mempty()
        };
        res
    }
}

impl<B: Clone, E:Default> Traversable for Concrete<Result<forall_t, E>, Vec<B>> {
    type Output = Concrete<Vec<forall_t>, Result<B, E>>;

    fn sequence(self) -> Self::Output {
        let res = match self.unwrap {
            Ok(val) => Functor::map(|x| Ok(x), Concrete::of(val)),
            Err(_) => Concrete::of(Vec::default()),
        };
        res
    }
}

impl<B: Clone, E: Default> Traversable for Concrete<Result<forall_t, E>, Option<B>> {
    type Output = Concrete<Option<forall_t>, Result<B, E>>;
    
    fn sequence(self) -> Self::Output {
        let res = match self.unwrap {
            Ok(val) => Functor::map(|x| Ok(x), Concrete::of(val)),
            Err(_) => Concrete::of(None),
        };
        res
    }
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

impl<A: Clone, E> Monad for Concrete<Result<forall_t, E>, A> {
    fn returns(a: A) -> Self {
        Concrete::of(Ok(a))
    }
    fn bind<G, B>(g: G, s: Self) -> <Self as Plug<B>>::result_t
    where
        G: FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone,
    {
        let mon_option = match s.unwrap {
            Ok(value) => g.clone()(value),
            Err(err) => Concrete::of(Err(err)),
        };
        mon_option
    }
}
