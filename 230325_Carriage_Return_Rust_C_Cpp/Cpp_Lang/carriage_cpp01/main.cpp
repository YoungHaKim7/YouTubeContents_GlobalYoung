
#include <iostream>
#include <unistd.h>

int main() {
  char chars[] = {'-', '\\', '|', '/'};
  unsigned int i;

  for (i = 0;; ++i) {
      std::cout << chars[i % sizeof(chars)] << '\r';
    fflush(stdout);
    usleep(200000);
  }

  return 0;
}
