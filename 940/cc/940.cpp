
#include <iostream>
#include <vector>

int main() {
  std::vector<int> numbers = {1, 2, 3, 4, 5};
  numbers.push_back(6);

  for (int number : numbers) {
    std::cout << number << " ";
  }
  std::cout << std::endl;

  return 0;
}




