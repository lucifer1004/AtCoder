#include <iostream>

using namespace std;
int main() {
  char c;
  cin >> c;
  int delta = c - 'a';
  if (delta >= 0 && delta <= 25)
    cout << "a";
  else
    cout << "A";
}
