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

