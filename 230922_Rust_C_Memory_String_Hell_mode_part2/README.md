# How to iterate over Unicode grapheme clusters in Rust?

https://stackoverflow.com/questions/58770462/how-to-iterate-over-unicode-grapheme-clusters-in-rust


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


# 
<center><p><img width=100px align="space-around" alt="c" src="https://user-images.githubusercontent.com/67513038/218466687-3ac83bee-4621-4e75-9141-39724ec2b37b.png" /></p></center>

#  A little Rust with your C

- Using Rust code inside a C or C++ project mostly consists of tw

https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

<hr>


# C vs Rust (All Rust string types explanined | Let's Get Rusty
https://youtu.be/CpvzeyzgQdw?si=CkYzWCJMZYfIcx-c

- C언어는 1개뿐
```
char()
```

- Rust의 String종류
```
&str
String
&[u8;N]
Vec[u8]
Cow<`a, str>
CStr
OsStr
OsString
Path
PathBuff
```

```
String: 포인터, 길이, 용량(24바이트)
&str: 포인터, 길이(16바이트)
str: 배열(컴파일 타임에 크기를 예측 불가)

// 네 [u8]인데 인코딩이 보장된

// 기본적으로 &str이랑 str이 &[u8]이랑 [u8]이랑 구조가 같을걸요

```


```

[T] 는 슬라이스

&[T]는 길이정보를 담고있는 팻포인터

T는 그냥 우리가 아는 타입


&T는 레퍼런스 (길이정보 없는 씬포인터)

""리터럴인 경우에만
```
https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str


```
""리터럴인 경우에만 &'static 라이프타임으로 바이너리 위치에요


컴파일 타임에 계산해서 보관해두면 되니 구럴 필요가 있을까 싶네요 immutable & static 하면 되니깐용

코드 내에 적힌 "abc" 형태의 &str 의 라이프타임lifetime이 static이라는거겠죠


그러게용 여튼 &str에는 사이즈가 있다~ 정도만 알아도..
Dynamically Sized Type
str구현
utf-8을 보장해서 그냥 [char]이나 [u8]이랑도 다르다니까
슬라이스 연산할때 utf-8에 맞게 잘라서 포인터로 만들어줄텐데
First off, a str is nothing but a type level thing; it can only be reasoned about at the type level because it's a so-called dynamically-sized type (DST). The size the str takes up cannot be known at compile time and depends on runtime information
```

https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html

1. 데이터 섹션에 들어가지만
2. 컴파일 타임에 사이즈를 알 수 없고
3. &str 타입 등의 변수로 런타임에 요리조리

https://lib.rs/crates/ropey

- Strings in Rust FINALLY EXPLAINED! | LGR
  - Follow along as we go through strings in Rust. We will be talking about UTF-8, the &str and String types, indexing into strings, and more!
    - https://youtu.be/Mcuqzx3rBWc?si=FsCwVKReq3SRuvOZ

- Rust vs C String 더 깊게 들어가기
  - Working with strings in Rust 
    - https://fasterthanli.me/articles/working-with-strings-in-rust

<hr>

<br>

<hr>

<br>

<center><img width=100px src="https://user-images.githubusercontent.com/67513038/218466731-1c232ee4-7fe7-4c73-a201-c129e16959c2.png" /></center>
