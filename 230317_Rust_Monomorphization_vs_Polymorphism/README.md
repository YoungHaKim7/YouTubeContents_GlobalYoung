# Rust 의 다형성 - static 과 dynamic dispatch (C++ 을 곁들인..) | 출처: 모두의 코드

- Rust vs C++ 개념을 비교해서 친절히 설명해줌

  - https://modoocode.com/334

<hr>

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
    RunTime : Runtime
         : Polymorphism
         : Dynamic Dispatch
         : dyn Trait
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
  - Poly 사전적 의미 https://www.dictionary.com/browse/poly
    - a combining form with the meanings “much, many” and, in chemistry, “polymeric,” used in the formation of compound words: polyandrous; polyculture; polyethylene.

  - Morpho Definition & Meaning https://www.dictionary.com/browse/morpho

    - a combining form meaning “form, structure,” used in the formation of compound words:

# Debugging LLVM

```
$ rustc main.rs --emit=llvm-ir -O -C no-prepopulate-passes

```

https://rustc-dev-guide.rust-lang.org/backend/debugging.html

# Profile-guided Optimization

```
rustup component add llvm-tools-preview


# STEP 1: Compile the binary with instrumentation

rustc -Cprofile-generate=/tmp/pgo-data -O ./main.rs

```

https://doc.rust-lang.org/rustc/profile-guided-optimization.html

# dump-mir

```
rustc -Z dump-mir=main main.rs

```

https://rustc-dev-guide.rust-lang.org/mir/debugging.html

# Reading a .pdb file

Microsoft released the source code of their PDB formats, so other compiler developers like the LLVM team can implement the PDB format easier.

https://github.com/Microsoft/microsoft-pdb/

To actually dump the output of a file, just use this:

https://github.com/Microsoft/microsoft-pdb/blob/master/cvdump/cvdump.exe

```
cvdump Whatever.pdb

```

https://stackoverflow.com/questions/2040132/reading-a-pdb-file

# Rustup book

https://rust-lang.github.io/rustup/overrides.html

# pacak/cargo-show-asm

cargo subcommand showing the assembly, LLVM-IR and MIR generated for Rust code

https://github.com/pacak/cargo-show-asm

- Install

```
$ cargo install cargo-show-asm
```

- cargo asm

```
$ cargo asm --lib
Try one of those
"<&T as core::fmt::Display>::fmt" [17, 12, 12, 12, 12, 19, 19, 12]
"<&mut W as core::fmt::Write>::write_char" [20]
"<&mut W as core::fmt::Write>::write_fmt" [38]
"<&mut W as core::fmt::Write>::write_str" [90]
"<F as nom::internal::Parser<I,O,E>>::parse" [263]
#
```

## cargo asm --llvm-input 

```
cargo asm --llvm-input
   Compiling trait_impl v0.1.0 (/Users/globalyoung/Documents/test/test/YouTubeContents_GlobalYoung/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl)
    Finished release [optimized] target(s) in 0.29s

Try one of those by name or a sequence number
 0 "<() as std::process::Termination>::report" [6]
 1 "<trait_impl::Bear as trait_impl::Growler>::growl" [13]
 2 "<trait_impl::Lion as trait_impl::Growler>::growl" [13]
 3 "<trait_impl::Tiger as trait_impl::Growler>::growl" [13]
 4 "Function Attrs: uwtable" [23]
 5 "core::fmt::Arguments::new_v1" [60]
 6 "core::ops::function::FnOnce::call_once" [36, 8]
 8 "core::ops::function::FnOnce::call_once{{vtable.shim}}" [10]
 9 "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>" [6]
10 "std::rt::lang_start" [17]
11 "std::rt::lang_start::{{closure}}" [17]
12 "std::sys_common::backtrace::__rust_begin_short_backtrace" [22]
13 "trait_impl::main" [12]
14 "trait_impl::static_dispatch" [34, 34, 34]
```

## cargo asm --llvm

```
cargo asm --llvm
   Compiling trait_impl v0.1.0 (/Users/globalyoung/Documents/test/test/YouTubeContents_GlobalYoung/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl)
    Finished release [optimized] target(s) in 0.09s

Try one of those by name or a sequence number
0 "<trait_impl::Bear as trait_impl::Growler>::growl" [20]
1 "<trait_impl::Lion as trait_impl::Growler>::growl" [20]
2 "<trait_impl::Tiger as trait_impl::Growler>::growl" [20]
3 "Function Attrs: uwtable" [23]
4 "core::ops::function::FnOnce::call_once{{vtable.shim}}" [9]
5 "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>" [6]
6 "std::rt::lang_start" [12]
7 "std::rt::lang_start::{{closure}}" [9]
8 "std::sys_common::backtrace::__rust_begin_short_backtrace" [8]
9 "trait_impl::main" [12]
```

