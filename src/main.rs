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
    let sum_a: Concrete<Option<forall_t>, i32> = Concrete::of(Some( 2 )); 
    let sum_none: Concrete<Option<forall_t>, i32> = Concrete::of(None);

    let f_fn = |x: i32, y:i32| x * y;
    let f_to_mon = |x: i32| SumMonoid::new(x);
    let foldr_ans = Foldable::foldr(f_fn, 100, sum_a.clone());
    let foldr_ans2 = Foldable::foldr(f_fn, 100, sum_none.clone());

    let foldmap_ans: SumMonoid<i32> = Foldable::foldMap(f_to_mon, sum_a);
    let foldmap_ans2: SumMonoid<i32> = Foldable::foldMap(f_to_mon, sum_none);

    


    print!("\n");

    print!("foldr {:?}\n", foldr_ans);
    print!("foldr {:?}\n", foldr_ans2);

    print!("foldmap {:?}\n", foldmap_ans);
    print!("foldmap {:?}\n", foldmap_ans2); 
  

    print!("\n");
    print!("It compiles!!!\n");
}

