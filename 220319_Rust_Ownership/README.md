# 소유권 규칙

먼저, 소유권에 적용되는 규칙부터 살펴보자. 앞으로 살펴볼 예제들은 이 규칙들을 설명하기 위한 것이므로 잘 기억하도록 하자.

1. 러스트가 다루는 각각의 값은 소유자(owner)라고 부르는 변수를 가지고 있다.
2. 특정 시점에 값의 소유자는 단 하나뿐이다.
3. 소유자가 범위를 벗어나면 그 값은 제거된다.

# Ownership Rules

First, let's take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them"

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

# Borrowing rules

1. At any given time,
   you can have either one mutable reference or any number of immutable references.

2. References must always be valid.

# Borrow Checker(Rustlang)

<table border="1">
    <tr>
    <td colspan="2" align="center">Rust Borrow Checker (해외 유튜버 자료)</td>
    </tr>
    <tr align="center">
        <td>Date</td>
        <td>Title & Link</td>
    </tr>
    <tr align="center">
        <td>21-5-15(sat.)</td>
        <td><a href="https://youtu.be/HG1fppexRMA">The Rust Borrow Checker - A Deep Dive<br>A Deep Dive - Nell Shamrell-Harrington, Microsoft</td>
    <tr align="center">
        <td>23-1-21(sun.)</td>
        <td><a href="https://youtu.be/HwupNf9iCJk">Sneaking By The Rust Borrow Checker - Interior Mutability<br>Code to the Moon</td>
</table>

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

What is Ownership? - The Rust Programming Language

Ownership is a set of rules that governs how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no-longer used memory as the prog

https://doc.rust-lang.org

# 메모리를 빌려간 참조가 있는 동안은 해제할 수 없습니다.

- 소유권자의 수명이 다하기 전에 갚아야 합니다.

20분17초
https://www.youtube.com/watch?v=sv9UHD67_rQ

- 무효화

메모리를 빌려간 참조가 있는 동안은 변경할 수 없습니다.
컴파일러가 참조의 수명을 추척합니다.

22분16초
https://www.youtube.com/watch?v=sv9UHD67_rQ

<br>

<hr>

# Ownership(Rustlang)

<table border="1">
    <tr>
    <td colspan="2" align="center">Rust Option</td>
    </tr>
    <tr align="center">
        <td>Date</td>
        <td>Title & Link</td>
    </tr>
    <tr align="center">
        <td>22-3-19(sun.)</td>
        <td><a href="https://youtu.be/5f5Ua0ebeEc">rust한글강의_러스트_오너쉽개념Borrowing이해하기_메모리할당_What is Ownership_rust programming#rust #ownership #borrowing</td>
    <tr align="center">
        <td>22-6-07(tue.)</td>
        <td><a href="https://youtu.be/__7cMs4gqSU">자바(Java)_vs_러스트_비교하면서 러스트오너쉽개념이해_기본syntax연습하기part3_#java #rust #ownership</td>
</table>

# rust -memory-container

https://github.com/usagi/rust-memory-container-cs

![rust-container](https://github.com/usagi/rust-memory-container-cs/blob/master/3840x2160/rust-memory-container-cs-3840x2160-dark-back.png)

<br>

# Ownership Concept Diagram

![rust-ownvership](https://i.redd.it/nhhxzcwqd6q61.png)

## 출처:

https://www.reddit.com/r/rust/comments/mgh9n9/ownership_concept_diagram/?utm_source=share&utm_medium=ios_app&utm_name=iossmf

<br>

![Screenshot 2023-01-21 at 10 56 20 AM](https://user-images.githubusercontent.com/67513038/213838895-8194e55a-abe4-472e-8ed4-f34e7770425a.png)
<br><a href="https://youtu.be/__7cMs4gqSU">220607자바(Java)*vs*러스트*비교하면서 러스트오너쉽개념이해*기본syntax연습하기part3\_#java #rust #ownership</a><br>

<br>

출처

Rust for Java Developers 3/3 - Understanding Ownership
https://youtu.be/Vg1LGHuAPP8

Rust소유권 규칙Ownership Rules & Borrowing rules
https://economiceco.tistory.com/12591

Rust) shared reference ❤️ unique reference
https://youtu.be/Bfqx_V2gp1Y

<br>

<hr>

# Easy Rust

- 프로그래밍 언어 러스트를 배웁시다! 019 Easy Rust in Korean: references and shadowing

https://youtu.be/oOXM9Aafem8

## Easy Rust 모아보기

https://www.youtube.com/watch?v=oOXM9Aafem8&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=20

<br>

# Understanding Ownership in Rust | Let's Get Rusty

https://youtu.be/VFIOSWy93H0

<hr>

# 같이 보면 좋은 자료

- stack&heap메모리개념잡기
  https://youtu.be/OwQxo4sGVWo

- 깊은 복사 vs 얇은 복사 이해
  shallow_copy**vs**deep_copy
  https://youtu.be/J11bAkyMbN0

## 내 블로그에 정리

Rust소유권 규칙Ownership Rules &Borrowing rules

- https://economiceco.tistory.com/12591

★★★Rust Toturial 로드맵(Road Map)첫시작!-★★★(총정리)Rustacean이 되어 보자!!Let's go!

- https://economiceco.tistory.com/m/8614
