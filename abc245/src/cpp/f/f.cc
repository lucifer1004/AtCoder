#include <atcoder/scc>
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
    int n, m;
    read(n), read(m);

    atcoder::scc_graph g(n);
    vector<pair<int, int>> edges;
    for (int i = 0; i < m; ++i) {
      int u, v;
      read(u), read(v);
      g.add_edge(u - 1, v - 1);
      edges.emplace_back(u - 1, v - 1);
    }

    vector<int> belong(n);
    auto scc = g.scc();
    int k = scc.size();
    for (int i = 0; i < k; ++i) {
      for (int u : scc[i]) belong[u] = i;
    }

    vector<unordered_set<int>> adj(k);
    for (auto [u, v] : edges)
      if (belong[u] != belong[v]) adj[belong[v]].insert(belong[u]);

    queue<int> q;
    vector<bool> vis(k);
    for (int i = 0; i < k; ++i)
      if (scc[i].size() >= 2) q.push(i), vis[i] = true;

    int ans = 0;

    while (!q.empty()) {
      int u = q.front();
      q.pop();
      ans += scc[u].size();
      for (int v : adj[u]) {
        if (!vis[v]) {
          vis[v] = true;
          q.push(v);
        }
      }
    }

    printf("%d\n", ans);
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}