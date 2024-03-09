pub mod plug;
pub mod classes;
mod vec;

use plug::{Concrete, Unplug, Plug, forall_t};
use classes::{Monoid, Functor, Applicative, Monad, Foldable};

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
    let fn_fn = |x: i32| int_to_string(x);
    let fn_fn2 = |x: i32| int_to_conc_string(x);
    let closure = |x: i32| x * 2;
    let closure2 = |x: i32| x * 10;
    let closure3 = |x: i32, y: i32| x * y;
    let fn_pointer: fn(i32) -> i32 = closure as fn(i32) -> i32;
    let test_returns: Concrete<Vec<forall_t>, i32> = Monad::returns(6);
    let test_memtpy: Concrete<Vec<forall_t>, i32> = Monoid::mempty(); 
    let mappend1: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![10,11,12]);
    let mappend2: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![13,14,15]);
    let mappendchar: Concrete<Vec<forall_t>, char> = Concrete::of(vec!['a', 'b', 'c']);
    let mappend_test = Monoid::mappend(mappend1, mappend2);
    println!("{}\n", fn_pointer(5));
    
    let conc_vec_fn_pointer = Concrete::of(vec![fn_fn]);
    let conc_vec1_i32: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![1,2,3,4]);
    let conc_vec2_i32: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![1,3,2,4]);
    let conc_vec3_i32: Concrete<Vec<forall_t>, i32> = Concrete::of(vec![2,2,2]);

    let answer1 = Applicative::app(conc_vec_fn_pointer, conc_vec1_i32);
    let answer2 = Monad::bind(fn_fn2, conc_vec2_i32);
    let answer3 = Foldable::foldr(closure3, 2, conc_vec3_i32);

    print!("mempty :: {:?}\n", test_memtpy.unwrap);
    print!("mapped :: {:?}\n", mappend_test.unwrap);
    print!("returns :: {:?}\n", test_returns.unwrap);
    print!("foldable :: {}\n", answer3);
    print!("vec1 :: {:?}\n", answer1.unwrap);
    print!("vec2 :: {:?}\n", answer2.unwrap);
    print!("It compiles!!!\n");
}
