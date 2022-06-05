#include <atcoder/segtree>
#include <cstdio>
#include <iostream>
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

int e() { return 0; }

class Solution {
 public:
  void solve() {
    int n, q;
    read(n), read(q);
    vector<int> a(n + 1), b(n + 1);
    for (int i = 1; i <= n; ++i) read(a[i]);
    for (int i = 1; i <= n; ++i) read(b[i]);
    atcoder::segtree<int, __gcd, e> sa(n + 1), sb(n + 1);
    for (int i = 1; i <= n; ++i) {
      sa.set(i, a[i] - a[i - 1]);
      sb.set(i, b[i] - b[i - 1]);
    }

    while (q--) {
      int h1, h2, w1, w2;
      read(h1), read(h2), read(w1), read(w2);
      sa.set(h1, a[h1] + b[w1]);
      sb.set(w1, a[h1] + b[w1]);
      int ans = abs(__gcd(sa.prod(h1, h2 + 1), sb.prod(w1, w2 + 1)));
      printf("%d\n", ans);
      sa.set(h1, a[h1] - a[h1 - 1]);
      sb.set(w1, b[w1] - b[w1 - 1]);
    }
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}