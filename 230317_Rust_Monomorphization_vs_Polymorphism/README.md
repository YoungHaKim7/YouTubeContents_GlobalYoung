# Monomorphization vs Polymorphism

- Monomorphization

     This approach is extremely performant (in Rust this is known as a “zero-cost abstraction”) - however, due to monomorphization, this does create a larger binary size.

- Polymorphism

    This cuts down on binary size (as no monomorphization is used here) but incurs a performance penalty due to the extra lookup at runtime. This approach also explicitly forbids the use of generics.


```mermaid

timeline
    title Monomorphism vs Polymorphism
    Code : cargo run
    Compile Time : Compile time
         : Monomorphization
         : Static Dispatch
         : T<br>(Using Generic Types in Rust)
         : Function , Struct, Enum, Method Definitions
         : larger binary size.
    RunTime : Run time
         : Polymorphism
         : Dynamic Dispatch
         : dyn
         : Trait Objects
         : This cuts down on binary size<br>but incurs a performance penalty due to the extra lookup at runtime.

```


# Monomorphism 설명

- https://oswalt.dev/2021/06/using-generic-types-in-rust/

- https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics

- https://en.wikipedia.org/wiki/Monomorphization

# Polymorphism 설명

- https://oswalt.dev/2021/06/polymorphism-in-rust/

- https://en.wikipedia.org/wiki/Parametric_polymorphism