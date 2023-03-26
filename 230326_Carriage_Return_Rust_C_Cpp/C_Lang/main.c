#include <stdio.h>
#include <unistd.h>

int main() {
  char chars[] = {'-', '\\', '|', '/'};
  unsigned int i;

  for (i = 0;; ++i) {
    printf("%c\r", chars[i % sizeof(chars)]);
    fflush(stdout);
    usleep(200000);
  }

  return 0;
}
