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
