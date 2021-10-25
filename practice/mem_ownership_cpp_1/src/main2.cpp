// gcc src/main2.cpp -lstdc++ -o cpp2 && ./cpp2
#include <vector>
#include <iostream>
int main()
{
  std::vector< int > vec = { 1, 2 };
  auto& a = vec[ 0 ];
  vec.push_back( 3 );
  std::cout << "a @ " << &a << " : " << a << std::endl;
  std::cout << "vec[ 0 ] @ " << &vec[ 0 ] << " : " << vec[ 0 ] << std::endl;
  // dangled reference. fault state.
}
