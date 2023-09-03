# coroutines 

https://gcc.gnu.org/wiki/cxx-coroutines

The main C++ Coroutines implementation was merged into GCC-10. This branch remains open for any "ideas" or other WIP and is kept up to date with GCC master. 

https://github.com/iains/gcc-cxx-coroutines

# build 23

```bash
g++ -std=c++2b  -O2 -Wall -Wextra -pedantic -pthread -pedantic-errors main.cpp -lm  -latomic  && ./a.out
```