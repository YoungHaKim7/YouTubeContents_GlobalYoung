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

<img width="30" alt="c" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/1ff1c447-9b46-4775-85e2-66818ff2c318" />

# c 에서 test

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

<img width="30" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" />


---

1

<!-- _color: white -->

<img width="30" alt="Rust" src="https://github.com/user-attachments/assets/a30fbba4-d246-4f97-9a48-d9738c302160" />


---



<!-- _color: white -->

# Rust 점점 강해지는 언어

## 감사합니다.

# Rust 유료강의 문의는

## ytok1108@kakao.com

---

