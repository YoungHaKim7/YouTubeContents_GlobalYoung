// https://www.tutorialspoint.com/cplusplus/cpp_polymorphism.htm
#include <iostream>

class Shape {
protected:
  int width, height;

public:
  Shape(int a = 0, int b = 0) {
    width = a;
    height = b;
  }
  virtual int area() {
    std::cout << "Parent class area :" << width * height << std::endl;
    return width * height;
  }
};
class Rectangle : public Shape {
public:
  Rectangle(int a = 0, int b = 0) : Shape(a, b) {}

  int area() {
    std::cout << "Rectangle class area :" << width * height << std::endl;
    return (width * height);
  }
};

class Triangle : public Shape {
public:
  Triangle(int a = 0, int b = 0) : Shape(a, b) {}

  int area() {
    std::cout << "Triangle class area :" << (width * height) / 2 << std::endl;
    return (width * height / 2);
  }
};

// Main function for the program
int main() {
  Shape *shape;
  Rectangle rec(10, 7);
  Triangle tri(10, 5);

  // store the address of Rectangle
  shape = &rec;

  // call rectangle area.
  shape->area();

  // store the address of Triangle
  shape = &tri;

  // call triangle area.
  shape->area();

  return 0;
}
