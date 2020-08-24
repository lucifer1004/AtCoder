#include <iostream>

using namespace std;
int main() {
  string s;
  cin >> s;
  s += "$";
  int ans = 0, cnt = 0;
  char c = '#';
  for (char i : s) {
    if (i == c)
      cnt++;
    else {
      if (c == 'R')
        ans = max(ans, cnt);
      c = i;
      cnt = 1;
    }
  }
  cout << ans;
}