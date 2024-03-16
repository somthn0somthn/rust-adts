#[derive(Clone)]
pub struct forall_t;

pub trait Unplug {
    type F;
    type A;
}

pub trait Plug<A> {
    type result_t;
}

pub struct Concrete<M: Unplug + Plug<A>, A> {
    pub unwrap: <M as Plug<A>>::result_t,
}

impl<M: Unplug + Plug<A>, A> Concrete<M, A> {
    pub fn of<MA: Unplug<F = M, A = A> + Plug<A>>(x: MA) -> Self
    where
        M: Plug<A, result_t = MA>,
    {
        Concrete { unwrap: x }
    }
}

impl<M, A> Unplug for Concrete<M, A>
where
    M: Unplug + Plug<A>,
{
    type F = M;
    type A = A;
}

impl<M, A, B> Plug<A> for Concrete<M, B>
where
    M: Unplug + Plug<A> + Plug<B>,
{
    type result_t = Concrete<M, A>;
}

impl<M, A> Clone for Concrete<M, A>
where
    M: Unplug + Plug<A>,
    <M as Plug<A>>::result_t: Clone,
{
    fn clone(&self) -> Self {
        Concrete {
            unwrap: self.unwrap.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Wrapper<T> {
    pub value: T,
}

impl<A> Unplug for Wrapper<A> {
    type F = Wrapper<forall_t>;
    type A = A;
}

impl<A, B> Plug<A> for Wrapper<B> {
    type result_t = Wrapper<A>;
}
