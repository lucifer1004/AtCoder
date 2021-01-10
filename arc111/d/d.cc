#include <iostream>
#include <vector>

using namespace std;
int ans[5000]{}, c[105]{}, n, m;
int depth[105]{};
vector<pair<int, int>> adj[105]{};

void dfs(int u, int p) {
  for (auto [v, e] : adj[u]) {
    if (ans[abs(e)] != 0)
      continue;
    ans[abs(e)] = e > 0 ? 1 : -1;
    if (!depth[v]) {
      depth[v] = depth[u] + 1;
      dfs(v, u);
    }
  }
}

int main() {
  cin >> n >> m;
  vector<pair<int, int>> edges{{}};
  for (int i = 1; i <= m; ++i) {
    int u, v;
    cin >> u >> v;
    edges.emplace_back(u, v);
  }
  for (int i = 1; i <= n; ++i)
    cin >> c[i];
  for (int i = 1; i <= m; ++i) {
    int u = edges[i].first, v = edges[i].second;
    if (c[u] > c[v])
      ans[i] = 1;
    else if (c[u] < c[v])
      ans[i] = -1;
    else {
      adj[u].emplace_back(v, i);
      adj[v].emplace_back(u, -i);
    }
  }

  for (int i = 1; i <= n; ++i)
    if (!depth[i])
      dfs(i, 0);

  for (int i = 1; i <= m; ++i)
    cout << (ans[i] == 1 ? "->" : "<-") << endl;
}