## cargo asm --mir

```
cargo asm --mir
   Compiling trait_impl v0.1.0 (/Users/globalyoung/Documents/test/test/YouTubeContents_GlobalYoung/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl)
    Finished release [optimized] target(s) in 0.09s

Try one of those by name or a sequence number
 0 "fn <impl at src/main.rs:13:1: 13:23>::growl(_1: &Tiger)" [63]
 1 "fn <impl at src/main.rs:20:1: 20:22>::growl(_1: &Bear)" [63]
 2 "fn <impl at src/main.rs:6:1: 6:22>::growl(_1: &Lion)" [65]
 3 "fn main()" [46]
 4 "fn static_dispatch(_1: T)" [34]
 5 "promoted[0] in <impl at src/main.rs:13:1: 13:23>::growl: &[ArgumentV1<'_>; 0]" [10]
 6 "promoted[0] in <impl at src/main.rs:20:1: 20:22>::growl: &[ArgumentV1<'_>; 0]" [10]
 7 "promoted[0] in <impl at src/main.rs:6:1: 6:22>::growl: &[ArgumentV1<'_>; 0]" [10]
 8 "promoted[1] in <impl at src/main.rs:13:1: 13:23>::growl: &[&str; 1]" [13]
 9 "promoted[1] in <impl at src/main.rs:20:1: 20:22>::growl: &[&str; 1]" [13]
10 "promoted[1] in <impl at src/main.rs:6:1: 6:22>::growl: &[&str; 1]" [13]
```

## cargo asm 기타 등등

```
cargo asm -h
Show the code rustc generates for any function

Usage: [-p SPEC] [--lib | --test TEST | --bench BENCH | --example EXAMPLE | --bin BIN] [--release | --dev | --profile PROFILE] [--target TRIPLE] -C FLAG... -Z FLAG... [--native | --target-cpu CPU] [--rust] [--simplify] -M ARG... [--intel | --att | --llvm | --llvm-input | --mir | --wasm | --mca-intel | --mca-att] [--everything | <ITEM_INDEX> | <FUNCTION> [<INDEX>]]

Usage:
  1. Focus on a single assembly producing target:
     % cargo asm -p isin --lib   # here we are targeting lib in isin crate
  2. Narrow down a function:
     % cargo asm -p isin --lib from_ # here "from_" is part of the function you are interested intel
  3. Get the full results:
     % cargo asm -p isin --lib isin::base36::from_alphanum

Available positional items:
    <ITEM_INDEX>  Dump name with this index
    <FUNCTION>    Dump function with that specific name / filter functions containing this string
    <INDEX>       Select specific function when there's several with the same name

Available options:
    -p, --package <SPEC>        Package to use, defaults to a current one, required for workspace projects, can also point
                                to a dependency
        --lib                   Show results from library code
        --test <TEST>           Show results from an integration test
        --bench <BENCH>         Show results from a benchmark
        --example <EXAMPLE>     Show results from an example
        --bin <BIN>             Show results from a binary
        --manifest-path <PATH>  Path to Cargo.toml, defaults to one in current folder
        --target-dir <DIR>      [env:CARGO_TARGET_DIR: N/A]
                                Use custom target directory for generated artifacts, create if missing
        --dry                   Produce a build plan instead of actually building
        --frozen                Requires Cargo.lock and cache are up to date
        --locked                Requires Cargo.lock is up to date
        --offline               Run without accessing the network
        --no-default-features   Do not activate `default` feature
        --all-features          Activate all available features
        --features <FEATURE>    A feature to activate, can be used multiple times
        --release               Compile in release mode (default)
        --dev                   Compile in dev mode
        --profile <PROFILE>     Build for this specific profile
        --target <TRIPLE>       Build for the target triple
    -C <FLAG>                   Codegen flags to rustc, see 'rustc -C help' for details
    -Z <FLAG>                   Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
        --native                Optimize for the CPU running the compiler
        --target-cpu <CPU>      Optimize code for a specific CPU, see 'rustc --print target-cpus'
        --rust                  Print interleaved Rust code
        --color                 Enable color highlighting
        --no-color              Disable color highlighting
        --full-name             Include full demangled name instead of just prefix
        --keep-labels           Keep all the original labels
    -v, --verbose               more verbose output, can be specified multiple times
        --simplify              Try to strip some of the non-assembly instruction information
    -M, --mca-arg <ARG>         Pass parameter to llvm-mca for mca targets
        --intel                 Show assembly using Intel style
        --att                   Show assembly using AT&T style
        --llvm                  Show llvm-ir
        --llvm-input            Show llvm-ir before any LLVM passes
        --mir                   Show MIR
        --wasm                  Show WASM, needs wasm32-unknown-unknown target installed
        --mca-intel             Show llvm-mca analysis, Intel style asm
        --mca-att               Show llvm-mca analysis, AT&T style asm
        --everything            Dump the whole asm file
    -h, --help                  Prints help information
    -V, --version               Prints version information
```

