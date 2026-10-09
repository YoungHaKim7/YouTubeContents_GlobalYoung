# C & C++되는거 확인

- 버젼 높아야함
- gcc 21이상 , clang 21 version이상

# C test

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
```


# C++26 test

- Result

- c++26

```bash
$ clang++ main.cpp -std=c++26 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.cpp   README.md

LD_LIBRARY_PATH=../../target/release ./app
300
```

