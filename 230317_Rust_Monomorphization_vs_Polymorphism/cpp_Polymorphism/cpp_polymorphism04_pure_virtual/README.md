# Result

```
$ ./build.sh

main.cpp:4:7: warning: 'Shape' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
class Shape {
      ^
main.cpp:20:7: warning: 'Rectangle' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
class Rectangle: public Shape {
      ^
main.cpp:30:7: warning: 'Triangle' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
class Triangle: public Shape {
      ^
3 warnings generated.

Rectangle class area :70
Triangle class area :25

```
