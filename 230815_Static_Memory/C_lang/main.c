#include <stdio.h>

void function_with_static_variable(void)
{
  static int COUNT01;

  COUNT01 = COUNT01 + 1;
  printf("%d\n", COUNT01);
}

void main(void)
{
  function_with_static_variable();
  function_with_static_variable();
  function_with_static_variable();
  function_with_static_variable();
}
