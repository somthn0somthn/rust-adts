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
    let sum_a: Concrete<Option<forall_t>, SumMonoid<i32>> = Concrete::of(Some( SumMonoid { value: 1})); 
    let sum_b: Concrete<Option<forall_t>, SumMonoid<i32>> = Concrete::of(Some( SumMonoid { value: 2}));
    let sum_none: Concrete<Option<forall_t>, SumMonoid<i32>> = Concrete::of(None);

    let answer: Concrete<Option<forall_t>, SumMonoid<i32>> = Monoid::mappend(sum_a, sum_b);
    let answer_mempty: Concrete<Option<forall_t>, SumMonoid<i32>> = Monoid::mempty();
    let answer2: Concrete<Option<forall_t>, SumMonoid<i32>> = Monoid::mappend(answer_mempty.clone(), answer.clone());
    let answer3: Concrete<Option<forall_t>, SumMonoid<i32>> = Monoid::mappend(answer.clone(), answer_mempty.clone());
    let answer4: Concrete<Option<forall_t>, SumMonoid<i32>> = Monoid::mappend(answer_mempty.clone(), answer_mempty.clone());


    print!("\n");

    print!("mapp {:?}\n", answer.unwrap);
    print!("mempty {:?}\n", answer_mempty.unwrap);
    print!("mapp2 {:?}\n", answer2.unwrap);
    print!("mapp3 {:?}\n", answer3.unwrap);
    print!("mapp4 {:?}\n", answer4.unwrap);

    print!("\n");
    print!("It compiles!!!\n");
}

