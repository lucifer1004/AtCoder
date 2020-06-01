#include <cmath>
#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n;
  cin >> n;
  string s;
  cin >> s;
  vector<int> b(n - 1);
  bool has_one = false;
  for (int i = 0; i < n - 1; ++i) {
    b[i] = abs(s[i + 1] - s[i]);
    if (b[i] == 1)
      has_one = true;
  }
  int factor = 1;
  if (!has_one) {
    factor = 2;
    for (int i = 0; i < n - 1; ++i)
      b[i] >>= 1;
  }
  int sum = 0;
  for (int i = 0; i < n - 1; ++i) {
    if (((n - 2) & i) == i)
      sum = (sum + b[i]) % 2;
  }
  cout << sum * factor;
}
