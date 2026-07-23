---
marp: true
theme: uncover
---

![width: 50px](https://private-user-images.githubusercontent.com/67513038/405572633-3489669b-63c0-439e-b507-9b2bfb3fdd5e.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODQ4MTUwMjcsIm5iZiI6MTc4NDgxNDcyNywicGF0aCI6Ii82NzUxMzAzOC80MDU1NzI2MzMtMzQ4OTY2OWItNjNjMC00MzllLWI1MDctOWIyYmZiM2ZkZDVlLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA3MjMlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNzIzVDEzNTIwN1omWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTNjZmNlMmE4ZGI1M2E4MzU3ZGIzZGZlZTMyNDIzYjhkOTAzMzJjOGU4MDBhN2I4MWYyNzA2OGRkOWE5MWI0MDYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.gdTmZ4TECWmlClgQFCk4uTbHsYj-CYoUQT0Qu6wh_NU)

# Hello, FP(Functional Programming) 기초 001

Imperative VS FP

명령형 프로그래밍과 함수형 프로그래밍을 알아보자

C++23 코드
러스트 코드를 활용해서 설명~

<!-- paginate : true -->

---

<!-- backgroundColor: black -->
<!-- _color: white -->

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
—FAQ for comp.lang.functional’

- 함수형 프로그래밍은 명령 실행보다 표현식의 평가를 강조하는 프로그래밍 스타일입니다. 이러한 언어의 표현은 기본 값을 결합하기 위해 함수를 사용하여 형성됩니다. 함수형 언어는 함수형 스타일의 프로그래밍을 지원하고 장려하는 언어입니다.

—Comp.lang.functional에 대한 FAQ

- 다음에서 발췌
  -  Functional Programming in C++
    - Ivan Čukić

---

<!-- _color: white -->

Ivan Cukic

# Functional Programming in C++: How to improve your C++ programs using functional techniques

https://www.amazon.com/Functional-Programming-programs-functional-techniques/dp/B0978262WN/


---

<!-- _color: white -->

# 5가지 핵심 개념 (FP)

- The core concepts of functional programming include:

  - 1. Pure Functions: 
    - 함수 자체를 변수로 저장 가능
  - 2. Immutability:
    - 데이터의 불변성 - 병렬 실행시 변수가 예상 되기 때문에 통제하기 쉽다.
  - 3. First-Class Functions: 
    - 1급 함수로 관리한다
    - 일급 함수: 함수는 다른 데이터 유형과 마찬가지로 취급됩니다. 변수에 저장하거나, 다른 함수의 인수로 전달하거나, 함수에서 반환할 수 있습니다.
  - 4. Higher-Order Functions: 
    - 고차 함수: 다른 함수를 인수로 받거나 다른 함수를 반환하는 함수입니다. 자바스크립트나 파이썬에서 map, filter, reduce와 같은 전통적인 반복문 구조를 대체하는 데 많이 사용됩니다.
  - 5. 최대 장점 ( side effect가 없다) 
    - 결과값의 안정성(1번의 원리를 활용한 효과)

---

<!-- _color: white -->

# FP
- 1. Pure Functions: 
    - Functions that always return the exact same output for the same input and have no observable side effects (such as modifying a global variable or saving to a database).
  - 2. Immutability:
    - Data cannot be modified after it is created. To change a value, you must create a new copy of it, which eliminates the risk of hidden changes occurring elsewhere in your application.
  - 3. First-Class Functions: 
    - Functions are treated just like any other data type; they can be stored in variables, passed as arguments to other functions, or returned from them.
  - 4. Higher-Order Functions: 
    - F


<!-- _color: white -->

감사합니다.

End

# <!--fit--> :+1:

---
