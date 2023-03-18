# Source

https://www.mygreatlearning.com/blog/polymorphism-in-cpp/

# Source 2

https://www.tutorialspoint.com/cplusplus/cpp_polymorphism.htm


# build.sh

```
#!/bin/bash

g++ -g -O3 -std=c++11 -Wall -Wextra -pedantic -Wformat=2 -op -Weffc++ -Wold-style-cast -Woverloaded-virtual -Wredundant-decls -Wshadow -Wsign-conversion -Wsign-promo -Wstrict-overflow=5 -Wswitch-default -Wfloat-equal -Wmissing-format-attribute -Wunreachable-code -fmerge-all-constants main.cpp && ./p
```

# build2.sh

```
#!/bin/bash

g++ -std=c++11 -Wall -Wextra -ggdb -o main main.cpp
```

# objdump활용법

ARM Aseembly(M1 Pro MacBook test)

```
$ gcc -std=c++11 -Wall -Wextra -ggdb -c main.cpp

$ objdump --disassemble -S -C main.o

$ objdump --disassemble -S -C main.o | grep "main" -A 15 -B 15

main.o:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000000000000 <ltmp0>:
; int main(void) {
       0: ff c3 00 d1  	sub	sp, sp, #48
       4: fd 7b 02 a9  	stp	x29, x30, [sp, #32]
       8: fd 83 00 91  	add	x29, sp, #32
       c: 08 00 80 52  	mov	w8, #0
      10: a8 43 1f b8  	stur	w8, [x29, #-12]
      14: bf c3 1f b8  	stur	wzr, [x29, #-4]
      18: a0 17 00 d1  	sub	x0, x29, #5
      1c: e0 07 00 f9  	str	x0, [sp, #8]
      20: 01 10 80 52  	mov	w1, #128
      24: e2 01 80 52  	mov	w2, #15
;   std::cout << obj.ADD(128, 15) << std::endl; // first method is called
      28: 00 00 00 94  	bl	0x28 <ltmp0+0x28>
      2c: e1 03 00 aa  	mov	x1, x0
      30: 00 00 00 90  	adrp	x0, 0x0 <ltmp0+0x30>
      34: 00 00 40 f9  	ldr	x0, [x0]
```
