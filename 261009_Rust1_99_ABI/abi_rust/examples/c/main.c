#include <stdio.h>
#include "rust_ffi.h"

int main(void) {
    printf("%d\n", rust_ffi_add(10, 20));
    return 0;
}
