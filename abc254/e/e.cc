#include <cstdio>
#include <iostream>
#include <queue>
#include <unordered_set>
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
    int n, m, q;
    read(n), read(m);
    vector<vector<int>> adj(n + 1);
    for (int i = 0; i < m; ++i) {
      int u, v;
      read(u), read(v);
      adj[u].push_back(v);
      adj[v].push_back(u);
    }

    read(q);
    while (q--) {
      int x, k;
      read(x), read(k);
      unordered_set<int> s;
      int ans = 0;
      queue<pair<int, int>> q;
      q.emplace(x, 0);
      s.insert(x);
      while (!q.empty()) {
        auto [u, d] = q.front();
        q.pop();
        ans += u;
        if (d == k) {
          continue;
        }
        for (auto v : adj[u]) {
          if (s.count(v)) continue;
          s.insert(v);
          q.emplace(v, d + 1);
        }
      }
      printf("%d\n", ans);
    }
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}