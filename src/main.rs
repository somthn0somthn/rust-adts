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

fn main() {
    let start: Concrete<Result<forall_t, String>, i32> = Concrete::of(Ok(51));
    let start_err: Concrete<Result<forall_t, String>, i32> = Concrete::of(Err("Error".to_string()));
    let fn_f = |x: i32| x * 1000;
    let fn_f_app = |x: i32| x * 253;
    let result_func: Result<fn(i32) -> i32, String> = Ok(|x| x + 253);
    let result_func2: Result<fn(i32) -> String, String> = Ok(|x: i32| "a number".to_string());
    let conc_res_app: Concrete<Result<forall_t, String>, fn(i32) -> i32> = Concrete::of(result_func);
    let conc_res_app2: Concrete<Result<forall_t, String>, fn(i32) -> String> = Concrete::of(result_func2);

    let fmap_ok = Functor::map(fn_f.clone(), start.clone());
    let fmap_err = Functor::map(fn_f, start_err.clone());

    let app_pur_res: Concrete<Result<forall_t, String>, i32> = Applicative::pure(1965);
    let app_apply_ok: Concrete<Result<forall_t, String>, i32> = Applicative::app(conc_res_app.clone(), start.clone());
    let app_apply_ok2: Concrete<Result<forall_t, String>, String> = Applicative::app(conc_res_app2.clone(), start.clone());
    let app_apply_err: Concrete<Result<forall_t, String>, i32> = Applicative::app(conc_res_app, start_err.clone());
    let app_apply_err2: Concrete<Result<forall_t, String>, String> = Applicative::app(conc_res_app2, start_err.clone());

    
    print!("\n");
    
    print!("fmap ok {:?} \n", fmap_ok.unwrap);
    print!("fmap err {:?} \n", fmap_err.unwrap);

    print!("\n");

    print!("app pure {:?}\n", app_pur_res.unwrap);
    print!("app apply ok {:?}\n", app_apply_ok.unwrap);
    print!("app apply ok change type {:?}\n", app_apply_ok2.unwrap);
    print!("app_apply err {:?}\n", app_apply_err.unwrap);
    print!("app_apply err {:?}\n", app_apply_err2.unwrap);

    print!("\n");
    print!("It compiles!!!\n");
}

