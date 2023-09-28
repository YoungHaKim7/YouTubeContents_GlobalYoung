# Project suggestions

You are encouraged to suggest your own project, here are some suggestions. We will add more ideas as they come up.

- Use a popular crate to build something
  - tokio (network applications)
  - bitvec (lowlevel binary protocols)
  - bevy (games)
  - a serializer/deserializer using Serde
  - RTIC or Embassy (embedded applications)
- Build a GUI application (https://www.areweguiyet.com/)
  - https://www.areweguiyet.com/
- Build Rust markdown-to-slide-deck renderer, as an alternative to sli.dev that we've been using
- Implement a more complex data structure
  - implement and benchmark a doubly linked list
    - https://github.com/pendulum-project/ntpd-rs  - benchmark the ntpd-rs ipfilter https://github.com/pendulum-project/ntpd-rs/blob/main/ntp-daemon/src/ipfilter.rs
  - add "seamless slices" to the Rust implementation of RocList (ask Folkert)
  - Image renderer and manipulator (PNG, SVG)
  - Implement a simple HTTP1 static file server on raw TCP sockets
- Programming languages
  - an interpreter for False (https://strlen.com/false-language/)
    - https://strlen.com/false-language/
  - an interpreter for (a subset of) webassembly
  - contribute to Roc (Folkert is a maintainer and will help you)
  - An implementation of Lox (https://craftinginterpreters.com/)
    - https://craftinginterpreters.com/
- Develop a simple OS (https://os.phil-opp.com/)
  - https://os.phil-opp.com/
  - https://wiki.osdev.org/Exceptions#Breakpoint
- Make an open source contribution NOTE: make sure your contribution has a good chance of being accepted; don't just create extra work for project maintainers
  - update inkwell's kaleidoscope example so it also works with llvm 15 https://github.com/TheDan64/inkwell/blob/master/examples/kaleidoscope/main.rs#L199
  - https://github.com/TheDan64/inkwell/blob/master/examples/kaleidoscope/main.rs#L199
  - It's a New Kind of Wrapper for Exposing LLVM (Safely)
    - Inkwell aims to help you pen your own programming languages by safely wrapping llvm-sys. It provides a more strongly typed interface than the underlying LLVM C API so that certain types of errors can be caught at compile time instead of at LLVM's runtime. This means we are trying to replicate LLVM IR's strong typing as closely as possible. The ultimate goal is to make LLVM safer from the rust end and a bit easier to learn (via documentation) and use.
  - https://github.com/TheDan64/inkwell
  - 나만의 언어 만드는 법. 