<br>

<hr>

# WindowsOS(Objdump)

https://github.com/CyberGrandChallenge/binutils/blob/master/binutils/objdump.c

https://github.com/CyberGrandChallenge/binutils

# LLVM tools(Rust)

- cargo-binutils

  - cargo-binutils[![crates.io](https://img.shields.io/crates/v/cargo-binutils.svg)](https://crates.io/crates/binutils)![Crates.io](https://img.shields.io/crates/l/binutils)![wasmtimeDownloads](https://img.shields.io/crates/d/cargo-binutils.svg)<a href="https://github.com/rust-embedded/cargo-binutils/"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
    ![druidstar](https://img.shields.io/github/stars/rust-embedded/cargo-binutils.svg)

  - <p dir="auto"><a href="https://github.com/rust-embedded/cargo-binutils/" rel="nofollow">Guides</a> | <a href="https://github.com/rust-embedded/cargo-binutils#cargo-binutils" rel="nofollow">API Docs</a></p>

  https://github.com/rust-embedded/cargo-binutils/

  https://crates.io/crates/cargo-binutils

  Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain

  - Installation

```
$ cargo install cargo-binutils

$ rustup component add llvm-tools-preview
```

https://releases.llvm.org/11.0.0/docs/CommandGuide/llvm-objdump.html

# cargo-binutils(간단한 사용법)

https://github.com/YoungHaKim7/YouTubeContents_GlobalYoung/tree/main/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl

https://crates.io/crates/cargo-binutils

- 잘정리됨
  https://github.com/rust-embedded/cargo-binutils/

-Arm Assebmly(M1 pro MacBook으로 나온 화면)

https://github.com/oowekyala/arm-cheatsheet

```
$ cargo objdump --release -- --disassemble --no-show-raw-insn | grep -A 10 -B 10 "main"

Finished release [optimized] target(s) in 0.00s
100002aa8: str xzr, [sp]
100002aac: adr x8, #196404
100002ab0: nop
100002ab4: stp x8, xzr, [sp, #32]
100002ab8: mov x0, sp
100002abc: bl 0x100017d64 <\_std::io::stdio::\_print::hf5189a9887145206>
100002ac0: ldp x29, x30, [sp, #48]
100002ac4: add sp, sp, #64
100002ac8: ret

0000000100002acc <_trait_impl::main::hc7d33c912fbb6232>:
100002acc: stp x29, x30, [sp, #-16]!
100002ad0: mov x29, sp
100002ad4: bl 0x100002a0c <_<trait*impl::Lion as trait_impl::Growler>::growl::h8fae6da4cb71fbad>
100002ad8: bl 0x100002a4c <*<trait*impl::Tiger as trait_impl::Growler>::growl::h818e891a091e4820>
100002adc: ldp x29, x30, [sp], #16
100002ae0: b 0x100002a8c <*<trait_impl::Bear as trait_impl::Growler>::growl::hd762be638330fd3d>

0000000100002ae4 <\_main>:
100002ae4: sub sp, sp, #32
100002ae8: stp x29, x30, [sp, #16]
100002aec: add x29, sp, #16
100002af0: mov x3, x1
100002af4: sxtw x2, w0
100002af8: adr x8, #-44
100002afc: nop
100002b00: str x8, [sp, #8]
100002b04: adr x1, #251716
100002b08: nop

```

# rust_웹으로 어셈블리 보기Assembly

https://rust.godbolt.org/

- 뒤에 최적화 옵션

```
-C opt-level=3 --target i686-unknown-linux-gnu
```

>>- <a href="https://github.com/YoungHaKim7/my_rust_project/tree/main/06_Rust_Optimization#rustc">target list </a>
>>```
>>$ rustc --print target-list
>>```

  - Rust Atomics and Locks 같이 읽기 004: Borrowing and Data Races

    - https://youtu.be/ePKy1YbG1f4

# Easy Rust \_\_ mithradates(Rust스승님)

https://youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE

<br>

<hr>

<br>

# Rust tutorial

<br>

- Monomorphization vs Polymorphism

<table border="1">
    <tr>
    <td colspan="2" align="center"></td>
    </tr>
    <tr align="center">
        <td>Date</td>
        <td>Title & Link</td>
    </tr>
    <tr align="center">
        <td>23-3-21(tue.)</td>
        <td><a href="https://youtu.be/mVtVbVFCSUo">한글Rust_068⭐️Rust Monomorphism vs Polymorphism</a></td>
    </tr>
</table>
