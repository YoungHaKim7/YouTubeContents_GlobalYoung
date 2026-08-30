---
marp: true
color: white
background-color: #050d1a
---

<!-- class: invert -->

![width:100px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgxMDA4NjQsIm5iZiI6MTc4ODEwMDU2NCwicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA4MzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwODMwVDE0MzYwNFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmM2Y5ZmQxZDhkMWYxMWMwYzU0Mzg2NWYxNTBlZTUyYWM3ODM5MmQ1OGRlNGVmYmFjODU5OTMwZGM1YTE3NzUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.1xOnh10yaR1U5aRaI4QHew0P4cjDfbS5U4s0xIVPgYU)


# Hello, FP(Functional Programming) 기초 002

## Type Theory 타입 이론 이해

- C/C++/zig/ Rust에 쓰는 타입은 어떤 원리로 돌아가나?

<!-- paginate : true -->

---

<!-- _color: white -->

# 사전 지식
- Rust 기초를 다 이해하는 상태
  - easy rust 를 먼저 다 보고 오세요.
  
## enum, struct, impl, traits개념을 완벽이 이해해야 지금 내용이 이해가 가능합니다.
  

---

<!-- _color: white -->

# Rust의 모든 문법과 원리를 다 이해한 상태라 생각하고 설명 이어 가겠습니다.

## Let's Go! Rust

---

<!-- _color: white -->

# Type만으로 Sum(덧셈을 하겠습니다.)

## result = a + b

### 이런 느낌으로 보여 드릴께요.

# 이게 바로 타입시스템의 장점입니다.

---

<!-- _color: white -->

![width:30px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgxMDA4NjQsIm5iZiI6MTc4ODEwMDU2NCwicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA4MzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwODMwVDE0MzYwNFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmM2Y5ZmQxZDhkMWYxMWMwYzU0Mzg2NWYxNTBlZTUyYWM3ODM5MmQ1OGRlNGVmYmFjODU5OTMwZGM1YTE3NzUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.1xOnh10yaR1U5aRaI4QHew0P4cjDfbS5U4s0xIVPgYU)


# Rust의 `PhantomData` 활용하도록 하겠습니다.

- Zero-sized type used to mark things that “act like” they own a T.
- Adding a `PhantomData<T>` field to your type tells the compiler that your type acts as though it stores a value of type T, even though it doesn’t really
  - 자신이 소유한 것처럼 '행동하는' 것들을 표시하는 데 사용되는 제로 크기 타입입니다. T.
  - `PhantomData를` 추가하기`<T>` field to your type은 컴파일러에게 당신의 타입이 실제로는 그렇지 않더라도, 마치 타입 T의 값을 저장하는 것처럼 동작한다고 알려줍니다. 

---


<!-- _color: white -->

# Rust에는 독특하게 size가 0bytes인게 존재합니다.

## 많이 생소 하시죠?

# 안되는걸 되게 하려면 어쩔수 없습니다.~~

---

<!-- _color: white -->

![width:30px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgxMDA4NjQsIm5iZiI6MTc4ODEwMDU2NCwicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA4MzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwODMwVDE0MzYwNFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmM2Y5ZmQxZDhkMWYxMWMwYzU0Mzg2NWYxNTBlZTUyYWM3ODM5MmQ1OGRlNGVmYmFjODU5OTMwZGM1YTE3NzUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.1xOnh10yaR1U5aRaI4QHew0P4cjDfbS5U4s0xIVPgYU)


# `PantomData`는 무엇인가?

- PhantomData는 표준 라이브러리에서 대략 다음과 같이 정의되어 있습니다:

```rust
pub struct PhantomData<T: ?Sized>;
```

- 이를 개념적으로 이렇게 생각할 수 있습니다:

> 0바이트를 차지하지만 컴파일러를 위한 타입 정보를 담고 있는 값입니다.


---

<!-- _color: white -->

# 한마디로 Generics처럼 활용할 수 있다는 이야기입니다.

# 자세히 들어가겠습니다.

---

<!-- _color: white -->

![width:30px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgxMDA4NjQsIm5iZiI6MTc4ODEwMDU2NCwicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA4MzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwODMwVDE0MzYwNFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmM2Y5ZmQxZDhkMWYxMWMwYzU0Mzg2NWYxNTBlZTUyYWM3ODM5MmQ1OGRlNGVmYmFjODU5OTMwZGM1YTE3NzUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.1xOnh10yaR1U5aRaI4QHew0P4cjDfbS5U4s0xIVPgYU)

# For example:

```rust
use std::marker::PhantomData;

struct Container<T> {
    marker: PhantomData<T>,
}
```

---

<!-- _color: white -->

# 비록 `Container<T>`는 실제로 `T`를 포함하지 않으며, Rust는 `Container<i32>`와 `Container<String>`이 서로 다른 타입임을 알고 있습니다.

## 중요하게:

- 디버깅 해보면 0 bytes가 나옵니다.

```rust
std::mem::size_of::<PhantomData<String>>()
```


---

<!-- _color: white -->

# 0 bytes란 의미는

## So `PhantomData` has no runtime memory cost.

# 이런 의미는 런타임 메모리 비용이 없습니다.

# 대박입니다.~

---

<!-- _color: white -->

![width:30px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgxMDA4NjQsIm5iZiI6MTc4ODEwMDU2NCwicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA4MzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwODMwVDE0MzYwNFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmM2Y5ZmQxZDhkMWYxMWMwYzU0Mzg2NWYxNTBlZTUyYWM3ODM5MmQ1OGRlNGVmYmFjODU5OTMwZGM1YTE3NzUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.1xOnh10yaR1U5aRaI4QHew0P4cjDfbS5U4s0xIVPgYU)

```rust
use std::marker::PhantomData;
struct Zero;
struct Succ<N>(PhantomData<N>);

fn main() {
    // Rust 에서`: <>` 이부분이 타입입니다.
    let one: Succ<Zero> = Succ(PhantomData);
    let two: Succ<Succ<Zero>> = Succ(PhantomData);
    let three: Succ<Succ<Succ<Zero>>> = Succ(PhantomData);

    println!("Type theory ( basic 001) 1+1+1");

    println!("one : {one:?}");
    println!("two : {two:?}");
    println!("three : {three:?}");
}

```

---

<!-- _color: white -->

# 이렇게 하면 1, 2, 3

# 나옵니다.   but 안 나오겠죠???

# 중간에 `impl` 구현해 줘야합니다.

---

<!-- _color: white -->

# `impl` 좀 빡쎄요 이해 안되면 난중에 다시 보세요

## 마법의 `trait` 소환하겠습니다.

# 러스트의 진정한 꽃은 `trait` ~ ~~!

---




<!-- _color: white -->

```rust
// Type-level natural numbers: map each type to its numeric value.
trait Peano {
    const VALUE: usize;
}

impl Peano for Zero {
    const VALUE: usize = 0;
}

impl<N: Peano> Peano for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

impl std::fmt::Debug for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Peano>::VALUE)
    }
}

impl<N: Peano> std::fmt::Debug for Succ<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Peano>::VALUE)
    }
}
```

---

<!-- _color: white -->

# Rust의 정형적인 패턴이 나오는 코드입니다.~~

## 감사합니다.

# Rust 유료강의 문의는

## ytok1108@kakao.com


