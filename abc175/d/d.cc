#include <climits>
#include <cstdio>
#include <iostream>
#include <vector>

using namespace std;
typedef long long ll;

template <typename T> void read(T &x) {
  x = 0;
  char c = getchar();
  T sig = 1;
  for (; !isdigit(c); c = getchar())
    if (c == '-')
      sig = -1;
  for (; isdigit(c); c = getchar())
    x = (x << 3) + (x << 1) + c - '0';
  x *= sig;
}

class Solution {
public:
  void solve() {
    int n, k;
    read(n), read(k);
    vector<int> p(n), c(n);
    for (int i = 0; i < n; ++i)
      read(p[i]);
    for (int i = 0; i < n; ++i)
      read(c[i]);
    ll ans = LLONG_MIN;
    for (int i = 0; i < n; ++i) {
      vector<bool> vis(n);
      int idx = i;
      vector<ll> sum = {0}, hi = {LLONG_MIN};
      while (!vis[p[idx] - 1]) {
        idx = p[idx] - 1;
        vis[idx] = true;
        sum.emplace_back(sum.back() + c[idx]);
        hi.emplace_back(max(hi.back(), sum.back()));
      }
      int m = sum.size() - 1;
      int f = k / m, rem = k % m;
      ll result;
      if (f > 0)
        result = max(hi[m], max(sum[m] * f + (rem == 0 ? 0 : hi[rem]),
                                sum[m] * (f - 1) + hi[m]));
      else
        result = hi[rem];
      ans = max(ans, result);
    }
    printf("%lld", ans);
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}