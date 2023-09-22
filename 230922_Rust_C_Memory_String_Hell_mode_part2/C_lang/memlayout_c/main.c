/**
Output in a typical compiler / architecture will be:
sizeof(a) = 24
  offset(a.f16) = 0
  offset(a.f32) = 4
  offset(a.f08) = 8
  offset(a.f64) = 16
sizeof(b) = 16
  offset(b.f64) = 0
  offset(b.f32) = 8
  offset(b.f16) = 12
  offset(b.f08) = 14
*/
#include <stdint.h>
#include <stdio.h>

typedef struct {
  int16_t  f16;
  int32_t  f32;
  int8_t   f08;
  int64_t  f64;
} A;

typedef struct {
  int64_t  f64;
  int32_t  f32;
  int16_t  f16;
  int8_t   f08;
} B;

#define OFFSET(x,f) ((int8_t*) &(x.f)) - (int8_t*) &(x)

int main(int argc, char** argv) {
  A a;
  B b;

  printf("sizeof(a) = %d\n", sizeof(a));
  printf("  offset(a.f16) = %d\n", OFFSET(a,f16));
  printf("  offset(a.f32) = %d\n", OFFSET(a,f32));
  printf("  offset(a.f08) = %d\n", OFFSET(a,f08));
  printf("  offset(a.f64) = %d\n", OFFSET(a,f64));

  printf("sizeof(b) = %d\n", sizeof(b));
  printf("  offset(b.f64) = %d\n", OFFSET(b,f64));
  printf("  offset(b.f32) = %d\n", OFFSET(b,f32));
  printf("  offset(b.f16) = %d\n", OFFSET(b,f16));
  printf("  offset(b.f08) = %d\n", OFFSET(b,f08));

  return 0;
}