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
