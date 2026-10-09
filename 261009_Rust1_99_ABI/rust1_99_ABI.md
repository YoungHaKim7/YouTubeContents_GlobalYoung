---
marp: true
color: white
background-color: #050d1a
---

<!-- class: invert -->

<img width="50" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" />


# ABI 이해
  
  
## Rust의 모든 문법과 원리를 다 이해한 상태라 생각하고 설명 이어 가겠습니다.



<!-- paginate : true -->

---

<!-- _color: white -->


## Let's Go! Rust ABI

- https://en.wikipedia.org/wiki/Application_binary_interface

### Rust ABI 안정화 됨 (Rust 1.99)

- https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/

---

<!-- _color: white -->

<img width="10" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" />

<br />

<img width="660" height="420" alt="Image" src="https://github.com/user-attachments/assets/034359d7-5566-44a6-af51-39bc47ea7e80" />


---

<!-- _color: white -->

<img width="30" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" />


# Rust lib 프로젝트에서 `cargo b --release`


- You then distribute:

```bash
macOS
    librust_ffi.dylib

Linux
    librust_ffi.so

Windows
    rust_ffi.dll
```

---


<!-- _color: white -->


- tree 랑 비슷함 러스트로 만든 `eza`

```bash
$ eza -TL3
.
├── Cargo.toml
├── examples
│   ├── c
│   │   ├── abi_rust.dll  # WindowsOS 에서는 이게 필요하다.
│   │   ├── app
│   │   └── main.c
│   ├── cpp
│   │   ├── abi_rust.dll  # WindowsOS 에서는 이게 필요하다.
│   │   └── main.cpp
│   ├── go
│   │   ├── go.mod
│   │   └── main.go
│   └── python
│       └── main.py
├── include
│   └── rust_ffi.h
├── src
│   └── lib.rs
└── target
    └── release
        ├── rust_ffi.dll      # WindowsOS
        ├── librust_ffi.so    # LinuxOS
        └── libabi_rust.dylib # macOS
```

---

<!-- _color: white -->

<img width="30" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" /> Rust `src/lib.rs`


```rs
// src/lib.rs
#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_mul(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rust_ffi_add(2, 2);
        assert_eq!(result, 4);
        let result = rust_ffi_sub(6, 2);
        assert_eq!(result, 4);
        let result = rust_ffi_mul(2, 2);
        assert_eq!(result, 4);
    }
}
```


---


<!-- _color: white -->

<img width="30" alt="c" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/1ff1c447-9b46-4775-85e2-66818ff2c318" /> C23 에서 test

```c
#include <stdio.h>
#include "rust_ffi.h"

int main(void) {
    printf("%d\n", rust_ffi_add(10, 20));
    return 0;
}
```


---

<!-- _color: white -->

### C & C++ 에서 쓸 `h` 해더파일 만들어주기

```c
// ./include/rust_ffi.h
#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Rust FFI API
 *
 * These functions are implemented in Rust and exported
 * using the C ABI.
 */

/* Add two 32-bit integers. */
int32_t rust_ffi_add(int32_t a, int32_t b);

/* Subtract b from a. */
int32_t rust_ffi_sub(int32_t a, int32_t b);

/* Multiply two 32-bit integers. */
int32_t rust_ffi_mul(int32_t a, int32_t b);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* RUST_FFI_H */
```


---


<!-- _color: white -->

<img width="20" alt="c" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/1ff1c447-9b46-4775-85e2-66818ff2c318" /> C23 build


```bash
$ clang --version
Homebrew clang version 23.1.2

$ cargo build --release

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


---

<!-- _color: white -->

<img width="30" alt="cpp" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/02580529-b8e2-4aa9-b80e-dd1f56a08491" />C++26


```cpp
#include <iostream>
#include "rust_ffi.h"

int main() {
    std::cout << rust_ffi_add(100, 200) << '\n';
}
```


---

<!-- _color: white -->

<img width="40" alt="cpp" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/02580529-b8e2-4aa9-b80e-dd1f56a08491" /> C++26 build

```bash
$ clang++ --version
Homebrew clang version 23.1.2

$ cargo build --release

$ clang++ main.cpp -std=c++26 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.cpp   README.md

