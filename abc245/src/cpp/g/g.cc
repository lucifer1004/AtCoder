#include <cstdio>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;
using ll = long long;
const ll INF = 1e18;

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

int a[100005];

vector<vector<ll>> dijkstra(vector<vector<pair<int, int>>> &adj, int s) {
  int n = adj.size();
  priority_queue<tuple<ll, int, int>, vector<tuple<ll, int, int>>, greater<>>
      pq;
  vector<vector<ll>> dis(n, vector<ll>(4, INF));
  dis[s][0] = 0, dis[s][1] = s;
  pq.emplace(0, s, s);

  while (!pq.empty()) {
    auto [d, u, from] = pq.top();
    pq.pop();
    if ((d == dis[u][0] && from == dis[u][1]) ||
        (d == dis[u][2] && from == dis[u][3])) {
      for (auto [v, w] : adj[u]) {
        if (d + w < dis[v][0]) {
          if (dis[v][0] < INF) {
            if (a[dis[v][1]] != a[from]) {
              dis[v][2] = dis[v][0];
              dis[v][3] = dis[v][1];
            }
          }
          dis[v][0] = d + w;
          dis[v][1] = from == s ? v : from;
          pq.emplace(dis[v][0], v, dis[v][1]);
        } else if (d + w < dis[v][2] && a[from] != a[dis[v][1]]) {
          dis[v][2] = d + w;
          dis[v][3] = from == -1 ? u : from;
          pq.emplace(dis[v][2], v, dis[v][3]);
        }
      }
    }
  }

  return dis;
}

class Solution {
 public:
  void solve() {
    int n, m, k, l;
    read(n), read(m), read(k), read(l);
    vector<int> popular(n);
    vector<vector<pair<int, int>>> adj(n + 1);
    for (int i = 0; i < n; ++i) {
      read(a[i]);
      a[i]--;
    }
    a[n] = -1;
    for (int i = 0; i < l; ++i) {
      int bi;
      read(bi);
      bi--;
      popular[bi] = true;
      adj[n].emplace_back(bi, 0);
    }
    for (int i = 0; i < m; ++i) {
      int u, v, c;
      read(u), read(v), read(c);
      u--, v--;
      adj[u].emplace_back(v, c);
      adj[v].emplace_back(u, c);
    }

    auto dis1 = dijkstra(adj, n);
    vector<vector<pair<int, int>>> adj2(adj);
    adj2[n].clear();
    for (int i = 0; i < n; ++i) {
      if (popular[i] && a[i] != 0) adj2[n].emplace_back(i, 0);
    }
    auto dis2 = dijkstra(adj2, n);

    vector<ll> ans(n);
    for (int i = 0; i < n; ++i) {
      int from = -1;
      if (a[i] == 0) {
        ans[i] = dis2[i][0];
        from = dis2[i][1];
      } else {
        if (dis1[i][1] != INF && a[dis1[i][1]] != a[i])
          ans[i] = dis1[i][0], from = dis1[i][1];
        else
          ans[i] = dis1[i][2], from = dis1[i][3];
      }
      if (ans[i] == INF) ans[i] = -1;
    }
    for (ll &ansi : ans) printf("%lld ", ansi);
    printf("\n");
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  Solution solution = Solution();
  solution.solve();
}