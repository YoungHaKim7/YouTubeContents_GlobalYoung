use std::marker::PhantomData;

struct Zero;
struct Succ<N>(PhantomData<N>);

// Type-level natural numbers: map each type to its numeric value.
trait Peano {
    const VALUE: usize;
}

impl Peano for Zero {
    const VALUE: usize = 0;
}

impl<N: Peano> Peano for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

impl std::fmt::Debug for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Peano>::VALUE)
    }
}

impl<N: Peano> std::fmt::Debug for Succ<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Peano>::VALUE)
    }
}

fn main() {
    let one: Succ<Zero> = Succ(PhantomData);
    let two: Succ<Succ<Zero>> = Succ(PhantomData);
    let three: Succ<Succ<Succ<Zero>>> = Succ(PhantomData);

    println!("Type theory ( basic 001) 1+1+1");

    println!("one : {one:?}");
    println!("two : {two:?}");
    println!("three : {three:?}");
}