LD_LIBRARY_PATH=../../target/release ./app
300


# windowsOS
> clang++ main.cpp -std=c++26 `
>> -I../../include `
>> -L../../target/release `
>> -labi_rust `
>> -o app.exe
```


---



<!-- _color: white -->

<img width="30" alt="Go" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/8b4f734b-d251-467a-8dc5-58104d2aa38b" /> go

```go
package main

/* #cgo CFLAGS: -I../../include
#cgo LDFLAGS: -L../../target/release -labi_rust
#include "rust_ffi.h"
*/
import "C"

import "fmt"

func main() {
    a := C.int32_t(10)
    b := C.int32_t(20)

    fmt.Println("Rust ABI called from Go")
    fmt.Println("10 + 20 =", C.rust_ffi_add(a, b))
    fmt.Println("20 - 10 =", C.rust_ffi_sub(b, a))
    fmt.Println("10 * 20 =", C.rust_ffi_mul(a, b))
}
```


---


<!-- _color: white -->

<img width="30" alt="Go" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/8b4f734b-d251-467a-8dc5-58104d2aa38b" /> go (go version 1.27.2)

```fish
abi_rust/examples/go on  main [!] via 🐹 v1.27.2

$ CGO_ENABLED=1 \
      CC=clang \
      go clean -cache \
      go build -o app .
go: clean -cache cannot be used with package arguments

$ CGO_ENABLED=1 \
      CC=clang \
      go build -o app .

$ ls
app*       go.mod     main.go    README.md

$ ./app
Rust ABI called from Go
10 + 20 = 30
20 - 10 = 10
10 * 20 = 200
```


---

<!-- _color: white -->

<img width="30" alt="py" src="https://user-images.githubusercontent.com/67513038/146173763-af249b79-1838-4c27-943e-12c59be7eace.jpg" /> python 파이썬

```py
import ctypes
import platform
import os
from pathlib import Path

# Find the project root:
# abi_rust/
# ├── target/release/
# │   └── abi_rust.dll
# └── examples/python/main.py
PROJECT_ROOT = Path(__file__).resolve().parents[2]
RELEASE_DIR = PROJECT_ROOT / "target" / "release"

system = platform.system()

if system == "Windows":
    library_path = RELEASE_DIR / "abi_rust.dll"
elif system == "Linux":
    library_path = RELEASE_DIR / "libabi_rust.so"
elif system == "Darwin":
    library_path = RELEASE_DIR / "libabi_rust.dylib"
else:
    raise RuntimeError(f"Unsupported operating system: {system}")

if not library_path.is_file():
    raise FileNotFoundError(
        f"Rust shared library not found: {library_path}\n"
        "Build the Rust library first with: cargo build --release"
    )

# On Windows, help the loader find dependent DLLs.
dll_directory = None
if system == "Windows":
    dll_directory = os.add_dll_directory(str(RELEASE_DIR))

lib = ctypes.CDLL(str(library_path))

# Rust FFI function signatures
lib.rust_ffi_add.argtypes = [ctypes.c_int32, ctypes.c_int32]
lib.rust_ffi_add.restype = ctypes.c_int32

lib.rust_ffi_sub.argtypes = [ctypes.c_int32, ctypes.c_int32]
lib.rust_ffi_sub.restype = ctypes.c_int32

lib.rust_ffi_mul.argtypes = [ctypes.c_int32, ctypes.c_int32]
lib.rust_ffi_mul.restype = ctypes.c_int32

print("10 + 20 =", lib.rust_ffi_add(10, 20))
print("20 - 10 =", lib.rust_ffi_sub(20, 10))
print("10 * 20 =", lib.rust_ffi_mul(10, 20))
```


---

<!-- _color: white -->

<img width="30" alt="py" src="https://user-images.githubusercontent.com/67513038/146173763-af249b79-1838-4c27-943e-12c59be7eace.jpg" /> python 파이썬 실행

```bash
$ python3 main.py
10 + 20 = 30
20 - 10 = 10
10 * 20 = 200
```

---

<!-- _color: white -->

# Rust 점점 강해지는 언어

## 감사합니다.

# Rust 유료강의 문의는

## ytok1108@kakao.com

---

