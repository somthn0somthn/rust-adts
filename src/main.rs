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
    let start: Concrete<Result<forall_t, String>, i32> = Concrete::of(Ok(53));
    let start_err: Concrete<Result<forall_t, String>, i32> = Concrete::of(Err("Error".to_string()));
    let fn_f= |x| int_to_conc_result_string(x);
    
    let mon_ret_res: Concrete<Result<forall_t, String>, i32> = Monad::returns(1965);
    let mon_bind_ok = Monad::bind(fn_f.clone(), start.clone());
    let mon_bind_err = Monad::bind(fn_f.clone(), start_err.clone());
    
    print!("\n");
    
    print!("returns {:?}\n", mon_ret_res.unwrap);
    print!("bind ok {:?}\n", mon_bind_ok.unwrap);
    print!("bind err {:?}\n", mon_bind_err.unwrap);

    print!("\n");
    print!("It compiles!!!\n");
}

