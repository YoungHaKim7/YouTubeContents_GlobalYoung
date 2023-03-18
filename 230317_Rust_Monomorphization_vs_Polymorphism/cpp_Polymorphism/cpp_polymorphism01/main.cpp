#include <iostream>

class Addition {
public:
  int ADD(int X, int Y) // Function with parameter
  {
    return X + Y; // this function is performing addition of  two Integer value
  }
  int ADD() { // Function with same name but without parameter
    std::string a = "HELLO";
    std::string b = "SAM"; // in this function concatenation is performed
    std::string c = a + b;
    std::cout << c << std::endl;
  }
};
int main(void) {
  Addition obj;                               // Object is created
  std::cout << obj.ADD(128, 15) << std::endl; // first method is called
  obj.ADD();                                  // second method is called
  return 0;
}
