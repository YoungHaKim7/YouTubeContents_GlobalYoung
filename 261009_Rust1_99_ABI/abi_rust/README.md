# Result
- check만 가능하고 build는 안되네..
  - WindowsOS에서 build해서 `dll` 나오는지 확인하자
  - ✅ macOS에서 build해서 `dylib` 나오는지 확인 
  - ✅ LinuxOS `librust_ffi.so` 나오는것 확인

# clang --version

```bash
$ clang++ --version

Homebrew clang version 23.1.2
Target: arm64-apple-darwin27.0.0
Thread model: posix
InstalledDir: /opt/homebrew/Cellar/llvm/23.1.2/bin
Configuration file: /opt/homebrew/Cellar/llvm/23.1.2/etc/clang/arm64-apple-darwin27.cfg

# gcc
$ gcc --version
Apple clang version 21.0.0 (clang-2100.3.34.2)
Target: arm64-apple-darwin27.0.0
Thread model: posix
InstalledDir: /Library/Developer/CommandLineTools/usr/bin
```

```bash
$ cargo build --target x86_64-pc-windows-msvc --release
   Compiling abi_rust v0.1.0 (/home/y/my_project/Rust_Lang/rust_release/rust1_99_ABI/ABI/abi_rust)
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-pc-windows-msvc` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-pc-windows-msvc`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `abi_rust` (lib) due to 1 previous error

$ rustup target add x86_64-pc-windows-msvc
info: downloading component rust-std
     rust-std installed                       21.99 MiB

$ cargo build --target x86_64-pc-windows-msvc --release
   Compiling abi_rust v0.1.0 (/home/y/my_project/Rust_Lang/rust_release/rust1_99_ABI/ABI/abi_rust)
error: linker `link.exe` not found
  |
  = note: No such file or directory (os error 2)

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option

note: VS Code is a different product, and is not sufficient

error: could not compile `abi_rust` (lib) due to 1 previous error

$ cargo c --target x86_64-pc-windows-msvc --release
    Checking abi_rust v0.1.0 (/home/y/my_project/Rust_Lang/rust_release/rust1_99_ABI/ABI/abi_rust)
    Finished `release` profile [optimized] target(s) in 0.38s

```


# clang (c test)

- `clang`

```bash
# 젤 먼저
cargo build --release

$ clang main.c -std=c23 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.c     README.md

$ ./app
30
  
```
- gcc

```bash
$ gcc main.c \
      -I../../include \
      -L../../target/release \
      -labi_rust \
       -o app

$ LD_LIBRARY_PATH=../../target/release ./app
30
`
