#include <iostream>

using namespace std;
int main() {
  int n = 0;
  string s, t;
  cin >> s >> t;
  for (int i = 0; i < s.size(); ++i)
    n += s[i] != t[i];
  cout << n;
}
