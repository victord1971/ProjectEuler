
#include <iostream>
#include <vector>

int main() {
  std::vector <long long int> numbers = {9999999999, 2, 3, 4, 5};
  numbers.push_back(32);

  for (int number : numbers)
  {
    std::cout << number << " ";
  }
  std::cout << std::endl;

  return 0;
}




