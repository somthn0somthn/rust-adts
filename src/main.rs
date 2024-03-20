pub mod plug;
pub mod classes;
pub mod vec;
pub mod sum;
pub mod product;
pub mod option;

use plug::{Concrete, Unplug, Plug, forall_t, Wrapper};
use classes::{Monoid, Functor, Applicative, Monad, Foldable};
use sum::SumMonoid;
use product::ProductMonoid;

//TODO: move these into utils.rs or helpers.rs later
fn functor_test<M: Functor, A, B, C>(
    functor: M,
    fun0: impl Fn(A) -> B,
    fun1: impl Fn(B) -> C,
) -> <M as Plug<C>>::result_t
where
    M: Plug<A> + Plug<B> + Plug<C> + Unplug<A = A>,
    <M as Unplug>::F: Plug<A> + Plug<B> + Plug<C>,
{
    let cmp = |x| fun1(fun0(x));
    Functor::map(cmp, functor)
}

fn int_to_string(i: i32) -> String {
    if i % 2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}

fn int_to_conc_string(i: i32) -> Concrete<Vec<forall_t>, String> {
    if i % 2 == 0 {
        Concrete::of(vec!["even".to_string()])
    } else {
        Concrete::of(vec!["odd".to_string()])
    }
}

fn int_to_conc_opt_string(i: i32) -> Concrete<Option<forall_t>, String> {
    if i % 2 == 0 {
        Concrete::of(Some("even".to_string()))
    } else {
        Concrete::of(Some("odd".to_string()))
    }
}

fn main() {
    let opt_returns: Concrete<Option<forall_t>, char> = Monad::returns('a');
    let fn_monadic = |x: i32| int_to_conc_opt_string(x);
    let opt_some: Concrete<Option<forall_t>, i32> = Concrete::of(Some(420));
    let opt_none: Concrete<Option<forall_t>, i32> = Concrete::of(None);

    let answer1 = Monad::bind(fn_monadic.clone(), opt_some);
    let answer2 = Monad::bind(fn_monadic, opt_none);
    
  
    print!("\n");

    print!("returns :: {:?}\n", opt_returns.unwrap);
    print!("opt_some :: {:?}\n", answer1.unwrap);
    print!("opt_none :: {:?}\n", answer2.unwrap); 

    print!("It compiles!!!\n");
}

