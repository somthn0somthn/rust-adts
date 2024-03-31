pub mod plug;
pub mod classes;
pub mod vec;
pub mod sum;
pub mod product;
pub mod option;
pub mod result;

use plug::{Concrete, Unplug, Plug, forall_t, Wrapper};
use classes::{Monoid, Functor, Applicative, Monad, Foldable, Traversable};
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

    let vec_res: Concrete<Vec<forall_t>, Result<i32, String>> = Concrete::of(vec![Ok(1), Ok(2), Ok(3)]);
    let vec_opt: Concrete<Vec<forall_t>, Option<i32>> = Concrete::of(vec![Some(7), Some(8), Some(9)]);
    let res: Concrete<Option<forall_t>, Vec<i32>> = Traversable::sequence(vec_opt);
    let res1 = Traversable::sequence(vec_res);

    let opt_vec: Concrete<Option<forall_t>, Vec<i32>> = Concrete::of(Some(vec![4, 5, 6]));
    let opt_vec_none: Concrete<Option<forall_t>, Vec<i32>> = Concrete::of(None);;

    let res2: Concrete<Vec<forall_t>, Option<i32>> = Traversable::sequence(opt_vec);
    let res3: Concrete<Vec<forall_t>, Option<i32>> = Traversable::sequence(opt_vec_none);

    let opt_res: Concrete<Option<forall_t>, Result<i32, String>> = Concrete::of(Some(Ok(100)));
    let opt_res_err: Concrete<Option<forall_t>, Result<i32, String>> = Concrete::of(Some(Err("Oh no".to_string())));
    let opt_res_none: Concrete<Option<forall_t>, Result<i32, String>> = Concrete::of(None);

    let res4: Concrete<Result<forall_t, String>, Option<i32>> = Traversable::sequence(opt_res);
    let res5: Concrete<Result<forall_t, String>, Option<i32>> = Traversable::sequence(opt_res_err);
    let res6: Concrete<Result<forall_t, String>, Option<i32>> = Traversable::sequence(opt_res_none);

    let res_opt: Concrete<Result<forall_t, String>, Option<i32>> = Concrete::of(Ok(Some(239)));
    let res_opt_none: Concrete<Result<forall_t, String>, Option<i32>> = Concrete::of(Ok(None));
    let res_opt_err: Concrete<Result<forall_t, String>, Option<i32>> = Concrete::of(Err("Whoops".to_string()));

    let res7: Concrete<Option<forall_t>, Result<i32, String>> = Traversable::sequence(res_opt);
    let res8: Concrete<Option<forall_t>, Result<i32, String>> = Traversable::sequence(res_opt_none);
    let res9: Concrete<Option<forall_t>, Result<i32, String>> = Traversable::sequence(res_opt_err);

    
    print!("\n");

    print!("trav::seq {:?}\n", res.unwrap);
    print!("trav::seq {:?}\n\n", res1.unwrap);
    print!("trav::seq {:?}\n", res2.unwrap);
    print!("trav::seq {:?}\n\n", res3.unwrap);
    print!("trav::seq {:?}\n", res4.unwrap);
    print!("trav::seq {:?}\n", res5.unwrap);
    print!("trav::seq {:?}\n\n", res6.unwrap);
    print!("trav::seq {:?}\n", res7.unwrap);
    print!("trav::seq {:?}\n", res8.unwrap);
    print!("trav::seq {:?}\n\n", res9.unwrap);

    print!("It compiles!\n");
}
