#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n, k;
  cin >> n >> k;
  vector<int> a(n);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  for (int i = 0; i < k; ++i) {
    vector<int> b(n + 1);
    for (int j = 0; j < n; ++j) {
      int l = max(1, j + 1 - a[j]);
      int r = min(n, j + 1 + a[j]);
      b[l]++;
      if (r + 1 <= n)
        b[r + 1]--;
    }
    bool end = true;
    for (int j = 0; j < n; ++j) {
      b[j + 1] += b[j];
      a[j] = b[j + 1];
      if (a[j] != n)
        end = false;
    }
    if (end)
      break;
  }
  for (int i : a)
    cout << i << " ";
}
