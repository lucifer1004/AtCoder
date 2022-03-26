#include <algorithm>
#include <cstdio>
#include <iostream>
#include <set>
#include <vector>

using namespace std;

template <typename T>
void read(T &x) {
  x = 0;
  char c = getchar();
  T sig = 1;
  for (; !isdigit(c); c = getchar())
    if (c == '-') sig = -1;
  for (; isdigit(c); c = getchar()) x = (x << 3) + (x << 1) + c - '0';
  x *= sig;
}

class Solution {
 public:
  void solve() {
    int n, m;
    read(n), read(m);
    vector<int> a(n), b(n), c(m), d(m);
    for (int i = 0; i < n; ++i) read(a[i]);
    for (int i = 0; i < n; ++i) read(b[i]);
    for (int i = 0; i < m; ++i) read(c[i]);
    for (int i = 0; i < m; ++i) read(d[i]);
    vector<int> order(n);
    for (int i = 0; i < n; ++i) order[i] = i;
    sort(order.begin(), order.end(), [&](int i, int j) {
      return a[i] > a[j] || (a[i] == a[j] && b[i] > b[j]);
    });

    vector<int> porder(m);
    for (int i = 0; i < m; ++i) porder[i] = i;
    sort(porder.begin(), porder.end(), [&](int i, int j) {
      return c[i] > c[j] || (c[i] == c[j] && d[i] > d[j]);
    });
    multiset<int> h;
    int ptr = -1;

    for (int i = 0; i < n; ++i) {
      while (ptr + 1 < m && c[porder[ptr + 1]] >= a[order[i]]) {
        h.insert(d[porder[++ptr]]);
      }

      auto it = h.lower_bound(b[order[i]]);
      if (it == h.end()) {
        printf("No");
        return;
      }
      h.erase(it);
    }

    printf("Yes");
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}