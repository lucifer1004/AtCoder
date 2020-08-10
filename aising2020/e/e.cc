#include <cstdio>
#include <iostream>
#include <queue>
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
    int n;
    read(n);
    vector<vector<int>> red(n + 1), blue(n + 1);
    vector<int> L(n), R(n);
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> pq;
    ll ans = 0;
    for (int i = 0; i < n; ++i) {
      int k, l, r;
      read(k), read(l), read(r);
      if (l >= r)
        red[k].emplace_back(i);
      else
        blue[k].emplace_back(i);
      L[i] = l;
      R[i] = r;
      ans += min(l, r);
    }
    for (int i = 1; i <= n; ++i) {
      for (int j : red[i])
        pq.push({L[j] - R[j], j});
      while (pq.size() > i)
        pq.pop();
    }
    while (!pq.empty()) {
      ans += pq.top().first;
      pq.pop();
    }

    for (int i = n; i >= 1; --i) {
      for (int j : blue[i])
        pq.push({R[j] - L[j], j});
      while (pq.size() > n - i)
        pq.pop();
    }
    while (!pq.empty()) {
      ans += pq.top().first;
      pq.pop();
    }
    cout << ans << endl;
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  read(t);
  while (t--) {
    Solution solution = Solution();
    solution.solve();
  }
}