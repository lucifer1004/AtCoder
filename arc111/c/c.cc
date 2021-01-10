#include <iostream>
#include <vector>

using namespace std;
int main() {
  int n;
  cin >> n;
  vector<int> a(n), b(n), p(n), q(n);
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  for (int i = 0; i < n; ++i)
    cin >> b[i];
  for (int i = 0; i < n; ++i) {
    cin >> p[i];
    p[i]--;
    q[p[i]] = i;
  }
  vector<pair<int, int>> ops;

  auto swap_baggage = [&](int i, int j) {
    ops.emplace_back(i + 1, j + 1);
    q[p[i]] = j, q[p[j]] = i;
    swap(p[i], p[j]);
  };

  for (int i = 0; i < n; ++i) {
    if (p[i] == i)
      continue;
    int idx = p[i];
    int hi = a[i];
    int hi_num = i;
    while (idx != i) {
      if (a[idx] > hi) {
        hi = a[idx];
        hi_num = idx;
      }
      idx = p[idx];
    }
    idx = hi_num;
    while (p[idx] != idx) {
      int nxt = p[idx];
      if (a[idx] <= b[p[idx]] || a[nxt] <= b[p[nxt]]) {
        cout << -1 << endl;
        return 0;
      }
      swap_baggage(idx, nxt);
    }
  }

  cout << ops.size() << endl;
  for (auto [i, j] : ops)
    cout << i << " " << j << endl;
}