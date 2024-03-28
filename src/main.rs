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
    
    let sum1: SumMonoid<i32> = SumMonoid::new(1);
    let sum20: SumMonoid<i32> = SumMonoid::new(20);

    let res_sumM1: Concrete<Result<forall_t, String>, SumMonoid<i32>> = Concrete::of(Ok(sum1.clone()));
    let res_sumM20: Concrete<Result<forall_t, String>, SumMonoid<i32>> = Concrete::of(Ok(sum20.clone()));
    let res_sumErr: Concrete<Result<forall_t, String>, SumMonoid<i32>> = Concrete::of(Err("this non-defaul err".to_string()));

    let ans1 = Monoid::mappend(res_sumM1.clone(), res_sumM20.clone());
    let ans2= Monoid::mappend(Monoid::mempty(), res_sumM20.clone());
    let ans3 = Monoid::mappend(res_sumM1.clone(), Monoid::mempty());
    let ans4= Monoid::mappend(res_sumErr.clone(), res_sumM20.clone());
    let ans5 = Monoid::mappend(res_sumM1.clone(), res_sumErr.clone());

    print!("\n");
    
    print!("m1 <> m2 {:?}\n", ans1.unwrap);
    print!("mempt <> m2 {:?}\n", ans2.unwrap);
    print!("m1 <> mempty {:?}\n", ans3.unwrap);
    print!("Err <> m2 {:?}\n", ans4.unwrap);
    print!("m1 <> Err {:?}\n", ans5.unwrap);
    
    

    print!("\n");
    print!("It compiles!!!\n");
}

