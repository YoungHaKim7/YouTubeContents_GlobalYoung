# ```--fsanitize=address``` 사용법

- C언어
```
clang -g -fsanitize=address test01.c
```

- C++
```
g++ -fsanitize=address -g3 -std=c++11 vector_ex01.cpp -o vector_ex01
```


<hr>

# Memory & Data, Video 3: Memory address - Luis Ceze

https://economiceco.tistory.com/12219

<hr>

# C Dynamic Memory Debugging with Valgrind

https://youtu.be/bb1bTJtgXrI


<br>

```
valgrind --leak-check=full ./mem_leak.out


valgrind --leak-check=yes ./main
```

<hr>