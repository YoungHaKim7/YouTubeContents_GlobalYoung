#include <iostream>
#include "rust_ffi.h"

int main() {
    std::cout << rust_ffi_add(100, 200) << '\n';
}
