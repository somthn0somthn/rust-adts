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

fn main() {
    let f_fn = |x: i32| x * 10;
    let conc_appl_opt = Concrete::of(Some(f_fn));
    let conc_opt = Concrete::of(Some(2));
    let conc_opt_none: Concrete<Option<forall_t>, i32> = Concrete::of(None);
    let answer1 = Applicative::app(conc_appl_opt.clone(), conc_opt);
    let answer1_none = Applicative::app(conc_appl_opt, conc_opt_none);

    let fn_fn2 = |x: i32| int_to_string(x);
    let conc_appl_opt2 = Concrete::of(Some(fn_fn2));
    let conc_opt2 = Concrete::of(Some(3));
    let conc_opt2_none: Concrete<Option<forall_t>, i32> = Concrete::of(None);
    let answer2: Concrete<Option<forall_t>, String> = Applicative::app(conc_appl_opt2.clone(), conc_opt2);
    let answer2_none = Applicative::app(conc_appl_opt2, conc_opt2_none);

    let opt_pure: Concrete<Option<forall_t>, i32> = Applicative::pure(100);

    print!("\n");

    print!("{:?}\n", answer1.unwrap);
    print!("{:?}\n", answer1_none.unwrap);
    print!("{:?}\n", answer2.unwrap);
    print!("{:?}\n", answer2_none.unwrap);
    print!("{:?}\n", opt_pure.unwrap);
    print!("It compiles!!!\n");
}
