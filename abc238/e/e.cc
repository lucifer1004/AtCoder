#include <cstdio>
#include <iostream>
#include <queue>
#include <vector>

using namespace std;

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
    int n, q;
    read(n), read(q);
    vector<vector<int>> adj(n + 1);

    for (int i = 0; i < q; ++i) {
      int l, r;
      read(l), read(r);
      adj[l - 1].emplace_back(r);
      adj[r].emplace_back(l - 1);
    }

    vector<bool> vis(n + 1);
    vis[0] = true;
    queue<int> que;
    que.emplace(0);

    while (!que.empty()) {
      int u = que.front();
      que.pop();
      for (int v : adj[u]) {
        if (!vis[v]) {
          vis[v] = true;
          que.emplace(v);
        }
      }
    }

    if (vis[n]) {
      cout << "Yes" << endl;
    } else {
      cout << "No" << endl;
    }
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}