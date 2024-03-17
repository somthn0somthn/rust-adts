pub mod plug;
pub mod classes;
pub mod vec;
pub mod sum;
pub mod product;

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
    let conc_sum_type1 = Concrete::of(Wrapper { value: ProductMonoid { value: 4 }});
    let conc_sum_type2 = Concrete::of(Wrapper { value: ProductMonoid { value: 5 }});
    let sum_mappend = Monoid::mappend(conc_sum_type1, conc_sum_type2); 

    let fn_to_monoid = |x: i32| ProductMonoid::new(x);
    let foldmap_vec: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![1, 2, 3, 4]);
    let foldmap_test_answer = <Concrete<Vec<forall_t>, i32>>::foldMap(fn_to_monoid, foldmap_vec);

    print!("\nproduct & product mappend :: {:?}\n", sum_mappend.unwrap.value);
    print!("foldmap using Product data constructor ::{:?}\n\n", foldmap_test_answer);
    print!("It compiles!!!\n");
}
