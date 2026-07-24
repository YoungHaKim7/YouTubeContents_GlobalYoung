---
marp: true
color: white
background-color: #050d1a
---

<!-- class: invert -->

![width:60px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MTUwMjcsIm5iZiI6MTc4NDgxNDcyNywicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDEzNTIwN1omWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTNjZmNlMmE4ZGI1M2E4MzU3ZGIzZGZlZTMyNDIzYjhkOTAzMzJjOGU4MDBhN2I4MWYyNzA2OGRkOWE5MWI0MDYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.gdTmZ4TECWmlClgQFCk4uTbHsYj-CYoUQT0Qu6wh_NU) ![width:52px](https://private-user-images.githubusercontent.com/67513038/359108754-41f357e5-7664-4b2a-8d70-bdfcf0102d36.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MjU1NzgsIm5iZiI6MTc4NDgyNTI3OCwicGF0aCI6Ii82NzUxMzAzOC8zNTkxMDg3NTQtNDFmMzU3ZTUtNzY2NC00YjJhLThkNzAtYmRmY2YwMTAyZDM2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDE2NDc1OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ0NTZmZmRkOGNhOWJkMWVmMmZkN2NjN2M1MzY2ZmQ5ZTM2OTExZWVlZjFlOTdjYzA0MThmMTk5N2ZlOGZiN2ImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.s7qqhGaoOqkU-4NFAgw2YR9cPgoiHX92pHdHo7tMTeo)

# Hello, FP(Functional Programming) 기초 001

## Imperative VS FP

- 명령형 프로그래밍과 함수형 프로그래밍을 알아보자
  - C++23 코드 &&
  - 러스트(Rust) 코드를 활용해서 설명~

<!-- paginate : true -->

---


## Watch and preview

Marp CLI is supported watch mode and preview window.

```bash
npx @marp-team/marp-cli@latest -w FP_vs_Imperative.md
```
- https://marpit.marp.app/

- https://www.npmjs.com/package/@marp-team/marp-cli

---

<!-- _color: white -->

유튜버 : Gyoung
- https://youtube.com/@globalyoung7

- 유료 러스트 언어 강의 문의 010-2895-7174



취 미 : 러스트 개발

https://github.com/Younghakim7



---

<!-- _color: white -->

# FP(Functional programming) 기본 개념
- ‘Functional programming is a style of programming that emphasizes the evaluation of expressions, rather than execution of commands. The expressions in these languages are formed by using functions to combine basic values. A functional language is a language that supports and encourages programming in a functional style.
- — FAQ for comp.lang.functional’

- 함수형 프로그래밍은 명령 실행보다 표현식의 평가를 강조하는 프로그래밍 스타일입니다. 이러한 언어의 표현은 기본 값을 결합하기 위해 함수를 사용하여 형성됩니다. 함수형 언어는 함수형 스타일의 프로그래밍을 지원하고 장려하는 언어입니다.

- — Comp.lang.functional에 대한 FAQ

- 다음에서 발췌
  -  Functional Programming in C++
    - Ivan Čukić

---

<!-- _color: white -->

- 함수형 프로그래밍은 명령 실행보다 표현식의 평가를 강조하는 프로그래밍 스타일입니다. 이러한 언어의 표현은 기본 값을 결합하기 위해 함수를 사용하여 형성됩니다. 함수형 언어는 함수형 스타일의 프로그래밍을 지원하고 장려하는 언어입니다.
  - —Comp.lang.functional에 대한 FAQ

- 다음에서 발췌
  -  Functional Programming in C++ | Ivan Čukić


---

<!-- _color: white -->

Ivan Cukic

# Functional Programming in C++: How to improve your C++ programs using functional techniques

https://www.amazon.com/Functional-Programming-programs-functional-techniques/dp/B0978262WN/


---

<!-- _color: white -->

- FP의 최대장점.(GY내가 생각하는 개인적인 생각)
  - 1. Functions that always return the exact same output for the same input and have no observable side effects (such as modifying a global variable or saving to a database).
    - 똑같은 입력 동일한 출력(부작용 없음. side effects ..x..)
  - 2. Safe Threads (안정적인 쓰레드 관리, 동시실행, 병렬 코딩에 문제가 없다.)
    - Concurrency & Parallels
  - 3. 숙련자 개발자끼리 해당(코드 리딩이 쉽고, 코드가 무슨 의미를 하는지 & 원하는 결과값이 무엇인지 한눈에 알아보기 좋다. & (FP)다른 개발자 끼리도 금방 알아본다. 굳이 설명 안해줘도 다 알아먹음.)
  - 4. 코드가 Imperative(명령형) 보다 FP가 짧게 코딩이 가능함.

---

<!-- _color: white -->

## FP역사

- 함수형 프로그래밍의 역사는 1930년대 수학자 알론조 처치(Alonzo Church)의 람다 대수(Lambda Calculus)에서 시작되어, Lisp 탄생, 학계 중심의 발전, 그리고 멀티코어 시대의 부활 과정을 거쳐왔습니다.
  - https://ko.wikipedia.org/wiki/%ED%95%A8%EC%88%98%ED%98%95_%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%B0%8D

- 이론적 태동과 Lisp (1930년대 ~ 1950년대)
  - 람다 대수: 알론조 처치가 수학적 계산과 재귀를 연구하기 위해 개발한 형식 체계입니다.
  - Lisp 탄생: 1958년 존 매카시(John McCarthy)가 람다 대수 아이디어를 바탕으로 최초의 함수형 특징을 지닌 프로그래밍 언어 Lisp를 개발했습니다.


---

<!-- _color: white -->


- 학계의 발전과 현대적 언어 (1970년대 ~ 1990년대)
  - ML과 Haskell: 1970년대 후반 에든버러 대학에서 ML 언어가 개발되었고, 1980년대에는 순수 함수형 언어인 Haskell이 등장했습니다.
  - 학술적 도구: 이 시기의 함수형 프로그래밍은 주로 인공지능 연구나 학계에서 사용되었으며, 일반 산업계에서는 큰 주류가 되지 못했습니다.
- 멀티코어 시대와 주류 언어의 통합 (2000년대 이후)
  - 패러다임의 부활: <u>컴퓨터 CPU가 멀티코어 환경으로 진입하면서 동시성 처리가 중요</u>해졌고, 부작용(Side Effect)이 없는 함수형 프로그래밍이 재조명받았습니다.
  - 현대 언어 적용: 자바(Java 8+), 자바스크립트, 파이썬 등 대다수의 현대 언어가 람다 표현식과 고차 함수 등 함수형 프로그래밍 기능을 적극적으로 수용하고 있습니다

---



<!-- _color: white -->

# 5가지 핵심 개념 (FP)

- The core concepts of functional programming include:

  - 1. Pure Functions: 
    - 함수 자체를 변수로 저장 가능
  - 2. Immutability:
    - 데이터의 불변성 - 병렬 실행시 변수가 예상 되기 때문에 통제하기 쉽다.

---

<!-- _color: white -->

  - 3. First-Class Functions: 
    - 1급 함수로 관리한다
    - 일급 함수: 함수는 다른 데이터 유형과 마찬가지로 취급됩니다. 변수에 저장하거나, 다른 함수의 인수로 전달하거나, 함수에서 반환할 수 있습니다.
  - 4. Higher-Order Functions: 
    - 고차 함수: 다른 함수를 인수로 받거나 다른 함수를 반환하는 함수입니다. 자바스크립트나 파이썬에서 `map`, `filter`, `reduce` 와 같은 전통적인 반복문 구조를 대체하는 데 많이 사용됩니다.
  - 5. 최대 장점 ( side effect가 없다) 
    - 결과값의 안정성(1번의 원리를 활용한 효과)

---


<!-- _color: white -->


- https://velog.io/@kyusung/%ED%95%A8%EC%88%98%ED%98%95-%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%B0%8D-%EC%9A%94%EC%95%BD

# 러스트 코드와 C++23코드를 예시로 하나씩 알아보자

---

<!-- _color: white -->

- 명령형 vs 함수형 프로그래밍의 비교

||명령형 프로그래밍|함수형 프로그래밍|
|-|-|-|
|프로그램이란?|프로그램은 명령의 수행이다.|프로그램은 함수의 계산이다.|
|중점적 시작|어떻게에 초점<br />(how to)|무엇에 초점<br />(what)|
|이론적 배경|튜링 머신|람다 계산식|
|주요 프로그래밍 언어|C, Java 등 대부분의 언어|Scheme, Haskell, ML, Erlang|


---

<!-- _color: white -->

![width:52px](https://private-user-images.githubusercontent.com/67513038/359108754-41f357e5-7664-4b2a-8d70-bdfcf0102d36.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MjU1NzgsIm5iZiI6MTc4NDgyNTI3OCwicGF0aCI6Ii82NzUxMzAzOC8zNTkxMDg3NTQtNDFmMzU3ZTUtNzY2NC00YjJhLThkNzAtYmRmY2YwMTAyZDM2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDE2NDc1OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ0NTZmZmRkOGNhOWJkMWVmMmZkN2NjN2M1MzY2ZmQ5ZTM2OTExZWVlZjFlOTdjYzA0MThmMTk5N2ZlOGZiN2ImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.s7qqhGaoOqkU-4NFAgw2YR9cPgoiHX92pHdHo7tMTeo)

- 여러분들은 아마도 이런 코딩에 익숙하실겁니다.

- C++23

```cpp
// Imperative
#include <print>

int main() {
    int sum = 0;

    for (int i = 1; i <= 10; ++i) {
        sum += i;
    }

    std::println("{}", sum);
}
```

<style>
/*

Orginal Style from ethanschoonover.com/solarized (c) Jeremy Hull <sourdrums@gmail.com>
// https://github.com/highlightjs/highlight.js/blob/0c4cc8a1c3aa373aee06796433c1389e4d2a3a45/src/styles/solarized-dark.css

*/

.hljs {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: #030d0f;
  color: #839496;
}

.hljs-comment,
.hljs-quote {
  color: #586e75;
}

/* Solarized Green */
.hljs-keyword,
.hljs-selector-tag,
.hljs-addition {
  color: #859900;
}

/* Solarized Cyan */
.hljs-number,
.hljs-string,
.hljs-meta .hljs-meta-string,
.hljs-literal,
.hljs-doctag,
.hljs-regexp {
  color: #2aa198;
}

/* Solarized Blue */
.hljs-title,
.hljs-section,
.hljs-name,
.hljs-selector-id,
.hljs-selector-class {
  color: #268bd2;
}

/* Solarized Yellow */
.hljs-attribute,
.hljs-attr,
.hljs-variable,
.hljs-template-variable,
.hljs-class .hljs-title,
.hljs-type {
  color: #b58900;
}

/* Solarized Orange */
.hljs-symbol,
.hljs-bullet,
.hljs-subst,
.hljs-meta,
.hljs-meta .hljs-keyword,
.hljs-selector-attr,
.hljs-selector-pseudo,
.hljs-link {
  color: #cb4b16;
}

/* Solarized Red */
.hljs-built_in,
.hljs-deletion {
  color: #dc322f;
}

.hljs-formula {
  background: #073642;
}

.hljs-emphasis {
  font-style: italic;
}

.hljs-strong {
  font-weight: bold;
}


</style>

---

![width:52px](https://private-user-images.githubusercontent.com/67513038/359108754-41f357e5-7664-4b2a-8d70-bdfcf0102d36.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MjU1NzgsIm5iZiI6MTc4NDgyNTI3OCwicGF0aCI6Ii82NzUxMzAzOC8zNTkxMDg3NTQtNDFmMzU3ZTUtNzY2NC00YjJhLThkNzAtYmRmY2YwMTAyZDM2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDE2NDc1OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ0NTZmZmRkOGNhOWJkMWVmMmZkN2NjN2M1MzY2ZmQ5ZTM2OTExZWVlZjFlOTdjYzA0MThmMTk5N2ZlOGZiN2ImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.s7qqhGaoOqkU-4NFAgw2YR9cPgoiHX92pHdHo7tMTeo)

<!-- _color: white -->

- FP(Functional Programming) style(C++23)

```cpp
// FP style
#include <numeric>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11); // 1..10

    int sum = std::accumulate(numbers.begin(), numbers.end(), 0);

    std::println("{}", sum);
}

```

<style>
/*

Orginal Style from ethanschoonover.com/solarized (c) Jeremy Hull <sourdrums@gmail.com>
// https://github.com/highlightjs/highlight.js/blob/0c4cc8a1c3aa373aee06796433c1389e4d2a3a45/src/styles/solarized-dark.css

*/

.hljs {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: #030d0f;
  color: #839496;
}

.hljs-comment,
.hljs-quote {
  color: #586e75;
}

/* Solarized Green */
.hljs-keyword,
.hljs-selector-tag,
.hljs-addition {
  color: #859900;
}

/* Solarized Cyan */
.hljs-number,
.hljs-string,
.hljs-meta .hljs-meta-string,
.hljs-literal,
.hljs-doctag,
.hljs-regexp {
  color: #2aa198;
}

/* Solarized Blue */
.hljs-title,
.hljs-section,
.hljs-name,
.hljs-selector-id,
.hljs-selector-class {
  color: #268bd2;
}

/* Solarized Yellow */
.hljs-attribute,
.hljs-attr,
.hljs-variable,
.hljs-template-variable,
.hljs-class .hljs-title,
.hljs-type {
  color: #b58900;
}

/* Solarized Orange */
.hljs-symbol,
.hljs-bullet,
.hljs-subst,
.hljs-meta,
.hljs-meta .hljs-keyword,
.hljs-selector-attr,
.hljs-selector-pseudo,
.hljs-link {
  color: #cb4b16;
}

/* Solarized Red */
.hljs-built_in,
.hljs-deletion {
  color: #dc322f;
}

.hljs-formula {
  background: #073642;
}

.hljs-emphasis {
  font-style: italic;
}

.hljs-strong {
  font-weight: bold;
}


</style>


---

<!-- _color: white -->

![width:52px](https://private-user-images.githubusercontent.com/67513038/359108754-41f357e5-7664-4b2a-8d70-bdfcf0102d36.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MjU1NzgsIm5iZiI6MTc4NDgyNTI3OCwicGF0aCI6Ii82NzUxMzAzOC8zNTkxMDg3NTQtNDFmMzU3ZTUtNzY2NC00YjJhLThkNzAtYmRmY2YwMTAyZDM2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDE2NDc1OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ0NTZmZmRkOGNhOWJkMWVmMmZkN2NjN2M1MzY2ZmQ5ZTM2OTExZWVlZjFlOTdjYzA0MThmMTk5N2ZlOGZiN2ImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.s7qqhGaoOqkU-4NFAgw2YR9cPgoiHX92pHdHo7tMTeo)

- FP(Functional Programming) style(C++23)
  - `fold_left`

```cpp
// FP style
#include <algorithm>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11); // 1..10

    int sum = std::ranges::fold_left(numbers, 0,
                                     [](int acc, int x) { return acc + x; });

    std::println("{}", sum);
}


```

<style>
/*

Orginal Style from ethanschoonover.com/solarized (c) Jeremy Hull <sourdrums@gmail.com>
// https://github.com/highlightjs/highlight.js/blob/0c4cc8a1c3aa373aee06796433c1389e4d2a3a45/src/styles/solarized-dark.css

*/

.hljs {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: #030d0f;
  color: #839496;
}

.hljs-comment,
.hljs-quote {
  color: #586e75;
}

/* Solarized Green */
.hljs-keyword,
.hljs-selector-tag,
.hljs-addition {
  color: #859900;
}

/* Solarized Cyan */
.hljs-number,
.hljs-string,
.hljs-meta .hljs-meta-string,
.hljs-literal,
.hljs-doctag,
.hljs-regexp {
  color: #2aa198;
}

/* Solarized Blue */
.hljs-title,
.hljs-section,
.hljs-name,
.hljs-selector-id,
.hljs-selector-class {
  color: #268bd2;
}

/* Solarized Yellow */
.hljs-attribute,
.hljs-attr,
.hljs-variable,
.hljs-template-variable,
.hljs-class .hljs-title,
.hljs-type {
  color: #b58900;
}

/* Solarized Orange */
.hljs-symbol,
.hljs-bullet,
.hljs-subst,
.hljs-meta,
.hljs-meta .hljs-keyword,
.hljs-selector-attr,
.hljs-selector-pseudo,
.hljs-link {
  color: #cb4b16;
}

/* Solarized Red */
.hljs-built_in,
.hljs-deletion {
  color: #dc322f;
}

.hljs-formula {
  background: #073642;
}

.hljs-emphasis {
  font-style: italic;
}

.hljs-strong {
  font-weight: bold;
}


</style>

---

<!-- _color: white -->

![width:52px](https://private-user-images.githubusercontent.com/67513038/359108754-41f357e5-7664-4b2a-8d70-bdfcf0102d36.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MjU1NzgsIm5iZiI6MTc4NDgyNTI3OCwicGF0aCI6Ii82NzUxMzAzOC8zNTkxMDg3NTQtNDFmMzU3ZTUtNzY2NC00YjJhLThkNzAtYmRmY2YwMTAyZDM2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDE2NDc1OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ0NTZmZmRkOGNhOWJkMWVmMmZkN2NjN2M1MzY2ZmQ5ZTM2OTExZWVlZjFlOTdjYzA0MThmMTk5N2ZlOGZiN2ImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.s7qqhGaoOqkU-4NFAgw2YR9cPgoiHX92pHdHo7tMTeo)

- FP(Functional Programming) style(C++23)
  - even shorter code
  - `fold_left`

```cpp
// FP style(even shorter code)
#include <algorithm>
#include <functional>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11);

    int sum = std::ranges::fold_left(numbers, 0, std::plus{});

    std::println("{}", sum);
}
```

<style>
/*

Orginal Style from ethanschoonover.com/solarized (c) Jeremy Hull <sourdrums@gmail.com>
// https://github.com/highlightjs/highlight.js/blob/0c4cc8a1c3aa373aee06796433c1389e4d2a3a45/src/styles/solarized-dark.css

*/

.hljs {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: #030d0f;
  color: #839496;
}

.hljs-comment,
.hljs-quote {
  color: #586e75;
}

/* Solarized Green */
.hljs-keyword,
.hljs-selector-tag,
.hljs-addition {
  color: #859900;
}

/* Solarized Cyan */
.hljs-number,
.hljs-string,
.hljs-meta .hljs-meta-string,
.hljs-literal,
.hljs-doctag,
.hljs-regexp {
  color: #2aa198;
}

/* Solarized Blue */
.hljs-title,
.hljs-section,
.hljs-name,
.hljs-selector-id,
.hljs-selector-class {
  color: #268bd2;
}

/* Solarized Yellow */
.hljs-attribute,
.hljs-attr,
.hljs-variable,
.hljs-template-variable,
.hljs-class .hljs-title,
.hljs-type {
  color: #b58900;
}

/* Solarized Orange */
.hljs-symbol,
.hljs-bullet,
.hljs-subst,
.hljs-meta,
.hljs-meta .hljs-keyword,
.hljs-selector-attr,
.hljs-selector-pseudo,
.hljs-link {
  color: #cb4b16;
}

/* Solarized Red */
.hljs-built_in,
.hljs-deletion {
  color: #dc322f;
}

.hljs-formula {
  background: #073642;
}

.hljs-emphasis {
  font-style: italic;
}

.hljs-strong {
  font-weight: bold;
}


</style>

---

![width:60px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MTUwMjcsIm5iZiI6MTc4NDgxNDcyNywicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDEzNTIwN1omWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTNjZmNlMmE4ZGI1M2E4MzU3ZGIzZGZlZTMyNDIzYjhkOTAzMzJjOGU4MDBhN2I4MWYyNzA2OGRkOWE5MWI0MDYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.gdTmZ4TECWmlClgQFCk4uTbHsYj-CYoUQT0Qu6wh_NU)


- The C++23 `std::ranges::fold_left` example can be converted to Rust like this:

- FP(Functional Programming) style(Rust Lang)
  - `fold_left`(C++23) = `fold`(Rust)

```rs
fn main() {
    let numbers = 1..=10;

    let sum = numbers.fold(0, |acc, x| acc + x);

    println!("{sum}");
}
```

<style>
/*

Orginal Style from ethanschoonover.com/solarized (c) Jeremy Hull <sourdrums@gmail.com>
// https://github.com/highlightjs/highlight.js/blob/0c4cc8a1c3aa373aee06796433c1389e4d2a3a45/src/styles/solarized-dark.css

*/

.hljs {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: #030d0f;
  color: #839496;
}

.hljs-comment,
.hljs-quote {
  color: #586e75;
}

/* Solarized Green */
.hljs-keyword,
.hljs-selector-tag,
.hljs-addition {
  color: #859900;
}

/* Solarized Cyan */
.hljs-number,
.hljs-string,
.hljs-meta .hljs-meta-string,
.hljs-literal,
.hljs-doctag,
.hljs-regexp {
  color: #2aa198;
}

/* Solarized Blue */
.hljs-title,
.hljs-section,
.hljs-name,
.hljs-selector-id,
.hljs-selector-class {
  color: #268bd2;
}

/* Solarized Yellow */
.hljs-attribute,
.hljs-attr,
.hljs-variable,
.hljs-template-variable,
.hljs-class .hljs-title,
.hljs-type {
  color: #b58900;
}

/* Solarized Orange */
.hljs-symbol,
.hljs-bullet,
.hljs-subst,
.hljs-meta,
.hljs-meta .hljs-keyword,
.hljs-selector-attr,
.hljs-selector-pseudo,
.hljs-link {
  color: #cb4b16;
}

/* Solarized Red */
.hljs-built_in,
.hljs-deletion {
  color: #dc322f;
}

.hljs-formula {
  background: #073642;
}

.hljs-emphasis {
  font-style: italic;
}

.hljs-strong {
  font-weight: bold;
}


</style>


---

<!-- _color: white -->


# The C++23 `std::ranges::fold_left` example can be converted to Rust like this:

| C++23                                    | Rust                 |
| ---------------------------------------- | -------------------- |
| `std::views::iota(1, 11)`                | `1..=10`             |
| `std::ranges::fold_left(...)`            | `.fold(...)`         |
| `0`                                      | `0`                  |
|

---


<!-- _color: white -->

# FP
- ① . Pure Functions: 
  - Functions that always return the exact same output for the same input and have no observable side effects (such as modifying a global variable or saving to a database).
- ② . Immutability:
  - Data cannot be modified after it is created. To change a value, you must create a new copy of it, which eliminates the risk of hidden changes occurring elsewhere in your application.
- ③ . First-Class Functions: 
  - Functions are treated just like any other data type; they can be stored in variables, passed as arguments to other functions, or returned from them.
- ④ . Higher-Order Functions: 
  -  Functions that take other functions as arguments or return them. They are heavily used to replace traditional looping constructs (e.g., using `map`, `filter`, and `reduce` in Javascript or Python).

- Because FP avoids state changes, it is highly favored for writing concurrent, **multi-threaded** software where **thread safety** and data consistency are critical
  - FP는 상태 변경을 피하기 때문에 스레드 안전성과 데이터 일관성이 중요한 멀티 스레드 소프트웨어를 동시에 작성하는 데 매우 선호됩니다

---

<!-- _color: white -->

- 1억줄 되는 긴 csv를 읽을때 멀티 쓰레드는 빛을 바랜다.

1️⃣🐝🏎️ The One Billion Row Challenge -- A fun exploration of how quickly 1B rows from a text file can be aggregated with Java
- https://github.com/gunnarmorling/1brc

---

<!-- _color: white -->

- ① 1억줄 되는 긴 csv를 읽고
- ③ cpu 멀티 쓰레딩 가능한 갯수를 확인 후
- ③ chunks로 나눠서 따로 계산 후
- ④ 계산 하면

- Single Thread 보다 더 빠른 연산 결과를 얻을 수 있다.

---

<!-- _color: white -->

- 내 계획표
- The parallel architecture

The complete architecture looks like this:

---

<!-- _color: white -->


```
                    1 Billion Row File
                           │
                           │
                  Determine file size
                           │
                           ▼
                ┌─────────────────────┐
                │  Divide into chunks │
                └─────────────────────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
           ▼               ▼               ▼
       Chunk 0         Chunk 1         Chunk 2       ...
           │               │               │
           ▼               ▼               ▼
       Thread 0         Thread 1         Thread 2
           │               │               │
           ▼               ▼               ▼
    Local HashMap     Local HashMap     Local HashMap
           │               │               │
           └───────────────┼───────────────┘
                           │
                           ▼
                    Merge partial maps
                           │
                           ▼
                    Final result
```


---

<!-- _color: white -->

---


<!-- _color: white -->

# 감사합니다.

## End

# <!--fit--> :+1:

---
