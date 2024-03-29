pub mod plug;
pub mod classes;
pub mod vec;
pub mod sum;
pub mod product;
pub mod option;
pub mod result;

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

fn int_to_conc_result_string(i: i32) -> Concrete<Result<forall_t, String>, String> {
    if i % 2 == 0 {
        Concrete::of(Ok("even".to_string()))
    } else {
        Concrete::of(Ok("odd".to_string()))
    }
}


fn main() {

    let fn_foldr = |x, y| x + y;
    let fn_to_monoid = |x: i32| ProductMonoid::new(x);
    let foldable_res_ok: Concrete<Result<forall_t, String>, i32> = Concrete::of(Ok(42));
    let foldable_res_err: Concrete<Result<forall_t, String>, i32> = Concrete::of(Err("No int".to_string()));

    let foldr_ok: i32 = Foldable::foldr(fn_foldr.clone(), 100, foldable_res_ok.clone());
    let foldr_err: i32 = Foldable::foldr(fn_foldr.clone(), 25, foldable_res_err.clone());

    let foldM_ok: ProductMonoid<i32> = Foldable::foldMap(fn_to_monoid.clone(), foldable_res_ok.clone());
    let foldM_err: ProductMonoid<i32> = Foldable::foldMap(fn_to_monoid, foldable_res_err);

    print!("\n");
    
    print!("foldr_ok {}\n", foldr_ok);
    print!("foldr_err {}\n", foldr_err);


    print!("nesting {:?}\n", Monoid::mappend(Foldable::foldMap(fn_to_monoid.clone(), foldable_res_ok.clone()), Foldable::foldMap(fn_to_monoid.clone(), foldable_res_ok.clone())));
    print!("foldM_ok {:?}\n", foldM_ok);
    print!("foldM_err {:?}\n", foldM_err);
   

    print!("\n");
    print!("It compiles!!!\n");
}
