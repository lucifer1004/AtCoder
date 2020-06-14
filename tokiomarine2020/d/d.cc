#include <cstdio>
#include <cstring>
#include <iostream>
#include <vector>
#define MAXN 515
#define MAXL 100005

using namespace std;
int dp[MAXN][MAXL];
int main() {
  int n;
  scanf("%d", &n);
  vector<int> v(n + 1), w(n + 1);
  for (int i = 1; i <= n; ++i)
    scanf("%d%d", &v[i], &w[i]);
  memset(dp, 0, sizeof(dp));
  for (int i = 1; i < min(n + 1, 1 << 9); ++i) {
    int p = i / 2;
    for (int j = 1; j < MAXL; ++j)
      dp[i][j] = max(dp[p][j], j >= w[i] ? dp[p][j - w[i]] + v[i] : 0);
  }
  int q;
  scanf("%d", &q);
  string ans;
  for (int i = 0; i < q; ++i) {
    int vi, l;
    scanf("%d%d", &vi, &l);
    if (vi < (1 << 9)) {
      ans += to_string(dp[vi][l]) + "\n";
      continue;
    }
    vector<int> path;
    while (vi >= (1 << 9)) {
      path.emplace_back(vi);
      vi >>= 1;
    }
    int best = 0;
    int m = path.size();
    for (int j = 0; j < (1 << m); ++j) {
      int vs = 0, ws = 0;
      for (int k = 0; k < m; ++k)
        if (j & (1 << k)) {
          vs += v[path[k]];
          ws += w[path[k]];
        }
      if (ws > l)
        continue;
      best = max(best, vs + dp[vi][l - ws]);
    }
    ans += to_string(best) + "\n";
  }
  printf("%s", ans.c_str());
}
