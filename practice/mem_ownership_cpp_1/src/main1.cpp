// gcc src/main1.cpp -lstdc++ -o cpp1 && ./cpp1
#include <vector>
#include <iostream>
int main()
{
  std::vector< int > vec = { 1, 2 };
  auto& a = vec[ 0 ];
  vec.push_back( 3 );
  std::cout << a;
  // dangled reference. fault state.
}
