#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n;
  cin >> n;
  vector<int> a(n);
  int ans = 0;
  vector<bool> f(1000001, true);
  vector<vector<int>> v(1000001);
  for (int i = 0; i < n; ++i) {
    cin >> a[i];
    v[a[i]].emplace_back(i);
  }
  for (int i = 1; i <= 1000000; ++i) {
    if (v[i].empty() || !f[i])
      continue;
    if (v[i].size() == 1)
      ans++;
    for (int j = 2 * i; j <= 1000000; j += i)
      f[j] = false;
  }
  cout << ans;
}