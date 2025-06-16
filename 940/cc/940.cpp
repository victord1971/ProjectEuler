
#include <iostream>
#include <vector>

int main() {
  vector <long> numbers = {1, 2, 3, 4, 5};
  numbers.push_back(6);

  for (int number : numbers)
  {
    cout << number << " ";
  }
  cout << endl;

  return 0;
}




