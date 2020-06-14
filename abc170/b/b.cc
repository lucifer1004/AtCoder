#include <iostream>

using namespace std;
int main() {
  int x, y;
  cin >> x >> y;
  bool ok = false;
  for (int i = 0; i <= x; ++i)
    if (i * 2 + (x - i) * 4 == y) {
      cout << "Yes";
      exit(0);
    }
  cout << "No";
}